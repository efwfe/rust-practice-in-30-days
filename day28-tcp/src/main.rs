use std::net::TcpStream;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let host = "www.rustaction.com:80";
    let mut conn = TcpStream::connect(host)?;

    conn.write_all(b"GET / HTTP/1.0")?;
    conn.write_all(b"\r\n");
    conn.write_all(b"Host: www.rustaction.com")?;
    conn.write_all(b"\r\n\r\n");

    std::io::copy(&mut conn, &mut std::io::stdout());
    Ok(())
}
