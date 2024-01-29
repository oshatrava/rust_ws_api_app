use std::sync::Arc;

use futures::{SinkExt, StreamExt};
use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade}, 
        State,
    },
    response::IntoResponse,
};
use serde::Deserialize;
use tokio::sync::broadcast;

use crate::{AppState, RoomState};

pub async fn wsocket_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
    // user_agent: Option<TypedHeader<headers::UserAgent>>,
    // ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> impl IntoResponse {
    // let user_agent = if let Some(TypedHeader(user_agent)) = user_agent {
    //     user_agent.to_string()
    // } else {
    //     String::from("Unknown browser")s
    // };
    // println!("{}` at {} connected.", user_agent, addr);

    ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn handle_socket(socket: WebSocket, state: Arc<AppState>) {
    let (mut sender, mut receiver) = socket.split();
    let mut username = String::new();
    let mut channel = String::new();
    let mut tx = None::<broadcast::Sender<String>>;

    while let Some(Ok(msg)) = receiver.next().await {
        if let Message::Text(name) = msg {

            #[derive(Deserialize, Debug)]
            struct Connect {
                username: String,
                channel: String,
            }

            let connect: Connect = match serde_json::from_str(&name) {
                Ok(conn) => conn,
                Err(err) => {
                    tracing::info!("{:#?}", &name);
                    tracing::error!("{:#?}", err);
                    let _ = sender.send(Message::from("Failed to connect to room!")).await;
                    break;
                },
            };

            {
                let mut rooms = state.rooms.lock().unwrap();
                channel = connect.channel.clone();

                let room = rooms
                    .entry(connect.channel)
                    .or_insert_with(RoomState::new);
                tx = Some(room.tx.clone());

                if !room.users.lock().unwrap().contains(&connect.username) {
                    room.users.lock().unwrap().insert(connect.username.to_owned());
                    username = connect.username.clone();
                }
            }

            if tx.is_some() && !username.is_empty() {
                break;
            } else {
                let _ = sender
                    .send(Message::Text(String::from("Username already taken.")))
                    .await;

                return;
            }
        }
    }

    let tx = tx.unwrap();
    let mut rx = tx.subscribe();

    let joined = format!("{} joined the chat!", username);
    let _ = tx.send(joined);

    let mut recv_messages = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if sender.send(Message::Text(msg)).await.is_err() {
                break;
            }     
        }
    });

    let mut send_messages = {
        let tx = tx.clone();
        let name = username.clone();

        tokio::spawn(async move {
            while let Some(Ok(Message::Text(text))) = receiver.next().await {
                let _ = tx.send(format!("{}: {}", name, text));
            }
        })
    };

    tokio::select! {
        _ = (&mut send_messages) => recv_messages.abort(),
        _ = (&mut recv_messages) => send_messages.abort(),
    };

    let _ = tx.send(format!("{} left the chat!", username));
    let mut rooms = state.rooms.lock().unwrap();
    rooms.get_mut(&channel).unwrap().users.lock().unwrap().remove(&username);

    if rooms.get_mut(&channel).unwrap().users.lock().unwrap().len() == 0 {
        rooms.remove(&channel);
    }
}


