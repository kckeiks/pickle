use clap::{Parser, Subcommand};
use warp::{Filter, Rejection};

use rick_and_morty::{character, episode, location};
use rick_and_morty::location::Location;

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
    Proxy,
}

#[tokio::main]
async fn main() -> Result<(), String> {
    let cli = Cli::parse();

    match cli.character {
        Commands::Character { id } => {
            if let Some(id) = id {
                println!(
                    "Character: {:?} ",
                    character::get(id)
                        .await
                        .map_err(|_| "Failed to get character")?
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
                    episode::get(id)
                        .await
                        .map_err(|_| "Failed to get episode.")?
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
                    location::get(id)
                        .await
                        .map_err(|_| "Failed to get episode.")?
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
        Commands::Proxy => {
            let routes = warp::any().and_then(|| async {
                let res = location::get(1).await.unwrap();
                Ok::<String, Rejection>(serde_json::to_string(&res).unwrap())
            });

            warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
            println!("shutting down..");
        }
    }

    Ok(())
}
