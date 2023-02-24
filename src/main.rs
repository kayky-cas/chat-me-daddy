use websocket::sync::Server;

fn main() -> anyhow::Result<()> {
    //    let addr = std::env::args()
    //        .next()
    //        .unwrap_or("127.0.0.1:2794".to_owned());
    //
    let addr = "127.0.0.1:2794";

    println!("INFO: Runnign webscoket server at {addr}...");
    let server = Server::bind(addr)?;

    for request in server.filter_map(Result::ok) {
        println!("Request accepted!");
    }

    Ok(())
}

#[cfg(test)]
mod client_ws_tests {
    use websocket::{ClientBuilder, OwnedMessage};

    #[test]
    fn send_message() -> anyhow::Result<()> {
        let addr = "ws://127.0.0.1:2794";
        let client = ClientBuilder::new(addr)?.connect_insecure()?;

        let (_, mut sender) = client.split()?;

        let message = OwnedMessage::Text("Oi kk".to_owned());

        dbg!(&message);

        sender.send_message(&message)?;

        Ok(())
    }
}
