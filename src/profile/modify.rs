use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::global::{
    Constraints, Guidelines, Links, NCName, Parameters, Parts, Properties, Select,
};

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
struct SetParameter {
    pub param_id: NCName,
    pub class: Option<NCName>,
    pub depends_on: Option<NCName>,
    pub props: Option<Properties>,
    pub links: Option<Links>,
    pub label: Option<String>,
    pub usage: Option<String>,
    pub constraints: Option<Constraints>,
    pub guidelines: Option<Guidelines>,
    pub values: Option<Vec<String>>,
    pub select: Option<Select>,
}
type SetParameters = Vec<SetParameter>;

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Remove {
    #[serde(rename = "by-name")]
    pub by_name: Option<NCName>,
    #[serde(rename = "by-class")]
    pub by_class: Option<NCName>,
    #[serde(rename = "by-id")]
    pub by_id: Option<NCName>,
    #[serde(rename = "by-item-name")]
    pub by_item_name: Option<NCName>,
    #[serde(rename = "by-ns")]
    pub by_ns: Option<NCName>,
}
type Removes = Vec<Remove>;

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Add {
    pub position: Option<String>,
    #[serde(rename = "by-id")]
    pub by_id: Option<NCName>,
    pub title: Option<String>,
    pub params: Option<Parameters>,
    pub props: Option<Properties>,
    pub links: Option<Links>,
    pub parts: Option<Parts>,
}
type Adds = Vec<Add>;

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Alter {
    #[serde(rename = "control-id")]
    pub control_id: NCName,
    pub removes: Option<Removes>,
    pub adds: Option<Adds>,
}
type Alters = Vec<Alter>;

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Modify {
    #[serde(rename = "set-parameters")]
    set_parameters: Option<SetParameters>,
    alters: Option<Alters>,
}
