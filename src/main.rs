use warp::Filter;

#[tokio::main]
async fn main() {
	let index = warp::path::end().and(warp::fs::dir("web/static"));

	warp::serve(index).run(([127, 0, 0, 1], 3030)).await;
}
