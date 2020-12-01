use std::time::{SystemTime, UNIX_EPOCH};

// Represents a peer node
#[derive(Debug, Clone)]
pub struct Peer {
    pub host: String,
    pub port: u16,
}
/// Represents payload for notification message
#[derive(Debug)]
pub struct Payload {
    pub sender: String,
    pub peer: Peer,
}

// Represents list of peer nodes maintained by each instance of running node
#[derive(Debug, Clone)]
pub struct PeerList {
    list: Vec<Peer>,
}

// Gossip message types: 3 types
// Join:  A node sends this message when it starts up (if --connect specified)
// NotifyNew: The anchor node (mentioned in --connect parm) sends this msg to other nodes
// HeartBeat: Sent by all nodes on a cadence

#[derive(Debug)]
pub enum GossipMessage {
    Join(Peer),
    NotifyNew(Payload), //
    HeartBeat(String),  // Sent by all nodes
}

impl GossipMessage {
    // Serialization fn: Written this but not used it, though I should have
    pub fn _as_bytes(self) -> String {
        match self {
            GossipMessage::Join(peer) => format!("Join {} {}\n", peer.host, peer.port),
            GossipMessage::NotifyNew(node) => {
                format!("Joined {} {}\n", node.peer.host, node.peer.port)
            }
            GossipMessage::HeartBeat(from) => format!("HeartBeat {}", from),
        }
    }
    // For incoming messages- this is the deserialization function
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
            // Identifier for Join msg type
            "Join" => Some(GossipMessage::Join(Peer {
                host: host.into(),
                port: port,
            })),
            // Identifier for NotifyNew message type
            "Joined" => Some(GossipMessage::NotifyNew(Payload {
                sender: sender.into(),
                peer: Peer {
                    host: host.into(),
                    port: port,
                },
            })),
            // Identifier for HeartBeat message type
            "HeartBeat" => Some(GossipMessage::HeartBeat(heartbeat_from)),
            _ => None,
        }
    }
}
// Used for calculations to compute when to send heartbeat message
pub fn get_epoch_millis() -> u64 {
    let now = SystemTime::now();
    let epoch = now
        .duration_since(UNIX_EPOCH)
        .expect("Unable to get Unix epoch time");
    epoch.as_millis() as u64
}
