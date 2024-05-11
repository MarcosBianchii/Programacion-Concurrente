//! Totals struct calculates the top N chattiest sites and tags.

use serde::Serialize;

use super::{site::Site, tag::Tag};
use crate::analyzing::analyzer::ntop_names;
use crate::analyzing::NCHATTIEST;

#[derive(Serialize)]
pub struct Totals<'a> {
    chatty_sites: Vec<&'a str>,
    chatty_tags: Vec<&'a str>,
}

impl<'a> Totals<'a> {
    /// Instanciates a new `Totals` struct. Calculates the
    /// chattiest sites and tags.
    pub fn new<S, T>(sites: S, tags: T) -> Self
    where
        S: Send + IntoIterator<Item = (&'a &'a str, &'a Site)>,
        T: Send + IntoIterator<Item = (&'a &'a str, &'a Tag)>,
    {
        let (mut chatty_sites, mut chatty_tags): (Vec<_>, Vec<_>) = rayon::join(
            || ntop_names(sites, NCHATTIEST).map(|&n| n).collect(),
            || ntop_names(tags, NCHATTIEST).map(|&n| n).collect(),
        );

        chatty_sites.reverse();
        chatty_tags.reverse();

        Self {
            chatty_sites,
            chatty_tags,
        }
    }
}
