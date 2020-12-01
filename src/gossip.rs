use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
pub struct Peer {
    pub host: String,
    pub port: u16,
}
#[derive(Debug)]
pub struct Payload {
    pub sender: String,
    pub peer: Peer,
}

#[derive(Debug, Clone)]
pub struct PeerList {
    list: Vec<Peer>,
}

#[derive(Debug)]
pub enum GossipMessage {
    Join(Peer),
    NotifyNew(Payload),
    HeartBeat(String),
}

impl GossipMessage {
    pub fn _as_bytes(self) -> String {
        match self {
            GossipMessage::Join(peer) => format!("Join {} {}\n", peer.host, peer.port),
            GossipMessage::NotifyNew(node) => {
                format!("Joined {} {}\n", node.peer.host, node.peer.port)
            }
            GossipMessage::HeartBeat(from) => format!("HeartBeat {}", from),
        }
    }
    pub fn from_bytes(buf: &[u8], _port: u16) -> Option<GossipMessage> {
        let msg = String::from_utf8_lossy(buf);
        let msg = msg.lines().next().unwrap();
        let mut msg_iter = msg.split_ascii_whitespace();
        let msg_type = msg_iter.next().unwrap();
        let mut sender: String = "".into();
        let mut heartbeat_from: String = "".into();
        let mut host: String = "".into();
        let mut port = 0;
        if msg_type == "Joined" {
            sender = msg_iter.next().unwrap().into();
            host = msg_iter.next().unwrap().into();
            port = msg_iter.next().unwrap().parse::<u16>().unwrap();
        } else if msg_type == "HeartBeat" {
            heartbeat_from = msg_iter.next().unwrap().to_string().trim().to_string();
        } else if msg_type == "Join" {
            host = msg_iter.next().unwrap().into();
            port = msg_iter.next().unwrap().parse::<u16>().unwrap();
        }

        match msg_type {
            "Join" => Some(GossipMessage::Join(Peer {
                host: host.into(),
                port: port,
            })),
            "Joined" => Some(GossipMessage::NotifyNew(Payload {
                sender: sender.into(),
                peer: Peer {
                    host: host.into(),
                    port: port,
                },
            })),
            "HeartBeat" => Some(GossipMessage::HeartBeat(heartbeat_from)),
            _ => None,
        }
    }
}
pub fn get_current_millis() -> u64 {
    let now = SystemTime::now();
    let epoch = now
        .duration_since(UNIX_EPOCH)
        .expect("Failed to get unix epoch time");
    epoch.as_millis() as u64
}
