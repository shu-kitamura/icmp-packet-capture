mod error;
mod device;
mod handler;

use device::get_network_interface;
use handler::{create_ethernet_frame, handle_ethernet_frame};
use pnet_datalink::{
    NetworkInterface,
    Channel::Ethernet,
};
use clap::Parser;

fn main() {
    // コマンドライン引数をパースする
    let args = Args::parse();
    let interface_name: String = args.network_interface;

    let network_interface: NetworkInterface = match get_network_interface(&interface_name) {
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
                let ethernet_packet = match create_ethernet_frame(frame) {
                    Ok(p) => p,
                    Err(e) => panic!("{e}")
                };
                if let Err(e) = handle_ethernet_frame(ethernet_packet) {
                    eprintln!("{e}");
                }
            },
            Err(e) => panic!("Failed to read packet. {}", e),
        }
    }
}

// コマンドライン引数の定義
#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Network interface name
    #[arg(value_name = "NETWORK_INTERFACE")]
    network_interface: String,
}
