pub mod appdata;
mod command;
mod device;
mod framebuffers;
mod instance;
mod pipeline;
pub mod queue;
mod swapchain;
mod sync;

use std::u64;

use crate::constants::*;
use appdata::AppData;
use command::{create_command_buffers, create_command_pool};
use device::{create_logical_device, pick_physical_device};
use framebuffers::create_frame_buffers;
use instance::*;
use pipeline::{create_pipeline, create_render_pass};
use swapchain::{create_swapchain, create_swapchain_image_views};
use sync::create_sync_objects;

use vulkanalia::loader::{LibloadingLoader, LIBRARY};
use vulkanalia::prelude::v1_0::*;
use vulkanalia::window as vk_window;

use vulkanalia::vk::ExtDebugUtilsExtension;
use vulkanalia::vk::KhrSurfaceExtension;
use vulkanalia::vk::KhrSwapchainExtension;

use winit::window::Window;

use anyhow::{anyhow, Result};

#[derive(Clone, Debug)]
pub struct App {
    entry: Entry,
    instance: Instance,
    data: AppData,
    device: Device,
}

impl App {
    pub unsafe fn create(window: &Window) -> Result<Self> {
        let loader = LibloadingLoader::new(LIBRARY)?;
        let entry = Entry::new(loader).map_err(|b| anyhow!("{}", b))?;
        let mut data = AppData::default();
        let instance = create_instance(window, &entry, &mut data)?;
        data.surface = vk_window::create_surface(&instance, &window, &window)?;
        pick_physical_device(&instance, &mut data)?;
        let device = create_logical_device(&entry, &instance, &mut data)?;
        create_swapchain(window, &instance, &device, &mut data)?;
        create_swapchain_image_views(&device, &mut data)?;
        create_render_pass(&instance, &device, &mut data)?;
        create_pipeline(&device, &mut data)?;
        create_frame_buffers(&device, &mut data)?;
        create_command_pool(&instance, &device, &mut data)?;
        create_command_buffers(&device, &mut data)?;
        create_sync_objects(&device, &mut data)?;
        Ok(Self {
            entry,
            instance,
            data,
            device,
        })
    }

    pub unsafe fn render(&mut self, window: &Window) -> Result<()> {
        let image_index = self
            .device
            .acquire_next_image_khr(
                self.data.swapchain,
                u64::MAX,
                self.data.image_available_semaphore,
                vk::Fence::null(),
            )?
            .0 as usize;

        let wait_semaphores = &[self.data.image_available_semaphore];
        let wait_stages = &[vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT];
        let command_buffers = &[self.data.command_buffers[image_index as usize]];
        let signal_semaphores = &[self.data.render_finished_semaphore];
        let submit_info = vk::SubmitInfo::builder()
            .wait_semaphores(wait_semaphores)
            .wait_dst_stage_mask(wait_stages)
            .command_buffers(command_buffers)
            .signal_semaphores(signal_semaphores);

        self.device
            .queue_submit(self.data.graphics_queue, &[submit_info], vk::Fence::null())?;

        let swapchains = &[self.data.swapchain];
        let image_indices = &[image_index as u32];
        let present_info = vk::PresentInfoKHR::builder()
            .wait_semaphores(signal_semaphores)
            .swapchains(swapchains)
            .image_indices(image_indices);

        self.device
            .queue_present_khr(self.data.present_queue, &present_info)?;

        Ok(())
    }

    pub unsafe fn destroy(&mut self) {
        self.device
            .destroy_semaphore(self.data.image_available_semaphore, None);
        self.device
            .destroy_semaphore(self.data.render_finished_semaphore, None);

        self.device
            .destroy_command_pool(self.data.command_pool, None);

        self.data
            .framebuffers
            .iter()
            .for_each(|fb| self.device.destroy_framebuffer(*fb, None));

        self.device.destroy_pipeline(self.data.pipeline, None);
        self.device
            .destroy_pipeline_layout(self.data.pipeline_layout, None);
        self.device.destroy_render_pass(self.data.render_pass, None);

        self.data
            .swapchain_image_views
            .iter()
            .for_each(|v| self.device.destroy_image_view(*v, None));
        self.device.destroy_swapchain_khr(self.data.swapchain, None);

        self.device.destroy_device(None);
        self.instance.destroy_surface_khr(self.data.surface, None);

        if VALIDATION_ENABLED {
            self.instance
                .destroy_debug_utils_messenger_ext(self.data.messenger, None)
        }
        self.instance.destroy_instance(None);
    }
}
