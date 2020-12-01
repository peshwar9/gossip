use std::net::{SocketAddr, ToSocketAddrs, UdpSocket};
use std::str;
use std::time::SystemTime;
use structopt::StructOpt;
mod gossip;
use gossip::{GossipMessage, Peer, PeerList};

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(long)]
    period: u8,
    #[structopt(long)]
    port: u16,
    #[structopt(long)]
    connect: Option<String>,
}

fn main() {
    // Get commandline parameters
    let opt = Opt::from_args();
    // Capture sys time
    let srv_start_time = SystemTime::now();

    // Create a UDP socket server binding to port from command-line parameter: port,
    let udp_socket =
        UdpSocket::bind(format!("127.0.0.1:{}", opt.port)).expect("Unable to bind to port");
    println!("00:00:00 My address is: 127.0.0.1:{}", opt.port);

    // If connect parameter is provided, send a Gossip::Join message to that address
    if let Some(remote_peer) = opt.connect {
        let send_buffer = format!("Join localhost {}\n", opt.port);
        let addr = remote_peer.to_socket_addrs().unwrap().next().unwrap();
        println!("Going to {}, following: {:?}", addr, send_buffer);
        let _ = udp_socket.send_to(send_buffer.as_bytes(), addr);
    }

    let mut buffer = [0; 1024];
    loop {
        let socket_new = udp_socket.try_clone().expect("Unable to clone socket");
        match socket_new.recv_from(&mut buffer) {
            Ok((_, _)) => {
                let msg = GossipMessage::from_bytes(&buffer).unwrap();
                match msg {
                    GossipMessage::Join(peer) => {
                        println!("Join message from {}:{}", peer.host, peer.port)
                    }
                    _ => println!("Got invalid gossip message"),
                }
            }
            Err(_) => println!("Error in receiving gossip message"),
        }
    }
}
