//! アプリケーションで使用するエラーの型を定義する

use thiserror::Error;

#[derive(Error, Debug)]
pub enum PacketCaptureError {
    #[error("Failed to get default interface because following error.\n{0}")]
    FailedToGetDefaultInterface(String),

    #[error("Failed to create ethernet packet.")]
    FailedToCreateEthernetPacket,

    #[error("Failed to create IPv4 packet.")]
    FailedToCreateIpv4Packet,

    #[error("Failed to create ICMP packet.")]
    FailedToCreateIcmpPacket,
}