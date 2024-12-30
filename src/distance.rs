use error_stack::ResultExt;
use roxmltree::Node;
use serde::{Deserialize, Serialize};

use crate::{Attribute, Error, Result};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Distance {
    pub value: u32,
}

impl Distance {
    pub fn parse(node: Node) -> Result<Self> {
        let value = node
            .attribute("value")
            .ok_or(Error::MissedAttribute)
            .attach_printable(Attribute(("value", "distance")))
            .map(str::parse::<u32>)?
            .change_context(Error::FailedToParseAttribute)
            .attach_printable(Attribute(("value", "distance")))?;

        Ok(Distance { value })
    }
}
