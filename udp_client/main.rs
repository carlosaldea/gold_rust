use std::net::UdpSocket;

fn main() {

    let socket = UdpSocket::bind("127.0.0.1:45234").expect("couldn't bind to address");
    let message = "hello dear";
    socket.send_to(message.as_bytes(), "127.0.0.1:34254").expect("couldn't send data");
}
