use std::net::UdpSocket;
extern crate reqwest;

pub fn GetServerInfo(bind_to: String, send_to: String) -> Result<String,String> {
    let mut socket = UdpSocket::bind(bind_to).expect("couldn't bind to address");
    socket.send_to(&[0; 10], send_to).expect("couldn't send data");

    // Receives a single datagram message on the socket. If `buf` is too small to hold
    // the message, it will be cut off.
    let mut buf = [0; 10];
    let (amt, src) = socket.recv_from(&mut buf)?;

    // Redeclare `buf` as slice of the received data and send reverse data back to origin.
    let buf = &mut buf[..amt];
    buf.reverse();
} // the socket is closed here