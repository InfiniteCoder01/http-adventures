use axum::body::Bytes;
use axum::extract::ws::*;

pub fn texture(texture: &str) -> Bytes {
    // TODO: caching
    let mut message = Vec::new();
    message.push(b't');
    message.extend_from_slice(texture.as_bytes());
    message.push(0);
    message.extend(std::fs::read(std::path::Path::new("textures/").join(texture)).unwrap());
    message.into()
}

pub async fn handle_socket(mut socket: WebSocket) {
    // use axum::extract::ws::Message;
    socket
        .send(Message::binary(texture("test.png")))
        .await
        .unwrap();
    // Message::binary()
    // // Send a greeting message to the client
    // if let Err(e) = socket
    //     .send(Message::Text("Hello from the server!".to_string()))
    //     .await
    // {
    //     eprintln!("Error sending message: {}", e);
    //     return;
    // }

    use futures_util::StreamExt;
    let (_tx, mut rx) = socket.split();
    while let Some(Ok(msg)) = rx.next().await {
        match msg {
            Message::Text(msg) => {
                println!("Received message: {}", msg);
            }
            Message::Close(_) => {
                println!("Closing WebSocket connection.");
                break;
            }
            _ => {}
        }
    }
}
