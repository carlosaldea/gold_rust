use std::net::UdpSocket;

fn main() -> std::io::Result<()> {
    {
        let socket = UdpSocket::bind("127.0.0.1:34254")?;

        // Receives a single datagram message on the socket. If `buf` is too small to hold
        // the message, it will be cut off.
        let mut buf = [0; 10];
        let (amt, _src) = socket.recv_from(&mut buf)?;

        for n in 0 .. amt
        {
            print!("{}", buf[n] as char);
        }
        println!(" ");
    } // the socket is closed here
    Ok(())
}
