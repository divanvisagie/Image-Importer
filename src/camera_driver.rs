use rusb::{Device, DeviceDescriptor, GlobalContext, UsbContext};
use std::fmt;
use std::{collections::HashMap, hash::Hash};
use usb_ids::Vendors;

pub struct FriendlyDevice {
    descriptor: DeviceDescriptor,
    pub vendor_name: Option<String>,
    pub device_name: Option<String>,
}

impl fmt::Debug for FriendlyDevice {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Vendor: {:?}:{:?} \nDevice: {:?}:{:?}\n------------------------",
            self.descriptor.vendor_id(),
            self.vendor_name,
            self.descriptor.product_id(),
            self.device_name
        )
    }
}

impl FriendlyDevice {
    pub fn new(descriptor: DeviceDescriptor) -> FriendlyDevice {
        let mut vendorname = None;
        let mut devicename = None;
        for vendor in Vendors::iter() {
            if descriptor.vendor_id() == vendor.id() {
                vendorname = Some(vendor.name().to_string());
                for device in vendor.devices() {
                    if device.id() == descriptor.product_id() {
                        devicename = Some(device.name().to_string());
                        break;
                    }
                }
            }
        }

        FriendlyDevice {
            descriptor,
            vendor_name: vendorname,
            device_name: devicename,
        }
    }
}

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

    println!("Bus {bus:03} Device {addr:03} ID {vendor_name} - {product_id:04x}");
}

pub fn get_friendly_devices() -> Vec<FriendlyDevice> {
    let device_list = if let Ok(dl) = rusb::devices() {
        dl.iter().collect()
    } else {
        Vec::new()
    };

    let devices = device_list
        .iter()
        .map(|d| FriendlyDevice::new(d.device_descriptor().unwrap()));

    devices.collect()
}

pub fn list_devices() -> bool {
    // let devices = rusb::devices().unwrap().iter();

    // let devices = devices.map(|d| {
    //     tracing::info!("Just for fun");
    //     FriendlyDevice::new(d.device_descriptor().unwrap())
    // });

    let devices = get_friendly_devices();

    println!("{:#?}", devices);
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
