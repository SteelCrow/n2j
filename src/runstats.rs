use error_stack::ResultExt;
use roxmltree::Node;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{Attribute, Error, Result};

#[skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct RunStats {
    pub finished: Option<Finished>,
    pub hosts: Option<Hosts>,
}

impl RunStats {
    pub fn parse(node: Node) -> Result<RunStats> {
        let mut runstats = RunStats::default();
        for child in node.children() {
            match child.tag_name().name() {
                "finished" => runstats.finished = Finished::parse(child).map(Option::Some)?,
                "hosts" => runstats.hosts = Hosts::parse(child).map(Option::Some)?,
                _ => {}
            }
        }

        Ok(runstats)
    }
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Finished {
    pub time: i64,
    pub elapsed: Option<f64>,
}

impl Finished {
    pub fn parse(node: Node) -> Result<Finished> {
        let time = node
            .attribute("time")
            .ok_or(Error::MissedAttribute)
            .attach_printable(Attribute(("time", "finished")))
            .map(str::parse::<i64>)?
            .change_context(Error::FailedToParseAttribute)
            .attach_printable(Attribute(("time", "finished")))?;

        let elapsed = node
            .attribute("elapsed")
            .map(str::parse::<f64>)
            .transpose()
            .change_context(Error::FailedToParseAttribute)
            .attach_printable(Attribute(("elapsed", "finished")))?;

        Ok(Finished { time, elapsed })
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Hosts {
    pub up: i64,
    pub down: i64,
    pub total: i64,
}

impl Hosts {
    pub fn parse(node: Node) -> Result<Hosts> {
        let up = node
            .attribute("up")
            .ok_or(Error::MissedAttribute)
            .attach_printable(Attribute(("up", "hosts")))
            .map(str::parse::<i64>)?
            .change_context(Error::FailedToParseAttribute)
            .attach_printable(Attribute(("up", "hosts")))?;

        let down = node
            .attribute("down")
            .ok_or(Error::MissedAttribute)
            .attach_printable(Attribute(("down", "hosts")))
            .map(str::parse::<i64>)?
            .change_context(Error::FailedToParseAttribute)
            .attach_printable(Attribute(("down", "hosts")))?;

        let total = node
            .attribute("total")
            .ok_or(Error::MissedAttribute)
            .attach_printable(Attribute(("total", "hosts")))
            .map(str::parse::<i64>)?
            .change_context(Error::FailedToParseAttribute)
            .attach_printable(Attribute(("total", "hosts")))?;

        Ok(Hosts { up, down, total })
    }
}
