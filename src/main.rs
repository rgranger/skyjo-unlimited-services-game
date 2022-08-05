mod skyjo;

use futures_util::SinkExt;
use futures_util::{future, stream::TryStreamExt, StreamExt};
use skyjo::Game;
use skyjo::PlayerDeck::*;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::tungstenite::Message;

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let ws_server_handle = tokio::spawn(create_websocket_server());

    let game = Game::new(3);

    println!("{:?}", game);

    //let game = game.start();
    //let card = game.players[0].reveal(Slot_1_1);
    //card = game.players[0].reveal(Slot_1_2);
    //card = game.players[1].reveal(Slot_2_2);
    //card = game.players[1].reveal(Slot_3_3);
    //card = game.players[2].reveal(Slot_1_2);
    //card = game.players[2].reveal(Slot_1_1);

    ws_server_handle.await;
}

async fn create_websocket_server() {
    let addr = "127.0.0.1:4000";
    let try_socket = TcpListener::bind(&addr).await;
    let listener = try_socket.expect("Failed to bind");
    println!("Listening on: {}", &addr);

    while let Ok((stream, addr)) = listener.accept().await {
        tokio::spawn(handle_connection(stream));
    }
}

async fn handle_connection(raw_stream: TcpStream) {
    let ws_stream = tokio_tungstenite::accept_async(raw_stream)
        .await
        .expect("Error during the websocket handshake occurred");

    let (mut outgoing, incoming) = ws_stream.split();

    incoming
        .try_for_each(|msg| {
            println!("Received a message: {}", msg.to_text().unwrap());

            outgoing.send(Message::Text("world".to_string()));

            future::ok(())
        })
        .await;
}
