//! Ethrnet フレーム、 ICMP パケットを処理するためのハンドラの機能を提供する

use pnet::packet::{ethernet::{EtherTypes, EthernetPacket}, icmp::IcmpPacket, ip::IpNextHeaderProtocols, ipv4::Ipv4Packet, Packet};

use crate::error::AppError;

pub fn create_ethernet_packet(frame: &[u8]) -> Result<EthernetPacket, AppError> {
    match EthernetPacket::new(frame) {
        Some(p) => Ok(p),
        None => Err(AppError::FailedToCreateEthernetPacket),
    }
}

pub fn handle_ethernet_frame(ethernet_packet: EthernetPacket) -> Result<(), AppError> {
    match ethernet_packet.get_ethertype() {
        EtherTypes::Ipv4 => {
            match Ipv4Packet::new(ethernet_packet.payload()) {
                Some(p) => handle_ipv4_packet(p),
                None => return Err(AppError::FailedToCreateIpv4Packet),
            }
        },
        _ => Ok(()), // IPv4 以外のパケットには何もしない
    }
}

fn handle_ipv4_packet(ipv4_packet: Ipv4Packet) -> Result<(), AppError> {
    match ipv4_packet.get_next_level_protocol() {
        IpNextHeaderProtocols::Icmp => {
            print!("Source address: {:?}, ", ipv4_packet.get_source());
            match IcmpPacket::new(ipv4_packet.payload()) {
                Some(p) => handle_icmp_packet(p),
                None => return Err(AppError::FailedToCreateIcmpPacket),
            }
        },
        _ => Ok(()) // ICMP 以外のパケットには何もしない
    }
}


fn handle_icmp_packet(icmp_packet: IcmpPacket) -> Result<(), AppError> {
    println!("ICMP packet: {:?}", icmp_packet);
    Ok(())
}