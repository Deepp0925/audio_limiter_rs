use cpal::{
    default_host,
    platform::{Device, Host},
    traits::{DeviceTrait, HostTrait},
};
fn main() {
    let host = default_host();
    for device in host.devices().unwrap() {
        println!("device name: {:?}", device.name().unwrap());
        println!("device info output: {:?}", device.default_output_config());
        println!()
    }
}
