mod tools;
use clap::Parser;
use tools::argparse::Options;
use warp::Filter;

#[tokio::main]
async fn main() {
    let server_args = Options::parse();

    let hello_world = warp::path::end()
        .map(|| "Hello world !");

    warp::serve(hello_world)
        .run(([127, 0, 0, 1], server_args.http))
        .await;

    return;
}
