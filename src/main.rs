mod proxy;

use clap::{Parser, Subcommand};
use std::path::Path;

use rick_and_morty::{character, episode, location};
use securestore::{ErrorKind, KeySource, SecretsManager};

use crate::proxy::Proxy;
use thiserror::Error;
use uuid::Uuid;

type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Secret not found")]
    SecretNotFound,
    #[error("Username already exists")]
    UsernameAlreadyExists,
    #[error("Record does not exist")]
    RecordNotFound,
    #[error("Unknown error occurred")]
    Unknown,
}

#[derive(Parser)]
#[command(author, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    character: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Character { id: Option<i64> },
    Episode { id: Option<i64> },
    Location { id: Option<i64> },
    SignUp { keyword: String },
    Proxy,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.character {
        Commands::Character { id } => {
            if let Some(id) = id {
                println!(
                    "Character: {:?} ",
                    character::get(id)
                        .await
                        .map_err(|_| Error::RecordNotFound)?
                );
            } else {
                println!(
                    "Characters: {:?} ",
                    character::get_all()
                        .await
                        .expect("Fetching all characters to succeed.")
                );
            }
        }
        Commands::Episode { id } => {
            if let Some(id) = id {
                println!(
                    "Episode: {:?} ",
                    episode::get(id).await.map_err(|_| Error::RecordNotFound)?
                );
            } else {
                println!(
                    "Episodes: {:?} ",
                    episode::get_all()
                        .await
                        .expect("Fetching all episodes to succeed.")
                );
            }
        }
        Commands::Location { id } => {
            if let Some(id) = id {
                println!(
                    "Location: {:?} ",
                    location::get(id).await.map_err(|_| Error::RecordNotFound)?
                );
            } else {
                println!(
                    "Locations: {:?} ",
                    location::get_all()
                        .await
                        .expect("Fetching all locations to succeed.")
                );
            }
        }
        Commands::SignUp { keyword } => sign_up(keyword)?,
        Commands::Proxy => Proxy::new().run().await?,
    }

    Ok(())
}

fn sign_up(mut keyword: String) -> Result<()> {
    let mut manager = SecretsManager::load(
        Path::new(proxy::JSON_API_KEYS_PATH),
        KeySource::Path(Path::new(proxy::SECRET_KEY_PATH)),
    )
    .map_err(|_| Error::Unknown)?;

    if let Err(ErrorKind::SecretNotFound) = manager.get(&keyword).map_err(|e| e.kind()) {
        let api_key = Uuid::new_v4().simple().to_string();
        manager.set(&keyword, api_key.as_str());
        manager.save().map_err(|_| Error::Unknown)?;
        keyword.push_str("-");
        keyword.push_str(api_key.as_str());

        println!("Key: {}", keyword);

        return Ok(());
    }

    Err(Error::UsernameAlreadyExists)
}
