use crate::Result;
use rick_and_morty::{character, episode, location};
use securestore::{KeySource, SecretsManager};
use std::path::Path;
use warp::{Filter, Rejection};

pub const JSON_API_KEYS_PATH: &str = "api-keys.json";
pub const SECRET_KEY_PATH: &str = "secret.key";

async fn fetch_location(id: i64) -> std::result::Result<String, Rejection> {
    let res = location::get(id).await.unwrap();
    Ok::<String, Rejection>(serde_json::to_string(&res).unwrap())
}

async fn fetch_location_all() -> std::result::Result<String, Rejection> {
    let res = location::get_all().await.unwrap();
    Ok::<String, Rejection>(serde_json::to_string(&res).unwrap())
}

async fn fetch_character(id: i64) -> std::result::Result<String, Rejection> {
    let res = character::get(id).await.unwrap();
    Ok::<String, Rejection>(serde_json::to_string(&res).unwrap())
}

async fn fetch_character_all() -> std::result::Result<String, Rejection> {
    let res = character::get_all().await.unwrap();
    Ok::<String, Rejection>(serde_json::to_string(&res).unwrap())
}

async fn fetch_episode(id: i64) -> std::result::Result<String, Rejection> {
    let res = episode::get(id).await.unwrap();
    Ok::<String, Rejection>(serde_json::to_string(&res).unwrap())
}

async fn fetch_episode_all() -> std::result::Result<String, Rejection> {
    let res = episode::get_all().await.unwrap();
    Ok::<String, Rejection>(serde_json::to_string(&res).unwrap())
}

pub struct Proxy {
    manager: SecretsManager,
}

impl Proxy {
    pub fn new() -> Self {
        Self {
            manager: SecretsManager::load(
                Path::new(JSON_API_KEYS_PATH),
                KeySource::Path(Path::new(SECRET_KEY_PATH)),
            )
            .expect("Api key JSON file and secret key to exist."),
        }
    }

    pub async fn run(self) -> Result<()> {
        let location_all = warp::path!("location").and_then(fetch_location_all);
        let location_id = warp::path!("location" / i64).and_then(fetch_location);
        let character_all = warp::path!("character").and_then(fetch_character_all);
        let character_id = warp::path!("character" / i64).and_then(fetch_character);
        let episode_all = warp::path!("episode").and_then(fetch_episode_all);
        let episode_id = warp::path!("episode" / i64).and_then(fetch_episode);

        let routes = character_id
            .or(character_all)
            .or(location_id)
            .or(location_all)
            .or(episode_id)
            .or(episode_all);

        warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;

        Ok(())
    }
}
