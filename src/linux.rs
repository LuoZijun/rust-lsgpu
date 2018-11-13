#![cfg(target_os = "linux")]

use crate::vulkano::device::{
    Device, DeviceExtensions, Features,
};
use crate::vulkano::instance::{
    Instance, InstanceExtensions, PhysicalDevice, PhysicalDeviceType,
    Version, Features,
 };



#[derive(Debug, Clone)]
pub struct GpuInfo {
    // name: String,
    // vendor: String,
    // family: String,
    // registry_id: u64,
    // features: Vec<MTLFeatureSet>,
    // is_low_power: bool, // Integrated GPU
    // is_headless: bool,
    // d24_s8_supported: bool,

    index: usize,
    name: String,
    kind: PhysicalDeviceType,
    api_version: Version,
    supported_features: Features,
    driver_version: u32,
    pci_device_id: u32,
    pci_vendor_id: u32,
    uuid: [u8; 16],
}

fn get_gpu_info(phy_device: PhysicalDevice) -> GpuInfo {
    GpuInfo {
        index: phy_device.index(),
        name: phy_device.name(),
        kind: phy_device.ty(),
        api_version: phy_device.api_version(),
        supported_features: phy_device.supported_features().clone(),
        driver_version: phy_device.driver_version(),
        pci_device_id: phy_device.pci_device_id(),
        pci_vendor_id: phy_device.pci_vendor_id(),
        uuid: phy_device.uuid().clone(),
    }
}

pub fn default() -> GpuInfo {
    Instance::new(None, &InstanceExtensions::none(), None)
        .map(|instance| {
            PhysicalDevice::enumerate(&instance)
                .next()
                .and_then()
                .and_then(|physical_device| get_gpu_info(physical_device) )
                .or_else(|| panic!("Couldn't find phy device!") )
        })
        .map_err(|e| panic!("Couldn't build instance: {:?}", e) )
}


pub fn all() -> Vec<GpuInfo> {
    let instance = match Instance::new(None, &InstanceExtensions::none(), None) {
        Ok(i) => i,
        Err(err) => panic!("Couldn't build instance: {:?}", err)
    };
    
    PhysicalDevice::enumerate(&instance)
        .map(|physical_device| get_gpu_info(physical_device) )
        .collect::<Vec<GpuInfo>>()
}