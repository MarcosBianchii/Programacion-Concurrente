//! Example of the Fork-Join concurrency model by counting lines in files of a given directory.

use std::{
    env,
    fs::{self, File},
    io::{self, BufRead, BufReader},
    path::{Path, PathBuf},
    process,
    thread::{self, JoinHandle},
};

fn count_lines_in_files(dir: &Path) -> io::Result<()> {
    let mut handles: Vec<JoinHandle<io::Result<_>>> = vec![];

    for entry in fs::read_dir(dir)?.flatten() {
        let path = entry.path();

        if entry.file_type()?.is_dir() {
            count_lines_in_files(&path)?;
        } else {
            handles.push(thread::spawn(move || {
                let data = BufReader::new(File::open(&path)?);
                let lines = data.lines().count();
                Ok((entry.file_name(), lines))
            }));
        }
    }

    for (name, line_count) in handles
        .into_iter()
        .filter_map(|thread| thread.join().ok())
        .flatten()
    {
        println!("file: {name:?} has {line_count} lines");
    }

    Ok(())
}

fn main() -> io::Result<()> {
    let dir = env::args()
        .nth(1)
        .map(PathBuf::from)
        .expect("Use: cargo run -- <directory>");

    if !dir.is_dir() {
        eprintln!("Given path is not a directory");
        process::exit(1);
    }

    count_lines_in_files(&dir)
}
