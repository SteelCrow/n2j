use error_stack::ResultExt;
use roxmltree::Node;
use serde::{Deserialize, Serialize};

use crate::{Attribute, Error, Result};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Script {
    pub id: String,
    pub output: String,
}

impl Script {
    pub fn parse(node: Node) -> Result<Self> {
        let id = node
            .attribute("id")
            .ok_or(Error::MissedAttribute)
            .attach_printable(Attribute(("id", "script")))?
            .to_string();

        let output = node
            .attribute("output")
            .ok_or(Error::MissedAttribute)
            .attach_printable(Attribute(("output", "script")))?
            .to_string();

        Ok(Script { id, output })
    }
}
