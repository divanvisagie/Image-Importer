use rusb::{Device, DeviceDescriptor, GlobalContext, UsbContext};
use std::fmt;
use std::{collections::HashMap, hash::Hash};
use usb_ids::Vendors;

#[derive(Debug)]
pub struct SupportedDevice {
    pub vendor_name: String,
    pub device_name: String,
}

fn hash_device(product_id: u16, vendor_id: u16) -> String {
    format!("{vendor_id}:{product_id}")
}

fn get_supported_devices() -> HashMap<String, SupportedDevice> {
    let mut map: HashMap<String, SupportedDevice> = HashMap::new();
    map.insert(
        hash_device(0x2da, 0x4cb),
        SupportedDevice {
            vendor_name: "Fuji Photo Film Co., Ltd".to_string(),
            device_name: "X T100".to_string(),
        },
    );
    map
}

pub fn list_devices() -> bool {
    let devices = rusb::devices().unwrap();
    let supported = get_supported_devices();
    for device in devices.iter() {
        let descriptor = device.device_descriptor().unwrap();
        let hash = hash_device(descriptor.product_id(), descriptor.vendor_id());
        match supported.get(&hash) {
            Some(x) => {
                println!("Found Device: {} - {}", x.vendor_name, x.device_name);
            }
            None => {
                tracing::info!("This devices is not supported")
            }
        }
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
