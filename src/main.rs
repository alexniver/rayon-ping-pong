use std::time::Duration;

fn main() {
    let (tx_server, mut rx_client) = tokio::sync::mpsc::channel(1);
    let (tx_clinet, mut rx_server) = tokio::sync::mpsc::channel(1);
    rayon::join(
        // server
        || loop {
            tx_server.blocking_send("hi, I'm server").unwrap();
            let v = rx_server.blocking_recv().unwrap();
            println!("server receive: {}", v);
            std::thread::sleep(Duration::from_secs(1));
        },
        // client
        || loop {
            let v = rx_client.blocking_recv().unwrap();
            println!("client receive: {}", v);
            std::thread::sleep(Duration::from_secs(1));
            tx_clinet.blocking_send("hi, I'm client").unwrap();
        },
    );
}
