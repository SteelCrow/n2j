use error_stack::ResultExt;
use roxmltree::Node;
use serde::{Deserialize, Serialize};

use crate::{
    address::Address, distance::Distance, hostname::Hostname, os::Os, ports::Ports, script::Script,
    status::Status, Attribute, Element, Error, Result,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Host {
    pub start_time: Option<u32>,
    pub end_time: Option<u32>,
    pub timed_out: Option<bool>,

    pub status: Status,
    pub addresses: Vec<Address>,

    pub host_names: Option<Vec<Hostname>>,
    pub ports: Option<Ports>,
    pub os: Option<Os>,
    pub distance: Option<Distance>,
    pub host_scripts: Option<Vec<Script>>,
}

impl Host {
    pub(crate) fn parse(node: Node) -> Result<Self> {
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

        let mut status: Option<Status> = None;
        let mut addresses: Vec<Address> = Vec::new();
        let mut host_names = None;
        let mut ports = None;
        let mut os = None;
        let mut distance = None;
        let mut host_scripts = None;

        for child in node.children() {
            match child.tag_name().name() {
                "status" => status = Some(Status::parse(child)?),
                "address" => addresses.push(Address::parse(child)?),
                "hostnames" => host_names = Some(parse_host_names_node(child)?),
                "ports" => ports = Some(Ports::parse(child)?),
                "os" => os = Some(Os::parse(child)?),
                "distance" => distance = Some(Distance::parse(child)?),
                "hostscript" => host_scripts = Some(parse_host_scripts_node(child)?),
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
            os,
            distance,
            host_scripts,
        })
    }
}

fn parse_host_scripts_node(node: Node) -> Result<Vec<Script>> {
    let mut r = Vec::new();

    for child in node.children() {
        if child.tag_name().name() == "script" {
            r.push(Script::parse(child)?);
        }
    }

    Ok(r)
}

fn parse_host_names_node(node: Node) -> Result<Vec<Hostname>> {
    let mut r = Vec::new();

    for child in node.children() {
        if child.tag_name().name() == "hostname" {
            r.push(Hostname::parse(child)?);
        }
    }

    Ok(r)
}
