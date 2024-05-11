//! View struct models the stat result of the whole data processing.

use serde::Serialize;

use super::{site::Site, tag::Tag, totals::Totals};

use std::{
    collections::HashMap,
    fmt::{self, Display},
};

#[derive(Serialize)]
pub struct View<'a> {
    #[serde(rename = "padron")]
    id: usize,
    sites: &'a HashMap<&'a str, Site>,
    tags: &'a HashMap<&'a str, Tag>,
    totals: Totals<'a>,
}

impl<'a> Display for View<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match serde_json::to_string_pretty(self) {
            Ok(json) => write!(f, "{json}"),
            Err(_) => fmt::Result::Err(fmt::Error),
        }
    }
}

impl<'a> View<'a> {
    pub fn new(
        sites: &'a HashMap<&'a str, Site>,
        tags: &'a HashMap<&'a str, Tag>,
        totals: Totals<'a>,
    ) -> Self {
        Self {
            id: 108921,
            sites,
            tags,
            totals,
        }
    }
}
