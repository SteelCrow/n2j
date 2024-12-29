use error_stack::ResultExt;
use roxmltree::Node;
use serde::{Deserialize, Serialize};
use std::net::IpAddr;

use crate::{Attribute, Error, Result};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Address {
    IpAddr(IpAddr),
    MacAddr(String),
}

impl Address {
    pub fn parse(node: Node) -> Result<Self> {
        let addrtype = node
            .attribute("addrtype")
            .ok_or(Error::MissedAttribute)
            .attach_printable(Attribute(("addrtype", "address")))?;

        let addr = node
            .attribute("addr")
            .ok_or(Error::MissedAttribute)
            .attach_printable(Attribute(("addr", "address")))?;

        #[allow(clippy::single_match_else)]
        match addrtype {
            "mac" => Ok(Address::MacAddr(addr.to_string())),
            _ => {
                let addr = addr
                    .parse::<IpAddr>()
                    .change_context(Error::FailedToParseAttribute)
                    .attach_printable(Attribute(("addr", "address")))?;
                Ok(Address::IpAddr(addr))
            }
        }
    }
}
