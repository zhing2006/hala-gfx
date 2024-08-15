use std::rc::Rc;
use std::cell::RefCell;
use std::time::Duration;

use crate::{
  HalaGPURequirements,
  HalaInstance,
  HalaPhysicalDevice,
  HalaSurface,
  HalaLogicalDevice,
  HalaSwapchain,
  HalaCommandPools,
  HalaCommandBufferSet,
  HalaPipelineStageFlags2,
  HalaQueryPool,
  HalaImage,
  HalaGfxError,
  HalaFormat,
  HalaSampleCountFlags,
};

/// The context of the hala-gfx crate.
pub struct HalaContext {
  pub name: String,
  pub gpu_req: HalaGPURequirements,
  pub instance: std::mem::ManuallyDrop<HalaInstance>,
  pub physical_device: HalaPhysicalDevice,
  pub surface: std::mem::ManuallyDrop<HalaSurface>,
  pub swapchain: std::mem::ManuallyDrop<HalaSwapchain>,
  pub pools: std::mem::ManuallyDrop<Rc<RefCell<HalaCommandPools>>>,
  pub short_time_pools: std::mem::ManuallyDrop<Rc<RefCell<HalaCommandPools>>>,
  pub logical_device: std::mem::ManuallyDrop<Rc<RefCell<HalaLogicalDevice>>>,
  pub timestamp_query_pool: std::mem::ManuallyDrop<HalaQueryPool>,

  pub multisample_count: HalaSampleCountFlags,
}

/// The Drop trait implementation of the context of the hala-gfx crate.
impl Drop for HalaContext {
  fn drop(&mut self) {
    unsafe {
      std::mem::ManuallyDrop::drop(&mut self.timestamp_query_pool);
      std::mem::ManuallyDrop::drop(&mut self.short_time_pools);
      std::mem::ManuallyDrop::drop(&mut self.pools);
      std::mem::ManuallyDrop::drop(&mut self.swapchain);
      std::mem::ManuallyDrop::drop(&mut self.surface);
      std::mem::ManuallyDrop::drop(&mut self.logical_device);
      std::mem::ManuallyDrop::drop(&mut self.instance);
    }
    log::debug!("A HalaContext is dropped.");
  }
}

/// The implementation of the context of the hala-gfx crate.
impl HalaContext {
  /// Create a new context.
  /// param name: The name of the context.
  /// param gpu_req: The GPU requirements.
  /// param window: The window.
  /// return: The context.
  pub fn new(name: &str, gpu_req: &HalaGPURequirements, window: &winit::window::Window) -> Result<Self, HalaGfxError> {
    // Validate the GPU requirements.
    if gpu_req.require_10bits_output && gpu_req.require_srgb_surface {
      return Err(HalaGfxError::new("10bits output and sRGB surface can't be required at the same time.", None));
    }

    // Create instance.
    let instance = crate::HalaInstance::new(name, gpu_req)?;

    // Create physical device.
    let physical_device = crate::HalaPhysicalDevice::new(gpu_req, &instance)?;
    log::debug!("We choose the physical device {}", physical_device.device_name);

    // Create surface.
    let surface = crate::HalaSurface::new(&instance, window)?;

    // Create logical device.
    let logical_device = Rc::new(
      RefCell::new(
        crate::HalaLogicalDevice::new(gpu_req, &instance, &physical_device, &surface)?
      )
    );

    // Create swapchain.
    let swapchain = crate::HalaSwapchain::new(
      gpu_req,
      &instance,
      &physical_device,
      Rc::clone(&logical_device),
      &surface)?;

    let pools = Rc::new(
      RefCell::new(
        crate::HalaCommandPools::new(
          Rc::clone(&logical_device),
          false,
          "main.command_pool",
        )?
      )
    );
    let short_time_pools = Rc::new(
      RefCell::new(
        crate::HalaCommandPools::new(
          Rc::clone(&logical_device),
          true,
          "short_time.command_pool",
        )?
      )
    );

    // Create timestamp query pool.
    let timestamp_query_pool = HalaQueryPool::new_timestamp(
      &physical_device,
      Rc::clone(&logical_device),
      (swapchain.num_of_images * 2) as u32,
      "timestamp.query_pool",
    )?;

    log::debug!("A HalaContext is created.");
    Ok(
      Self {
        name: name.to_string(),
        gpu_req: gpu_req.clone(),
        instance: std::mem::ManuallyDrop::new(instance),
        physical_device,
        surface: std::mem::ManuallyDrop::new(surface),
        swapchain: std::mem::ManuallyDrop::new(swapchain),
        logical_device: std::mem::ManuallyDrop::new(logical_device),
        pools: std::mem::ManuallyDrop::new(pools),
        short_time_pools: std::mem::ManuallyDrop::new(short_time_pools),
        timestamp_query_pool: std::mem::ManuallyDrop::new(timestamp_query_pool),
        multisample_count: HalaSampleCountFlags::TYPE_1,
      }
    )
  }

