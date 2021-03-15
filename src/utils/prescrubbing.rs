pub fn prescrub(command: &str, raw_command_help: &str) -> String {
    match command {
        "importaddress" | "importpubkey" | "encryptwallet" | "addnode" | "disconnectnode" | "importprivkey" | "importwallet" | "setlogfilter" => {
            raw_command_help.replace("Examples:", "Result: \nExamples:")
        }
        "settxfee" | "getgenerate" | "generate" => { 
            raw_command_help.replace("Result","Result:")
        } 
        "help" => {
            raw_command_help.replace("The help text","The help text \n Examples:")
        } 
        "estimatepriority" => {
            raw_command_help.replace("n :", r#""n :""#)       
        }
        "getunconfirmedbalance" => {
            raw_command_help.replace("unconfirmed balance", r#"unconfirmed balance \n Result: "numberic""#)       
        } 
        "createrawtransaction" | "getrawmempool" => {
            raw_command_help.replace("Examples", "Examples:")       
        }
        "prioritisetransaction" => {
            raw_command_help.replace(r#"Result
true"#, r#"Result: \n "true""#)
        }
    "z_getmigrationstatus" => {
        raw_command_help.replace("}", "}\nExamples:\n")       
        } 
    "zcrawjoinsplit" => {
        raw_command_help.replace(r#"Output: {
  "encryptednote1": enc1,
  "encryptednote2": enc2,
  "rawtxn": rawtxout
}"#, r#"Result: {
  "encryptednote1": enc1,
  "encryptednote2": enc2,
  "rawtxn": rawtxout
}\nExamples:\n"#)       
    }
        _ => { 
            "to".to_string()
        }
    }
}
    /*
    "z_listunspent" => {
        raw_command_help.replace("", "")       
    } else if command == "
zcrawreceive
        raw_command_help.replace("", "")       
    } else if command == "
getbestblockhash
        raw_command_help.replace("", "")       
    } else if command == "
zcrawkeygen
        raw_command_help.replace("", "")       
    } else if command == "
stop
        raw_command_help.replace("", "")       
    } else if command == "
setban
        raw_command_help.replace("", "")       
    } else if command == "
keypoolrefill
        raw_command_help.replace("", "")       
    } else if command == "
gettxoutproof
        raw_command_help.replace("", "")       
    } else if command == "
getaddressutxos
        raw_command_help.replace("", "")       
    } else if command == "
verifytxoutproof
        raw_command_help.replace("", "")       
    } else if command == "
estimatefee
        raw_command_help.replace("", "")       
    } else if command == "
z_importwallet
        raw_command_help.replace("", "")       
    } else if command == "
getblockheader
        raw_command_help.replace("", "")       
    } else if command == "
listunspent
        raw_command_help.replace("", "")       
    } else if command == "
clearbanned
        raw_command_help.replace("", "")       
    } else if command == "
setaccount
        raw_command_help.replace("", "")       
    } else if command == "
setgenerate
        raw_command_help.replace("", "")       
    } else if command == "
zcbenchmark
}
*/

