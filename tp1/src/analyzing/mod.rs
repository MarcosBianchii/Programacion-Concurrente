mod analyzer;
mod entry;
mod inner;

pub use analyzer::process_data;

const NCHATTIEST: usize = 10;
const EXTENSION: &str = "jsonl";
