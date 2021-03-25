macro_rules! getaddressdeltas {
    ($result_data:expr) => {
        $result_data
            .replace(r#"(or, if chainInfo is true):"#, "Result:")
            .replace("number", "numeric")
            .replace(", ...", "")
            .replace(
                r#"  "start":
    {
      "hash"          (string)  The start block hash
      "height"        (numeric) The height of the start block
    }"#,
                r#"  "start":
    {
      "hash":         (string)  The start block hash
      "height":       (numeric) The height of the start block
    }"#,
            )
            .replace(
                r#"  "end":
    {
      "hash"          (string)  The end block hash
      "height"        (numeric) The height of the end block
    }"#,
                r#"  "end":
    {
      "hash":         (string)  The end block hash
      "height":       (numeric) The height of the end block
    }"#,
            )
    };
}

macro_rules! getaddressutxos {
    ($result_data:expr) => {
        $result_data
            .replace(r#"(or, if chainInfo is true):"#, "Result:")
            .replace("number", "numeric")
            .replace(", ...", "")
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
            .replace(",...", "")
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

macro_rules! getblockchaininfo {
    ($result_data:expr) => {
    $result_data.replace("[0..1]", "").replace(
        "{ ... }      (object) progress toward rejecting pre-softfork blocks",
        "{
\"status\": (boolean)
\"found\": (numeric)
\"required\": (numeric)
\"window\": (numeric)
}").replace("(same fields as \"enforce\")", "").replace(", ...", "")
    };
}

macro_rules! getblockdeltas {
    ($result_data:expr) => {
        $result_data
            .replace(r#"hex string"#, r#"hexadecimal"#)
            .replace(r#"hexstring"#, r#"hexadecimal"#)
            .replace(r#", ..."#, r#""#)
    };
}

macro_rules! getblockhashes {
    ($result_data:expr) => {
        $result_data
            .replace(r#"hex string"#, r#"hexadecimal"#)
            .replace(r#"hexstring"#, r#"hexadecimal"#)
            .replace(r#", ..."#, r#""#)
    };
}

macro_rules! getchaintips {
    ($result_data:expr) => {
    $result_data.replace(
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
        $result_data
            .replace(r#"MagicBean:x.y.z[-v]"#, r#"MagicBean"#)
            .replace(r#",..."#, r#""#)
    };
}

macro_rules! getpeerinfo {
    ($result_data:expr) => {
        $result_data
            .replace(r#"MagicBean:x.y.z[-v]"#, r#"MagicBean"#)
            .replace(r#",..."#, r#""#)
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

// TODO the submitblock macro returns an ad-hoc string rather
// than following a suggested pattern for amending help output
macro_rules! submitblock {
    ($result_data:expr) => {
        r#""(enum: duplicate, duplicate-invalid, duplicate-inconclusive, inconclusive, rejected)""#.to_string()
    };
}

const INSUFFICIENT: &str = r#"INSUFFICIENT_INFORMATION
Result:
"do_not_use_this": (INSUFFICIENT)

Examples:
None"#;
macro_rules! getblocktemplate {
    ($result_data:expr) => {
        INSUFFICIENT.to_string()
    };
}

macro_rules! z_getoperationresult {
    ($result_data:expr) => {
        INSUFFICIENT.to_string()
    };
}

macro_rules! z_getoperationstatus {
    ($result_data:expr) => {
        INSUFFICIENT.to_string()
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

//TODO turn into individual scrubbers
macro_rules! dotdotdot {
    ($result_data:expr) => {
        $result_data
            .replace(", ...\n", r#""#)
            .replace(",...\n", r#""#)
    };
}

pub(crate) fn scrub(cmd_name: String, result_data: String) -> String {
    if cmd_name == "getaddressdeltas".to_string() {
        getaddressdeltas!(result_data)
    } else if cmd_name == "getaddressutxos".to_string() {
        getaddressutxos!(result_data)
    } else if cmd_name == "getblock".to_string() {
        let x = getblock!(result_data);
        println!("{}", x);
        x
    } else if cmd_name == "getrawtransaction".to_string() {
        getrawtransaction!(result_data)
    } else if cmd_name == "getblockheader".to_string() {
        getblockheader!(result_data)
    } else if cmd_name == "getrawmempool".to_string() {
        getrawmempool!(result_data)
    } else if cmd_name == "getaddressmempool".to_string() {
        getaddressmempool!(result_data)
    } else if cmd_name == "getchaintips".to_string() {
        getchaintips!(result_data)
    } else if cmd_name == "getblockchaininfo".to_string() {
        getblockchaininfo!(result_data)
    } else if cmd_name == "getblockdeltas".to_string() {
        getblockdeltas!(result_data)
    } else if cmd_name == "getblockhashes".to_string() {
        getblockhashes!(result_data)
    } else if cmd_name == "getdeprecationinfo".to_string() {
        getdeprecationinfo!(result_data)
    } else if cmd_name == "getnetworkinfo".to_string() {
        getnetworkinfo!(result_data)
    } else if cmd_name == "getpeerinfo".to_string() {
        getpeerinfo!(result_data)
    } else if cmd_name == "getspentinfo".to_string() {
        getspentinfo!(result_data)
    } else if cmd_name == "gettransaction".to_string() {
        gettransaction!(result_data)
    } else if cmd_name == "listaccounts".to_string() {
        listaccounts!(result_data)
    } else if cmd_name == "listreceivedbyaccount".to_string() {
        listreceivedbyaccount!(result_data)
    } else if cmd_name == "listreceivedbyaddress".to_string() {
        listreceivedbyaddress!(result_data)
    } else if cmd_name == "listtransactions".to_string() {
        listtransactions!(result_data)
    } else if cmd_name == "submitblock".to_string() {
        submitblock!(result_data)
    } else if cmd_name == "z_getoperationresult".to_string() {
        z_getoperationresult!(result_data)
    } else if cmd_name == "z_getoperationstatus".to_string() {
        z_getoperationstatus!(result_data)
    } else if cmd_name == "z_listreceivedbyaddress".to_string() {
        z_listreceivedbyaddress!(result_data)
    } else if cmd_name == "z_validateaddress".to_string() {
        z_validateaddress!(result_data)
    } else if cmd_name == "getblocktemplate".to_string() {
        getblocktemplate!(result_data)
    } else {
        dotdotdot!(result_data)
    }
}
