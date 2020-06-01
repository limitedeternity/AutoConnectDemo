use std::net;
use std::string::String;

pub fn socket(listen_on: String) -> net::UdpSocket {
    return match net::UdpSocket::bind(listen_on) {
        Ok(s) => s,
        Err(e) => panic!("Couldn't bind socket: {}", e),
    };
}

pub fn read_message(read_socket: &net::UdpSocket) -> (String, net::SocketAddr) {
    let mut buf = [0; 2048];
    let message;
    let source_addr;

    match &read_socket.recv_from(&mut buf) {
        Ok((_, src)) => {
            let mut data_ends_at = buf.len();
            if let Some(i) = (&buf).iter().rposition(|x| *x != 0) {
                data_ends_at = i + 1;
            }

            message = String::from_utf8((&buf[..data_ends_at]).to_vec()).unwrap_or(String::new());
            source_addr = *src;
        }
        Err(e) => panic!("Read error: {}", e),
    };

    return (message, source_addr);
}
