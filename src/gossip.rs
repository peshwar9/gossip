use std::str;

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
    //   List(PeerList),
    //   RandomMessage(String),
}

impl GossipMessage {
    pub fn as_bytes(self) -> String {
        match self {
            GossipMessage::Join(peer) => format!("Join {} {}\n", peer.host, peer.port),
            GossipMessage::NotifyNew(node) => {
                format!("Joined {} {}\n", node.peer.host, node.peer.port)
            }
        }
    }
    pub fn from_bytes(buf: &[u8], port: u16) -> Option<GossipMessage> {
        let msg = String::from_utf8_lossy(buf);
        let msg = msg.lines().next().unwrap();
     //   println!("Message received is {}",msg);
        let mut msg_iter = msg.split_ascii_whitespace();
        let msg_type = msg_iter.next().unwrap();
        let mut sender: String = "".into();
        if msg_type == "Joined" {
            sender = msg_iter.next().unwrap().into();
        }
       
        let host = msg_iter.next().unwrap();
        let port = msg_iter.next().unwrap();
        match msg_type {
            "Join" => Some(GossipMessage::Join(Peer {
                host: host.into(),
                port: port.parse().unwrap(),
            })),
            "Joined" => Some(GossipMessage::NotifyNew(Payload {
                sender: sender.into(),
                peer: Peer {
                    host: host.into(),
                    port: port.parse().unwrap(),
                },
            })),
            _ => None,
        }
    }
}
