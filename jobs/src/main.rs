mod jobs;
use clap::Parser;
use dotenvy::dotenv;
use reko::Reko;

const AVAILABLE: &str = "available jobs:
    - update_old_users
    - update_airing_anime
    - request_missing_anime";

#[derive(Parser, Debug)]
#[command(author, version, about = AVAILABLE, long_about = None)]
pub struct Args {
    /// job ids
    #[arg(short, long)]
    pub jobs: Vec<String>,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let args = Args::parse();

    let reko = Reko::default();

    for id in args.jobs {
        match id.as_str() {
            "update_old_users" => jobs::update_old_users(&reko).await,
            "update_airing_anime" => jobs::update_airing_anime(&reko).await,
            "request_missing_anime" => jobs::request_missing_anime(&reko).await,
            _ => println!("job {} does not exist", id),
        };
    }
}
