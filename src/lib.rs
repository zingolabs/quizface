pub mod utils;
use crate::logging::create_log_dirs;
use crate::logging::log_masterhelp_output;
use std::collections::HashMap;
use std::path::Path;
use utils::logging;

pub fn ingest_commands() -> Vec<String> {
    create_log_dirs();
    let cli_help_output = get_command_help("");
    check_success(&cli_help_output.status);

    let raw_help = std::string::String::from_utf8(cli_help_output.stdout)
        .expect("Invalid, not UTF-8. Error!");
    log_masterhelp_output(&raw_help);

    let help_lines_iter = raw_help.lines();
    let mut help_lines = Vec::new();
    for li in help_lines_iter {
        if li != "" && !li.starts_with("=") {
            help_lines.push(li);
        }
    }

    // currently, with zcashd from version 4.1.0, 132 lines.
    // this matches 151 (`zcash-cli | wc -l`) - 19 (manual count of
    // empty lines or 'category' lines that begin with "=")

    let mut commands_str = Vec::new();
    for line in help_lines {
        let mut temp_iter = line.split_ascii_whitespace();
        match temp_iter.next() {
            Some(x) => commands_str.push(x),
            None => panic!("error during command parsing"),
        }
    }

    let mut commands = Vec::new();
    for c in commands_str {
        commands.push(c.to_string());
    }
    commands
}

pub fn get_command_help(cmd: &str) -> std::process::Output {
    let command_help = std::process::Command::new(Path::new("zcash-cli"))
        .arg("help")
        .arg(&cmd)
        .output()
        .expect("failed to execute command help");
    command_help
}

pub fn check_success(output: &std::process::ExitStatus) {
    // simple boolean that output succeeded by spawning
    // and monitoring child process, if false: panic
    assert!(output.success());
    // then match output exit code
    match output.code() {
        Some(0) => (),
        Some(_) => panic!("exit code not 0"),
        None => panic!("error! no exit code"),
    }
}

fn extract_result_section(raw_command_help: &str) -> String {
    raw_command_help.split("Result:\n").collect::<Vec<&str>>()[1]
        .split("Examples:\n")
        .collect::<Vec<&str>>()[0]
        .trim()
        .to_string()
}

use serde_json::{json, map::Map, Value};

fn clean_observed(raw_observed: String) -> Vec<String> {
    let mut ident_labels = raw_observed
        .trim_end()
        .lines()
        .map(|x| x.to_string())
        .collect::<Vec<String>>();
    match ident_labels.remove(0) {
        empty if empty.is_empty() => (),
        description if description.contains("(object)") => (),
        reject if reject == "..." => (), // Special case
        catchall @ _ => {
            dbg!(catchall);
            panic!("This was unexpected ");
        }
    }
    ident_labels
}
fn bind_idents_labels(raw_observed: String) -> Map<String, Value> {
    clean_observed(raw_observed)
        .iter()
        .map(|ident_rawlabel| label_identifier(ident_rawlabel.to_string()))
        .map(|(a, b)| (a.to_string(), json!(b.to_string())))
        .collect::<Map<String, Value>>()
}
fn list_idents() -> Vec<Value> {
    vec![]
}
fn bind_ident() -> String {
    String::from("")
}

pub fn parse_raw_output(raw_command_help: &str) -> Value {
    let mut data = extract_result_section(raw_command_help);
    let last_observed = data.remove(0);
    annotate_result_section(last_observed, &mut data.chars())
}

fn annotate_result_section(
    last_observed: char,
    mut incoming_data: &mut std::str::Chars,
) -> serde_json::Value {
    match last_observed {
        '{' => {
            let mut ident_label_bindings = Map::new();
            let mut observed = String::from(""); // Each call gets its own!!
            loop {
                match incoming_data.next().unwrap() {
                    '}' => {
                        ident_label_bindings =
                            bind_idents_labels(observed.clone());
                        break;
                    }
                    lastobs if lastobs == '[' || lastobs == '{' => {
                        let inner =
                            serde_json::to_string(&annotate_result_section(
                                lastobs,
                                &mut incoming_data,
                            ))
                            .expect("couldn't get string from json");
                        observed.push_str(&inner);
                    }
                    // TODO: Handle unbalanced braces
                    x if x.is_ascii() => observed.push(x),
                    _ => panic!(),
                }
            }
            return Value::Object(ident_label_bindings);
        }
        _ => unimplemented!(),
    }
}
fn label_identifier(ident_with_metadata: String) -> (String, String) {
    let mut ident_and_metadata = ident_with_metadata
        .trim()
        .splitn(2, ':')
        .collect::<Vec<&str>>();
    let ident = ident_and_metadata[0].trim_matches('"');
    let meta_data = ident_and_metadata[1].trim();
    let mut annotation = String::from("");
    if meta_data.starts_with('{') {
        annotation = meta_data.to_string();
    } else {
        let raw_label: &str = dbg!(&meta_data)
            .split(|c| c == '(' || c == ')')
            .collect::<Vec<&str>>()[1];

        annotation = make_label(raw_label);
    }
    (ident.to_string(), annotation)
}

fn make_label(raw_label: &str) -> String {
    let mut annotation = String::new();

    if raw_label.starts_with("numeric") {
        annotation.push_str("Decimal");
    } else if raw_label.starts_with("string") {
        annotation.push_str("String");
    } else if raw_label.starts_with("boolean") {
        annotation.push_str("bool");
    } else {
        panic!("annotation should have a value at this point.");
    }

    if raw_label.contains(", optional") {
        return format!("Option<{}>", annotation);
    }
    annotation
}

