use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::global::{Constraints, Guidelines, Links, Properties, Select, Token};

/// Provides information about the publication and availability of the containing document.
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Parameter {
    /// **(deprecated)** Another parameter invoking this one. This construct has been deprecated and should not be used.
    pub id: Token,
    /// **(deprecated)** Another parameter invoking this one. This construct has been deprecated and should not be used.
    pub class: Option<Token>,
    /// **(deprecated)** Another parameter invoking this one. This construct has been deprecated and should not be used.
    pub depends_on: Option<Vec<Token>>,
    /// An attribute, characteristic, or quality of the containing object expressed as a namespace qualified name/value pair. The value of a property is a simple scalar value, which may be expressed as a list of values.
    pub props: Option<Properties>,
    /// Describes the purpose and use of a parameter
    pub links: Option<Links>,
    /// Describes the purpose and use of a parameter
    pub label: Option<String>,
    /// Describes the purpose and use of a parameter
    pub usage: Option<String>,
    /// **(deprecated)** Another parameter invoking this one. This construct has been deprecated and should not be used.
    pub constraints: Option<Constraints>,
    /// A prose statement that provides a recommendation for the use of a parameter.
    pub guidelines: Option<Guidelines>,
    /// A parameter value or set of values.
    pub values: Option<Vec<String>>,

    pub select: Option<Select>,
    /// Additional commentary on the containing object.
    pub remarks: Option<String>,
}

pub type Parameters = Vec<Parameter>;