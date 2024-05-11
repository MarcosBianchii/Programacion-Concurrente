//! Site struct models the stats of a site in the data.

use rayon::prelude::*;
use serde::Serialize;

use super::{super::entry::Entry, tag::Tag};
use crate::analyzing::analyzer::{ntop_names, sum_entry};
use crate::analyzing::NCHATTIEST;

use std::{cmp::Ordering, collections::HashMap};

#[derive(Serialize, Eq)]
pub struct Site {
    questions: usize,
    words: usize,
    tags: HashMap<String, Tag>,
    chatty_tags: Vec<String>,
}

impl PartialEq for Site {
    fn eq(&self, other: &Self) -> bool {
        (self.score() - other.score()).abs() < f32::EPSILON
    }
}

impl PartialOrd for Site {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Site {
    fn cmp(&self, other: &Self) -> Ordering {
        let score_a = (100.0 * self.score()) as u64;
        let score_b = (100.0 * other.score()) as u64;
        score_a.cmp(&score_b)
    }
}

impl<I> From<I> for Site
where
    I: IntoParallelIterator<Item = Entry>,
{
    fn from(entries: I) -> Self {
        let (questions, words, tags) = entries
            .into_par_iter()
            .map(|entry| {
                let (words, tags) = entry.consume();

                let tags: HashMap<_, _> = tags
                    .into_iter()
                    .map(|name| (name, Tag::with(1, words)))
                    .collect();

                (1, words, tags)
            })
            .reduce(
                || (0, 0, HashMap::new()),
                |(site_qs, site_ws, site_ts), (xqs, xws, xts)| {
                    let site_ts = xts.into_iter().fold(site_ts, sum_entry);
                    (site_qs + xqs, site_ws + xws, site_ts)
                },
            );

        let mut chatty_tags: Vec<_> = ntop_names(&tags, NCHATTIEST)
            .map(|name| name.to_string())
            .collect();

        chatty_tags.reverse();

        Self {
            words,
            questions,
            chatty_tags,
            tags,
        }
    }
}

impl Site {
    /// Calculates the scoring given the amount of
    /// words and questions for this site.
    pub fn score(&self) -> f32 {
        self.words as f32 / self.questions as f32
    }

    /// Returns an iterator over the tags in the site.
    pub fn tags(&self) -> impl Iterator<Item = (&str, Tag)> {
        self.tags.iter().map(|(name, &tag)| (name.as_ref(), tag))
    }
}
