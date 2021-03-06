// rust-xmpp
// Copyright (c) 2014 Florian Zeitz
//
// This project is MIT licensed.
// Please see the COPYING file for more information.

use xml;
use ns;
pub use self::IqType::{Set,Get,Result,Error};

use super::Stanza;

pub enum IqType {
    Set,
    Get,
    Result,
    Error
}

pub struct Iq { elem: xml::Element }

impl_Stanza!("iq", Iq, IqType,
    |ty: &str| {
        match ty {
            "get" => Some(Get),
            "set" => Some(Set),
            "result" => Some(Result),
            "error" => Some(Error),
            _ => None
        }
    }
, None)
