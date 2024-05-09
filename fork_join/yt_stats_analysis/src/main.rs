pub mod analyzing;

use analyzing::video::Video;
use csv::ReaderBuilder;
use rayon::prelude::*;
use std::{collections::HashMap, env, fs};

const EXTENSION: &str = "csv";

fn main() -> Result<(), String> {
    let mut args = env::args().skip(1);

    let nthreads = args.next().ok_or("Use: cargo run -- [1..]")?;
    env::set_var("RAYON_NUM_THREADS", nthreads);

    let videos = fs::read_dir("data")
        .unwrap()
        .flat_map(|entry| {
            let path = entry.ok()?.path();
            path.is_file().then_some(path)
        })
        .collect::<Vec<_>>()
        .into_par_iter()
        .filter_map(|path| {
            path.extension().filter(|&ext| ext == EXTENSION)?;
            ReaderBuilder::new().from_path(path).ok()
        })
        .map(|reader| {
            let videos = reader.into_deserialize().flatten();
            let mut region_videos = HashMap::new();

            videos.for_each(|video: Video| {
                let views = region_videos.entry(video.channel).or_insert(0);
                *views += video.views;
            });
            region_videos
        })
        .reduce(
            || HashMap::new(),
            |mut acc, region_videos| {
                for (channel, views) in region_videos {
                    let total_views = acc.entry(channel).or_insert(0);
                    *total_views += views;
                }

                acc
            },
        );

    let mut videos: Vec<_> = videos.into_iter().collect();
    videos.sort_by(|(_, v1), (_, v2)| v2.cmp(v1));

    let json = serde_json::to_string_pretty(&videos).unwrap();
    println!("{json}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn csv_testing() {
        let mut reader = ReaderBuilder::new().from_path("data/JPvideos.csv").unwrap();
        for r in reader.records().flatten() {
            println!("{r:?}");
        }
    }
}
