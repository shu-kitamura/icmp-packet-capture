//! ネットワークデバイスに関する機能を提供します。

use crate::error::PacketCaptureError;
use pnet_datalink::NetworkInterface;

pub fn get_default_network_interface() -> Result<NetworkInterface, PacketCaptureError> {
    let all_interfaces: Vec<NetworkInterface> = pnet_datalink::interfaces();
    let network_interface: Option<NetworkInterface> = match netdev::get_default_interface() {
        Ok(i) => all_interfaces
                                .into_iter()
                                .find(|iface| iface.name == i.name),
        Err(e) => return Err(PacketCaptureError::FailedToGetDefaultInterface(e)),
    };

    match network_interface {
        Some(i) => Ok(i),
        None => Err(PacketCaptureError::FailedToGetDefaultInterface("No default interface found.".to_string())),
    }
}
