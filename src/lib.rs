#![forbid(unsafe_code)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![allow(clippy::missing_errors_doc)]

pub mod address;
pub mod distance;
pub mod host;
pub mod hostname;
pub mod os;
pub mod ports;
pub mod runstats;
pub mod scaninfo;
pub mod script;
pub mod status;

use error_stack::ResultExt;
use roxmltree::Document;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::{borrow::Cow, fmt};

use crate::host::Host;
use crate::runstats::RunStats;
use crate::scaninfo::ScanInfo;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("invalid XML format")]
    InvalidXml,
    #[error("missing expected element")]
    MissedElement,
    #[error("missing required attribute")]
    MissedAttribute,
    #[error("failed to parse element")]
    FailedToParseElement,
    #[error("failed to parse attribute")]
    FailedToParseAttribute,
    #[error("failed to parse XML document")]
    FailedToParseXml,
    #[error("this tool works only with \"nmap\" report")]
    InvalidScannerType,
}

type Result<T> = error_stack::Result<T, Error>;

#[derive(Debug)]
pub(crate) struct Element((&'static str, &'static str));
#[derive(Debug)]
pub(crate) struct Attribute((&'static str, &'static str));

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (name, parent) = &self.0;
        write!(f, "element: \"{name}\" (parent: \"{parent}\")")
    }
}

impl fmt::Display for Attribute {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (name, parent) = &self.0;
        write!(f, "attribute: \"{name}\" (parent: \"{parent}\")")
    }
}

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NmapRun {
    pub scanner: String,
    pub args: String,
    pub start: u32,
    pub version: String,
    pub xmloutputversion: String,
    pub scaninfos: Option<Vec<ScanInfo>>,
    pub runstats: Option<RunStats>,
    pub hosts: Option<Vec<Host>>,
}

impl NmapRun {
    pub fn parse_and_fix<'a>(xml: impl Into<Cow<'a, str>>) -> Result<Self> {
        let xml = xml.into();
        let mut nmaprun = NmapRun::parse(xml.as_ref());
        if let Err(ref e) = nmaprun {
            if e.downcast_ref::<roxmltree::Error>()
                .is_some_and(|e| matches!(e, roxmltree::Error::UnclosedRootNode))
            {
                nmaprun = NmapRun::parse(&(xml + "\n</nmaprun>"));
            }
        }
        nmaprun
    }

    pub fn parse(xml: &str) -> Result<Self> {
        let opt = roxmltree::ParsingOptions {
            allow_dtd: true,
            ..Default::default()
        };

        let doc = Document::parse_with_options(xml, opt).change_context(Error::FailedToParseXml)?;

        let root_element = doc.root_element();
        if root_element.tag_name().name() != "nmaprun" {
            return Err(Error::MissedElement).attach_printable(Element(("nmaprun", "root")));
        }

        let start = root_element
            .attribute("start")
            .ok_or(Error::MissedAttribute)
            .attach_printable(Attribute(("start", "nmaprun")))
            .map(str::parse::<u32>)?
            .change_context(Error::FailedToParseAttribute)
            .attach_printable(Attribute(("start", "nmaprun")))?;

        let scanner = root_element
            .attribute("scanner")
            .ok_or(Error::MissedAttribute)
            .attach_printable(Attribute(("scanner", "nmaprun")))?
            .to_string();

        // Check that we're working with NMAP only
        if scanner != "nmap" {
            return Err(error_stack::Report::new(Error::InvalidScannerType));
        }

        let args = root_element
            .attribute("args")
            .ok_or(Error::MissedAttribute)
            .attach_printable(Attribute(("args", "nmaprun")))?
            .to_string();

        let version = root_element
            .attribute("version")
            .ok_or(Error::MissedAttribute)
            .attach_printable(Attribute(("version", "nmaprun")))?
            .to_string();

        let xmloutputversion = root_element
            .attribute("xmloutputversion")
            .ok_or(Error::MissedAttribute)
            .attach_printable(Attribute(("xmloutputversion", "nmaprun")))?
            .to_string();

        let parts = || -> Result<Self> {
            let mut scaninfos = Vec::new();
            let mut hosts = Vec::new();
            let mut runstats = None;

            for child in root_element.children() {
                match child.tag_name().name() {
                    "scaninfo" => scaninfos.push(ScanInfo::parse(child)?),
                    "host" => hosts.push(Host::parse(child)?),
                    "runstats" => runstats = Some(RunStats::parse(child)?),
                    _ => {}
                }
            }

            Ok(NmapRun {
                scaninfos: Some(scaninfos).filter(|v| !v.is_empty()),
                hosts: Some(hosts).filter(|v| !v.is_empty()),
                scanner,
                version: version.clone(),
                xmloutputversion: xmloutputversion.clone(),
                args,
                start,
                runstats,
            })
        };

        // Global context of nmap & xml versions for convenient debug
        parts().attach_printable(format!(
            "version: {version}, xmloutputversion: {xmloutputversion}"
        ))
    }
}

#[cfg(test)]
mod test {
    use error_stack::ResultExt;

    use super::NmapRun;
    use std::error::Error;
    use std::ffi::OsStr;
    use std::fs::{self, File};
    use std::io::Read;

    #[test]
    fn parse_reports() -> Result<(), Box<dyn Error>> {
        let mut counter = 0u32;

        for entry in fs::read_dir("reports")? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() && path.extension().is_some_and(|ext| ext == "xml") {
                let mut file = File::open(&path)?;
                let mut content = String::new();
                let filename = path
                    .file_name()
                    .and_then(OsStr::to_str)
                    .unwrap_or("unknown");
                file.read_to_string(&mut content)?;

                let report = NmapRun::parse_and_fix(content)
                    .attach_printable(format!("filename: {filename}"))
                    .attach_printable(format!("checked: {counter}"))?;

                counter += 1;

                println!("{report:#?}");
            }
        }

        Ok(())
    }
}
