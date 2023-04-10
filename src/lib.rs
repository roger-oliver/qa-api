use warp::Filter;

pub async fn run() {
    let hello_route = warp::path!("hello" / String)
        .map(|name| format!("hello {}", name));
    
    warp::serve(hello_route)
        .run(([127, 0, 0, 1], 8081))
        .await;
}   