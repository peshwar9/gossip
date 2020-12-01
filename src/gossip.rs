use std::str;

#[derive(Debug)]
pub struct Peer {
    pub host: String,
    pub port: u16,
}

#[derive(Debug)]
pub struct PeerList {
    list: Vec<Peer>,
}

#[derive(Debug)]
pub enum GossipMessage {
    Join(Peer),
    //   List(PeerList),
    //   RandomMessage(String),
}

impl GossipMessage {
    pub fn as_bytes(self) -> String {
        match self {
            GossipMessage::Join(peer) => format!("Join {} {}\n", peer.host, peer.port),
        }
    }
    pub fn from_bytes(buf: &[u8]) -> Option<GossipMessage> {
        let msg = String::from_utf8_lossy(buf);
        let mut msg_iter = msg.split_ascii_whitespace();
        let msg_type = msg_iter.next().unwrap();

        let host = msg_iter.next().unwrap();
        let port = msg_iter.next().unwrap();
        match msg_type {
            "Join" => Some(GossipMessage::Join(Peer {
                host: host.into(),
                port: port.parse().unwrap(),
            })),
            _ => None,
        }
    }
}
