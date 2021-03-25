pub fn prescrub(command: &str, raw_command_help: &str) -> String {
    use regex::Regex;
    match command {
        "importaddress" | "importpubkey" | "encryptwallet" | "addnode"
        | "disconnectnode" | "importprivkey" | "importwallet"
        | "setlogfilter" | "setban" | "keypoolrefill" | "z_importwallet"
        | "clearbanned" | "setaccount" | "setgenerate" => {
            raw_command_help.replace("Examples:", "Result:\nExamples:")
        }
        "getrawtransaction" => Regex::new(r"Result \(if verbose.*\):")
            .unwrap()
            .replace_all(raw_command_help, "Result:")
            .to_string(),

        "getblock" => Regex::new(r"Result \(for verbosity = [012]\):")
            .unwrap()
            .replace_all(raw_command_help, "Result:")
            .to_string(),
        "settxfee" | "getgenerate" | "generate" => {
            raw_command_help.replace("Result", "Result:")
        }
        "help" => raw_command_help
            .replace("The help text", "The help text \n Examples:"),
        "estimatepriority" | "estimatefee" => {
            raw_command_help.replace("Example:", "Examples:")
        }
        "getunconfirmedbalance" => raw_command_help.replace(
            "unconfirmed balance",
            r#"unconfirmed balance
                                     Result:
                                     "#,
        ),
        "createrawtransaction" => {
            raw_command_help.replace("Examples", "Examples:")
        }
        "prioritisetransaction" => raw_command_help.replace(
            r#"Result
true"#,
            r#"Result: \n "true""#,
        ),
        "z_getmigrationstatus" | "zcrawreceive" => {
            raw_command_help.replace("}", "}\nExamples:\n")
        }
        "zcrawjoinsplit" => raw_command_help.replace(
            r#"Output: {
  "encryptednote1": enc1,
  "encryptednote2": enc2,
  "rawtxn": rawtxout
}"#,
            r#"Result: {
  "encryptednote1": enc1,
  "encryptednote2": enc2,
  "rawtxn": rawtxout
Examples:
"#,
        ),
        "z_listunspent" | "getbestblockhash" | "listunspent" => {
            let intermediate_command_help =
                raw_command_help.replace("Result", "Result:");
            intermediate_command_help
                .as_str()
                .replace("Examples", "Examples:")
        }
        "zcrawkeygen" => {
            let intermediate_command_help =
                raw_command_help.replace("Output:", "Result:");
            intermediate_command_help.as_str().replace(
                "}",
                r#"}
            Examples:
            "#,
            )
        }
        "stop" => raw_command_help.replace(
            r#"stop"#,
            r#"stop
                                 Result:
                                 Examples:
                                 "#,
        ),
        "gettxoutproof" => raw_command_help.replace(
            r#"proof."#,
            r#"proof.
                                 Examples:
                                 "#,
        ),
        "verifytxoutproof" => raw_command_help.replace(
            r#"proof is invalid"#,
            r#"proof is invalid
        Examples:
        "#,
        ),
        "zcbenchmark" => raw_command_help.replace(
            r#"Output: [
  {
    "runningtime": runningtime
  },
  {
    "runningtime": runningtime
  }
  ...
]"#,
            r#"Result: [
  {
    "runningtime": runningtime
  },
  {
    "runningtime": runningtime
  }
  ...
]
Examples:
"#,
        ),
        _ => raw_command_help.to_string(),
    }
}
