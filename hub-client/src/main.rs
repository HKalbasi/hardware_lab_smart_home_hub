use std::{
    cmp,
    collections::HashMap,
    io::{BufRead, BufReader},
    mem,
    net::TcpStream,
    process::{Command, Stdio},
    thread::sleep,
    time::{Duration, SystemTime},
};

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
    devices: HashMap<String, Vec<Device>>,
    rules: Vec<(RuleState, Job)>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
enum Operator {
    Le,
    Ge,
    Eq,
}

#[derive(Debug, Serialize, Deserialize)]
enum RuleState {
    Timer {
        next: SystemTime,
        interval: Duration,
        count: usize,
    },
    Sensor {
        operator: Operator,
        location: String,
        device: String,
        value: i32,
        is_sat: bool,
    },
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
enum Rule {
    Timer {
        interval: Duration,
        count: usize,
    },
    Sensor {
        operator: Operator,
        location: String,
        device: String,
        value: i32,
    },
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
enum Job {
    Beep,
}

impl Job {
    fn do_it(&self) {
        match self {
            Job::Beep => println!("\x07"),
        }
    }
}

impl State {
    fn check_rules(&mut self) {
        let mut rules = mem::take(&mut self.rules);
        rules.retain(|x| match x.0 {
            RuleState::Timer { count, .. } => count != 0,
            RuleState::Sensor { .. } => true,
        });
        for (rule, job) in &mut rules {
            match rule {
                RuleState::Timer {
                    next,
                    interval,
                    count,
                } => {
                    if *next < SystemTime::now() {
                        *next = *next + *interval;
                        job.do_it();
                        *count -= 1;
                    }
                }
                RuleState::Sensor {
                    operator,
                    location,
                    device,
                    value,
                    is_sat,
                } => {
                    let condition_result = self
                        .check_condition(&location, &device, *value, *operator)
                        .unwrap_or(false);
                    if *is_sat != condition_result {
                        *is_sat = condition_result;
                        if *is_sat {
                            job.do_it();
                        }
                    }
                }
            }
        }
        self.rules = rules;
    }

    fn check_condition(
        &self,
        location: &String,
        device: &String,
        value: i32,
        operator: Operator,
    ) -> Option<bool> {
        let location = self.devices.get(location)?;
        let device = location.iter().find(|x| x.name == *device)?;
        let their_value: i32 = device.value.parse().ok()?;
        Some(match operator {
            Operator::Le => their_value <= value,
            Operator::Ge => their_value >= value,
            Operator::Eq => their_value == value,
        })
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
#[serde(tag = "kind")]
#[serde(rename_all = "snake_case")]
enum ReceiveMessage {
    SendAct {
        location: String,
        device: String,
        value: String,
    },
    NewRule {
        rule: Rule,
        job: Job,
    },
    DeleteRule {
        index: usize,
    },
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
enum DeviceKind {
    Sensor,
    Actuator,
}

#[derive(Debug, Serialize, Deserialize)]
struct Device {
    #[serde(rename = "type")]
    name: String,
    kind: DeviceKind,
    uri: String,
    ip: String,
    location: String,
    value: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
enum DeviceEvent {
    HeartBeat,
    NewDevice { value: Device },
    UpdateDevice { value: Device },
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
        devices: HashMap::new(),
        rules: vec![],
    };
    let mut connection = Connection { client };
    connection.send(SendMessage::Hello {
        me: "hub",
        state: &state,
    });

    let mut python = Command::new("python")
        .arg("handler.py")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("failed to execute child");
    let mut python_stdout = BufReader::new(python.stdout.as_mut().unwrap());
    loop {
        println!("heart beat");
        let mut x = String::new();
        python_stdout.read_line(&mut x).unwrap();
        let device_event: DeviceEvent = serde_json::from_str(&x).unwrap();
        match device_event {
            DeviceEvent::HeartBeat => (),
            DeviceEvent::NewDevice { mut value } => {
                if value.kind == DeviceKind::Actuator {
                    value.value = match value.name.as_str() {
                        "curtain" => "close",
                        "outlet" => "off",
                        _ => "",
                    }
                    .to_owned();
                }
                state
                    .devices
                    .entry(value.location.clone())
                    .or_default()
                    .push(value);
            }
            DeviceEvent::UpdateDevice { value } => {
                for d in state.devices.get_mut(&value.location).unwrap().iter_mut() {
                    if d.uri == value.uri {
                        d.value = value.value.clone();
                    }
                }
            }
        }
        state.check_rules();
        connection.send(SendMessage::Update { state: &state });
        let Some(msg) = connection.receive() else { continue; };
        match msg {
            ReceiveMessage::NewRule { rule, job } => {
                let rule_state = match rule {
                    Rule::Timer { interval, count } => RuleState::Timer {
                        next: SystemTime::now() + interval,
                        interval,
                        count,
                    },
                    Rule::Sensor {
                        operator,
                        location,
                        device,
                        value,
                    } => RuleState::Sensor {
                        operator,
                        location,
                        device,
                        value,
                        is_sat: false,
                    },
                };
                state.rules.push((rule_state, job));
            }
            ReceiveMessage::DeleteRule { index } => {
                state.rules.remove(index);
            }
            ReceiveMessage::SendAct {
                location,
                device,
                value,
            } => {
                (|| {
                    let device = state
                        .devices
                        .get_mut(&location)?
                        .iter_mut()
                        .find(|x| x.name == device)?;
                    let uri = format!("http://{}:8080{}/{value}", device.ip, device.uri);
                    device.value = value;
                    reqwest::blocking::get(uri).unwrap();
                    Some(())
                })();
            }
        }
    }
}
