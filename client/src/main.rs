mod methods;
use methods::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let recv_sock = socket("0.0.0.0:8888".to_string());
    let server_addr;
    loop {
        let message = read_message(&recv_sock);
        if message.0 == "pfg_ip_broadcast_serv".to_string() {
            server_addr = format!("http://{}:1234", message.1.ip());
            drop(recv_sock);
            break;
        }
    }

    println!("Server: {}", server_addr);
    let response = reqwest::blocking::get(format!("{}/", server_addr).as_str())?;
    let body = response.text()?;
    println!("Body /: {}", body);
    Ok(())
}
