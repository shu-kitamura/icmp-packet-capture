mod error;
mod device;
mod handler;

use device::get_default_network_interface;
use handler::{create_ethernet_packet, handle_ethernet_frame};
use pnet_datalink::{
    NetworkInterface,
    Channel::Ethernet,
};

fn main() {
    let network_interface: NetworkInterface = match get_default_network_interface() {
        Ok(i) => i,
        Err(e) => panic!("{e}"),
    };

    let (_, mut rx) = match pnet_datalink::channel(&network_interface, Default::default()) {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unhandled channel type."),
        Err(e) => panic!("Failed to create channel. {}", e),
    };

    println!("Listening started on {}.", network_interface.name);

    loop {
        match rx.next() {
            Ok(frame) => {
                let ethernet_packet = match create_ethernet_packet(frame) {
                    Ok(p) => p,
                    Err(e) => panic!("{e}")
                };
                if let Err(e) = handle_ethernet_frame(ethernet_packet) {
                    eprint!("{e}");
                }
            },
            Err(e) => panic!("Failed to read packet. {}", e),
        }
    }

}
