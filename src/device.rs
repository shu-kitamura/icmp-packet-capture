//! ネットワークデバイスに関する機能を提供します。

use crate::error::AppError;
use pnet_datalink::NetworkInterface;

pub fn get_default_network_interface() -> Result<NetworkInterface, AppError> {
    let all_interfaces: Vec<NetworkInterface> = pnet_datalink::interfaces();
    let network_interface: Option<NetworkInterface> = match netdev::get_default_interface() {
        Ok(i) => all_interfaces
                                .iter()
                                .find(|iface| iface.name == i.name)
                                .cloned(),
        Err(e) => return Err(AppError::FailedToGetDefaultInterface(e)),
    };

    match network_interface {
        Some(i) => Ok(i),
        None => Err(AppError::FailedToGetDefaultInterface("No default interface found.".to_string())),
    }
}
