fn main() {
    let address = local_ip_address::local_ip().expect("Failed to get local IP address");
    println!("The local ip address is: {address}");

    let interfaces = local_ip_address::get_interfaces().expect("Failed to get interfaces");
    println!("Interfaces: {interfaces:?}")
}
