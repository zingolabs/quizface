pub mod utils;
use std::collections::HashMap;

pub fn parse_raw_output(raw_command_help: String) -> HashMap<String, String> {
    let command_help_lines_iter = raw_command_help.split("\n");

    let mut command_help_lines = Vec::new();

    // for 'well formed' command help outputs (such as getinfo):
    // the relevant fields are in between `{` and `}`, assumed
    // to be alone on a line
    let mut start: bool = false;
    let mut end: bool = false;

    // TODO insure this pattern happens exactly once
    for li in command_help_lines_iter {
        if li == "}" {
            end = true;
        }

        // XOR: after `{` but before `}`
        if start ^ end {
            command_help_lines.push(li);
        }

        if li == "{" {
            start = true;
        }
    }

    let mut command_map = HashMap::new();

    for line in command_help_lines {
        let (key, value) = annotate_identifier(line.to_string());
        command_map.insert(key, value);
    }
    command_map
}

pub fn annotate_identifier(ident_with_metadata: String) -> (String, String) {
    // find key. begin by selecting first str before
    // whitespace and eliminating leading whitespace.
    let ident = &ident_with_metadata.split('"').collect::<Vec<&str>>()[1]
        .trim_matches('"');

    let mut temp_iter = ident_with_metadata.split_ascii_whitespace();
    let unparsed_key_str = match temp_iter.next() {
        Some(x) => x,
        None => panic!("error during command parsing"),
    };
    dbg!(&unparsed_key_str);

    let unparsed_key_str_vec: Vec<&str> = unparsed_key_str.split('"').collect();

    dbg!(&unparsed_key_str_vec);
    // unparsed_key_str_vec should still contain leading "" element
    // and trailing ":" element, and be exactly 3 elements in length
    if &unparsed_key_str_vec.len() == &3 {
        // do nothing
    } else {
        panic!("unparsed_key_str_vec != 3")
    }

    // one more intermediate Vec
    let mut isolated_key_str_vec: Vec<&str> = Vec::new();
    for element in unparsed_key_str_vec {
        if element != "" && element != ":" {
            isolated_key_str_vec.push(element);
        }
    }

    // check that isolated_key_str_vec has exactly 1 element
    if &isolated_key_str_vec.len() != &1 {
        panic!("more than one element in isolated_key_str_vec !")
    }

    let key_str = isolated_key_str_vec[0];
    let key_str = ident;
    //dbg!(&key_str);

    // find 'keywords' in ident_with_metadata to eventually produce values
    // that match rust types in resulting HashMap.
    // to prevent extra detecting extra occurances, find only
    // these 'keywords' within first set of paranthesis.

    // split with an closure to support multiple 'splitters'
    let unparsed_value_str_vec: Vec<&str> = ident_with_metadata
        .split(|c| c == '(' || c == ')')
        .collect();

    // because unparsed_value_str_vec will have an element before
    // the first '(', and there may be more sets of parenthesis,
    // only the second element with is examined with [1].
    // if there are nested parenthesis this scheme will fail.
    // TODO possibly check for nested parenthesis?

    // determine if optional.
    let mut optional: bool = false;
    if unparsed_value_str_vec[1].contains("optional") {
        optional = true;
    }

    // create value collecting Vec, to then check vec
    // has only one valid value
    let mut value_collector_vec: Vec<&str> = Vec::new();

    // transforming for HashMap, provide values to vec
    if unparsed_value_str_vec[1].contains("numeric") {
        value_collector_vec.push("Decimal");
    }
    if unparsed_value_str_vec[1].contains("string") {
        value_collector_vec.push("String");
    }
    if unparsed_value_str_vec[1].contains("boolean") {
        value_collector_vec.push("bool");
    }

    if &value_collector_vec.len() != &1 {
        panic!("only 1 element allowed in value_collector_vec!")
    }

    let temp_value_string: String;
    let value_str: &str;
    // form value for Hashmap, cosidering the boolean optional
    if optional {
        temp_value_string = format!("Option<{}>", value_collector_vec[0]);
        value_str = &temp_value_string;
    } else {
        value_str = value_collector_vec[0];
    }
    dbg!(value_str);
    (key_str.to_string(), value_str.to_string())
}
#[cfg(test)]
mod unit {
    use super::*;
    use crate::utils::test;

    #[test]
    fn annotate_identifier_observed_input_valid() {
        let raw_version =
            r#""version": xxxxx,           (numeric) the server version"#;
        let valid_annotation = ("version".to_string(), "Decimal".to_string());
        assert_eq!(
            valid_annotation,
            annotate_identifier(raw_version.to_string())
        );
    }
    #[test]
    fn parse_raw_output_observed_input_valid() {
        let valid_help_in = parse_raw_output(test::HELP_GETINFO.to_string());
        assert_eq!(valid_help_in, test::valid_getinfo_annotation());
    }
    #[test]
    fn parse_raw_output_early_lbracket_input() {
        let valid_help_in =
            parse_raw_output(test::LBRACKETY_HELP_GETINFO.to_string());
        assert_eq!(valid_help_in, test::valid_getinfo_annotation());
    }
    #[test]
    fn parse_raw_output_early_rbracket_input() {
        let valid_help_in =
            parse_raw_output(test::RBRACKETY_HELP_GETINFO.to_string());
        assert_eq!(valid_help_in, test::valid_getinfo_annotation());
    }
}
