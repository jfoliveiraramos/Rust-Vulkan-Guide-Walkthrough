use super::AppData;
use anyhow::Result;
use vulkanalia::prelude::v1_0::*;

pub unsafe fn create_sync_objects(device: &Device, data: &mut AppData) -> Result<()> {
    let semaphore_info = vk::SemaphoreCreateInfo::builder();

    data.image_available_semaphore = device.create_semaphore(&semaphore_info, None)?;
    data.render_finished_semaphore = device.create_semaphore(&semaphore_info, None)?;

    Ok(())
}