  /// Prepare some sync signals for this frame.
  pub fn prepare_frame(&self) -> Result<usize, HalaGfxError> {
    let image_index = self.swapchain.acquire_next_image()?;
    self.swapchain.wait_for_fence(image_index)?;
    self.swapchain.reset_fence(image_index)?;
    Ok(image_index)
  }

  /// Submit and present the frame.
  /// param index: The index of the frame image.
  /// param command_buffers: The graphics command buffer set.
  pub fn submit_and_present_frame(&mut self, index: usize, command_buffers: &HalaCommandBufferSet) -> Result<(), HalaGfxError> {
    self.swapchain.submit(command_buffers, index, 0)?;
    self.swapchain.present(index as u32)?;
    Ok(())
  }

  /// Get GPU frame time.
  /// param index: The index of the frame image.
  /// return: The GPU frame time.
  pub fn get_gpu_frame_time(&self, index: usize) -> Result<Duration, HalaGfxError> {
    let result = self.timestamp_query_pool.wait((index * 2) as u32, 2)?;
    let time = Duration::from_nanos(
      (result[1].saturating_sub(result[0]) as f64 * self.timestamp_query_pool.timestamp_period) as u64);

    Ok(time)
  }

  /// Reset the swapchain.
  /// param width: The width of the swapchain.
  /// param height: The height of the swapchain.
  /// return: The result.
  pub fn reset_swapchain(&mut self, width: u32, height: u32) -> Result<(), HalaGfxError> {
    self.logical_device.borrow().wait_idle()?;

    unsafe {
      std::mem::ManuallyDrop::drop(&mut self.swapchain);
    }

    self.gpu_req.width = width;
    self.gpu_req.height = height;
    let swapchain = crate::HalaSwapchain::new(
      &self.gpu_req,
      &self.instance,
      &self.physical_device,
      Rc::clone(&self.logical_device),
      &self.surface)?;

    self.swapchain = std::mem::ManuallyDrop::new(swapchain);

    Ok(())
  }

