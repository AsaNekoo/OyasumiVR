use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OSCValue {
    pub kind: String,
    pub value: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OSCMessage {
    pub address: String,
    pub values: Vec<OSCValue>,
}
#[derive(Serialize, Deserialize, Debug)]
pub enum SupportedOscType {
    Int,
    Float,
    Boolean,
    String,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OSCMethod {
    pub address: String,
    pub ad_type: OSCMethodAccessType,
    /// Only required for "Read" advertisement types.
    pub value_type: Option<OSCMethodValueType>,
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

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum OSCMethodValueType {
    Bool,
    Int,
    Float,
    String,
}
