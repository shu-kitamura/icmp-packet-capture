//! Ethernet フレーム、 ICMP パケットを処理するためのハンドラの機能を提供する。
//! 送信元の IP アドレスと ICMP パケットの内容を表示する。

use pnet::packet::{
    ethernet::{EtherTypes, EthernetPacket},
    icmp::IcmpPacket,
    ip::IpNextHeaderProtocols,
    ipv4::Ipv4Packet, Packet
};

use crate::error::PacketCaptureError;

/// 指定されたバイト列から Ethernet フレームを生成する関数
pub fn create_ethernet_frame(bytes: &[u8]) -> Result<EthernetPacket, PacketCaptureError> {
    match EthernetPacket::new(bytes) {
        Some(p) => Ok(p),
        None => Err(PacketCaptureError::FailedToCreateEthernetPacket),
    }
}

/// 受信した Ethernet フレームを処理する関数
pub fn handle_ethernet_frame(ethernet_packet: EthernetPacket) -> Result<Option<()>, PacketCaptureError> {
    match ethernet_packet.get_ethertype() {
        EtherTypes::Ipv4 => {
            match Ipv4Packet::new(ethernet_packet.payload()) {
                Some(p) => handle_ipv4_packet(p),
                None => return Err(PacketCaptureError::FailedToCreateIpv4Packet),
            }
        },
        _ => Ok(None), // IPv4 以外のパケットには何もしない
    }
}

/// 受信した IPv4 パケットを処理する関数
fn handle_ipv4_packet(ipv4_packet: Ipv4Packet) -> Result<Option<()>, PacketCaptureError> {
    match ipv4_packet.get_next_level_protocol() {
        IpNextHeaderProtocols::Icmp => {
            print!("Source address: {:?}, ", ipv4_packet.get_source());
            match IcmpPacket::new(ipv4_packet.payload()) {
                Some(p) => handle_icmp_packet(p),
                None => return Err(PacketCaptureError::FailedToCreateIcmpPacket),
            }
        },
        _ => Ok(None) // ICMP 以外のパケットには何もしない
    }
}

/// 受信した ICMP パケットを処理する関数
fn handle_icmp_packet(icmp_packet: IcmpPacket) -> Result<Option<()>, PacketCaptureError> {
    println!("ICMP packet: {:?}", icmp_packet);
    Ok(Some(()))
}