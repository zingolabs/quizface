pub fn prescrub(rpc_name: &str, raw_rpc_name_help: &str) -> String {
    use regex::Regex;
    match rpc_name {
        "z_validatepaymentdisclosure" => {
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
        "keypoolrefill" => raw_rpc_name_help
            .replace("Examples:", "Result:\nExamples:")
            .replace("Arguments", "Arguments:"),
        "settxfee" => raw_rpc_name_help.replace("Result", "Result:"),
        "estimatepriority" | "estimatefee" => {
            raw_rpc_name_help.replace("Example:", "Examples:")
        }
        _ => raw_rpc_name_help.to_string(),
    }
}
