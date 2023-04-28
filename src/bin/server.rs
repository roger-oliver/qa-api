use qa_api::config::Config;

#[tokio::main]
pub async fn main() {
    dotenv::dotenv().ok();

    let config = Config::new().expect("Config can't be set");

    let store = qa_api::setup_store(&config).await;

    qa_api::run(&config, store).await;
}
