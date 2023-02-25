use std::thread;

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
        let _: thread::JoinHandle<anyhow::Result<()>> = thread::spawn(|| {
            let client = request
                .accept()
                .map_err(|err| anyhow::anyhow!("ERROR: {}", err.1))?;

            let ip = client.peer_addr()?;

            dbg!(ip);

            let (mut reciver, _) = client.split()?;

            for message in reciver.incoming_messages() {
                let message = message?;

                match message {
                    websocket::OwnedMessage::Text(m) => dbg!(m),
                    websocket::OwnedMessage::Binary(_) => todo!("Binary"),
                    websocket::OwnedMessage::Close(_) => todo!("Close"),
                    websocket::OwnedMessage::Ping(_) => todo!("Ping"),
                    websocket::OwnedMessage::Pong(_) => todo!("Pong"),
                };
            }

            Ok(())
        });
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

        //let message = OwnedMessage::Text("Oi kk".to_owned());
        let message = OwnedMessage::Ping(vec![]);

        dbg!(&message);

        sender.send_message(&message)?;

        Ok(())
    }
}
