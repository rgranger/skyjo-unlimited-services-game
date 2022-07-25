mod skyjo;

use skyjo::Game;

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let game = Game::new(3);

    println!("{:?}", game);
}
