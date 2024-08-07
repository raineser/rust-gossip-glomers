use serde::{Deserialize, Serialize, self};
use serde_json::Result;
use std::io::{self, BufRead, Write};

#[derive(Deserialize, Serialize)]
struct Node {
    node_id: Option<String>,
    node_ids: Option<Vec<String>>
}

impl Node {
    fn new() -> Self {
        Self{node_id: None, node_ids: None}
    }

    fn read(&mut self, message: Message, handle: &mut io::StdoutLock<'_>) -> io::Result<()>{
        match message.body.t {
            BodyType::echo { msg_id, echo } => {
                let response = Message {
                    src: self.node_id.clone().unwrap(),
                    dest: message.src,
                    body: Body {
                        t: BodyType::echo_ok { msg_id: msg_id, in_reply_to: msg_id, echo: echo}
                    }

                };
                println!("{}", serde_json::to_string(&response)?);
            }, 
            BodyType::echo_ok { msg_id, in_reply_to, echo } => {

            },
            BodyType::init { msg_id, node_id, node_ids } => {
                self.node_id = Some(node_id.clone());
                self.node_ids = Some(node_ids);
                let response = Message {
                    src: node_id,
                    dest: message.src,
                    body: Body {
                        t: BodyType::init_ok { in_reply_to: msg_id}
                    }
                };
                println!("{}", serde_json::to_string(&response)?);
            },
            BodyType::init_ok { in_reply_to } => {

            }
        }
        Ok(())
    }
}


#[derive(Deserialize, Serialize)]
struct Message {
    src: String,
    dest: String,
    body: Body
}

#[derive(Deserialize, Serialize)]
struct Body {
    #[serde(flatten)]
    t: BodyType
}

#[derive(Deserialize, Serialize)]
#[serde(tag = "type")]
enum BodyType {
    init{msg_id: usize, node_id: String, node_ids: Vec<String>},
    init_ok{in_reply_to: usize},
    echo{msg_id: usize, echo: String},
    echo_ok{msg_id: usize, in_reply_to: usize, echo: String}
}

fn main() -> io::Result<()> {
    let mut node = Node::new();
    let stdin = io::stdin();
    let stdout = io::stdout();

    for line in stdin.lock().lines() {
        match line {
            Ok(line) => {
                let message: Message = serde_json::from_str(&line)?;
                let mut handle: io::StdoutLock<'_> = stdout.lock();
                node.read(message, &mut handle);
            }, 
            Err(e) => {
                println!("Failed to parse line");
            }
        }
    }
    Ok(())
}
