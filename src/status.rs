use error_stack::ResultExt;
use roxmltree::Node;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use strum_macros::EnumString;

use crate::{Attribute, Error, Result};

#[derive(EnumString, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[strum(serialize_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum HostState {
    Up,
    Down,
    Unknown,
    Skipped,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Status {
    pub state: HostState,
    pub reason: Option<String>,
    pub reason_ttl: Option<u32>,
}

impl Status {
    pub fn parse(node: Node) -> Result<Self> {
        let state = node
            .attribute("state")
            .ok_or(Error::MissedAttribute)
            .attach_printable(Attribute(("state", "status")))
            .map(str::parse::<HostState>)?
            .change_context(Error::FailedToParseAttribute)
            .attach_printable(Attribute(("state", "status")))?;

        let reason = node.attribute("reason").map(str::to_string);

        let reason_ttl = node
            .attribute("reason_ttl")
            .map(str::parse::<u32>)
            .transpose()
            .change_context(Error::FailedToParseAttribute)
            .attach_printable(Attribute(("reasonttl", "status")))?;

        Ok(Status {
            state,
            reason,
            reason_ttl,
        })
    }
}
