macro_rules! verifytxoutproof {
    ($result_data:expr) => {
        $result_data
            .replace(
                r#"["txid"]      (array, strings) The txid(s) which the proof commits to, or empty array if the proof is invalid"#,
                "[\n\"txid\"   (string) The txid(s) which the proof commits to, or empty array if the proof is invalid\n]")
    };
}

macro_rules! setban {
    ($arguments_data:expr) => {
        $arguments_data.replace(r#"(/netmask)"#, r#""#)
    };
}

macro_rules! getaddressbalance {
    ($arguments_data:expr) => {
        $arguments_data
            .replace(r#",..."#, r#""#)
            .replace(r#""addresses:""#, r#""addresses":"#)
    };
}

macro_rules! args_fromaddresses_array {
    ($arguments_data:expr) => {
        $arguments_data.replace(r#"(array, required) A JSON array with addresses.
                         The following special strings are accepted inside the array:
                             - "ANY_TADDR":   Merge UTXOs from any taddrs belonging to the wallet.
                             - "ANY_SPROUT":  Merge notes from any Sprout zaddrs belonging to the wallet.
                             - "ANY_SAPLING": Merge notes from any Sapling zaddrs belonging to the wallet.
                         While it is possible to use a variety of different combinations of addresses and the above values,
                         it is not possible to send funds from both sprout and sapling addresses simultaneously. If a special
                         string is given, any given addresses of that type will be counted as duplicates and cause an error.
    [
      "address"          (string) Can be a taddr or a zaddr
      ,...
    ]"#, r#"[
    "address"          (string) Can be a taddr or a zaddr
    ]"#)
    };
}

pub(crate) fn scrub_arguments(
    rpc_name: &str,
    arguments_data: String,
) -> String {
    match rpc_name {
        "z_mergetoaddress" => {
            args_fromaddresses_array!(arguments_data)
        }
        "setban" => {
            setban!(arguments_data)
        }
        "getaddressbalance" => {
            getaddressbalance!(arguments_data)
        }
        _ => arguments_data,
    }
}

pub(crate) fn scrub_response(rpc_name: String, result_data: String) -> String {
    if rpc_name == "verifytxoutproof".to_string() {
        verifytxoutproof!(result_data)
    } else {
        result_data
    }
}
