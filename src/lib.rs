pub mod utils;
use crate::logging::create_log_dirs;
use crate::logging::log_masterhelp_output;
use crate::utils::scrubbing::scrub_arguments;
use crate::utils::scrubbing::scrub_response;
use serde_json::{json, map::Map, Value};
use std::collections::HashMap;
use std::path::Path;
use utils::logging;

pub fn ingest_commands() -> Vec<String> {
    create_log_dirs();
    let cli_help_output = get_command_help("");
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

pub fn get_command_help(rpc_name: &str) -> std::process::Output {
    let command_help = std::process::Command::new(Path::new("zcash-cli"))
        .arg("help")
        .arg(&rpc_name)
        .output()
        .expect("failed to execute command help");
    command_help
}

fn record_interpretation(
    rpc_name: String,
    full_response_interpretation: String,
    argument_interpretation: String,
) {
    let raw_response_location = &format!(
        "./output/{}/{}_response.json",
        utils::logging::create_version_name(),
        rpc_name
    );
    let response_location = std::path::Path::new(raw_response_location);
    std::fs::create_dir_all(response_location.parent().unwrap()).unwrap();
    use std::io::Write as _;
    let mut response_file = std::fs::File::create(response_location)
        .expect("Couldn't create append interface to output file.");
    response_file
        .write_all(full_response_interpretation.as_bytes())
        .unwrap();
    let raw_arguments_location = &format!(
        "./output/{}/{}_arguments.json",
        utils::logging::create_version_name(),
        rpc_name
    );
    let arguments_location = std::path::Path::new(raw_arguments_location);
    let mut arguments_file = std::fs::File::create(arguments_location)
        .expect("Couldn't create append interface to output file.");
    arguments_file
        .write_all(argument_interpretation.as_bytes())
        .unwrap();
}

pub fn produce_interpretation(raw_command_help: &str) {
    let (rpc_name, result_interpretations, arguments_interpretation) =
        interpret_help_message(raw_command_help);
    let full_response_interp = &result_interpretations
        .iter()
        .map(|x| x.clone())
        .collect::<Value>();
    record_interpretation(
        rpc_name,
        serde_json::ser::to_string_pretty(full_response_interp)
            .expect("Couldn't serialize prettily!"),
        serde_json::ser::to_string_pretty(&arguments_interpretation)
            .expect("Couldn't serialize prettily!"),
    );
}

fn partition_help_text(raw_command_help: &str) -> HashMap<String, String> {
    use regex::Regex;
    let mut sections = HashMap::new();

    //rpc_name
    let rpc_name = &raw_command_help
        .split_ascii_whitespace()
        .next()
        .expect("rpc_name not found!");
    sections.insert("rpc_name".to_string(), rpc_name.to_string());

    //response
    let response_delimiters =
        Regex::new(r"(?s)Result[:\s].*?Examples[:\s]").expect("Invalid regex");
    let response_section_match = response_delimiters
        .find(&raw_command_help)
        .expect("No response_section_match found!");
    let response_section = &raw_command_help
        [response_section_match.start()..(response_section_match.end() - 9)];
    sections.insert("response".to_string(), response_section.to_string());

    //description and arguments // TODO description still includes the rpc_name
    let arguments_delimiter =
        Regex::new(r"(?s).*?Arguments[:\s]").expect("Invalid regex!!");
    let description_section;
    let argument_section;
    if let Some(description_section_match) =
        arguments_delimiter.find(&raw_command_help)
    {
        description_section = &raw_command_help[description_section_match
            .start()
            ..(description_section_match.end() - "Arguments:".len())];
        argument_section = &raw_command_help
            [description_section_match.end()..response_section_match.start()];
    } else {
        description_section =
            &raw_command_help[..response_section_match.start()];
        argument_section = "";
    };
    sections.insert("description".to_string(), description_section.to_string());
    sections.insert("arguments".to_string(), argument_section.to_string());

    //examples
    let examples_section =
        &raw_command_help[(response_section_match.end() - "Examples:".len())..];
    sections.insert("examples".to_string(), examples_section.to_string());
    sections
}

fn split_response_into_results(response_section: String) -> Vec<String> {
    let resreg = regex::Regex::new(r"Result[:\s]").expect("invalid regex");
    let mut r: Vec<String> = resreg
        .split(&response_section)
        .map(|x| x.trim().to_string())
        .collect();
    r.remove(0);
    r
}

fn split_arguments(arguments_section: &str) -> Vec<String> {
    let argreg = regex::Regex::new(r"\n\d\.").expect("invalid regex");
    let mut a: Vec<String> = argreg
        .split(arguments_section)
        .map(|x| x.trim().to_string())
        .collect();
    a.remove(0);
    a
}

fn interpret_help_message(
    raw_command_help: &str,
) -> (String, Vec<serde_json::Value>, Vec<serde_json::Value>) {
    let sections = partition_help_text(raw_command_help);
    let rpc_name = sections.get("rpc_name").unwrap().to_string();
    if rpc_name == "submitblock" {
        //TODO special case, move to (pre)scrub
        return (rpc_name, vec![json!("ENUM: duplicate, duplicate-invalid, duplicate-inconclusive, inconclusive, rejected")], vec![json!({"1_hexdata": "String", "Option<2_jsonparametersobject>": "String"})]);
    }
    let response_data = sections.get("response").unwrap();
    let scrubbed_response =
        scrub_response(rpc_name.clone(), response_data.clone());
    let results = split_response_into_results(scrubbed_response);
    let mut result_vec = vec![];
    if results.len() == 1usize && results[0] == "" {
        // do not adjust result_vec
    } else {
        for result in results {
            result_vec.push(annotate_result(&mut result.chars()));
        }
    }

    let arguments_data = sections.get("arguments").unwrap();
    let scrubbed_arguments = scrub_arguments(&rpc_name, arguments_data.clone());
    let mut arguments_vec = vec![];
    if scrubbed_arguments.trim() == "" {
        // do not adjust arguments_vec
    } else if scrubbed_arguments.contains("(or)") {
        let split_arguments: Vec<&str> =
            scrubbed_arguments.split("(or)").collect();
        if !split_arguments.len() == 2 {
            panic!("not two arguments with '(or)'");
        }
        let mut arg_chars = split_arguments[0].trim().chars();
        match arg_chars.next().unwrap() {
            '{' => {
                let annotated_object = annotate_object(&mut arg_chars);
                arguments_vec.push(json!({ "1": annotated_object }));
            }
            _ => {
                panic!("no support for other formats");
            }
        }
        let vec = vec![(split_arguments[1].to_string())];
        arguments_vec.push(json!(annotate_arguments(vec)));
    } else {
        let arguments = split_arguments(&scrubbed_arguments);
        arguments_vec.push(json!(annotate_arguments(arguments)));
    }
    (rpc_name, result_vec, arguments_vec)
}

fn annotate_arguments(arguments: Vec<String>) -> serde_json::Value {
    let mut arg_map = serde_json::map::Map::new();
    let arg_regex = regex::Regex::new(r#"[^"]{1,}"#).expect("invalid regex");
    let mut argument_count = 1;
    for arg in arguments {
        let proto_ident = arg.split_whitespace().next();
        let naked_ident = &arg_regex.captures(proto_ident.unwrap()).unwrap()[0];
        let ident = format!("{}_{}", argument_count, &naked_ident);
        let (raw_label, annotated_ident) =
            label_identifier_optional(make_raw_label(arg), ident);
        arg_map.insert(
            annotated_ident,
            serde_json::Value::String(make_label(raw_label)),
        );
        argument_count += 1;
    }
    json!(arg_map)
}

fn annotate_result(result_chars: &mut std::str::Chars) -> serde_json::Value {
    match result_chars.next().unwrap() {
        '{' => annotate_object(result_chars),
        '[' => annotate_array(result_chars),
        i if i.is_alphabetic() || i == '"' => annotate_lonetype(format!(
            "{}{}",
            i,
            result_chars.as_str().to_string()
        )),
        x => {
            dbg!(x);
            todo!()
        }
    }
}

fn annotate_lonetype(lonetype_result: String) -> serde_json::Value {
    Value::String(make_label(make_raw_label(lonetype_result)))
}

fn annotate_object(result_chars: &mut std::str::Chars) -> serde_json::Value {
    let mut viewed = String::new();
    let mut ident_label_bindings = Map::new();
    loop {
        match result_chars.next().unwrap() {
            '}' => {
                if viewed.trim().is_empty() {
                    break;
                }
                let mut partial_ident_label_bindings =
                    bind_idents_labels(viewed.clone(), None);
                viewed.clear();
                // append works, but `.extend()` is more atomic, might
                // be worth looking at for refinements.
                ident_label_bindings.append(&mut partial_ident_label_bindings);
                break;
            }
            last_viewed if last_viewed == '[' || last_viewed == '{' => {
                let inner_value = match last_viewed {
                    '[' => annotate_array(result_chars),
                    '{' => annotate_object(result_chars),
                    _ => unreachable!("last_viewed is either '[' or '{'"),
                };
                let mut partial_ident_label_bindings =
                    bind_idents_labels(viewed.clone(), Some(inner_value));
                viewed.clear();
                ident_label_bindings.append(&mut partial_ident_label_bindings);
            }
            // TODO: Handle unbalanced braces? Create test.
            x if x.is_ascii() => viewed.push(x),
            _ => panic!("character is UTF-8 but not ASCII!"),
        }
    }
    Value::Object(ident_label_bindings)
}

fn annotate_array(result_chars: &mut std::str::Chars) -> serde_json::Value {
    let mut viewed = String::new();
    let mut ordered_results: Vec<Value> = vec![];
    loop {
        match result_chars.next().unwrap() {
            ']' => {
                if viewed.trim().is_empty() {
                    break;
                }
                ordered_results.push(get_array_terminal(viewed.clone()));
                viewed.clear();
                break;
            }
            last_viewed if last_viewed == '[' || last_viewed == '{' => {
                let inner_value = if last_viewed == '[' {
                    annotate_array(result_chars)
                } else {
                    annotate_object(result_chars)
                };
                viewed.clear();
                ordered_results.push(inner_value)
            }
            // TODO: Handle unbalanced braces? add test.
            x if x.is_ascii() => viewed.push(x),
            _ => panic!("character is UTF-8 but not ASCII!"),
        }
    }
    Value::Array(ordered_results)
}

fn get_array_terminal(viewed: String) -> Value {
    let viewed_lines = viewed_to_lines(viewed);
    let raw_label = make_raw_label((&viewed_lines[1]).to_string());
    json!(make_label(raw_label))
}

// TODO could be cleaned up, and/or broken into cases
// as opposed to internal conditional logic.
fn bind_idents_labels(
    viewed: String,
    inner_value: Option<Value>,
) -> Map<String, Value> {
    let mut viewed_lines = viewed_to_lines(viewed);
    //viewed_lines is now a Vec of strings that were lines in viewed.
    if viewed_lines[0].trim().is_empty()
        || !viewed_lines[0].trim().contains(":")
    {
        viewed_lines.remove(0); //.trim();
    }
    if inner_value != None {
        // possible if/let
        let mut viewed_lines_mutable = viewed_lines.clone();
        let last_ident_untrimmed = viewed_lines_mutable.pop().unwrap();
        let last_ident = last_ident_untrimmed
            .trim()
            .splitn(2, ':')
            .collect::<Vec<&str>>()[0]
            .trim()
            .trim_matches('"');
        let end_map = [(last_ident, inner_value.unwrap())]
            .iter()
            .cloned()
            .map(|(a, b)| (a.to_string(), b))
            .collect::<Map<String, Value>>();
        if viewed_lines_mutable.len() > 0 {
            viewed_lines_mutable
                .iter()
                .map(|ident_rawlabel| {
                    label_identifier(ident_rawlabel.to_string())
                })
                .map(|(a, b)| (a.to_string(), json!(b.to_string())))
                .chain(end_map)
                .collect::<Map<String, Value>>()
        } else {
            end_map
        }
    } else {
        viewed_lines
            .iter() // back into iter, could streamline?
            .map(|ident_rawlabel| label_identifier(ident_rawlabel.to_string()))
            .map(|(ident, annotation)| {
                (ident.to_string(), json!(annotation.to_string()))
            })
            .collect::<Map<String, Value>>()
    }
}

fn make_raw_label(meta_data: String) -> String {
    meta_data
        .split(|c| c == '(' || c == ')')
        .collect::<Vec<&str>>()[1]
        .to_string()
}

fn viewed_to_lines(viewed: String) -> Vec<String> {
    viewed
        .trim_end()
        .lines()
        .map(|line| line.to_string())
        .collect()
}

fn raw_to_ident_and_metadata(ident_with_metadata: String) -> (String, String) {
    let trimmed = ident_with_metadata.trim().to_string();
    let mut split = trimmed.splitn(3, '"').collect::<Vec<&str>>();
    if split[0].is_empty() {
        split.remove(0);
    }
    let ident = split[0].to_string();
    let metadata = split[1].trim_start_matches(":").trim().to_string();
    (ident, metadata)
}

fn label_identifier_optional(
    raw_label: String,
    mut ident: String,
) -> (String, String) {
    if raw_label.contains(", optional") {
        ident = format!("Option<{}>", ident);
    };
    (raw_label, ident)
}

// assumes well-formed `ident_with_metadata`
fn label_identifier(ident_with_metadata: String) -> (String, String) {
    let (ident, meta_data) = raw_to_ident_and_metadata(ident_with_metadata);
    let (raw_label, annotated_ident) =
        label_identifier_optional(make_raw_label(meta_data), ident);
    let annotation: String = make_label(raw_label);
    (annotated_ident.to_string(), annotation)
}

fn make_label(raw_label: String) -> String {
    match raw_label {
        label if label.starts_with("numeric") => "Decimal".to_string(),
        label if label.starts_with("string") => "String".to_string(),
        label if label.starts_with("boolean") => "bool".to_string(),
        label if label.starts_with("hexadecimal") => "hexadecimal".to_string(),
        label if label.starts_with("INSUFFICIENT") => {
            "INSUFFICIENT".to_string()
        }
        label => panic!("Label '{}' is invalid", label),
    }
}

// ------------------- tests ----------------------------------------

#[cfg(test)]
mod unit {
    use super::*;
    use crate::utils::test;
    use serde_json::json;

    // ------------------ partition_help_text --------
    #[test]
    fn partition_help_text_getblockchaininfo_enforce_fragment() {
        let expected_data = test::GETBLOCKCHAININFO_ENFORCE_FRAGMENT;
        let help_sections = partition_help_text(expected_data);
        let rpc_name = help_sections.get("rpc_name").unwrap().clone();
        let result = help_sections.get("response").unwrap().clone();
        let expected_result = test::GETBLOCKCHAININFO_ENFORCE_FRAGMENT_RESULT;
        //let bound = bind_idents_labels(fake_ident_label, rpc_name, None);
        assert_eq!(rpc_name, "getblockchaininfo".to_string());
        assert_eq!(result, expected_result);
    }

    // ----------------scrub_result-------------------
    #[test]
    fn scrub_result_getblockchaininfo_scrubbed() {
        let expected_result = test::HELP_GETBLOCKCHAININFO_RESULT_SCRUBBED;
        let result = scrub_response(
            "getblockchaininfo".to_string(),
            test::HELP_GETBLOCKCHAININFO_RESULT.to_string(),
        );
        assert_eq!(expected_result, result);
    }

    // ----------------label_identifier---------------

    #[test]
    fn label_identifier_with_expected_input_valid() {
        let raw_version =
            r#""version": xxxxx,           (numeric) the server version"#;
        let valid_annotation = ("version".to_string(), "Decimal".to_string());
        assert_eq!(valid_annotation, label_identifier(raw_version.to_string()));
    }

    // ----------------annotate_result---------------

    #[test]
    fn annotate_result_simple_unnested_generate() {
        let mut simple_unnested = &mut test::SIMPLE_UNNESTED.chars();
        let annotated = annotate_result(&mut simple_unnested);
        let expected_result = test::simple_unnested_json_generator();
        assert_eq!(expected_result, annotated);
    }

    #[test]
    fn annotate_result_simple_unnested_to_string() {
        let mut simple_unnested = &mut test::SIMPLE_UNNESTED.chars();
        let annotated = annotate_result(&mut simple_unnested);
        let expected_annotation = test::SIMPLE_UNNESTED_RESULT;
        assert_eq!(expected_annotation, annotated.to_string());
    }

    #[test]
    fn annotate_result_simple_unnested() {
        let mut simple_unnested = &mut test::SIMPLE_UNNESTED.chars();
        let annotated = annotate_result(&mut simple_unnested);
        let expected_annotation: Value =
            serde_json::de::from_str(test::SIMPLE_UNNESTED_RESULT).unwrap();
        assert_eq!(expected_annotation, annotated);
    }

    #[test]
    fn annotate_result_simple_nested_object_to_string() {
        let mut simple_nested = &mut test::SIMPLE_NESTED.chars();
        let annotated = annotate_result(&mut simple_nested);
        let expected_annotation = test::SIMPLE_NESTED_RESULT;
        assert_eq!(expected_annotation, annotated.to_string());
    }

    #[test]
    fn annotate_result_simple_nested_object() {
        let mut simple_nested = &mut test::SIMPLE_NESTED.chars();
        let annotated = annotate_result(&mut simple_nested);
        let expected_annotation: Value =
            serde_json::de::from_str(test::SIMPLE_NESTED_RESULT).unwrap();
        assert_eq!(expected_annotation, annotated);
    }

    #[test]
    fn annotate_result_multiple_nested_objects() {
        let mut multiple_nested = &mut test::MULTIPLE_NESTED.chars();
        let annotated = annotate_result(&mut multiple_nested);
        let expected_annotation: Value =
            serde_json::de::from_str(test::MULTIPLE_NESTED_ANNOTATION).unwrap();
        assert_eq!(expected_annotation, annotated);
    }

    #[test]
    fn annotate_result_multiple_nested_objects_2() {
        let mut multiple_nested = &mut test::MULTIPLE_NESTED_2.chars();
        let annotated = annotate_result(&mut multiple_nested);
        let expected_annotation: Value =
            serde_json::de::from_str(test::MULTIPLE_NESTED_2_ANNOTATION)
                .unwrap();
        assert_eq!(expected_annotation, annotated);
    }

    #[test]
    fn annotate_result_multiple_nested_objects_3() {
        let mut multiple_nested = &mut test::MULTIPLE_NESTED_3.chars();
        let annotated = annotate_result(&mut multiple_nested);
        let expected_annotation: Value =
            serde_json::de::from_str(test::MULTIPLE_NESTED_3_ANNOTATION)
                .unwrap();
        assert_eq!(expected_annotation, annotated);
    }

    #[test]
    fn annotate_result_multiple_nested_objects_4() {
        let mut multiple_nested = &mut test::MULTIPLE_NESTED_4.chars();
        let annotated = annotate_result(&mut multiple_nested);
        let expected_annotation: Value =
            serde_json::de::from_str(test::MULTIPLE_NESTED_4_ANNOTATION)
                .unwrap();
        assert_eq!(expected_annotation, annotated);
    }

    #[test]
    fn annotate_result_simple_unnested_getblockchaininfo() {
        let mut simple_unnested_blockchaininfo =
            &mut test::SIMPLE_UNNESTED_GETBLOCKCHAININFO.chars();
        let annotated = annotate_result(&mut simple_unnested_blockchaininfo);
        let expected_result = test::SIMPLE_UNNESTED_GETBLOCKCHAININFO_RESULT;
        assert_eq!(expected_result, annotated.to_string());
    }

    #[test]
    fn annotate_result_from_getinfo() {
        let expected_testdata_annotated = test::valid_getinfo_annotation();
        let help_sections = partition_help_text(test::HELP_GETINFO);
        let rpc_name = help_sections.get("rpc_name").unwrap().clone();
        let response = help_sections.get("response").unwrap().clone();
        let responses = split_response_into_results(response);
        let data_stream = &mut responses[0].chars();
        let annotated = annotate_result(data_stream);
        assert_eq!(annotated, expected_testdata_annotated);
        assert_eq!(rpc_name, "getinfo");
    }

    #[test]
    fn annotate_result_enforce_as_input() {
        let testmap = json!(test::BINDING_ENFORCE
            .iter()
            .map(|(a, b)| (a.to_string(), json!(b.to_string())))
            .collect::<HashMap<String, Value>>());
        assert_eq!(
            testmap,
            annotate_result(&mut test::ENFORCE_EXTRACTED.chars())
        );
    }

    #[test]
    fn annotate_result_simple_nested_object_generate() {
        let mut simple_nested = &mut test::SIMPLE_NESTED.chars();
        let annotated = annotate_result(&mut simple_nested);
        let expected_result = test::simple_nested_json_generator();
        assert_eq!(expected_result, annotated);
    }

    #[test]
    fn annotate_result_nested_obj_fragment_from_getblockchaininfo() {
        let mut expected_nested = test::GETBLOCKCHAININFO_FRAGMENT.chars();
        let annotated = annotate_result(&mut expected_nested);
        let expected_annotation: Value =
            serde_json::de::from_str(test::GETBLOCKCHAININFO_FRAGMENT_JSON)
                .unwrap();
        assert_eq!(expected_annotation, annotated);
    }

    #[test]
    fn annotate_result_simple_array_generate() {
        let mut simple_array_chars = &mut test::SIMPLE_ARRAY.chars();
        let annotated = annotate_result(&mut simple_array_chars);
        let expected_result = test::simple_array_json_generator();
        assert_eq!(expected_result, annotated);
    }

    #[test]
    fn annotate_result_simple_array_in_global_object_generate() {
        let mut simple_array_in_object_chars =
            &mut test::SIMPLE_ARRAY_IN_OBJECT.chars();
        let annotated = annotate_result(&mut simple_array_in_object_chars);
        let expected_result = test::simple_array_in_object_json_generator();
        assert_eq!(expected_result, annotated);
    }

    #[test]
    fn annotate_result_simple_array_in_nested_object_generate() {
        let mut simple_array_in_nested_object_chars =
            &mut test::SIMPLE_ARRAY_IN_NESTED_OBJECT.chars();
        let annotated =
            annotate_result(&mut simple_array_in_nested_object_chars);
        let expected_result =
            test::simple_array_in_nested_object_json_generator();
        assert_eq!(expected_result, annotated);
    }

    #[test]
    fn annotate_result_complex_array_in_nested_object_generate() {
        let mut complex_array_in_nested_object_chars =
            &mut test::COMPLEX_ARRAY_IN_NESTED_OBJECT.chars();
        let annotated =
            annotate_result(&mut complex_array_in_nested_object_chars);
        let expected_result =
            test::complex_array_in_nested_object_json_generator();
        assert_eq!(expected_result, annotated);
    }

    #[test]
    fn annotate_result_complex_array_with_nested_objects_in_nested_object_generate(
    ) {
        let mut complex_array_with_nested_objects_in_nested_object_chars =
            &mut test::COMPLEX_ARRAY_WITH_NESTED_OBJECTS_IN_NESTED_OBJECT
                .chars();
        let annotated = annotate_result(
            &mut complex_array_with_nested_objects_in_nested_object_chars,
        );
        let expected_result = test::complex_array_with_nested_objects_in_nested_object_json_generator();
        assert_eq!(expected_result, annotated);
    }

    #[test]
    fn annotate_result_nested_arrays_in_nested_object_generate() {
        let mut nested_arrays_in_nested_object_chars =
            &mut test::NESTED_ARRAYS_IN_NESTED_OBJECT.chars();
        let annotated =
            annotate_result(&mut nested_arrays_in_nested_object_chars);
        let expected_result =
            test::nested_arrays_in_nested_object_json_generator();
        assert_eq!(expected_result, annotated);
    }

    #[test]
    fn annotate_result_special_nested_blockchaininfo() {
        let mut special_nested_blockchaininfo =
            &mut test::SPECIAL_NESTED_GETBLOCKCHAININFO.chars();
        let annotated = annotate_result(&mut special_nested_blockchaininfo);
        let expected_result = serde_json::json!({"xxxx" :{"name":"String"}});
        assert_eq!(expected_result, annotated);
    }

    // ----------------annotate_arguments---------------

    #[test]
    fn simple_annotate_argument() {
        let simple_argument = test::SIMPLE_ARGUMENT;
        let annotated = annotate_arguments(vec![simple_argument.to_string()]);
        let expected_result = serde_json::json!({"1_filename": "String"});
        assert_eq!(expected_result, annotated);
    }

    #[test]
    fn annotate_multiple_arguments() {
        let multiple_arguments = vec![
            test::MULTIPLE_ARGUMENT_ONE.to_string(),
            test::MULTIPLE_ARGUMENT_TWO.to_string(),
            test::MULTIPLE_ARGUMENT_THREE.to_string(),
            test::MULTIPLE_ARGUMENT_FOUR.to_string(),
        ];
        let annotated = annotate_arguments(multiple_arguments);
        let expected_result = serde_json::json!({
        "1_arg_one": "String",
        "2_arg_two": "String",
        "3_arg_three": "String",
        "4_arg_four": "String"});
        assert_eq!(expected_result, annotated);
    }

    #[test]
    fn annotate_multiple_labels_over_arguments() {
        let multiple_arguments = vec![
            test::MULTIPLE_ARGS_LABEL_ONE.to_string(),
            test::MULTIPLE_ARGS_LABEL_TWO.to_string(),
            test::MULTIPLE_ARGS_LABEL_THREE.to_string(),
            test::MULTIPLE_ARGS_LABEL_FOUR.to_string(),
            test::MULTIPLE_ARGS_LABEL_FIVE.to_string(),
        ];
        let annotated = annotate_arguments(multiple_arguments);
        let expected_result = serde_json::json!({
        "1_arg_one": "Decimal",
        "2_arg_two": "String",
        "3_arg_three": "bool",
        "4_arg_four": "hexadecimal",
        "5_arg_five": "INSUFFICIENT",});
        assert_eq!(expected_result, annotated);
    }

    #[test]
    fn annotate_arguments_with_option() {
        let argument_with_option = test::ARGUMENT_WITH_OPTION;
        let annotated =
            annotate_arguments(vec![argument_with_option.to_string()]);
        let expected_result =
            serde_json::json!({"Option<1_argument_one_ident>": "String"});
        assert_eq!(expected_result, annotated);
    }

    #[test]
    fn disallow_improper_annotate_arguments_with_option() {
        let argument_with_option = test::ARGUMENT_WITH_OPTION;
        let annotated =
            annotate_arguments(vec![argument_with_option.to_string()]);
        let expected_result =
            serde_json::json!({"1_Option<argument_one_ident>": "String"});
        assert_ne!(expected_result, annotated);
    }

    // add test across interpret_help_message()

    // ----------------sanity_check---------------

    #[test]
    fn sanity_check_simple_unnested() {
        let simple_unnested_result = test::SIMPLE_UNNESTED_RESULT.to_string();
        let simple_unnested_json =
            test::simple_unnested_json_generator().to_string();
        assert_eq!(simple_unnested_result, simple_unnested_json);
    }

    #[test]
    fn sanity_check_simple_nested() {
        let simple_nested_result = test::SIMPLE_NESTED_RESULT.to_string();
        let simple_nested_json =
            test::simple_nested_json_generator().to_string();
        assert_eq!(simple_nested_result, simple_nested_json);
    }

    #[test]
    fn sanity_check_multiple_nested() {
        let multiple_nested_annotation =
            test::MULTIPLE_NESTED_ANNOTATION.to_string();
        let multiple_nested_json =
            test::multiple_nested_json_generator().to_string();
        assert_eq!(multiple_nested_annotation, multiple_nested_json);
    }

    /* more complex tests of the preceeding pattern fail
    due to the macro in use in
    `multiple_nested_2_json_generator().to_string()`
    serializing key-value pairs in a different order than is
    provided as the input to the macro. Therefore the following
    test will deserialize str test vectors into Values */
    #[test]
    fn sanity_check_multiple_nested_2() {
        let multiple_nested_2_value = serde_json::de::from_str::<Value>(
            test::MULTIPLE_NESTED_2_ANNOTATION,
        );
        let multiple_nested_2_json = test::multiple_nested_2_json_generator();
        assert_eq!(multiple_nested_2_value.unwrap(), multiple_nested_2_json);
    }

    // ----------------interpret_help_message---------------

    #[test]
    fn interpret_help_message_simple_unnested_full() {
        let simple_unnested_full = test::SIMPLE_UNNESTED_FULL;
        let interpreted = interpret_help_message(simple_unnested_full);
        let expected_result = json!({"outer_id":"String"});
        assert_eq!(interpreted.1[0], expected_result);
    }

    #[test]
    fn interpret_help_message_simple_nested_full() {
        use serde_json::json;
        let simple_nested_full = test::SIMPLE_NESTED_FULL;
        let interpreted = interpret_help_message(simple_nested_full);
        let expected_result = json!({"outer_id":{"inner_id":"String"}});
        assert_eq!(interpreted.1[0], expected_result);
    }

    #[test]
    #[should_panic]
    fn interpret_help_message_extrabrackets_within_input_lines() {
        let valid_help_in =
            interpret_help_message(test::EXTRABRACKETS3_HELP_GETINFO);
        assert_eq!(valid_help_in.1[0], test::valid_getinfo_annotation());
    }

    #[test]
    #[should_panic]
    fn interpret_help_message_more_than_one_set_of_brackets_input() {
        let valid_help_in =
            interpret_help_message(test::MORE_BRACKET_PAIRS_HELP_GETINFO);
        assert_eq!(valid_help_in.1[0], test::valid_getinfo_annotation());
    }
    #[test]
    #[should_panic]
    fn interpret_help_message_two_starting_brackets_input() {
        let valid_help_in =
            interpret_help_message(test::EXTRA_START_BRACKET_HELP_GETINFO);
        assert_eq!(valid_help_in.1[0], test::valid_getinfo_annotation());
    }
    #[test]
    #[should_panic]
    fn interpret_help_message_two_ending_brackets_input() {
        let valid_help_in =
            interpret_help_message(test::EXTRA_END_BRACKET_HELP_GETINFO);
        assert_eq!(valid_help_in.1[0], test::valid_getinfo_annotation());
    }
    #[test]
    #[should_panic]
    fn interpret_help_message_no_results_input() {
        let valid_help_in =
            interpret_help_message(test::NO_RESULT_HELP_GETINFO);
        assert_eq!(valid_help_in.1[0], test::valid_getinfo_annotation());
    }
    #[test]
    #[should_panic]
    fn interpret_help_message_no_end_bracket_input() {
        let valid_help_in =
            interpret_help_message(test::NO_END_BRACKET_HELP_GETINFO);
        assert_eq!(valid_help_in.1[0], test::valid_getinfo_annotation());
    }
    #[test]
    #[should_panic]
    fn interpret_help_message_no_start_bracket_input() {
        let valid_help_in =
            interpret_help_message(test::NO_START_BRACKET_HELP_GETINFO);
        assert_eq!(valid_help_in.1[0], test::valid_getinfo_annotation());
    }

    #[ignore]
    #[test]
    fn interpret_help_message_upgrades_in_obj_extracted() {
        dbg!(interpret_help_message(test::UPGRADES_IN_OBJ_EXTRACTED));
    }

    // ----------------interpret_help_message---------------

    #[test]
    fn interpret_help_message_expected_input_valid() {
        let valid_help_in = interpret_help_message(test::HELP_GETINFO);
        assert_eq!(valid_help_in.1[0], test::valid_getinfo_annotation());
    }

    #[test]
    fn interpret_help_message_early_lbracket_input() {
        let valid_help_in =
            interpret_help_message(test::LBRACKETY_HELP_GETINFO);
        assert_eq!(valid_help_in.1[0], test::valid_getinfo_annotation());
    }

    #[test]
    fn interpret_help_message_early_rbracket_input() {
        let valid_help_in =
            interpret_help_message(test::RBRACKETY_HELP_GETINFO);
        assert_eq!(valid_help_in.1[0], test::valid_getinfo_annotation());
    }

    #[test]
    fn interpret_help_message_early_extrabrackets_input() {
        let valid_help_in =
            interpret_help_message(test::EXTRABRACKETS1_HELP_GETINFO);
        assert_eq!(valid_help_in.1[0], test::valid_getinfo_annotation());
    }

    #[test]
    fn interpret_help_message_late_extrabrackets_input() {
        let valid_help_in =
            interpret_help_message(test::EXTRABRACKETS2_HELP_GETINFO);
        assert_eq!(valid_help_in.1[0], test::valid_getinfo_annotation());
    }

    #[test]
    fn interpret_help_message_getblockchaininfo_softforks_fragment() {
        let expected_incoming = test::GETBLOCKCHAININFO_SOFTFORK_FRAGMENT;
        let expected_result = serde_json::json!({"softforks":[{"enforce":{"found":"Decimal","required":"Decimal","status":"bool","window":"Decimal"},"id":"String","reject":{"found":"Decimal","required":"Decimal","status":"bool","window":"Decimal"},"version":"Decimal"}]});
        assert_eq!(
            interpret_help_message(expected_incoming).1[0],
            expected_result
        );
    }

    #[test]
    fn interpret_help_message_getblockchaininfo_enforce_and_reject_fragment() {
        let expected_incoming =
            test::GETBLOCKCHAININFO_ENFORCE_AND_REJECT_FRAGMENT;
        let expected_results = serde_json::json!({"enforce":{"found":"Decimal",
                                                             "required":"Decimal",
                                                             "status":"bool",
                                                             "window":"Decimal"},
                                                  "id":"String",
                                                  "reject":{"found":"Decimal",
                                                            "required":"Decimal",
                                                            "status":"bool",
                                                            "window":"Decimal"},
                                                  "version":"Decimal"});
        let interpreted = interpret_help_message(expected_incoming);
        assert_eq!(interpreted.1[0], expected_results);
    }

    #[ignore]
    #[test]
    fn interpret_help_message_getblockchaininfo_complete_does_not_panic() {
        dbg!(interpret_help_message(
            test::HELP_GETBLOCKCHAININFO_COMPLETE
        ));
    }
    fn getblockchaininfo_interpretation() -> serde_json::Value {
        serde_json::json!({"bestblockhash":"String",
                                          "blocks":"Decimal",
                                          "chain":"String",
                                          "chainwork":"String",
                                          "commitments":"Decimal",
                                          "consensus":{"chaintip":"String",
                                                       "nextblock":"String"},
                                          "difficulty":"Decimal",
                                          "estimatedheight":"Decimal",
                                          "headers":"Decimal",
                                          "initial_block_download_complete":"bool",
                                          "size_on_disk":"Decimal",
                                          "softforks":[{"enforce":{"found":"Decimal",
                                                                   "required":"Decimal",
                                                                   "status":"bool",
                                                                   "window":"Decimal"},
                                                        "id":"String",
                                                        "reject":{"found":"Decimal",
                                                                  "required":"Decimal",
                                                                  "status":"bool",
                                                                  "window":"Decimal"},
                                                        "version":"Decimal"}],
                                          "upgrades":{"xxxx":{"activationheight":"Decimal",
                                                              "info":"String",
                                                              "name":"String",
                                                              "status":"String"}},
                                          "verificationprogress":"Decimal"})
    }
    #[test]
    fn interpret_help_message_getblockchaininfo_complete() {
        let expected = getblockchaininfo_interpretation();
        assert_eq!(
            expected,
            interpret_help_message(test::HELP_GETBLOCKCHAININFO_COMPLETE).1[0]
        );
    }

    // ----------------serde_json_value----------------

    #[test]
    fn serde_json_value_help_getinfo() {
        let getinfo_serde_json_value = test::getinfo_export();
        let help_getinfo = interpret_help_message(test::HELP_GETINFO);
        assert_eq!(getinfo_serde_json_value, help_getinfo.1[0]);
    }

    #[test]
    fn record_interpretation_getblockchaininfo() {
        //! This test simply shows that record_interpretation doesn't mutate-or
        //! destroy any input.
        let test_rpc_name = "TEST_record_interpretation_getblockchaininfo";
        let location = format!(
            "./output/{}/{}_response.json",
            utils::logging::create_version_name(),
            test_rpc_name
        );
        let output = std::path::Path::new(&location);
        record_interpretation(
            test_rpc_name.to_string(),
            getblockchaininfo_interpretation().to_string(),
            "".to_string(),
        );

        //Now let's examine the results!
        let reader =
            std::io::BufReader::new(std::fs::File::open(output).unwrap());

        let read_in: serde_json::Value =
            serde_json::from_reader(reader).unwrap();
        assert_eq!(read_in, getblockchaininfo_interpretation());
        std::fs::remove_file(output).unwrap();
    }
}
