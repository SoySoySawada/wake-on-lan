use std::env;
use std::iter;
use std::net::UdpSocket;

fn main() {    
    // MAC Address(引数から取得)
    let mac_addr_vec = get_mac_addr_vec();

    for mac_addr in mac_addr_vec {
        send_magic_packet(mac_addr);
    }
}

/// 
/// 引数からMACアドレス(Vec<u8>型)のVectorを取得する
/// 
/// panic! 
/// ・引数がない場合
/// ・引数にMACアドレスの形式に合わない文字列が含まれる場合
fn get_mac_addr_vec() -> Vec<Vec<u8>> {
    let args: Vec<String> = env::args().collect();
    let mut mac_addr_vec: Vec<Vec<u8>> = Vec::new();

    if args.len() == 1 {
        panic!("MACアドレスを1つ以上指定してください。(例：wake-on-lan XX:XX:XX:XX:XX:XX)");
    }

    for arg in args[1..].iter() {
        let mac_addr: Vec<u8> = arg
            .split(":")
            .flat_map(|s| {
                match hex::decode(s) {
                    Ok(x) => x,
                    Err(e) => panic!("MACアドレスの形式が不正です。(例：wake-on-lan XX:XX:XX:XX:XX:XX){:?}{:?}", s, e),
                }})
            .collect();

        if mac_addr.len() != 6 {
            panic!("MACアドレスの形式が不正です。(例：wake-on-lan XX:XX:XX:XX:XX:XX)");
        }

        mac_addr_vec.push(mac_addr);
    }

    mac_addr_vec
}

fn send_magic_packet(mac_addr: Vec<u8>) {
    // Magic Packet
    let mut packet = vec![0xFFu8; 6];
    packet.extend(iter::repeat(mac_addr).take(16).flatten());

    // broadcast
    // 使用可能なポートを自動で使用
    let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
    socket.set_broadcast(true).unwrap();
    // ネットワーク上の任意のデバイスのポート9に対してパケット送信(ブロードキャスト)
    socket.send_to(&packet, "255.255.255.255:9").unwrap();
}


// fn main() {    
//     // MAC Address(引数から取得)
//     let mac_addr_vec = get_mac_addr_vec();

//     let mac_addr = "48:21:0B:5B:BC:D3";
//     let mac_addr: Vec<u8> = mac_addr
//         .split(":")
//         .flat_map(|s| hex::decode(s).unwrap())
//         .collect();

//     // Magic Packet
//     let mut packet = vec![0xFFu8; 6];
//     packet.extend(iter::repeat(mac_addr).take(16).flatten());

//     // broadcast
//     // 使用可能なポートを自動で使用
//     let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
//     socket.set_broadcast(true).unwrap();
//     // ネットワーク上の任意のデバイスのポート9に対してパケット送信(ブロードキャスト)
//     socket.send_to(&packet, "255.255.255.255:9").unwrap();
// }