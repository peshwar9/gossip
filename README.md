### Gossip server implemented with UDP for heartbeat messages

### How to run?

#### Clone the repo

```
git clone https://github.com/peshwar9/gossip.git
```

#### From first terminal, run:

```
RUST_LOG=info cargo run -- --period 5 --port 8080
```

#### From the second terminal, run:

```
RUST_LOG=info cargo run -- --period 6 --port 8081 --connect "127.0.0.1:8080"
```

#### From the third terminal, run:

```
RUST_LOG=info cargo run -- --period 7 --port 8082 --connect "127.0.0.1:8080"
```

### What can you observe?

- Whenever the second and third node is started, a join message is displayed on the first node.
- When the third node is started, a notification message goes from first to second node that node 3 has joined.
- Heartbeat is sent out from each node based on the value specified in --period parameter

### Known errors

- Heartbeat is not sent from node 3 to node 2

### Deviations from spec

- Used log crate for timing
- Message description (console output) slightly different from spec

### To do/ Improvements:

- A new message type to be added for when a node leaves the network.
- Code is a bit rough, with some short-cuts taken.
- Custom error handling can be done
- Code not fully idomatic
- Tests to be added
