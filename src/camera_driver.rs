use rusb::{Device, GlobalContext};
use std::collections::HashMap;
use usb_ids::Vendors;

pub fn get_vendor_map() -> HashMap<u16, &'static str> {
    let mut vendors: HashMap<u16, &str> = HashMap::new();
    vendors.insert(0x046d, "Logitech, Inc.");
    vendors.insert(0x1E71, "NZXT");
    vendors.insert(0x05ac, "Apple, Inc.");
    vendors.insert(0x05e3, "Genesys Logic, Inc.");

    for vendor in Vendors::iter() {
        vendors.insert(vendor.id(), vendor.name());
        // for device in vendor.devices() {
        //     println!("vendor: {}, device: {}", vendor.name(), device.name());
        // }
    }

    vendors
}

pub fn get_vendor_for_id(id: u16) -> String {
    let map = get_vendor_map();
    let ans = match map.get(&id) {
        Some(x) => x.to_string(),
        None => {
            format!("{:04x}", &id)
        }
    };
    ans
}

pub fn print_device(device: Device<GlobalContext>) {
    let device_desc = device.device_descriptor().unwrap();
    let bus = device.bus_number();
    let addr = device.address();
    let vendor_id = device_desc.vendor_id();
    let product_id = device_desc.product_id();

    let vendor_name = get_vendor_for_id(vendor_id);

    println!("Bus {bus:03} Device {addr:03} ID {vendor_name}:{product_id:04x}");
}

pub fn list_devices() -> bool {
    for device in rusb::devices().unwrap().iter() {
        print_device(device)
    }

    true
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_detect() {
        assert_eq!(true, list_devices());
    }
}
