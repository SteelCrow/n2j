use error_stack::ResultExt;
use roxmltree::Node;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::str::FromStr;
use strum_macros::EnumString;

use crate::{Attribute, Error, Result};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[skip_serializing_none]
pub struct Ports {
    pub ports: Vec<Port>,
    pub extraports: Option<Vec<Extraports>>,
}

impl Ports {
    pub(crate) fn parse(node: Node) -> Result<Self> {
        let mut ports = Vec::new();
        let mut extraports = Vec::new();

        for child in node.children() {
            match child.tag_name().name() {
                "port" => ports.push(Port::parse(child)?),
                "extraports" => extraports.push(Extraports::parse(child)?),
                _ => {}
            }
        }

        Ok(Ports {
            ports,
            extraports: Some(extraports).filter(|x| !x.is_empty()),
        })
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[skip_serializing_none]
pub struct Extraports {
    pub state: PortState,
    pub count: u32,
    pub extrareasons: Option<Vec<Extrareasons>>,
}

impl Extraports {
    pub fn parse(node: Node) -> Result<Self> {
        let state = node
            .attribute("state")
            .ok_or(Error::MissedAttribute)
            .attach_printable(Attribute(("state", "extraports")))
            .map(PortState::from_str)?
            .change_context(Error::FailedToParseAttribute)
            .attach_printable(Attribute(("state", "extraports")))?;

        let count = node
            .attribute("count")
            .ok_or(Error::MissedAttribute)
            .attach_printable(Attribute(("count", "extraports")))
            .map(str::parse::<u32>)?
            .change_context(Error::FailedToParseAttribute)
            .attach_printable(Attribute(("count", "extraports")))?;

        let mut extrareasons = Vec::new();

        for child in node.children() {
            #[allow(clippy::single_match)]
            match child.tag_name().name() {
                "extrareasons" => extrareasons.push(Extrareasons::parse(child)?),
                _ => {}
            }
        }

        Ok(Extraports {
            state,
            count,
            extrareasons: Some(extrareasons).filter(|x| !x.is_empty()),
        })
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[skip_serializing_none]
pub struct Extrareasons {
    pub reason: String,
    pub count: u32,
    pub proto: Option<PortProtocol>,
    pub ports: Option<String>,
}

impl Extrareasons {
    pub fn parse(node: Node) -> Result<Self> {
        let reason = node
            .attribute("reason")
            .ok_or(Error::MissedAttribute)
            .attach_printable(Attribute(("reason", "extrareasons")))?
            .to_string();

        let count = node
            .attribute("count")
            .ok_or(Error::MissedAttribute)
            .attach_printable(Attribute(("count", "extrareasons")))
            .map(str::parse::<u32>)?
            .change_context(Error::FailedToParseAttribute)
            .attach_printable(Attribute(("count", "extrareasons")))?;

        let proto = node
            .attribute("proto")
            .map(str::parse::<PortProtocol>)
            .transpose()
            .change_context(Error::FailedToParseAttribute)
            .attach_printable(Attribute(("proto", "extrareasons")))?;

        let ports = node.attribute("ports").map(Into::into);

        Ok(Extrareasons {
            reason,
            count,
            proto,
            ports,
        })
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[skip_serializing_none]
pub struct Port {
    pub protocol: PortProtocol,
    pub port_number: u16,
    pub status: PortStatus,
    pub service: Option<Service>,
}

impl Port {
    fn parse(node: Node) -> Result<Self> {
        let protocol = node
            .attribute("protocol")
            .ok_or(Error::MissedAttribute)
            .attach_printable(Attribute(("protocol", "port")))
            .map(str::parse::<PortProtocol>)?
            .change_context(Error::FailedToParseAttribute)
            .attach_printable(Attribute(("protocol", "port")))?;

        let port_number = node
            .attribute("portid")
            .ok_or(Error::MissedAttribute)
            .attach_printable(Attribute(("portid", "port")))
            .map(str::parse::<u16>)?
            .change_context(Error::FailedToParseAttribute)
            .attach_printable(Attribute(("portid", "port")))?;

        let mut status = None;
        let mut service = None;

        for child in node.children() {
            match child.tag_name().name() {
                "state" => status = Some(PortStatus::parse(child)?),
                "service" => service = Some(Service::parse(child)?),
                _ => {}
            }
        }

        let status = status
            .ok_or(Error::MissedAttribute)
            .attach_printable(Attribute(("status", "port")))?;

        Ok(Port {
            protocol,
            port_number,
            status,
            service,
        })
    }
}

#[derive(EnumString, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum PortProtocol {
    Ip,
    Tcp,
    Udp,
    Sctp,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[skip_serializing_none]
pub struct PortStatus {
    pub state: PortState,
    pub reason: Option<String>,
    pub reason_ttl: Option<u32>,
}

impl PortStatus {
    fn parse(node: Node) -> Result<Self> {
        let state = node
            .attribute("state")
            .ok_or(Error::MissedAttribute)
            .attach_printable(Attribute(("state", "portstatus")))
            .map(str::parse::<PortState>)?
            .change_context(Error::FailedToParseAttribute)
            .attach_printable(Attribute(("state", "portstatus")))?;

        let reason = node.attribute("reason").map(str::to_string);

        let reason_ttl = node
            .attribute("reason_ttl")
            .map(str::parse::<u32>)
            .transpose()
            .change_context(Error::FailedToParseAttribute)
            .attach_printable(Attribute(("reason_ttl", "portstatus")))?;

        Ok(PortStatus {
            state,
            reason,
            reason_ttl,
        })
    }
}

#[derive(EnumString, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum PortState {
    Open,
    Closed,
    Filtered,
    Unfiltered,
    #[strum(serialize = "open|filtered")]
    OpenFiltered,
    #[strum(serialize = "closed|filtered")]
    ClosedFiltered,
    Unknown,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[skip_serializing_none]
pub struct Service {
    pub name: String,
    pub product: Option<String>,
    pub extra_info: Option<String>,
    pub confidence_level: u8,
    pub method: Option<ServiceMethod>,
}

impl Service {
    fn parse(node: Node) -> Result<Self> {
        let name = node
            .attribute("name")
            .ok_or(Error::MissedAttribute)
            .attach_printable(Attribute(("name", "servicebnfo")))?
            .to_string();

        let product = node
            .attribute("product")
            .map(str::parse::<String>)
            .transpose()
            .change_context(Error::FailedToParseAttribute)
            .attach_printable(Attribute(("product", "service")))?;

        let extra_info = node
            .attribute("extrainfo")
            .map(str::parse::<String>)
            .transpose()
            .change_context(Error::FailedToParseAttribute)
            .attach_printable(Attribute(("extrainfo", "service")))?;

        let confidence_level = node
            .attribute("conf")
            .ok_or(Error::MissedAttribute)
            .attach_printable(Attribute(("conf", "service")))
            .map(str::parse::<u8>)?
            .change_context(Error::FailedToParseAttribute)
            .attach_printable(Attribute(("conf", "service")))?;

        let method = node
            .attribute("method")
            .map(str::parse::<ServiceMethod>)
            .transpose()
            .change_context(Error::FailedToParseAttribute)
            .attach_printable(Attribute(("method", "service")))?;

        Ok(Service {
            name,
            product,
            extra_info,
            confidence_level,
            method,
        })
    }
}

#[derive(EnumString, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum ServiceMethod {
    Table,
    Probed,
    Detection,
}
