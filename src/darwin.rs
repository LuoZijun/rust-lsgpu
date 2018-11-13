#![cfg(any(target_os = "macos", target_os = "ios", target_os = "tvos"))]

use crate::metal::*;


#[derive(Debug, Clone)]
pub struct GpuInfo {
    name: String,
    vendor: String,
    family: String,
    registry_id: u64,
    features: Vec<MTLFeatureSet>,
    is_low_power: bool, // Integrated GPU
    is_headless: bool,
    d24_s8_supported: bool,
}

fn get_features(device: &Device) -> Vec<MTLFeatureSet> {
    let mut features: Vec<MTLFeatureSet> = vec![];

    if device.supports_feature_set(MTLFeatureSet::iOS_GPUFamily1_v1) {
        features.push(MTLFeatureSet::iOS_GPUFamily1_v1);
    }
    if device.supports_feature_set(MTLFeatureSet::iOS_GPUFamily2_v1) {
        features.push(MTLFeatureSet::iOS_GPUFamily2_v1);
    }
    if device.supports_feature_set(MTLFeatureSet::iOS_GPUFamily1_v2) {
        features.push(MTLFeatureSet::iOS_GPUFamily1_v2);
    }
    if device.supports_feature_set(MTLFeatureSet::iOS_GPUFamily2_v2) {
        features.push(MTLFeatureSet::iOS_GPUFamily2_v2);
    }
    if device.supports_feature_set(MTLFeatureSet::iOS_GPUFamily3_v1) {
        features.push(MTLFeatureSet::iOS_GPUFamily3_v1);
    }
    if device.supports_feature_set(MTLFeatureSet::iOS_GPUFamily1_v3) {
        features.push(MTLFeatureSet::iOS_GPUFamily1_v3);
    }
    if device.supports_feature_set(MTLFeatureSet::iOS_GPUFamily2_v3) {
        features.push(MTLFeatureSet::iOS_GPUFamily2_v3);
    }
    if device.supports_feature_set(MTLFeatureSet::iOS_GPUFamily3_v2) {
        features.push(MTLFeatureSet::iOS_GPUFamily3_v2);
    }
    if device.supports_feature_set(MTLFeatureSet::iOS_GPUFamily1_v4) {
        features.push(MTLFeatureSet::iOS_GPUFamily1_v4);
    }
    if device.supports_feature_set(MTLFeatureSet::iOS_GPUFamily2_v4) {
        features.push(MTLFeatureSet::iOS_GPUFamily2_v4);
    }
    if device.supports_feature_set(MTLFeatureSet::iOS_GPUFamily3_v3) {
        features.push(MTLFeatureSet::iOS_GPUFamily3_v3);
    }
    if device.supports_feature_set(MTLFeatureSet::iOS_GPUFamily4_v1) {
        features.push(MTLFeatureSet::iOS_GPUFamily4_v1);
    }
    if device.supports_feature_set(MTLFeatureSet::tvOS_GPUFamily1_v1) {
        features.push(MTLFeatureSet::tvOS_GPUFamily1_v1);
    }
    if device.supports_feature_set(MTLFeatureSet::tvOS_GPUFamily1_v2) {
        features.push(MTLFeatureSet::tvOS_GPUFamily1_v2);
    }
    if device.supports_feature_set(MTLFeatureSet::tvOS_GPUFamily1_v3) {
        features.push(MTLFeatureSet::tvOS_GPUFamily1_v3);
    }
    if device.supports_feature_set(MTLFeatureSet::tvOS_GPUFamily2_v1) {
        features.push(MTLFeatureSet::tvOS_GPUFamily2_v1);
    }
    if device.supports_feature_set(MTLFeatureSet::macOS_GPUFamily1_v1) {
        features.push(MTLFeatureSet::macOS_GPUFamily1_v1);
    }
    if device.supports_feature_set(MTLFeatureSet::macOS_GPUFamily1_v2) {
        features.push(MTLFeatureSet::macOS_GPUFamily1_v2);
    }
    if device.supports_feature_set(MTLFeatureSet::macOS_GPUFamily1_v3) {
        features.push(MTLFeatureSet::macOS_GPUFamily1_v3);
    }

    features
}

fn get_gpu_info(device: &Device) -> GpuInfo {
    #[cfg(target_os = "macos")]
    let is_low_power = device.is_low_power();
    #[cfg(any(target_os = "iOS", target_os = "tvOS"))]
    let is_low_power = false;

    #[cfg(target_os = "macos")]
    let is_headless = device.is_headless();
    #[cfg(any(target_os = "iOS", target_os = "tvOS"))]
    let is_headless = false;

    #[cfg(target_os = "macos")]
    let d24_s8_supported = device.d24_s8_supported();
    #[cfg(any(target_os = "iOS", target_os = "tvOS"))]
    let d24_s8_supported = false;
    
    let gpu_info = GpuInfo {
        name: device.name().to_string(),
        vendor: unsafe { device.vendor().to_string() },
        family: unsafe { device.family_name().to_string() },
        registry_id: device.registry_id(),
        features: get_features(&device),
        is_low_power: is_low_power,
        is_headless: is_headless,
        d24_s8_supported: d24_s8_supported,
    };
    
    // println!("{:?}", gpu_info);
    // println!("Indirect argument buffer: {:?}", device.argument_buffers_support());
    gpu_info
}

pub fn default() -> GpuInfo {
    let device = Device::system_default();
    get_gpu_info(&device)
}

pub fn all() -> Vec<GpuInfo> {
    Device::all()
        .iter()
        .map(|device: &Device| get_gpu_info(&device))
        .collect::<Vec<GpuInfo>>()
}