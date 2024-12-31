use crate::{
    port::{PortProtocol, PortState},
    Attribute, Element, Error, Result,
};
use error_stack::ResultExt;
use roxmltree::Node;
use serde::Deserialize;
use serde::Serialize;
use serde_with::skip_serializing_none;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Os {
    pub portused: Vec<PortUsed>,
    pub osmatch: Vec<OsMatch>,
    pub osfingerprint: OsFingerprint,
}

impl Os {
    pub fn parse(node: Node) -> Result<Os> {
        let mut os = Os::default();
        for child in node.children() {
            match child.tag_name().name() {
                "portused" => os.portused.push(PortUsed::parse(child)?),
                "osmatch" => os.osmatch.push(OsMatch::parse(child)?),
                "osfingerprint" => os.osfingerprint = OsFingerprint::parse(child)?,
                _ => {}
            }
        }

        Ok(os)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortUsed {
    pub state: PortState,
    pub proto: PortProtocol,
    pub portid: i64,
}

impl PortUsed {
    pub fn parse(node: Node) -> Result<PortUsed> {
        let state = node
            .attribute("state")
            .ok_or(Error::MissedAttribute)
            .attach_printable(Attribute(("state", "portused")))
            .map(str::parse::<PortState>)?
            .change_context(Error::FailedToParseAttribute)
            .attach_printable(Attribute(("state", "portused")))?;

        let proto = node
            .attribute("proto")
            .ok_or(Error::MissedAttribute)
            .attach_printable(Attribute(("proto", "portused")))
            .map(str::parse::<PortProtocol>)?
            .change_context(Error::FailedToParseAttribute)
            .attach_printable(Attribute(("proto", "portused")))?;

        let portid = node
            .attribute("portid")
            .ok_or(Error::MissedAttribute)
            .attach_printable(Attribute(("portid", "portused")))
            .map(str::parse::<i64>)?
            .change_context(Error::FailedToParseAttribute)
            .attach_printable(Attribute(("portid", "portused")))?;

        Ok(PortUsed {
            state,
            proto,
            portid,
        })
    }
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OsMatch {
    pub name: String,
    pub accuracy: i64,
    pub line: Option<i64>,
    pub osclass: Option<Vec<OsClass>>,
}

impl OsMatch {
    pub fn parse(node: Node) -> Result<OsMatch> {
        let name = node
            .attribute("name")
            .ok_or(Error::MissedAttribute)
            .attach_printable(Attribute(("name", "osmatch")))
            .map(str::parse::<String>)?
            .change_context(Error::FailedToParseAttribute)
            .attach_printable(Attribute(("name", "osmatch")))?;

        let accuracy = node
            .attribute("accuracy")
            .ok_or(Error::MissedAttribute)
            .attach_printable(Attribute(("accuracy", "osmatch")))
            .map(str::parse::<i64>)?
            .change_context(Error::FailedToParseAttribute)
            .attach_printable(Attribute(("accuracy", "osmatch")))?;

        let line = node
            .attribute("line")
            .map(str::parse::<i64>)
            .transpose()
            .change_context(Error::FailedToParseAttribute)
            .attach_printable(Attribute(("line", "osmatch")))?;

        let mut osclass = Vec::new();

        for child in node.children() {
            #[allow(clippy::single_match)]
            match child.tag_name().name() {
                "osclass" => {
                    osclass.push(OsClass::parse(child)?);
                }
                _ => {}
            }
        }

        Ok(OsMatch {
            name,
            accuracy,
            line,
            osclass: Some(osclass).filter(|v| !v.is_empty()),
        })
    }
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OsClass {
    #[serde(rename = "type")]
    pub ttype: String,
    pub vendor: String,
    pub osfamily: String,
    pub osgen: Option<String>,
    pub accuracy: i64,
    pub cpe: Option<Vec<Cpe>>,
}

impl OsClass {
    pub fn parse(node: Node) -> Result<OsClass> {
        let ttype = node
            .attribute("type")
            .ok_or(Error::MissedAttribute)
            .attach_printable(Attribute(("type", "osclass")))?
            .to_string();

        let vendor = node
            .attribute("vendor")
            .ok_or(Error::MissedAttribute)
            .attach_printable(Attribute(("vendor", "osclass")))?
            .to_string();

        let osfamily = node
            .attribute("osfamily")
            .ok_or(Error::MissedAttribute)
            .attach_printable(Attribute(("osfamily", "osclass")))?
            .to_string();

        let osgen = node.attribute("osgen").map(Into::into);

        let accuracy = node
            .attribute("accuracy")
            .ok_or(Error::MissedAttribute)
            .attach_printable(Attribute(("accuracy", "osclass")))
            .map(str::parse::<i64>)?
            .change_context(Error::FailedToParseAttribute)
            .attach_printable(Attribute(("accuracy", "osclass")))?;

        let mut cpe = Vec::new();

        for child in node.children() {
            #[allow(clippy::single_match)]
            match child.tag_name().name() {
                "cpe" => cpe.push(Cpe::parse(child)?),
                _ => {}
            }
        }

        Ok(OsClass {
            ttype,
            vendor,
            osfamily,
            osgen,
            accuracy,
            cpe: Some(cpe).filter(|v| !v.is_empty()),
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cpe(String);

impl Cpe {
    pub fn parse(node: Node) -> Result<Cpe> {
        let cpe = node
            .text()
            .ok_or(Error::MissedElement)
            .attach_printable(Element(("cpe", "osclass")))?
            .to_string();

        Ok(Cpe(cpe))
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct OsFingerprint {
    pub fingerprint: String,
}

impl OsFingerprint {
    pub fn parse(node: Node<'_, '_>) -> Result<OsFingerprint> {
        let fingerprint = node
            .attribute("fingerprint")
            .ok_or(Error::MissedAttribute)
            .attach_printable(Attribute(("fingerprint", "osfingerprint")))
            .map(str::parse::<String>)?
            .change_context(Error::FailedToParseAttribute)
            .attach_printable(Attribute(("fingerprint", "osfingerprint")))?;

        Ok(OsFingerprint { fingerprint })
    }
}
