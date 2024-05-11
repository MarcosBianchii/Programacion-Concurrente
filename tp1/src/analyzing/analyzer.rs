//! This module contains the `process_data` function that initializes the data processing.

use rayon::prelude::*;

use super::entry::Entry;
use super::inner::{site::Site, tag::Tag, totals::Totals, view::View};
use crate::analyzing::EXTENSION;

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    fs::{self, File},
    io::{self, BufRead, BufReader},
    path::{Path, PathBuf},
    str::FromStr,
};

// Returns the file paths in the given directory.
fn get_file_paths(dir: &Path) -> io::Result<impl Iterator<Item = PathBuf>> {
    Ok(fs::read_dir(dir)?.filter_map(|entry| {
        let path = entry.ok()?.path();
        path.is_file().then_some(path)
    }))
}

/// Given an accumulator will add this new `entry Tag` to it. If a `Tag` with the
/// same name already exists, it will sum it's `questions` and `words` counts.
pub(super) fn sum_entry<N>(mut acc: HashMap<N, Tag>, (name, tag): (N, Tag)) -> HashMap<N, Tag>
where
    N: PartialEq + Eq + std::hash::Hash,
{
    let entry = acc.entry(name).or_default();
    entry.questions += tag.questions;
    entry.words += tag.words;
    acc
}

/// Takes the max N elements of an iterator and returns
/// the names of the items in ascending order.
pub(super) fn ntop_names<N, E, I>(elements: I, n: usize) -> impl Iterator<Item = N>
where
    N: Ord,
    E: Ord,
    I: IntoIterator<Item = (N, E)>,
{
    let mut heap = BinaryHeap::with_capacity(n + 1);

    for (name, e) in elements {
        heap.push(Reverse((e, name)));
        if heap.len() > n {
            heap.pop();
        }
    }

    (0..n)
        .scan(heap, |h, _| h.pop())
        .map(|Reverse((_, name))| name)
}

/// Initializes the process of analyzing the data given a directory's path.
pub fn process_data(dir: &Path) -> io::Result<String> {
    let paths: Vec<_> = get_file_paths(dir)?.collect();

    let sites: HashMap<&str, _> = paths
        .par_iter()
        .filter_map(|path| {
            path.extension().filter(|&ext| ext == EXTENSION)?;
            let site_name = path.file_stem()?.to_str()?;
            let file = File::open(&path).ok()?;
            Some((site_name, file))
        })
        .map(|(site_name, file)| {
            let reader = BufReader::new(file);
            let lines = reader.lines().map_while(Result::ok).par_bridge();
            let entries = lines.filter_map(|line| Entry::from_str(&line).ok());
            let site = Site::from(entries);
            (site_name, site)
        })
        .collect();

    let tags: HashMap<&str, _> = sites
        .values()
        .par_bridge()
        .map(|site| site.tags().collect())
        .reduce(HashMap::new, |acc, x| x.into_iter().fold(acc, sum_entry));

    let totals = Totals::new(&sites, &tags);
    let view = View::new(&sites, &tags, totals);

    Ok(view.to_string())
}
