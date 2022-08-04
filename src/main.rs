mod skyjo;

use skyjo::Game;
use skyjo::PlayerDeck::*;

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let game = Game::new(3);

    println!("{:?}", game);

    let game = game.start();
    let card = game.players[0].reveal(Slot_1_1);
    card = game.players[0].reveal(Slot_1_2);
    card = game.players[1].reveal(Slot_2_2);
    card = game.players[1].reveal(Slot_3_3);
    card = game.players[2].reveal(Slot_1_2);
    card = game.players[2].reveal(Slot_1_1);
}
