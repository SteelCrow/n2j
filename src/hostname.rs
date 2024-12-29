use crate::{Attribute, Error, Result};
use error_stack::ResultExt;
use roxmltree::Node;
use serde::{Deserialize, Serialize};
use strum_macros::EnumString;

#[derive(EnumString, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[strum(serialize_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum Type {
    User,
    Dns,
    #[strum(serialize = "PTR")]
    Ptr,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Hostname {
    pub name: String,
    #[serde(rename = "type")]
    pub ttype: Type,
}

impl Hostname {
    pub fn parse(node: Node) -> Result<Self> {
        let name = node
            .attribute("name")
            .ok_or(Error::MissedAttribute)
            .attach_printable(Attribute(("name", "hostname")))?
            .to_string();

        let ttype = node
            .attribute("type")
            .ok_or(Error::MissedAttribute)
            .attach_printable(Attribute(("type", "hostname")))
            .map(str::parse::<Type>)?
            .change_context(Error::FailedToParseAttribute)
            .attach_printable(Attribute(("type", "hostname")))?;

        Ok(Hostname { name, ttype })
    }
}
