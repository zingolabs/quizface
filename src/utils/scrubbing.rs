macro_rules! z_validateaddress {
    ($result_data:expr) => {
        $result_data
            .replace(r#"[sprout]"#, r#"<sprout>"#)
            .replace(r#"[sapling]"#, r#"<sapling>"#)
    };
}

macro_rules! getrawmempool {
    ($result_data:expr) => {
        $result_data
            .replace(r#"Result: (for verbose = false):
[                     (json array of string)
  "transactionid"     (string) The transaction id
  ,...
]

Result: (for verbose = true):
{                           (json object)
  "transactionid" : {       (json object)
    "size" : n,             (numeric) transaction size in bytes
    "fee" : n,              (numeric) transaction fee in ZEC
    "time" : n,             (numeric) local time transaction entered pool in seconds since 1 Jan 1970 GMT
    "height" : n,           (numeric) block height when transaction entered pool
    "startingpriority" : n, (numeric) priority when transaction entered pool
    "currentpriority" : n,  (numeric) transaction priority now
    "depends" : [           (array) unconfirmed transactions used as inputs for this transaction
        "transactionid",    (string) parent transaction id
       ... ]
  }, ...
}"#,

r#"Result:
[                     
  "transactionid"     (string) The transaction id
]

Result:
{
  "transactionid" : {
    "size" : n,             (numeric) transaction size in bytes
    "fee" : n,              (numeric) transaction fee in ZEC
    "time" : n,             (numeric) local time transaction entered pool in seconds since 1 Jan 1970 GMT
    "height" : n,           (numeric) block height when transaction entered pool
    "startingpriority" : n, (numeric) priority when transaction entered pool
    "currentpriority" : n,  (numeric) transaction priority now
    "depends" : [
        "transactionid",    (string) parent transaction id
        ]
  }
}"#)
     };
}

macro_rules! getblockheader {
    ($result_data:expr) => {
        $result_data.replace(r#"Result (for verbose = true):
{
  "hash" : "hash",     (string) the block hash (same as provided)
  "confirmations" : n,   (numeric) The number of confirmations, or -1 if the block is not on the main chain
  "height" : n,          (numeric) The block height or index
  "version" : n,         (numeric) The block version
  "merkleroot" : "xxxx", (string) The merkle root
  "finalsaplingroot" : "xxxx", (string) The root of the Sapling commitment tree after applying this block
  "time" : ttt,          (numeric) The block time in seconds since epoch (Jan 1 1970 GMT)
  "nonce" : n,           (numeric) The nonce
  "bits" : "1d00ffff", (string) The bits
  "difficulty" : x.xxx,  (numeric) The difficulty
  "previousblockhash" : "hash",  (string) The hash of the previous block
  "nextblockhash" : "hash"       (string) The hash of the next block
}

Result (for verbose=false):
"data"             (string) A string that is serialized, hex-encoded data for block 'hash'."#,
r#"Result:
"data"             (string) A string that is serialized, hex-encoded data for block 'hash'.

"Result:
{
  "hash" : "hash",     (string) the block hash (same as provided)
  "confirmations" : n,   (numeric) The number of confirmations, or -1 if the block is not on the main chain
  "height" : n,          (numeric) The block height or index
  "version" : n,         (numeric) The block version
  "merkleroot" : "xxxx", (string) The merkle root
  "finalsaplingroot" : "xxxx", (string) The root of the Sapling commitment tree after applying this block
  "time" : ttt,          (numeric) The block time in seconds since epoch (Jan 1 1970 GMT)
  "nonce" : n,           (numeric) The nonce
  "bits" : "1d00ffff", (string) The bits
  "difficulty" : x.xxx,  (numeric) The difficulty
  "previousblockhash" : "hash",  (string) The hash of the previous block
  "nextblockhash" : "hash"       (string) The hash of the next block
}"#)
    };
}

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
    } else if rpc_name == "getblockheader".to_string() {
        getblockheader!(result_data)
    } else if rpc_name == "getrawmempool".to_string() {
        getrawmempool!(result_data)
    } else if rpc_name == "z_validateaddress".to_string() {
        z_validateaddress!(result_data)
    } else {
        result_data
    }
}
