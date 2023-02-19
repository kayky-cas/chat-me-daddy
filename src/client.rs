use websocket::{ClientBuilder, OwnedMessage};

const CONNECTION: &'static str = "ws://127.0.0.1:3000/ws";

fn main() -> anyhow::Result<()> {
    println!("Executing client...");

    let client = ClientBuilder::new(CONNECTION)?.connect_insecure()?;

    let (_, mut sender) = client.split()?;

    let message: OwnedMessage = OwnedMessage::Text("Hello".to_string());

    sender.send_message(&message)?;

    return Ok(());
}
