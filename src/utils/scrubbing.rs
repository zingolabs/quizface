macro_rules! getaddressutxos {
    ($result_data:expr) => {
        $result_data
            .replace(r#"(or, if chainInfo is true):"#, "Result:")
            .replace("number", "numeric")
    };
}

const RAWTRANSACTION: &str = r#"{
  "in_active_chain": b,   (bool) Whether specified block is in the active chain or not (only present with explicit "blockhash" argument)
  "hex" : "data",       (string) The serialized, hex-encoded data for 'txid'
  "txid" : "id",        (string) The transaction id (same as provided)
  "size" : n,             (numeric) The transaction size
  "version" : n,          (numeric) The version
  "locktime" : ttt,       (numeric) The lock time
  "expiryheight" : ttt,   (numeric, optional) The block height after which the transaction expires
  "vin" : [               (array of json objects)
     {
       "txid": "id",    (string) The transaction id
       "vout": n,         (numeric)
       "scriptSig": {     (json object) The script
         "asm": "asm",  (string) asm
         "hex": "hex"   (string) hex
       },
       "sequence": n      (numeric) The script sequence number
     }
     ,...
  ],
  "vout" : [              (array of json objects)
     {
       "value" : x.xxx,            (numeric) The value in ZEC
       "n" : n,                    (numeric) index
       "scriptPubKey" : {          (json object)
         "asm" : "asm",          (string) the asm
         "hex" : "hex",          (string) the hex
         "reqSigs" : n,            (numeric) The required sigs
         "type" : "pubkeyhash",  (string) The type, eg 'pubkeyhash'
         "addresses" : [           (json array of string)
           "zcashaddress"          (string) Zcash address
           ,...
         ]
       }
     }
     ,...
  ],
  "vjoinsplit" : [        (array of json objects, only for version >= 2)
     {
       "vpub_old" : x.xxx,         (numeric) public input value in ZEC
       "vpub_new" : x.xxx,         (numeric) public output value in ZEC
       "anchor" : "hex",         (string) the anchor
       "nullifiers" : [            (json array of string)
         "hex"                     (string) input note nullifier
         ,...
       ],
       "commitments" : [           (json array of string)
         "hex"                     (string) output note commitment
         ,...
       ],
       "onetimePubKey" : "hex",  (string) the onetime public key used to encrypt the ciphertexts
       "randomSeed" : "hex",     (string) the random seed
       "macs" : [                  (json array of string)
         "hex"                     (string) input note MAC
         ,...
       ],
       "proof" : "hex",          (string) the zero-knowledge proof
       "ciphertexts" : [           (json array of string)
         "hex"                     (string) output note ciphertext
         ,...
       ]
     }
     ,...
  ],
  "blockhash" : "hash",   (string) the block hash
  "confirmations" : n,      (numeric) The confirmations
  "time" : ttt,             (numeric) The transaction time in seconds since epoch (Jan 1 1970 GMT)
  "blocktime" : ttt         (numeric) The block time in seconds since epoch (Jan 1 1970 GMT)
}"#;

macro_rules! getrawtransaction {
    ($result_data:expr) => {
        $result_data
            .replace("bool", "boolean")
            .replace("(array of json objects, only for version >= 2)", "")
            .replace("(array of json objects)", "")
            .replace("(json array of string)", "")
            .replace("(json object) The script", "")
            .replace("(json object)", "")
    };
}

