use warp::Filter;

#[tokio::main]
async fn main() {
    let hi = warp::path("hello")
        .and(warp::header("user-agent"))
        .map(|agent: String| format!("Your agent is {}", agent));
    warp::serve(hi).run(([127, 0, 0, 1], 3030)).await;
}
