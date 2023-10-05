use anyhow::{anyhow, Result};
use mail_parser::{parsers::message::IntoByteSlice, Addr, MessageParser};

use std::{convert::From, fs, hash::Hash, path::Path};

#[derive(Debug, Hash, PartialEq, Eq, Default)]
pub(crate) struct Address {
    pub(crate) name: Option<String>,
    pub(crate) addr: String,
}

impl From<Addr<'static>> for Address {
    fn from(m: Addr) -> Self {
        Self {
            name: m.name().map(|n| n.to_string()),
            addr: m.address().map(|n| n.to_string()).unwrap_or_default(),
        }
    }
}

#[derive(Debug)]
pub(crate) struct MailHeader {
    pub(crate) from: Vec<Address>,
    pub(crate) to: Vec<Address>,
    pub(crate) cc: Vec<Address>,
    pub(crate) bcc: Vec<Address>,
}

pub(crate) fn read_eml_headers(file: impl AsRef<Path>) -> Result<MailHeader> {
    let input = fs::read_to_string(file)?;
    let m = MessageParser::default()
        .parse_headers(input.into_byte_slice())
        .ok_or_else(|| anyhow!("Failed to parse"))?;

    // let from = m.from().unwrap().first().unwrap();
    let from = m.from().map(|l| l.clone().into_list()).unwrap_or_default();
    let to = m.to().map(|l| l.clone().into_list()).unwrap_or_default();
    let cc = m.cc().map(|l| l.clone().into_list()).unwrap_or_default();
    let bcc = m.bcc().map(|l| l.clone().into_list()).unwrap_or_default();

    Ok(MailHeader {
        from: from
            .iter()
            .map(|a| Address::from(a.clone().into_owned()))
            .collect(),
        to: to
            .iter()
            .map(|a| Address::from(a.clone().into_owned()))
            .collect(),
        cc: cc
            .iter()
            .map(|a| Address::from(a.clone().into_owned()))
            .collect(),
        bcc: bcc
            .iter()
            .map(|a| Address::from(a.clone().into_owned()))
            .collect(),
    })
}
