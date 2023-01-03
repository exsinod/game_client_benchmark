use std::{net::SocketAddr, process::Output, time::Duration};

use chrono::Utc;
use tokio::{net::UdpSocket, task::JoinHandle};
const SEND_PORT: u16 = 9999;
const SERVER_PORT: u16 = 8877;

#[tokio::main]
async fn main() {
    let mut threads: Vec<JoinHandle<()>> = vec![];
    for player in 5..=10 {
        threads.push(tokio::spawn(async move {
            match UdpSocket::bind(SocketAddr::from(([127, 0, 0, player], SEND_PORT))).await {
                Ok(socket) => {
                    socket
                        .send_to(
                            format!(
                                "{};L1;test_id{player};{player};player",
                                Utc::now().timestamp_millis()
                            )
                            .as_bytes(),
                            SocketAddr::from(([127, 0, 0, 1], SERVER_PORT)),
                        )
                        .await
                        .expect("nooo");
                    for direction in 0..=3 {
                        socket
                            .send_to(
                                format!(
                                    "{};M0;test_id{player};{direction}",
                                    Utc::now().timestamp_millis()
                                )
                                .as_bytes(),
                                SocketAddr::from(([127, 0, 0, 1], SERVER_PORT)),
                            )
                            .await
                            .expect("waaa");
                        println!("sent msg for player{player}");
                        for _ in 0..player {
                            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 20));
                        }
                        socket
                            .send_to(
                                format!("{};M0;test_id{player};4", Utc::now().timestamp_millis())
                                    .as_bytes(),
                                SocketAddr::from(([127, 0, 0, 1], SERVER_PORT)),
                            )
                            .await
                            .expect("waaa");
                    }
                }
                Err(error) => {
                    println!("error: {error}")
                }
            }
        }));
    }
    // for thread in threads {

    futures_util::future::join_all(threads).await;
    // let _ = tokio::join!(threads.to_vec());
    // }
}
// fn main() {
//     let socket = UdpSocket::bind(SocketAddr::from(([127, 0, 0, 1], 9978))).unwrap();
//     socket
//         .connect(SocketAddr::from(([127, 0, 0, 1], 8878)))
//         .unwrap();
//     loop {
//         recv(&socket);
//     }
// }
// fn recv(socket: &UdpSocket) {
//     let mut buf = [0; 2000];
//     match socket.recv(&mut buf) {
//         Ok(value) => println!("{value}"),
//         Err(_) => println!("nothing"),
//     }
//     ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 20));
// }
