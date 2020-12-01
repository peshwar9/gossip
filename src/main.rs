use std::net::{ToSocketAddrs, UdpSocket};
use structopt::StructOpt;
mod gossip;
use gossip::{get_epoch_millis, GossipMessage};
use log::info;
use std::time::Duration;

// Commandline parameters
#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(long)]
    period: u64,
    #[structopt(long)]
    port: u16,
    #[structopt(long)]
    connect: Option<String>,
}

fn main() {
    // Initialize logger
    env_logger::init();
    // Get commandline parameters
    let opt = Opt::from_args();
    // Configure read-timeout for UDP socket. Otherwise heartbeat can't be sent
    let read_timeout = opt.period * 10;
    // Accumulate list of peers
    let mut peer_list: Vec<String> = vec![];

    // Create a UDP socket server binding to port from command-line parameter
    let udp_socket =
        UdpSocket::bind(format!("127.0.0.1:{}", opt.port)).expect("Unable to bind to port");
    info!("My address is: 127.0.0.1:{}", opt.port);
    // Set timeout
    udp_socket
        .set_read_timeout(Some(Duration::from_millis(read_timeout)))
        .expect("fail to set read timeout"); // If connect parameter is provided, send a Gossip::Join message to that address

    // If --connect parameter is specified, send Join message to that address
    if let Some(remote_peer) = opt.connect {
        let send_buffer = format!("Join localhost {}\n", opt.port);
        let addr = remote_peer.to_socket_addrs().unwrap().next().unwrap();
        let _ = udp_socket.send_to(send_buffer.as_bytes(), addr);
        peer_list.push(format!("localhost:{}", addr.port()));
    }

    let mut buffer = [0; 1024];
    let mut last_heartbeat: u64 = 0;
    loop {
        let now = get_epoch_millis();
        let socket_new = udp_socket.try_clone().expect("Unable to clone socket");
        // Listen for incoming messages
        match socket_new.recv_from(&mut buffer) {
            Ok(_) => {
                let msg = GossipMessage::from_bytes(&buffer, opt.port).unwrap();
                match msg {
                    GossipMessage::Join(peer) => {
                        println!("Join message from {}:{}", peer.host, peer.port);
                        // Add the sender to peer list
                        peer_list.push(format!("localhost:{}", peer.port));
                        // Notify peers in list of new joinee
                        for node in peer_list.clone() {
                            // Don't send message back to sender
                            if node != format!("localhost:{}", peer.port) {
                                let send_buffer = format!(
                                    "Joined localhost:{} localhost {}\n",
                                    opt.port, peer.port
                                );
                                let _ = udp_socket.send_to(send_buffer.as_bytes(), &node);
                            }
                        }
                    }
                    GossipMessage::NotifyNew(node) => {
                        // Add the newly joined node to peer list
                        peer_list.push(format!("localhost:{}", node.peer.port));

                        println!(
                            "Notification message from {}: localhost {} Joined",
                            node.sender, node.peer.port
                        )
                    }
                    GossipMessage::HeartBeat(from) => {
                        // Add the newly joined node to peer list
                        let addr = format!("localhost:{}", opt.port);

                        if addr != from {
                            info!("Heartbeat message from {}", from);
                        }
                    }
                }
            }
            Err(_err) => (),
        }
        // Send heartbeat messages
        if now - last_heartbeat > (opt.period * 1000) {
            last_heartbeat = now;
            let send_buffer = format!("HeartBeat {}", format!("localhost:{}", opt.port));
            for node in peer_list.clone() {
                if node != format!("localhost:{}", opt.port) {
                    let _ = udp_socket.send_to(send_buffer.as_bytes(), node);
                }
            }
        }
    }
}
