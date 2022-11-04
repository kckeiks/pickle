use crate::Result;
use moka::future::Cache;
use rick_and_morty::{character, episode, location};
use securestore::{KeySource, SecretsManager};
use std::path::Path;
use warp::http::StatusCode;
use warp::reply::Response;
use warp::{reply, Filter, Reply};

pub const JSON_API_KEYS_PATH: &str = "api-keys.json";
pub const SECRET_KEY_PATH: &str = "secret.key";

async fn fetch_location(id: i64, cache: Cache<String, String>) -> Response {
    let key = format!("location/{}", id);
    if let Some(val) = cache.get(&key) {
        return Response::new(val.into());
    }
    let res = match location::get(id).await {
        Ok(res) => res,
        Err(_) => {
            return reply::with_status("INTERNAL_SERVER_ERROR", StatusCode::INTERNAL_SERVER_ERROR)
                .into_response()
        }
    };

    let val = serde_json::to_string(&res).expect("Upstream data to be valid JSON.");
    cache.insert(key, val.clone()).await;
    Response::new(val.into())
}

async fn fetch_location_all(cache: Cache<String, String>) -> Response {
    let key = String::from("location-all");
    if let Some(val) = cache.get(&key) {
        return Response::new(val.into());
    }
    let res = match location::get_all().await {
        Ok(res) => res,
        Err(_) => {
            return reply::with_status("INTERNAL_SERVER_ERROR", StatusCode::INTERNAL_SERVER_ERROR)
                .into_response()
        }
    };
    let val = serde_json::to_string(&res).expect("Upstream data to be valid JSON.");
    cache.insert(key, val.clone()).await;
    Response::new(val.into())
}

async fn fetch_character(id: i64, cache: Cache<String, String>) -> Response {
    let key = format!("character/{}", id);
    if let Some(val) = cache.get(&key) {
        return Response::new(val.into());
    }
    let res = match character::get(id).await {
        Ok(res) => res,
        Err(_) => {
            return reply::with_status("INTERNAL_SERVER_ERROR", StatusCode::INTERNAL_SERVER_ERROR)
                .into_response()
        }
    };
    let val = serde_json::to_string(&res).unwrap();
    cache.insert(key, val.clone()).await;
    Response::new(val.into())
}

async fn fetch_character_all(cache: Cache<String, String>) -> Response {
    let key = String::from("character-all");
    if let Some(val) = cache.get(&key) {
        return Response::new(val.into());
    }
    let res = match character::get_all().await {
        Ok(res) => res,
        Err(_) => {
            return reply::with_status("INTERNAL_SERVER_ERROR", StatusCode::INTERNAL_SERVER_ERROR)
                .into_response()
        }
    };
    let val = serde_json::to_string(&res).unwrap();
    cache.insert(key, val.clone()).await;
    Response::new(val.into())
}

async fn fetch_episode(id: i64, cache: Cache<String, String>) -> Response {
    let key = format!("episode/{}", id);
    if let Some(val) = cache.get(&key) {
        return Response::new(val.into());
    }
    let res = match episode::get(id).await {
        Ok(res) => res,
        Err(_) => {
            return reply::with_status("INTERNAL_SERVER_ERROR", StatusCode::INTERNAL_SERVER_ERROR)
                .into_response()
        }
    };
    let val = serde_json::to_string(&res).unwrap();
    cache.insert(key, val.clone()).await;
    Response::new(val.into())
}

async fn fetch_episode_all(cache: Cache<String, String>) -> Response {
    let key = String::from("episode-all");
    if let Some(val) = cache.get(&key) {
        return Response::new(val.into());
    }
    let res = match episode::get_all().await {
        Ok(res) => res,
        Err(_) => {
            return reply::with_status("INTERNAL_SERVER_ERROR", StatusCode::INTERNAL_SERVER_ERROR)
                .into_response()
        }
    };
    let val = serde_json::to_string(&res).unwrap();
    cache.insert(key, val.clone()).await;
    Response::new(val.into())
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
        let cache = Cache::new(1000);

        // Cloning the cache is cheap according to moka docs.
        let cache_location = cache.clone();
        let location_all = warp::path!("location").then(move || {
            let cache = cache_location.clone();
            async move { fetch_location_all(cache).await }
        });

        let cache_location = cache.clone();
        let location_id = warp::path!("location" / i64).then(move |id: i64| {
            let cache = cache_location.clone();
            async move { fetch_location(id, cache).await }
        });

        let cache_character = cache.clone();
        let character_all = warp::path!("character").then(move || {
            let cache = cache_character.clone();
            async move { fetch_character_all(cache).await }
        });

        let cache_character = cache.clone();
        let character_id = warp::path!("character" / i64).then(move |id: i64| {
            let cache = cache_character.clone();
            async move { fetch_character(id, cache).await }
        });

        let cache_episode = cache.clone();
        let episode_all = warp::path!("episode").then(move || {
            let cache = cache_episode.clone();
            async move { fetch_episode_all(cache).await }
        });

        let cache_episode = cache.clone();
        let episode_id = warp::path!("episode" / i64).then(move |id: i64| {
            let cache = cache_episode.clone();
            async move { fetch_episode(id, cache).await }
        });

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
