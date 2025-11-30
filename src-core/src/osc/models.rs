use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OSCValue {
    pub kind: SupportedOscType,
    pub value: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OSCMessage {
    pub address: String,
    pub values: Vec<OSCValue>,
}
#[derive(Serialize_repr, Deserialize_repr, Debug,Clone, Copy)]
#[repr(u8)]
pub enum SupportedOscType {
    Int=1,
    Float=2,
    Boolean=3,
    String=4,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OSCMethod {
    pub address: String,
    pub ad_type: OSCMethodAccessType,
    /// Only required for "Read" advertisement types.
    pub value_type: Option<SupportedOscType>,
    /// Only required for "Read" advertisement types. (Serialized)
    pub value: Option<String>,
    //, Optional human readable description
    pub description: Option<String>,
}
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum OSCMethodAccessType {
    /// External applications can only write to this value
    Write,
    /// External applications can only read this value
    Read,
    /// External applications can both read from- and write to this value
    ReadWrite,
}

