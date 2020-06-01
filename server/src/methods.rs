use std::net;
use std::string::String;

pub fn socket(listen_on: String) -> net::UdpSocket {
    return match net::UdpSocket::bind(listen_on) {
        Ok(s) => s,
        Err(e) => panic!("Couldn't bind socket: {}", e),
    };
}

pub fn send_message(send_socket: &net::UdpSocket, target: String, data: String) {
    match &send_socket.send_to(&(data.into_bytes()), target) {
        Ok(_) => (),
        Err(e) => panic!("Write error: {}", e),
    };
}
