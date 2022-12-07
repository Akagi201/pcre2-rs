use clap::Parser;
use std::net::SocketAddr;
use std::net::UdpSocket;

mod pcre2;
use pcre2::*;

#[derive(Parser, Debug)]
struct Cli {
    #[clap(short, long, default_value = "127.0.0.1")]
    addr: String,
    #[clap(short, long, default_value = "8327")]
    port: u16,
}

fn get_matched(target: &str, begin: usize, end: usize) -> String {
    String::from(&target[begin..end])
}

fn main() {
    let cli = Cli::parse();
    println!("Send udp result to {}:{}", cli.addr, cli.port);
    let target = r"a;jhgoqoghqoj0329 u0tyu10hg0h9Y0Y9827342482y(Y0y(G)_)lajf;lqjfgqhgpqjopjqa=)*(^!@#$%^&*())9999999";
    // Use Positive Lookbehind (?<=) and Positive Lookahead (?=)
    let pattern = r"(?<=\d{4})[^\d\s]{3,11}(?=\S)";
    // We use port 0 to let the operating system allocate an available port for us.
    let local_addr: SocketAddr = "0.0.0.0:0".parse().expect("Could not parse local address");
    let socket = UdpSocket::bind(local_addr).expect("Could not bind server!");
    for mat in Regex::new(pattern).unwrap().find_iter(target) {
        let matched = get_matched(target, mat.0, mat.1);
        println!("Matched: {matched}");
        let len = socket
            .send_to(matched.as_bytes(), &(cli.addr.to_string(), cli.port))
            .expect("Could not send data to server");
        println!("Send len: {len}");
    }
}
