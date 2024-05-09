use futures::{future::join_all, FutureExt};
use reqwest::{header::AUTHORIZATION, Client};
use serde::Deserialize;
use std::{collections::HashMap, error::Error};
use tokio::fs;

#[derive(Deserialize)]
struct Token {
    access_token: String,
    token_type: String,
}

#[derive(Deserialize)]
struct AlbumId {
    name: String,
    id: String,
}

#[derive(Deserialize)]
struct NewReleases {
    items: Vec<AlbumId>,
}

#[derive(Deserialize)]
struct AlbumCollection {
    albums: NewReleases,
}

#[derive(Deserialize)]
struct Song {
    name: String,
}

#[derive(Deserialize)]
struct AlbumSongs {
    #[serde(rename = "items")]
    songs: Vec<Song>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();

    // Get access token.
    let client_id = fs::read_to_string("client_id.txt").await?;
    let client_secret = fs::read_to_string("client_secret.txt").await?;

    let params = HashMap::from([
        ("grant_type", "client_credentials"),
        ("client_id", &client_id),
        ("client_secret", &client_secret),
    ]);

    let response = client
        .post("https://accounts.spotify.com/api/token")
        .form(&params)
        .send()
        .await?;

    let Token {
        access_token,
        token_type,
        ..
    } = serde_json::from_str(&response.text().await?)?;

    // Get new releases.
    let response = client
        .get("https://api.spotify.com/v1/browse/new-releases")
        .header(AUTHORIZATION, format!("{token_type} {access_token}"))
        .send()
        .await?;

    let AlbumCollection { albums, .. } = serde_json::from_str(&response.text().await?)?;
    let new_releases = albums.items;

    // Get the album data.
    let album_data: Vec<_> = new_releases
        .into_iter()
        .map(|AlbumId { id, name, .. }| {
            client
                .get(format!("https://api.spotify.com/v1/albums/{id}/tracks"))
                .header(AUTHORIZATION, format!("{token_type} {access_token}"))
                .send()
                .map(|res| (name, res))
        })
        .collect();

    let album_bodies_futs: Vec<_> = join_all(album_data)
        .await
        .into_iter()
        .filter_map(|(name, res)| Some(res.ok()?.text().map(|res| (name, res))))
        .collect();

    let album_bodies = join_all(album_bodies_futs)
        .await
        .into_iter()
        .filter_map(|(name, album)| Some((name, album.ok()?)));

    // Print the song names for each album.
    for (name, album) in album_bodies {
        let AlbumSongs { songs } = serde_json::from_str(&album)?;
        println!("{name}:");
        songs.iter().for_each(|Song { name }| println!("{name}"));
        println!();
    }

    Ok(())
}
