use color_eyre::Result;
use sha2::{Digest, Sha256};
use std::{env, fs};
use tracing::debug;
use xesite_types::mastodon::{Toot, User};

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    tracing_subscriber::fmt::init();

    let args: Vec<String> = env::args().collect();
    debug!("{args:?}");
    if args.len() != 2 {
        eprintln!("Usage: {} <mastodon post URL>", args[0]);
    }

    let mut post_url = args[1].clone();
    if !post_url.ends_with(".json") {
        post_url = format!("{post_url}.json");
    }

    let toot: Toot = reqwest::get(&post_url)
        .await?
        .error_for_status()?
        .json()
        .await?;

    debug!("got post by {}", toot.attributed_to);

    fs::create_dir_all("./data/toots")?;

    let post_hash = xesite::hash_string(post_url);

    let mut fout = fs::File::create(&format!("./data/toots/{post_hash}.json"))?;
    serde_json::to_writer_pretty(&mut fout, &toot)?;

    let user_url = format!("{}.json", toot.attributed_to);

    let user: User = reqwest::get(&user_url)
        .await?
        .error_for_status()?
        .json()
        .await?;

    fs::create_dir_all("./data/users")?;

    debug!("got user {} ({})", user.name, user.preferred_username);

    let user_hash = xesite::hash_string(user_url);

    let mut fout = fs::File::create(&format!("./data/toots/{user_hash}.json"))?;
    serde_json::to_writer_pretty(&mut fout, &user)?;

    Ok(())
}
