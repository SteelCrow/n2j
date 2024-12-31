use error_stack::ResultExt;
use roxmltree::Node;
use serde::{Deserialize, Serialize};

use crate::{port::PortProtocol, Attribute, Error, Result};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub struct ScanInfo {
    #[serde(rename = "type")]
    pub ttype: String,
    pub protocol: PortProtocol,
    pub numservices: u32,
    pub services: String,
}

impl ScanInfo {
    pub fn parse(node: Node) -> Result<Self> {
        let ttype = node
            .attribute("type")
            .ok_or(Error::MissedAttribute)
            .attach_printable(Attribute(("type", "scaninfo")))?
            .to_string();

        let protocol = node
            .attribute("protocol")
            .ok_or(Error::MissedAttribute)
            .attach_printable(Attribute(("protocol", "scaninfo")))
            .map(str::parse::<PortProtocol>)?
            .change_context(Error::FailedToParseAttribute)
            .attach_printable(Attribute(("protocol", "scaninfo")))?;

        let numservices = node
            .attribute("numservices")
            .ok_or(Error::MissedAttribute)
            .attach_printable(Attribute(("numservices", "scaninfo")))
            .map(str::parse::<u32>)?
            .change_context(Error::FailedToParseAttribute)
            .attach_printable(Attribute(("numservices", "scaninfo")))?;

        let services = node
            .attribute("services")
            .ok_or(Error::MissedAttribute)
            .attach_printable(Attribute(("services", "scaninfo")))?
            .to_string();

        Ok(ScanInfo {
            ttype,
            protocol,
            numservices,
            services,
        })
    }
}