macro_rules! getblock {
    ($result_data:expr) => {
        $result_data
			.replace(r#"(array of string) The transaction ids"#, "")
            .replace(
				"(array of Objects) The transactions in the format of the getrawtransaction RPC. Different from verbosity = 1 \"tx\" result.\n         ,...",
				&getrawtransaction!(RAWTRANSACTION)
			)
			.replace(
                r#"...,                     Same output as verbosity = 1."#,
                r#"  "hash" : "hash",       (string) the block hash (same as provided hash)
  "confirmations" : n,   (numeric) The number of confirmations, or -1 if the block is not on the main chain
  "size" : n,            (numeric) The block size
  "height" : n,          (numeric) The block height or index (same as provided height)
  "version" : n,         (numeric) The block version
  "merkleroot" : "xxxx", (string) The merkle root
  "finalsaplingroot" : "xxxx", (string) The root of the Sapling commitment tree after applying this block"#
            )
			.replace(
				r#",...                     Same output as verbosity = 1."#,
				r#""time" : ttt,          (numeric) The block time in seconds since epoch (Jan 1 1970 GMT)
  "nonce" : n,           (numeric) The nonce
  "bits" : "1d00ffff",   (string) The bits
  "difficulty" : x.xxx,  (numeric) The difficulty
  "previousblockhash" : "hash",  (string) The hash of the previous block
  "nextblockhash" : "hash"       (string) The hash of the next block"#
			)
    };
}

macro_rules! getaddressmempool {
    ($result_data:expr) => {
        $result_data.replace(r#"number"#, r#"numeric"#)
    };
}

macro_rules! listunspent {
    ($result_data:expr) => {
        $result_data.replace(r#"(bool)"#, r#"(boolean)"#)
    };
}

macro_rules! z_listunspent {
    ($result_data:expr) => {
        $result_data
            .replace(" (sprout) : n,", ": <sprout> n,")
            .replace(" (sapling) : n,", ": <sapling> n,")
    };
}

macro_rules! generate {
    ($result_data:expr) => {
        $result_data.replace(
            r#"[ blockhashes ]     (array) hashes of blocks generated"#,
            "[\nblockhashes     (string) hashes of blocks generated\n]",
        )
    };
}

macro_rules! getblockchaininfo {
    ($result_data:expr) => {
    $result_data.replace("[0..1]", "").replace(
        "{ ... }      (object) progress toward rejecting pre-softfork blocks",
        "{
\"status\": (boolean)
\"found\": (numeric)
\"required\": (numeric)
\"window\": (numeric)
}").replace("(same fields as \"enforce\")", "")
    };
}

macro_rules! getblockdeltas {
    ($result_data:expr) => {
        $result_data
            .replace(r#"hex string"#, r#"hexadecimal"#)
            .replace(r#"hexstring"#, r#"hexadecimal"#)
    };
}

macro_rules! getblockhashes {
    ($result_data:expr) => {
        $result_data
            .replace(r#"hex string"#, r#"hexadecimal"#)
            .replace(r#"hexstring"#, r#"hexadecimal"#)
    };
}

macro_rules! getchaintips {
    ($result_data:expr) => {
    $result_data.replace(r#",
  {
    "height": xxxx,
    "hash": "xxxx",
    "branchlen": 1          (numeric) length of branch connecting the tip to the main chain
    "status": "xxxx"        (string) status of the chain (active, valid-fork, valid-headers, headers-only, invalid)
  }"#, r#""#).replace(
            r#"Possible values for status:
1.  "invalid"               This branch contains at least one invalid block
2.  "headers-only"          Not all blocks for this branch are available, but the headers are valid
3.  "valid-headers"         All blocks are available for this branch, but they were never fully validated
4.  "valid-fork"            This branch is not part of the active chain, but is fully validated
5.  "active"                This is the tip of the active main chain, which is certainly valid"#, "")
.replace(r#""height": xxxx,
"#, r#""height": xxxx,         (numeric) height of the chain tip
"#).replace(r#""hash": "xxxx",
"#, r#""hash": "xxxx",         (string) block hash of the tip
"#)
    };
}

macro_rules! getdeprecationinfo {
    ($result_data:expr) => {
        $result_data.replace(r#"MagicBean:x.y.z[-v]"#, r#"MagicBean"#)
    };
}

macro_rules! getnetworkinfo {
    ($result_data:expr) => {
        $result_data.replace(r#"MagicBean:x.y.z[-v]"#, r#"MagicBean"#)
    };
}

macro_rules! getpeerinfo {
    ($result_data:expr) => {
        $result_data.replace(r#"MagicBean:x.y.z[-v]"#, r#"MagicBean"#)
    };
}

macro_rules! getspentinfo {
    ($result_data:expr) => {
        $result_data.replace(r#"number"#, r#"numeric"#).replace(
            r#"  ,...
"#,
            r#""#,
        )
    };
}

macro_rules! gettransaction {
    ($result_data:expr) => {
    $result_data.replace(r#"      "nullifiers" : [ string, ... ]      (string) Nullifiers of input notes
      "commitments" : [ string, ... ]     (string) Note commitments for note outputs
      "macs" : [ string, ... ]            (string) Message authentication tags"#,
    r#""nullifiers": [
        "nullifier" (string)
    ],
    "commitments": [
        "commitment" (string)
    ],
    "macs": [
        "mac" (string)
    ],"#).replace(r#",..."#,r#""#).replace(r#", ..."#,r#""#)
    };
}

macro_rules! listaccounts{
    ($result_data:expr) =>  {
    $result_data.replace(r#"                      (json object where keys are account names, and values are numeric balances"#, "")
        .replace(r#"  ...
"#, "")
    };
}

macro_rules! listreceivedbyaccount {
    ($result_data:expr) => {
        $result_data.replace(r#"bool"#, "boolean").replace(
            r#"  ,...
"#,
            "",
        )
    };
}

macro_rules! listreceivedbyaddress {
    ($result_data:expr) => {
        $result_data.replace(r#"bool"#, "boolean").replace(
            r#"  ,...
"#,
            "",
        )
    };
}
macro_rules! listtransactions {
    ($result_data:expr) => {
        $result_data
            .lines()
            .filter(|l| {
                !l.starts_with("                                         ")
            })
            .fold(String::new(), |mut accumulator, new| {
                accumulator.push_str(new);
                accumulator.push_str("\n");
                accumulator
            })
    };
}

macro_rules! getblocktemplate {
    ($result_data:expr) => {
        $result_data.replace(r#"{ ... },           (json object) information for coinbase transaction"#, r#"{     (json object) information for coinbase transaction
            "data":    (hexadecimal)
            "hash":    (hexadecimal)
            "depends":    [
            (numeric)
            ]
            "fee":    (numeric)
            "foundersreward":    (numeric)
            "sigops":    (numeric)
            "required":    (boolean)
            }"#
        ).replace(r#",..."#, r#""#);
    };
}

//TODO enumeration need for params field
macro_rules! z_getoperationresult {
    ($result_data:expr) => {
        $result_data.replace(
            r#""    [object, ...]"      (array) A list of JSON objects"#,
            r#"[
[
  {
    "id":    (string)
    "status":    (string)
    "creation_time":    (numeric)
    "method":    (string)
    "params": {
      "fromaddress":    (string)
      "amounts": [
        {
          "address":    (string)
          "amount":    (numeric)
        }
      ]
      "minconf":    (numeric)
      "fee":    (numeric)
    }
  },
  {
    "id":    (string)
    "status":    (string)
    "creation_time":     (numeric)
    "result": {
      "txid":    (hexadecimal)
    }
    "execution_secs": (numeric)
    "method":    (string)
    "params": {
      "fromaddress":    (string)
      "amounts": [
        {
          "address":    (string)
          "amount":    (numeric)
        }
      ]
      "minconf":    (numeric)
      "fee":    (numeric)
    }
  },
  {
    "id":    (string)
    "status":    (string)
    "creation_time":    (numeric)
    "error": {
      "code":    (numeric)
      "message":    (string)
    }
    "method":     (string)
    "params": {
      "fromaddress":    (string)
      "amounts": [
        {
          "address":    (string)
          "amount":    (numeric)
        }
      ]
      "minconf":    (numeric)
      "fee":    (numeric)
    }
  }
]

]"#,
        );
    };
}

macro_rules! z_getoperationstatus {
    ($result_data:expr) => {
        $result_data.replace(
            r#""    [object, ...]"      (array) A list of JSON objects"#,
            r#"[
[
  {
    "id":    (string)
    "status":    (string)
    "creation_time":    (numeric)
    "method":    (string)
    "params": {
      "fromaddress":    (string)
      "amounts": [
        {
          "address":    (string)
          "amount":    (numeric)
        }
      ]
      "minconf":    (numeric)
      "fee":    (numeric)
    }
  },
  {
    "id":    (string)
    "status":    (string)
    "creation_time":     (numeric)
    "result": {
      "txid":    (hexadecimal)
    }
    "execution_secs": (numeric)
    "method":    (string)
    "params": {
      "fromaddress":    (string)
      "amounts": [
        {
          "address":    (string)
          "amount":    (numeric)
        }
      ]
      "minconf":    (numeric)
      "fee":    (numeric)
    }
  },
{
    "id":    (string)
    "status":    (string)
    "creation_time":    (numeric)
    "error": {
      "code":    (numeric)
      "message":    (string)
    }
    "method":     (string)
    "params": {
      "fromaddress":    (string)
      "amounts": [
        {
          "address":    (string)
          "amount":    (numeric)
        }
      ]
      "minconf":    (numeric)
      "fee":    (numeric)
    }
  }
]

]"#,
        );
    };
}

macro_rules! z_listreceivedbyaddress {
    ($result_data:expr) => {
        $result_data
            .replace(r#" (sprout) : n,"#, r#": n, <sprout> "#)
            .replace(r#" (sapling) : n,"#, r#": n, <sapling> "#)
    };
}

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

macro_rules! dotdotdot {
    ($result_data:expr) => {
        $result_data
            .replace(", ...\n", r#""#)
            .replace(",...\n", r#""#)
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

macro_rules! args_bool {
    ($arguments_data:expr) => {
        $arguments_data.replace(r#"bool"#, r#"boolean"#)
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

macro_rules! args_example_values {
    ($arguments_data:expr) => {
        $arguments_data.replace(r#"
1. "operationid"         (array, optional) A list of operation ids we are interested in.  If not provided, examine all operations known to the node."#, r#"[
        (string, optional)
        ]
            A list of operation ids we are interested in.  If not provided, examine all operations known to the node."#)
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
macro_rules! args_amounts_array {
    ($arguments_data:expr) => {
        $arguments_data.replace(r#"(array, required) An array of json objects representing the amounts to send.
    [{
      "address":address  (string, required) The address is a taddr or zaddr
      "amount":amount    (numeric, required) The numeric amount in ZEC is the value
      "memo":memo        (string, optional) If the address is a zaddr, raw data represented in hexadecimal string format
    }, ... ]"#, r#"[{
      "address":address  (string, required) The address is a taddr or zaddr
      "amount":amount    (numeric, required) The numeric amount in ZEC is the value
      "memo":memo        (string, optional) If the address is a zaddr, raw data represented in hexadecimal string format
    }]"#)
    };
}

macro_rules! getdotdotdot {
    ($arguments_data:expr) => {
        $arguments_data.replace(r#",..."#, r#""#)
    };
}

pub(crate) fn scrub_arguments(
    rpc_name: &str,
    arguments_data: String,
) -> String {
    match rpc_name {
        "getbalance"
        | "getreceivedbyaccount"
        | "getreceivedbyaddress"
        | "gettransaction"
        | "listaccounts"
        | "listreceivedbyaccount"
        | "listreceivedbyaddress"
        | "listsinceblock"
        | "listtransactions"
        | "z_getbalance"
        | "z_gettotalbalance"
        | "z_listaddresses"
        | "z_listunspent" => {
            args_bool!(arguments_data)
        }
        "z_getoperationresult" | "z_getoperationstatus" => {
            args_example_values!(arguments_data)
        }
        "z_mergetoaddress" => {
            args_fromaddresses_array!(arguments_data)
        }
        "z_sendmany" => {
            args_amounts_array!(arguments_data)
        }
        "setban" => {
            setban!(arguments_data)
        }
        "getaddressbalance" => {
            getaddressbalance!(arguments_data)
        }
        "getaddressutxos" | "getaddressmempool" => {
            getdotdotdot!(arguments_data)
        }
        _ => arguments_data,
    }
}

pub(crate) fn scrub_response(rpc_name: String, result_data: String) -> String {
    if rpc_name == "verifytxoutproof".to_string() {
        verifytxoutproof!(result_data)
    } else if rpc_name == "getaddressutxos".to_string() {
        getaddressutxos!(result_data)
    } else if rpc_name == "listunspent".to_string() {
        listunspent!(result_data)
    } else if rpc_name == "z_listunspent".to_string() {
        z_listunspent!(result_data)
    } else if rpc_name == "generate".to_string() {
        generate!(result_data)
    } else if rpc_name == "getblock".to_string() {
        getblock!(result_data)
    } else if rpc_name == "getrawtransaction".to_string() {
        getrawtransaction!(result_data)
    } else if rpc_name == "getblockheader".to_string() {
        getblockheader!(result_data)
    } else if rpc_name == "getrawmempool".to_string() {
        getrawmempool!(result_data)
    } else if rpc_name == "getaddressmempool".to_string() {
        getaddressmempool!(result_data)
    } else if rpc_name == "getchaintips".to_string() {
        getchaintips!(result_data)
    } else if rpc_name == "getblockchaininfo".to_string() {
        getblockchaininfo!(result_data)
    } else if rpc_name == "getblockdeltas".to_string() {
        getblockdeltas!(result_data)
    } else if rpc_name == "getblockhashes".to_string() {
        getblockhashes!(result_data)
    } else if rpc_name == "getdeprecationinfo".to_string() {
        getdeprecationinfo!(result_data)
    } else if rpc_name == "getnetworkinfo".to_string() {
        getnetworkinfo!(result_data)
    } else if rpc_name == "getpeerinfo".to_string() {
        getpeerinfo!(result_data)
    } else if rpc_name == "getspentinfo".to_string() {
        getspentinfo!(result_data)
    } else if rpc_name == "gettransaction".to_string() {
        gettransaction!(result_data)
    } else if rpc_name == "listaccounts".to_string() {
        listaccounts!(result_data)
    } else if rpc_name == "listreceivedbyaccount".to_string() {
        listreceivedbyaccount!(result_data)
    } else if rpc_name == "listreceivedbyaddress".to_string() {
        listreceivedbyaddress!(result_data)
    } else if rpc_name == "listtransactions".to_string() {
        listtransactions!(result_data)
    } else if rpc_name == "z_getoperationresult".to_string() {
        z_getoperationresult!(result_data)
    } else if rpc_name == "z_getoperationstatus".to_string() {
        z_getoperationstatus!(result_data)
    } else if rpc_name == "z_listreceivedbyaddress".to_string() {
        z_listreceivedbyaddress!(result_data)
    } else if rpc_name == "z_validateaddress".to_string() {
        z_validateaddress!(result_data)
    } else if rpc_name == "getblocktemplate".to_string() {
        getblocktemplate!(result_data)
    } else {
        dotdotdot!(result_data)
    }
}
