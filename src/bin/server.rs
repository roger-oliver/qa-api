#[tokio::main]
pub async fn main() {
    let store = qa_api::setup_store().await;

    qa_api::run(store).await;
}