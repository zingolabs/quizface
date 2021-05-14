pub fn prescrub(rpc_name: &str, raw_rpc_name_help: &str) -> String {
    use regex::Regex;
    match rpc_name {
        "setgenerate"      // Za
        | "z_validatepaymentdisclosure" => {
            raw_rpc_name_help.replace("Examples:", "Result:\nExamples:")
        }
        "getrawtransaction" => Regex::new(r"Result \(if verbose.*\):")
            .unwrap()
            .replace_all(raw_rpc_name_help, "Result:")
            .to_string(),
        "getblock" => Regex::new(r"Result \(for verbosity = [012]\):")
            .unwrap()
            .replace_all(raw_rpc_name_help, "Result:")
            .to_string(),
        "keypoolrefill" => {
            raw_rpc_name_help.replace("Examples:", "Result:\nExamples:")
                .replace("Arguments", "Arguments:")
        }
        "settxfee" => {
            raw_rpc_name_help.replace("Result", "Result:")
        }
        "estimatepriority" | "estimatefee" => {
            raw_rpc_name_help.replace("Example:", "Examples:")
        }
        "z_getmigrationstatus" => {
            raw_rpc_name_help.replace("}", "}\nExamples:\n")
                .replace(r#""migration_txids": [txids]                (json array of strings) An array of all migration txids involving this wallet"#, "\"migration_txids\": [\n \"txids\"  (string) An array of all migration txids involving this wallet\n]")
        }
        "zcrawreceive" => {
            raw_rpc_name_help.replace("}", "}\nExamples:\n")
                .replace(r#"Output: {"#, "Result:\n{")
                .replace(r#""amount": value"#, r#""amount": (numeric) value"#)
                .replace(r#""note": noteplaintext"#, r#""note": (string) noteplaintext"#)
                .replace(r#""exists": exists"#, r#""exists": (boolean) exists"#)
        }
        "zcrawjoinsplit" => raw_rpc_name_help.replace(
            r#"Output: {
  "encryptednote1": enc1,
  "encryptednote2": enc2,
  "rawtxn": rawtxout
}"#,
            r#"Result: 
{
  "encryptednote1": (string) enc1,
  "encryptednote2": (string) enc2,
  "rawtxn": (string) rawtxout
}

Examples:
"#,
        ),
        _ => raw_rpc_name_help.to_string(),
    }
}
