use std::{net::TcpStream, thread::sleep, time::Duration};

use serde::{Deserialize, Serialize};
use websocket::{sync::Client, ClientBuilder};

struct Connection {
    client: Client<TcpStream>,
}

#[derive(Debug, Serialize)]
struct State {
    temperature: i32,
    cooler: bool,
    heater: bool,
}

impl State {
    fn check_sensors(&mut self) {
        self.temperature += if rand::random() { -1 } else { 1 };
    }
}

#[derive(Debug, Serialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
enum SendMessage<'a> {
    Hello { me: &'a str, state: &'a State },
    Update { state: &'a State },
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "object")]
#[serde(rename_all = "snake_case")]
enum ReceiveMessage {
    Cooler { enable: bool },
}

impl Connection {
    fn send(&mut self, data: impl Serialize) {
        let data = serde_json::to_string(&data).unwrap();
        self.client
            .send_message(&websocket::Message::text(data))
            .unwrap();
    }

    fn receive(&mut self) -> Option<ReceiveMessage> {
        let msg = self.client.recv_message().ok()?;
        match msg {
            websocket::OwnedMessage::Text(t) => Some(serde_json::from_str(&t).unwrap()),
            websocket::OwnedMessage::Binary(b) => {
                Some(serde_json::from_str(&String::from_utf8(b).unwrap()).unwrap())
            }
            _ => None,
        }
    }
}

fn main() {
    let client = ClientBuilder::new("ws://127.0.0.1:8080")
        .unwrap()
        .connect_insecure()
        .unwrap();
    client.set_nonblocking(true).unwrap();
    let mut state = State {
        temperature: 27,
        cooler: false,
        heater: false,
    };
    let mut connection = Connection { client };
    connection.send(SendMessage::Hello {
        me: "hub",
        state: &state,
    });
    loop {
        sleep(Duration::from_millis(500));
        state.check_sensors();
        connection.send(SendMessage::Update { state: &state });
        let Some(msg) = connection.receive() else { continue; };
        match dbg!(msg) {
            ReceiveMessage::Cooler { enable } => {
                state.cooler = enable;
                connection.send(SendMessage::Update { state: &state });
                sleep(Duration::from_millis(500));
                state.temperature += if enable { -3 } else { 3 };
                connection.send(SendMessage::Update { state: &state });
            }
        }
    }
}