#[cfg(test)]
mod unit {
    use super::*;
    use crate::utils::test;

    #[test]
    fn label_identifier_with_expected_input_valid() {
        let raw_version =
            r#""version": xxxxx,           (numeric) the server version"#;
        let valid_annotation = ("version".to_string(), "Decimal".to_string());
        assert_eq!(valid_annotation, label_identifier(raw_version.to_string()));
    }
    #[test]
    #[ignore = "in development"]
    fn parse_raw_output_expected_input_valid() {
        let valid_help_in = parse_raw_output(test::HELP_GETINFO);
        assert_eq!(valid_help_in, test::valid_getinfo_annotation());
    }
    #[test]
    #[ignore = "in development"]
    fn parse_raw_output_early_lbracket_input() {
        let valid_help_in = parse_raw_output(test::LBRACKETY_HELP_GETINFO);
        assert_eq!(valid_help_in, test::valid_getinfo_annotation());
    }
    #[test]
    #[ignore = "in development"]
    fn parse_raw_output_early_rbracket_input() {
        let valid_help_in = parse_raw_output(test::RBRACKETY_HELP_GETINFO);
        assert_eq!(valid_help_in, test::valid_getinfo_annotation());
    }
    #[test]
    #[ignore = "in development"]
    fn parse_raw_output_early_extrabrackets_input() {
        let valid_help_in = parse_raw_output(test::EXTRABRACKETS1_HELP_GETINFO);
        assert_eq!(valid_help_in, test::valid_getinfo_annotation());
    }
    #[test]
    #[ignore = "in development"]
    fn parse_raw_output_extrabrackets_within_input_lines() {
        let valid_help_in = parse_raw_output(test::EXTRABRACKETS3_HELP_GETINFO);
        assert_eq!(valid_help_in, test::valid_getinfo_annotation());
    }
    #[test]
    #[ignore = "in development"]
    fn parse_raw_output_late_extrabrackets_input() {
        let valid_help_in = parse_raw_output(test::EXTRABRACKETS2_HELP_GETINFO);
        assert_eq!(valid_help_in, test::valid_getinfo_annotation());
    }
    #[test]
    #[should_panic]
    #[ignore = "in development"]
    fn parse_raw_output_more_than_one_set_of_brackets_input() {
        let valid_help_in =
            parse_raw_output(test::MORE_BRACKET_PAIRS_HELP_GETINFO);
        assert_eq!(valid_help_in, test::valid_getinfo_annotation());
    }
    #[test]
    #[should_panic]
    #[ignore = "in development"]
    fn parse_raw_output_two_starting_brackets_input() {
        let valid_help_in =
            parse_raw_output(test::EXTRA_START_BRACKET_HELP_GETINFO);
        assert_eq!(valid_help_in, test::valid_getinfo_annotation());
    }
    #[test]
    #[should_panic]
    #[ignore = "in development"]
    fn parse_raw_output_two_ending_brackets_input() {
        let valid_help_in =
            parse_raw_output(test::EXTRA_END_BRACKET_HELP_GETINFO);
        assert_eq!(valid_help_in, test::valid_getinfo_annotation());
    }
    #[test]
    #[should_panic]
    #[ignore = "in development"]
    fn parse_raw_output_no_results_input() {
        let valid_help_in = parse_raw_output(test::NO_RESULT_HELP_GETINFO);
        assert_eq!(valid_help_in, test::valid_getinfo_annotation());
    }
    #[test]
    #[should_panic]
    #[ignore = "in development"]
    fn parse_raw_output_no_end_bracket_input() {
        let valid_help_in = parse_raw_output(test::NO_END_BRACKET_HELP_GETINFO);
        assert_eq!(valid_help_in, test::valid_getinfo_annotation());
    }
    #[test]
    #[should_panic]
    #[ignore = "in development"]
    fn parse_raw_output_no_start_bracket_input() {
        let valid_help_in =
            parse_raw_output(test::NO_START_BRACKET_HELP_GETINFO);
        assert_eq!(valid_help_in, test::valid_getinfo_annotation());
    }
    #[test]
    fn annotate_result_section_from_getinfo_expected() {
        let expected_testdata_annotated = test::valid_getinfo_annotation();
        let mut section_data = extract_result_section(test::HELP_GETINFO);
        let last_observed = section_data.remove(0);
        let annotated =
            annotate_result_section(last_observed, &mut section_data.chars());
        assert_eq!(annotated, expected_testdata_annotated);
    }
    #[test]
    fn annotate_result_section_enforce_as_input() {
        use std::collections::HashMap;
        let testmap = json!(test::INTERMEDIATE_REPR_ENFORCE
            .iter()
            .map(|(a, b)| (a.to_string(), json!(b.to_string())))
            .collect::<HashMap<String, Value>>());
        assert_eq!(
            testmap,
            annotate_result_section('{', &mut test::ENFORCE_EXTRACTED.chars(),)
        );
    }
    #[test]
    fn annotate_result_section_nested_obj_extracted_from_softfork() {
        let mut expected_nested = test::SIMPLIFIED_SOFTFORK;
        let mut obs_nested = expected_nested.chars();
        let last_observed = obs_nested.nth(0).unwrap();
        let annotated = annotate_result_section(last_observed, &mut obs_nested);
        //assert_eq!(annotated,);
    }
}
