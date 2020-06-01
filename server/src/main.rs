use std::{thread, time};

use warp::Filter;

mod methods;
use methods::*;

#[tokio::main]
async fn main() {
    let send_sock = socket("0.0.0.0:8889".to_string());
    send_sock
        .set_broadcast(true)
        .expect("Unable to switch to broadcast mode");

    thread::spawn(move || {
        println!("Started beacon broadcast");
        loop {
            send_message(
                &send_sock,
                "255.255.255.255:8888".to_string(),
                "pfg_ip_broadcast_serv".to_string(),
            );
            thread::sleep(time::Duration::from_millis(3000));
        }
    });

    println!("Started web server");
    let home = warp::any().map(|| "Hello, World!");
    warp::serve(home).run(([0, 0, 0, 0], 1234)).await;
}
