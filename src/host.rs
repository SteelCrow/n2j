use error_stack::ResultExt;
use roxmltree::Node;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    address::Address,
    distance::Distance,
    hostname::Hostname,
    os::Os,
    ports::{Extraports, Port, Ports},
    script::Script,
    status::Status,
    Attribute, Element, Error, Result,
};

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Host {
    pub start_time: Option<u32>,
    pub end_time: Option<u32>,
    pub timed_out: Option<bool>,

    pub status: Status,
    pub addresses: Vec<Address>,

    pub host_names: Option<Vec<Hostname>>,
    pub ports: Option<Vec<Port>>,
    pub extraports: Option<Vec<Extraports>>,
    pub os: Option<Os>,
    pub distance: Option<Distance>,
    pub host_scripts: Option<Vec<Script>>,
}

impl Host {
    pub fn parse(node: Node) -> Result<Self> {
        let start_time = node
            .attribute("starttime")
            .map(str::parse::<u32>)
            .transpose()
            .change_context(Error::FailedToParseAttribute)
            .attach_printable(Attribute(("starttime", "host")))?;

        let end_time = node
            .attribute("endtime")
            .map(str::parse::<u32>)
            .transpose()
            .change_context(Error::FailedToParseAttribute)
            .attach_printable(Attribute(("endtime", "host")))?;

        let timed_out = node
            .attribute("timedout")
            .map(str::parse::<bool>)
            .transpose()
            .change_context(Error::FailedToParseAttribute)
            .attach_printable(Attribute(("timeout", "host")))?;

        let mut status = None;
        let mut addresses = Vec::new();
        let mut host_names = None;
        let mut ports = None;
        let mut extraports = None;
        let mut os = None;
        let mut distance = None;
        let mut host_scripts = None;

        for child in node.children() {
            match child.tag_name().name() {
                "status" => status = Some(Status::parse(child)?),
                "address" => addresses.push(Address::parse(child)?),
                "hostnames" => host_names = parse_host_names_node(child)?,
                "ports" => {
                    let object = Ports::parse(child)?;
                    ports = object.ports;
                    extraports = object.extraports;
                }
                "os" => os = Some(Os::parse(child)?),
                "distance" => distance = Some(Distance::parse(child)?),
                "hostscript" => host_scripts = parse_host_scripts_node(child)?,
                _ => {}
            }
        }

        let status = status
            .ok_or(Error::MissedElement)
            .attach_printable(Element(("status", "host")))?;

        Ok(Host {
            start_time,
            end_time,
            timed_out,

            status,
            addresses,

            host_names,
            ports,
            extraports,
            os,
            distance,
            host_scripts,
        })
    }
}

fn parse_host_scripts_node(node: Node) -> Result<Option<Vec<Script>>> {
    let mut scripts = Vec::new();

    for child in node.children() {
        if child.tag_name().name() == "script" {
            scripts.push(Script::parse(child)?);
        }
    }

    Ok(Some(scripts).filter(|v| !v.is_empty()))
}

fn parse_host_names_node(node: Node) -> Result<Option<Vec<Hostname>>> {
    let mut hostnames = Vec::new();

    for child in node.children() {
        if child.tag_name().name() == "hostname" {
            hostnames.push(Hostname::parse(child)?);
        }
    }

    Ok(Some(hostnames).filter(|v| !v.is_empty()))
}
