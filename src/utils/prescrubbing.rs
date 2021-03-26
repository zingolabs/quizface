pub fn prescrub(command: &str, raw_command_help: &str) -> String {
    use regex::Regex;
    match command {
        "importaddress"
        | "importpubkey"
        | "encryptwallet"
        | "addnode"
        | "disconnectnode"
        | "importprivkey"
        | "importwallet"
        | "setlogfilter"
        | "setban"
        | "keypoolrefill"
        | "z_importwallet"
        | "clearbanned"
        | "setaccount"
        | "setgenerate"
        | "listbanned"
        | "ping"
        | "z_validatepaymentdisclosure" => {
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
        "createrawtransaction" => {
            raw_command_help.replace("Examples", "Examples:")
        }
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
        "zcsamplejoinsplit" | "z_setmigration" => {
            let mut r = raw_command_help.to_string();
            r.push_str("\nArguments:\nResult:\nExamples:\n");
            r
        }
        "getunconfirmedbalance" => raw_command_help.replace(
            "Returns the server's total unconfirmed balance",
            "\nResult:\n\"balance\"  (numeric) the server's total unconfirmed balance\n\nExamples:"),
        _ => raw_command_help.to_string(),
    }
}
