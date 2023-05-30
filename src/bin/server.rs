use qa_api::config::Config;

#[tokio::main]
pub async fn main() -> Result<(), warp::Rejection>{
    dotenv::dotenv().ok();

    let config = Config::new().expect("Config can't be set");

    let store = qa_api::setup_repository(&config).await?;

    qa_api::run(&config, store).await;

    Ok(())
}
