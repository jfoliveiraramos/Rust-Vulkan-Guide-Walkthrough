use vulkanalia::prelude::v1_0::*;

use anyhow::{anyhow, Result};
use log::*;
use std::collections::HashSet;

use super::appdata::AppData;
use super::queue::QueueFamilyIndices;
use super::swapchain::SwapchainSupport;
use crate::constants::DEVICE_EXTENSIONS;
use crate::utils::SuitabilityError;

pub unsafe fn pick_physical_device(instance: &Instance, data: &mut AppData) -> Result<()> {
    for physical_device in instance.enumerate_physical_devices()? {
        let properties = instance.get_physical_device_properties(physical_device);

        if let Err(error) = check_physical_device(instance, data, physical_device) {
            warn!(
                "Skipping physical_device (`{}`): {}",
                properties.device_name, error
            )
        } else {
            info!("Selected physical_device (`{}`): ", properties.device_name);
            data.physical_device = physical_device;
            return Ok(());
        }
    }
    Err(anyhow!("Failed to find suitable physical device."))
}

unsafe fn check_physical_device(
    instance: &Instance,
    data: &AppData,
    physical_device: vk::PhysicalDevice,
) -> Result<()> {
    QueueFamilyIndices::get(instance, data, physical_device)?;
    check_physical_device_extensions(instance, physical_device)?;

    let support = SwapchainSupport::get(instance, data, physical_device)?;
    if support.present_modes.is_empty() || support.formats.is_empty() {
        return Err(anyhow!(SuitabilityError("Insufficient swapchain support")));
    }

    Ok(())
}

unsafe fn check_physical_device_extensions(
    instance: &Instance,
    physical_device: vk::PhysicalDevice,
) -> Result<()> {
    let extensions = instance
        .enumerate_device_extension_properties(physical_device, None)?
        .iter()
        .map(|e| e.extension_name)
        .collect::<HashSet<_>>();

    if DEVICE_EXTENSIONS.iter().all(|e| extensions.contains(e)) {
        Ok(())
    } else {
        Err(anyhow!(SuitabilityError(
            "Missing required device extensions."
        )))
    }
}
