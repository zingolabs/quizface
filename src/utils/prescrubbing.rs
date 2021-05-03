pub fn prescrub(rpc_name: &str, raw_rpc_name_help: &str) -> String {
    use regex::Regex;
    match rpc_name {
        "importaddress"
        | "importpubkey"
        | "encryptwallet"
        | "addnode"
        | "disconnectnode"
        | "importprivkey"
        | "importwallet"
        | "setlogfilter"
        | "setban"
        | "z_importwallet"
        | "clearbanned"
        | "setaccount"
        | "setgenerate"
        | "listbanned"
        | "ping"
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
        "submitblock" => { 
            raw_rpc_name_help.replace("Arguments", "Arguments:") 
        }
        "settxfee" | "getgenerate" => {
            raw_rpc_name_help.replace("Result", "Result:")
        }
        "help" => raw_rpc_name_help
            .replace("The help text", "The help text \n Examples:"),
        "estimatepriority" | "estimatefee" => {
            raw_rpc_name_help.replace("Example:", "Examples:")
        }
        "createrawtransaction" => {
            raw_rpc_name_help.replace("Examples", "Examples:")
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
        "zcrawkeygen" => {
            let intermediate_rpc_name_help =
                raw_rpc_name_help.replace("Output:", "Result:")
                .replace("zcaddr,", "(string) zcaddr,")
                .replace("zcsecretkey,", "(string) zcsecretkey,")
                .replace("zcviewingkey,", "(string) zcviewingkey,");
            intermediate_rpc_name_help.as_str().replace(
                "}",
                r#"}
            Examples:
            "#,
            )
        }
        "stop" => raw_rpc_name_help.replace(
            r#"stop"#,
            r#"stop
                                 Result:
                                 Examples:
                                 "#,
        ),
        "gettxoutproof" => raw_rpc_name_help.replace(
            r#"proof."#,
            r#"proof.
                                 Examples:
                                 "#,
        ),
        "verifytxoutproof" => raw_rpc_name_help.replace(
            r#"proof is invalid"#,
            r#"proof is invalid
        Examples:
        "#,
        ),
        "zcbenchmark" => raw_rpc_name_help.replace(
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
    "runningtime": (numeric)
  }
]
Examples:
"#,
        ),
        "zcsamplejoinsplit"
        | "z_setmigration" => {
            let mut r = raw_rpc_name_help.to_string();
            r.push_str("\nArguments:\nResult:\nExamples:\n");
            r
        }
        "getunconfirmedbalance" => raw_rpc_name_help.replace(
            "Returns the server's total unconfirmed balance",
            "\nResult:\n\"balance\"  (numeric) the server's total unconfirmed balance\n\nExamples:"),
        _ => raw_rpc_name_help.to_string(),
    }
}
