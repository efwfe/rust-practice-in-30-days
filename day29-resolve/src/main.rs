use std::net::{SocketAddr, UdpSocket};
use std::time::Duration;

use clap::{App, Arg};
use rand;
use trust_dns::op::{Message, MessageType, OpCode, Query};
use trust_dns::rr::domain::Name;
use trust_dns::rr::record_type::RecordType;
use trust_dns::serialize::binary::BinEncoder;
use trust_dns::serialize::binary::*;

fn main(){
    let app = App::new("resolve")
            .about("A simple to use DNS resolver")
            .arg(Arg::with_name("dns-server").short("s").default_value("1.1.1.1"))
            .arg(Arg::with_name("domain-name").required(true))
            .get_matches();
    let domain_name_raw = app.value_of("domain-name").unwrap();
    let domain_name = Name::from_ascii(&domain_name_raw).unwrap(); // 把参数转换为域名

    let dns_server_raw = app.value_of("dns-server").unwrap();
    let dns_server : SocketAddr = format!("{}:53", dns_server_raw).parse().expect("invalid address");
    let mut request_as_bytes: Vec<u8> = Vec::with_capacity(512); // Vec<T> 长度为0，容量为512
    let mut response_as_bytes: Vec<u8> = vec![0; 512];          // Vec<T> 长度为512， 容量为512
    
    let mut msg = Message::new();
    msg.set_id(rand::random::<u16>())
        .set_message_type(MessageType::Query)
        .add_query(Query::query(domain_name, RecordType::A))
        .set_op_code(OpCode::Query)
        .set_recursion_desired(true);

    let mut encoder = BinEncoder::new(&mut request_as_bytes); // 转换message为原始字节
    msg.emit(&mut encoder).unwrap();

    let localhost = UdpSocket::bind("0.0.0.0:0").expect("Connot bind to local socket");
    let timeout = Duration::from_secs(3);
    localhost.set_read_timeout(Some(timeout)).unwrap();
    localhost.set_nonblocking(false).unwrap();

    let _amt = localhost.send_to(&request_as_bytes, dns_server)
                .expect("socket imsconfigured");
    let (_amt, _remote) = localhost.recv_from(&mut response_as_bytes)
                .expect("timeout reached");
    let dns_message = Message::from_vec(&response_as_bytes)
                .expect("unable to parse response");
    for anwser in dns_message.answers(){
        if anwser.record_type() == RecordType::A{
            let resource = anwser.rdata();
            let ip = resource.to_ip_addr().expect("invalid ip address received");
            println!("{}", ip.to_string());
        }
    }



}

// cargo run -- google.com -s 8.8.8.8