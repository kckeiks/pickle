use crate::Result;
use moka::future::Cache;
use rick_and_morty::{character, episode, location};
use securestore::KeySource;
use std::path::Path;
use std::sync::Arc;
use warp::http::StatusCode;
use warp::reply::Response;
use warp::{reply, Filter, Reply};

pub const JSON_API_KEYS_PATH: &str = "api-keys.json";
pub const SECRET_KEY_PATH: &str = "secret.key";

async fn fetch_location(id: i64, cache: Cache<String, String>, is_key_valid: bool) -> Response {
    let key = format!("location/{}", id);
    if let Some(val) = cache.get(&key) {
        if is_key_valid {
            log::info!("Returning cached response.");
            return Response::new(val.into());
        }
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

async fn fetch_location_all(cache: Cache<String, String>, is_key_valid: bool) -> Response {
    let key = String::from("location-all");
    if let Some(val) = cache.get(&key) {
        if is_key_valid {
            log::info!("Returning cached response.");
            return Response::new(val.into());
        }
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

async fn fetch_character(id: i64, cache: Cache<String, String>, is_key_valid: bool) -> Response {
    let key = format!("character/{}", id);
    if let Some(val) = cache.get(&key) {
        if is_key_valid {
            log::info!("Returning cached response.");
            return Response::new(val.into());
        }
    }
    let res = match character::get(id).await {
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

async fn fetch_character_all(cache: Cache<String, String>, is_key_valid: bool) -> Response {
    let key = String::from("character-all");
    if let Some(val) = cache.get(&key) {
        if is_key_valid {
            log::info!("Returning cached response.");
            return Response::new(val.into());
        }
    }
    let res = match character::get_all().await {
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

async fn fetch_episode(id: i64, cache: Cache<String, String>, is_key_valid: bool) -> Response {
    let key = format!("episode/{}", id);
    if let Some(val) = cache.get(&key) {
        if is_key_valid {
            log::info!("Returning cached response.");
            return Response::new(val.into());
        }
    }
    let res = match episode::get(id).await {
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

async fn fetch_episode_all(cache: Cache<String, String>, is_key_valid: bool) -> Response {
    let key = String::from("episode-all");
    if let Some(val) = cache.get(&key) {
        if is_key_valid {
            log::info!("Returning cached response.");
            return Response::new(val.into());
        }
    }
    let res = match episode::get_all().await {
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

#[derive(Clone)]
struct SecretsManager(Arc<securestore::SecretsManager>);

impl SecretsManager {
    fn is_key_valid(&self, api_key: Option<String>) -> bool {
        match api_key {
            Some(key) => {
                if let Some((keyword, in_key)) = key.split_once('-') {
                    if let Ok(k) = self.0.get(keyword) {
                        return k.as_str() == in_key;
                    }
                }
                false
            }
            None => false,
        }
    }
}

pub struct Proxy {
    manager: SecretsManager,
}

impl Proxy {
    pub fn new() -> Self {
        Self {
            manager: SecretsManager(Arc::new(
                securestore::SecretsManager::load(
                    Path::new(JSON_API_KEYS_PATH),
                    KeySource::Path(Path::new(SECRET_KEY_PATH)),
                )
                .expect("Api key JSON file and secret key to exist."),
            )),
        }
    }

    pub async fn run(self, port: Option<u16>) -> Result<()> {
        let cache = Cache::new(1000);

        let auth = warp::filters::header::optional::<String>("authorization");

        // Cloning the cache and manager are cheap because both are wrapped in Arc.
        let cache_location = cache.clone();
        let manager = self.manager.clone();

        let location_all = auth
            .map(move |api_key| manager.is_key_valid(api_key))
            .and(warp::path!("location"))
            .then(move |key_is_valid| {
                let cache = cache_location.clone();
                async move { fetch_location_all(cache, key_is_valid).await }
            });

        let cache_location = cache.clone();
        let manager = self.manager.clone();

        let location_id = auth
            .map(move |api_key| manager.is_key_valid(api_key))
            .and(warp::path!("location" / i64))
            .then(move |key_is_valid, id| {
                let cache = cache_location.clone();
                async move { fetch_location(id, cache, key_is_valid).await }
            });

        let cache_character = cache.clone();
        let manager = self.manager.clone();

        let character_all = auth
            .map(move |api_key| manager.is_key_valid(api_key))
            .and(warp::path!("character"))
            .then(move |key_is_valid| {
                let cache = cache_character.clone();
                async move { fetch_character_all(cache, key_is_valid).await }
            });

        let cache_character = cache.clone();
        let manager = self.manager.clone();

        let character_id = auth
            .map(move |api_key| manager.is_key_valid(api_key))
            .and(warp::path!("character" / i64))
            .then(move |key_is_valid, id| {
                let cache = cache_character.clone();
                async move { fetch_character(id, cache, key_is_valid).await }
            });

        let cache_episode = cache.clone();
        let manager = self.manager.clone();

        let episode_all = auth
            .map(move |api_key| manager.is_key_valid(api_key))
            .and(warp::path!("episode"))
            .then(move |key_is_valid| {
                let cache = cache_episode.clone();
                async move { fetch_episode_all(cache, key_is_valid).await }
            });

        let cache_episode = cache.clone();
        let manager = self.manager.clone();

        let episode_id = auth
            .map(move |api_key| manager.is_key_valid(api_key))
            .and(warp::path!("episode" / i64))
            .then(move |key_is_valid, id| {
                let cache = cache_episode.clone();
                async move { fetch_episode(id, cache, key_is_valid).await }
            });

        let routes = character_id
            .or(character_all)
            .or(location_id)
            .or(location_all)
            .or(episode_id)
            .or(episode_all);

        let port = port.unwrap_or(3030);
        println!("Running on localhost:{}", port);
        warp::serve(routes).run(([127, 0, 0, 1], port)).await;

        Ok(())
    }
}
