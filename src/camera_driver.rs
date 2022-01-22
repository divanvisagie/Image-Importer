


pub fn return_true() -> bool {
    for device in rusb::devices().unwrap().iter() {
        let device_desc = device.device_descriptor().unwrap();

        println!("Bus {:03} Device {:03} ID {:04x}:{:04x}",
            device.bus_number(),
            device.address(),
            device_desc.vendor_id(),
            device_desc.product_id());
    }

    true
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_detect() {
        assert_eq!(true, return_true());
    }
}