//! ネットワークデバイスに関する機能を提供します。

use crate::error::PacketCaptureError;
use pnet_datalink::NetworkInterface;

/// 指定された名前のネットワークインターフェースを取得します。
pub fn get_network_interface(interface_name: &str) -> Result<NetworkInterface, PacketCaptureError> {
    // ネットワークインターフェースの一覧を取得し、名前が一致するものを返す
    for iface in pnet_datalink::interfaces() {
        if iface.name == interface_name {
            return Ok(iface);
        }
    }
    return Err(PacketCaptureError::FailedToGetInterface(interface_name.to_string()))
}
