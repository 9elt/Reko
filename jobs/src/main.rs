mod jobs;
use clap::Parser;
use reko::Reko;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// job ids
    #[arg(short, long)]
    pub jobs: Vec<String>,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let jobs = args.jobs.iter().filter(|id| is_job(id)).collect::<Vec<_>>();
    if jobs.len() == 0 {
        return available_jobs();
    }

    let reko = Reko::new();

    for id in jobs {
        match id.as_str() {
            "update_old_users" => jobs::update_old_users(&reko).await,
            "update_airing_anime" => jobs::update_airing_anime(&reko).await,
            "request_missing_anime" => jobs::request_missing_anime(&reko).await,
            _ => (),
        };
    }
}

const JOBS: &[&str] = &[
    "update_old_users",
    "update_airing_anime",
    "request_missing_anime",
];

fn available_jobs() {
    println!("available jobs:\n{}", JOBS.join("\n"));
}

fn is_job(id: &String) -> bool {
    JOBS.contains(&id.as_str())
}
