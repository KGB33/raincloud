pub async fn start() {
    warp::serve(warp::fs::dir("."))
        .run(([127, 0, 0, 1], 4750))
        .await;
}