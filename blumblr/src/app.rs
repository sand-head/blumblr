use std::{env, error::Error, fs};

use blumblr_core::bsky::BskyClient;
use futures::executor;

#[derive(Clone)]
pub struct ApplicationState {
    pub bsky: BskyClient,
}

pub fn main() -> ApplicationState {
    match initialize_state() {
        Ok(state) => state,
        Err(error) => panic!("Could not initialize application state: {}", error),
    }
}

fn initialize_state() -> Result<ApplicationState, Box<dyn Error>> {
    load_dotenv()?;
    let bsky = executor::block_on(configure_bsky())?;
    Ok(ApplicationState { bsky })
}

fn load_dotenv() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv()?;
    if fs::exists(".env.dev")? {
        dotenvy::from_filename_override(".env.dev")?;
    }
    Ok(())
}

async fn configure_bsky() -> Result<BskyClient, Box<dyn Error>> {
    let client = BskyClient::init(env::var("BSKY_USER")?, env::var("BSKY_PASSWORD")?).await?;
    Ok(client)
}