  /// Record a graphics command buffer.
  /// param index: The index of the command buffer.
  /// param command_buffers: The command buffer set.
  /// param render_pass: The render pass.
  /// param framebuffers: The frame buffer set.
  /// param clear_values: The clear values(color, depth, stencil).
  /// param graphics_fn: The graphics command record function.
  /// param ray_tracing_image: The ray tracing image.
  /// param ray_tracing_fn: The ray tracing command record function.
  /// return: The result.
  #[allow(clippy::too_many_arguments)]
  pub fn record_graphics_command_buffer<
    F1: FnOnce(usize, &HalaCommandBufferSet) -> Result<(), HalaGfxError>,
    F2: FnOnce(usize, &HalaCommandBufferSet) -> Result<bool, HalaGfxError>
  > (
    &self,
    index: usize,
    command_buffers: &HalaCommandBufferSet,
    color_clear_values: Option<[f32; 4]>,
    depth_clear_value: Option<f32>,
    stencil_clear_value: Option<u32>,
    graphics_fn: F1,
    ray_tracing_image: Option<&HalaImage>,
    ray_tracing_fn: F2,
  ) -> Result<(), HalaGfxError> {
    command_buffers.reset(index, false)?;
    command_buffers.begin(index, crate::HalaCommandBufferUsageFlags::empty())?;
    command_buffers.reset_query_pool(index, &self.timestamp_query_pool, (index * 2) as u32, 2);
    command_buffers.write_timestamp(index, HalaPipelineStageFlags2::NONE, &self.timestamp_query_pool, (index * 2) as u32);

    let need_copy_to_swapchain = ray_tracing_fn(index, command_buffers)?;

    if need_copy_to_swapchain {
      command_buffers.set_image_barriers(
        index,
        &[
          crate::HalaImageBarrierInfo {
            old_layout: crate::HalaImageLayout::UNDEFINED,
            new_layout: crate::HalaImageLayout::TRANSFER_DST_OPTIMAL,
            src_access_mask: crate::HalaAccessFlags2::NONE,
            dst_access_mask: crate::HalaAccessFlags2::TRANSFER_WRITE,
            src_stage_mask: crate::HalaPipelineStageFlags2::TOP_OF_PIPE,
            dst_stage_mask: crate::HalaPipelineStageFlags2::TRANSFER,
            aspect_mask: crate::HalaImageAspectFlags::COLOR,
            image: self.swapchain.images[index],
            ..Default::default()
          },
          crate::HalaImageBarrierInfo {
            old_layout: crate::HalaImageLayout::GENERAL,
            new_layout: crate::HalaImageLayout::TRANSFER_SRC_OPTIMAL,
            src_access_mask: crate::HalaAccessFlags2::SHADER_WRITE,
            dst_access_mask: crate::HalaAccessFlags2::TRANSFER_READ,
            src_stage_mask: crate::HalaPipelineStageFlags2::RAY_TRACING_SHADER,
            dst_stage_mask: crate::HalaPipelineStageFlags2::TRANSFER,
            aspect_mask: crate::HalaImageAspectFlags::COLOR,
            image: ray_tracing_image.unwrap().raw,
            ..Default::default()
          }
        ],
      );

      command_buffers.copy_image_2_swapchain(
        index,
        ray_tracing_image.unwrap(),
        crate::HalaImageLayout::TRANSFER_SRC_OPTIMAL,
        &self.swapchain,
        crate::HalaImageLayout::TRANSFER_DST_OPTIMAL,
        index,
      );

      let mut barriers = vec![
        crate::HalaImageBarrierInfo {
          old_layout: crate::HalaImageLayout::TRANSFER_DST_OPTIMAL,
          new_layout: crate::HalaImageLayout::COLOR_ATTACHMENT_OPTIMAL,
          src_access_mask: crate::HalaAccessFlags2::TRANSFER_WRITE,
          dst_access_mask: crate::HalaAccessFlags2::COLOR_ATTACHMENT_WRITE,
          src_stage_mask: crate::HalaPipelineStageFlags2::TRANSFER,
          dst_stage_mask: crate::HalaPipelineStageFlags2::COLOR_ATTACHMENT_OUTPUT,
          aspect_mask: crate::HalaImageAspectFlags::COLOR,
          image: self.swapchain.images[index],
          ..Default::default()
        },
        crate::HalaImageBarrierInfo {
          old_layout: crate::HalaImageLayout::TRANSFER_SRC_OPTIMAL,
          new_layout: crate::HalaImageLayout::GENERAL,
          src_access_mask: crate::HalaAccessFlags2::TRANSFER_READ,
          dst_access_mask: crate::HalaAccessFlags2::NONE,
          src_stage_mask: crate::HalaPipelineStageFlags2::TRANSFER,
          dst_stage_mask: crate::HalaPipelineStageFlags2::ALL_COMMANDS,
          aspect_mask: crate::HalaImageAspectFlags::COLOR,
          image: ray_tracing_image.unwrap().raw,
          ..Default::default()
        }
      ];
      if self.swapchain.depth_stencil_format != HalaFormat::UNDEFINED {
        barriers.push(
          crate::HalaImageBarrierInfo {
            old_layout: crate::HalaImageLayout::UNDEFINED,
            new_layout: crate::HalaImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
            src_access_mask: crate::HalaAccessFlags2::NONE,
            dst_access_mask: crate::HalaAccessFlags2::DEPTH_STENCIL_ATTACHMENT_WRITE,
            src_stage_mask: crate::HalaPipelineStageFlags2::EARLY_FRAGMENT_TESTS | crate::HalaPipelineStageFlags2::LATE_FRAGMENT_TESTS,
            dst_stage_mask: crate::HalaPipelineStageFlags2::EARLY_FRAGMENT_TESTS | crate::HalaPipelineStageFlags2::LATE_FRAGMENT_TESTS,
            aspect_mask: crate::HalaImageAspectFlags::DEPTH | if self.swapchain.has_stencil { crate::HalaImageAspectFlags::STENCIL } else { crate::HalaImageAspectFlags::empty() },
            image: self.swapchain.depth_stencil_image,
            ..Default::default()
          }
        );
      }
      let barriers_slice = barriers.iter().collect::<Vec<_>>();
      command_buffers.set_image_barriers(
        index,
        barriers_slice.as_slice(),
      );
    } else {
      command_buffers.set_swapchain_image_barrier(
        index,
        &self.swapchain,
        &crate::HalaImageBarrierInfo {
          old_layout: crate::HalaImageLayout::UNDEFINED,
          new_layout: crate::HalaImageLayout::COLOR_ATTACHMENT_OPTIMAL,
          src_access_mask: crate::HalaAccessFlags2::NONE,
          dst_access_mask: crate::HalaAccessFlags2::COLOR_ATTACHMENT_WRITE,
          src_stage_mask: crate::HalaPipelineStageFlags2::TOP_OF_PIPE,
          dst_stage_mask: crate::HalaPipelineStageFlags2::COLOR_ATTACHMENT_OUTPUT,
          aspect_mask: crate::HalaImageAspectFlags::COLOR,
          ..Default::default()
        },
        &crate::HalaImageBarrierInfo {
          old_layout: crate::HalaImageLayout::UNDEFINED,
          new_layout: crate::HalaImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
          src_access_mask: crate::HalaAccessFlags2::NONE,
          dst_access_mask: crate::HalaAccessFlags2::DEPTH_STENCIL_ATTACHMENT_WRITE,
          src_stage_mask: crate::HalaPipelineStageFlags2::EARLY_FRAGMENT_TESTS | crate::HalaPipelineStageFlags2::LATE_FRAGMENT_TESTS,
          dst_stage_mask: crate::HalaPipelineStageFlags2::EARLY_FRAGMENT_TESTS | crate::HalaPipelineStageFlags2::LATE_FRAGMENT_TESTS,
          aspect_mask: crate::HalaImageAspectFlags::DEPTH | if self.swapchain.has_stencil { crate::HalaImageAspectFlags::STENCIL } else { crate::HalaImageAspectFlags::empty() },
          ..Default::default()
        }
      );
    }
    command_buffers.begin_rendering(
      index,
      &self.swapchain,
      (0, 0, self.gpu_req.width, self.gpu_req.height),
      color_clear_values,
      depth_clear_value,
      stencil_clear_value,
    );

    graphics_fn(index, command_buffers)?;

    command_buffers.end_rendering(index);
    command_buffers.set_image_barriers(
      index,
      &[crate::HalaImageBarrierInfo {
        old_layout: crate::HalaImageLayout::COLOR_ATTACHMENT_OPTIMAL,
        new_layout: crate::HalaImageLayout::PRESENT_SRC,
        src_access_mask: crate::HalaAccessFlags2::COLOR_ATTACHMENT_WRITE,
        dst_access_mask: crate::HalaAccessFlags2::NONE,
        src_stage_mask: crate::HalaPipelineStageFlags2::COLOR_ATTACHMENT_OUTPUT,
        dst_stage_mask: crate::HalaPipelineStageFlags2::BOTTOM_OF_PIPE,
        aspect_mask: crate::HalaImageAspectFlags::COLOR,
        image: self.swapchain.images[index],
        ..Default::default()
      }],
    );

    command_buffers.write_timestamp(
      index,
      HalaPipelineStageFlags2::ALL_COMMANDS,
      &self.timestamp_query_pool,
      (index * 2 + 1) as u32);
    command_buffers.end(index)?;

    Ok(())
  }

}
