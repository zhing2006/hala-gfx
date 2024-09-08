use std::rc::Rc;
use std::cell::RefCell;

use ash::vk;

use crate::{
  HalaGfxError,
  HalaCommandBufferSet,
  HalaFormat,
};

/// The swapchain.
pub struct HalaSwapchain {
  pub(crate) logical_device: Rc<RefCell<crate::HalaLogicalDevice>>,
  pub swapchain_loader: ash::khr::swapchain::Device,
  pub swapchain: vk::SwapchainKHR,
  pub images: Vec<vk::Image>,
  pub image_views: Vec<vk::ImageView>,
  pub format: HalaFormat,
  pub color_space: vk::ColorSpaceKHR,
  pub dims: vk::Extent2D,
  pub present_mode: vk::PresentModeKHR,
  pub depth_stencil_format: HalaFormat,
  pub depth_stencil_image: vk::Image,
  pub depth_stencil_image_view: vk::ImageView,
  pub depth_stencil_memory: vk::DeviceMemory,
  pub has_stencil: bool,
  pub num_of_images: usize,
  pub current_image_index: usize,
  pub image_availables: Vec<vk::Semaphore>,
  pub render_finisheds: Vec<vk::Semaphore>,
  pub draw_fences: Vec<vk::Fence>,
}

/// The Drop trait implementation for swapchain.
impl Drop for HalaSwapchain {
  fn drop(&mut self) {
    unsafe {
      let logical_device = self.logical_device.borrow();
      for df in self.draw_fences.iter() {
        logical_device.raw.destroy_fence(*df, None);
      }
      for ia in self.image_availables.iter() {
        logical_device.raw.destroy_semaphore(*ia, None);
      }
      for rf in self.render_finisheds.iter() {
        logical_device.raw.destroy_semaphore(*rf, None);
      }
      if self.depth_stencil_format != HalaFormat::UNDEFINED {
        logical_device.raw.destroy_image_view(self.depth_stencil_image_view, None);
        logical_device.raw.destroy_image(self.depth_stencil_image, None);
        logical_device.raw.free_memory(self.depth_stencil_memory, None);
      }
      for iv in self.image_views.iter() {
        logical_device.raw.destroy_image_view(*iv, None);
      }
      self.swapchain_loader.destroy_swapchain(self.swapchain, None);
    }
    log::debug!("A HalaSwapchain is dropped.");
  }
}

/// The implementation of swapchain.
impl HalaSwapchain {
  /// Create a new swapchain.
  /// param gpu_req: The GPU requirements.
  /// param instance: The instance.
  /// param physical_device: The physical device.
  /// param logical_device: The logical device.
  /// param surface: The surface.
  /// return: The swapchain.
  pub fn new(
    gpu_req: &crate::HalaGPURequirements,
    instance: &crate::HalaInstance,
    physical_device: &crate::HalaPhysicalDevice,
    logical_device: Rc<RefCell<crate::HalaLogicalDevice>>,
    surface: &crate::HalaSurface) -> Result<Self, crate::HalaGfxError>
  {
    let ld= logical_device.borrow();

    let swapchain_loader = ash::khr::swapchain::Device::new(&instance.raw, &ld.raw);
    let (
      swapchain,
      images,
      image_views,
      format,
      color_space,
      dims,
      present_mode,
    ) = Self::create_swapchain(
      gpu_req,
      physical_device,
      &ld,
      surface,
      &swapchain_loader,
    )?;

    let (
      depth_stencil_format,
      depth_stencil_image,
      depth_stencil_image_view,
      depth_stencil_memory,
    ) = Self::create_depth_stencil(
      gpu_req,
      instance,
      physical_device,
      &ld,
      dims,
    )?;
    let num_of_images = images.len();

    let (
      image_availables,
      render_finisheds,
      draw_fences
    ) = Self::create_sync_objects(&ld, num_of_images)?;
    drop(ld);

    log::debug!("A HalaSwapchain is created.");
    Ok(
      Self {
        logical_device,
        swapchain_loader,
        swapchain,
        images,
        image_views,
        format,
        color_space,
        dims,
        present_mode,
        depth_stencil_format,
        depth_stencil_image,
        depth_stencil_image_view,
        depth_stencil_memory,
        has_stencil: depth_stencil_format == HalaFormat::D16_UNORM_S8_UINT || depth_stencil_format == HalaFormat::D24_UNORM_S8_UINT || depth_stencil_format == HalaFormat::D32_SFLOAT_S8_UINT,
        num_of_images,
        current_image_index: 0,
        image_availables,
        render_finisheds,
        draw_fences,
      }
    )
  }

  /// Acquire the next image.
  pub(crate) fn acquire_next_image(&self) -> Result<usize, HalaGfxError> {
    let (image_index, _) = unsafe {
      let logical_device = self.logical_device.borrow();
      logical_device.raw.queue_wait_idle(logical_device.get_graphics_queue(0))
        .map_err(|err| HalaGfxError::new("Failed to wait for queue idle.", Some(Box::new(err))))?;

      self.swapchain_loader.acquire_next_image(
        self.swapchain,
        u64::MAX,
        self.image_availables[self.current_image_index],
        vk::Fence::null(),
      ).map_err(|err| HalaGfxError::new("Failed to acquire next image.", Some(Box::new(err))))?
    };
    Ok(image_index as usize)
  }

  /// Wait for draw fence.
  pub(crate) fn wait_for_fence(&self, command_buffer_index: usize) -> Result<(), HalaGfxError> {
    unsafe {
      self.logical_device.borrow().raw.wait_for_fences(
        &[self.draw_fences[command_buffer_index]],
        true,
        u64::MAX,
      ).map_err(|err| HalaGfxError::new("Failed to wait for fence.", Some(Box::new(err))))?;
    }
    Ok(())
  }

  /// Reset draw fence.
  pub(crate) fn reset_fence(&self, command_buffer_index: usize) -> Result<(), HalaGfxError> {
    unsafe {
      self.logical_device.borrow().raw.reset_fences(
        &[self.draw_fences[command_buffer_index]]
      ).map_err(|err| HalaGfxError::new("Failed to reset fence.", Some(Box::new(err))))?;
    }
    Ok(())
  }

  /// Submit with a command buffer.
  /// param command_buffers: The command buffers.
  /// param command_buffer_index: The command buffer index.
  /// param queue_index: The queue index.
  /// return: The result.
  pub(crate) fn submit(&self, command_buffers: &HalaCommandBufferSet, command_buffer_index: usize, queue_index: u32) -> Result<(), HalaGfxError> {
    // let wait_semaphore_submit_info = vk::SemaphoreSubmitInfo::default()
    //   .semaphore(self.image_availables[self.current_image_index])
    //   .stage_mask(vk::PipelineStageFlags2::COLOR_ATTACHMENT_OUTPUT);
    // let signal_semaphore_submit_info = vk::SemaphoreSubmitInfo::default()
    //   .semaphore(self.render_finisheds[self.current_image_index])
    //   .stage_mask(vk::PipelineStageFlags2::ALL_COMMANDS);
    // let command_buffer_submit_info = vk::CommandBufferSubmitInfoKHR::default()
    //   .command_buffer(command_buffers.raw[command_buffer_index]);
    // let submit_info = vk::SubmitInfo2::default()
    //   .command_buffer_infos(std::slice::from_ref(&command_buffer_submit_info))
    //   .wait_semaphore_infos(std::slice::from_ref(&wait_semaphore_submit_info))
    //   .signal_semaphore_infos(std::slice::from_ref(&signal_semaphore_submit_info));
    let submit_info = vk::SubmitInfo::default()
      .command_buffers(std::slice::from_ref(&command_buffers.raw[command_buffer_index]))
      .wait_semaphores(std::slice::from_ref(&self.image_availables[self.current_image_index]))
      .wait_dst_stage_mask(std::slice::from_ref(&vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT))
      .signal_semaphores(std::slice::from_ref(&self.render_finisheds[self.current_image_index]));
    unsafe {
      self.logical_device.borrow().raw.queue_submit(
        self.logical_device.borrow().get_graphics_queue(queue_index),
        std::slice::from_ref(&submit_info),
        self.draw_fences[command_buffer_index],
      ).map_err(|err| HalaGfxError::new("Failed to submit queue.", Some(Box::new(err))))?;
    }

    Ok(())
  }

  /// Present.
  pub(crate) fn present(&mut self, image_index: u32) -> Result<(), HalaGfxError> {
    let semaphores_finished = [self.render_finisheds[self.current_image_index]];
    let swapchains = [self.swapchain];
    let image_indices = [image_index];
    let present_info = vk::PresentInfoKHR::default()
      .wait_semaphores(&semaphores_finished)
      .swapchains(&swapchains)
      .image_indices(&image_indices);
    unsafe {
      let logical_device = self.logical_device.borrow();
      self.swapchain_loader.queue_present(
        logical_device.get_graphics_queue(0),
        &present_info,
      ).map_err(|err| HalaGfxError::new("Failed to present queue.", Some(Box::new(err))))?;
    }
    self.current_image_index = (self.current_image_index + 1) % self.num_of_images;
    Ok(())
  }

  /// Create a swapchain.
  /// param gpu_req: The GPU requirements.
  /// param physical_device: The physical device.
  /// param logical_device: The logical device.
  /// param surface: The surface.
  /// param swapchain_loader: The Vulkan swapchain loader.
  /// return: The Vulkan swapchain.
  fn create_swapchain(
    gpu_req: &crate::HalaGPURequirements,
    physical_device: &crate::HalaPhysicalDevice,
    logical_device: &crate::HalaLogicalDevice,
    surface: &crate::HalaSurface,
    swapchain_loader: &ash::khr::swapchain::Device,
  ) -> Result<(
    vk::SwapchainKHR,
    Vec<vk::Image>,
    Vec<vk::ImageView>,
    HalaFormat,
    vk::ColorSpaceKHR,
    vk::Extent2D,
    vk::PresentModeKHR,
  ), HalaGfxError> {
    let surface_capabilities = unsafe {
      surface.surface_loader.get_physical_device_surface_capabilities(physical_device.raw, surface.raw)
        .map_err(|err| HalaGfxError::new("Failed to get physical device surface capabilities.", Some(Box::new(err))))?
    };
    let surface_present_modes = unsafe {
      surface.surface_loader.get_physical_device_surface_present_modes(physical_device.raw, surface.raw)
        .map_err(|err| HalaGfxError::new("Failed to get physical device surface present modes.", Some(Box::new(err))))?
    };
    let present_mode = if gpu_req.is_immediate {
      if surface_present_modes.contains(&vk::PresentModeKHR::IMMEDIATE) {
        vk::PresentModeKHR::IMMEDIATE
      } else {
        return Err(HalaGfxError::new("Failed to find a immediate present mode.", None));
      }
    } else if gpu_req.is_low_latency {
      if surface_present_modes.contains(&vk::PresentModeKHR::MAILBOX) {
        vk::PresentModeKHR::MAILBOX
      } else if surface_present_modes.contains(&vk::PresentModeKHR::FIFO_RELAXED) {
        vk::PresentModeKHR::FIFO_RELAXED
      } else {
        vk::PresentModeKHR::FIFO
      }
    } else if surface_present_modes.contains(&vk::PresentModeKHR::FIFO) {
      vk::PresentModeKHR::FIFO
    } else {
      return Err(HalaGfxError::new("Failed to find a FIFO present mode.", None));
    };
    let surface_formats = unsafe {
      surface.surface_loader.get_physical_device_surface_formats(physical_device.raw, surface.raw)
        .map_err(|err| HalaGfxError::new("Failed to get physical device surface formats.", Some(Box::new(err))))?
    };
    let format = if gpu_req.require_10bits_output {
      let mut found = false;
      let mut found_format = vk::Format::UNDEFINED;
      for format in surface_formats.iter() {
        if (format.format == vk::Format::A2B10G10R10_UNORM_PACK32 ||
            format.format == vk::Format::A2B10G10R10_SINT_PACK32 ||
            format.format == vk::Format::A2B10G10R10_SNORM_PACK32 ||
            format.format == vk::Format::A2B10G10R10_UINT_PACK32 ||
            format.format == vk::Format::A2R10G10B10_UNORM_PACK32 ||
            format.format == vk::Format::A2R10G10B10_SINT_PACK32 ||
            format.format == vk::Format::A2R10G10B10_SNORM_PACK32 ||
            format.format == vk::Format::A2R10G10B10_UINT_PACK32)
          && format.color_space == vk::ColorSpaceKHR::SRGB_NONLINEAR
        {
          found = true;
          found_format = format.format;
          break;
        }
      }
      if !found {
        log::warn!("Failed to find a 10bits output format, use the first format instead.");
        surface_formats.first().unwrap().format
      } else {
        found_format
      }
    } else {
      let mut finding_passes = Vec::new();
      if gpu_req.require_srgb_surface {
        finding_passes.push(vec![vk::Format::R8G8B8A8_SRGB]);
        finding_passes.push(vec![vk::Format::B8G8R8A8_SRGB]);
      }
      finding_passes.push(vec![vk::Format::R8G8B8A8_UINT, vk::Format::R8G8B8A8_UNORM, vk::Format::R8G8B8A8_SINT, vk::Format::R8G8B8A8_SNORM]);
      finding_passes.push(vec![vk::Format::B8G8R8A8_UINT, vk::Format::B8G8R8A8_UNORM, vk::Format::B8G8R8A8_SINT, vk::Format::B8G8R8A8_SNORM]);

      let mut found = false;
      let mut found_format = vk::Format::UNDEFINED;
      for pass in finding_passes.iter() {
        for format in surface_formats.iter() {
          if pass.contains(&format.format) && format.color_space == vk::ColorSpaceKHR::SRGB_NONLINEAR {
            found = true;
            found_format = format.format;
            break;
          }
        }

        if found {
          break;
        }
      }
      if !found {
        log::warn!("Failed to find a 8bits output format, use the first format instead.");
        surface_formats.first().unwrap().format
      } else {
        if gpu_req.require_srgb_surface && found_format != vk::Format::R8G8B8A8_SRGB && found_format != vk::Format::B8G8R8A8_SRGB {
          log::warn!("Failed to find a sRGB format, {:?} format instead.", found_format);
        }
        found_format
      }
    };
    log::info!("Surface present mode: {:?}", present_mode);
    log::info!("Surface format: {:?} color space: {:?}", format, vk::ColorSpaceKHR::SRGB_NONLINEAR);

    let queue_family_indices = [logical_device.graphics_queue_family_index];
    let min_image_count = surface_capabilities.min_image_count;
    let max_image_count = if surface_capabilities.max_image_count == 0 {
      u32::MAX
    } else {
      surface_capabilities.max_image_count
    };
    let extent = vk::Extent2D::default()
      .width(gpu_req.width.min(surface_capabilities.current_extent.width))
      .height(gpu_req.height.min(surface_capabilities.current_extent.height));
    let swapchain_create_info = vk::SwapchainCreateInfoKHR::default()
      .surface(surface.raw)
      .min_image_count(
        3.max(min_image_count)
          .min(max_image_count)
      )
      .image_format(format)
      .image_color_space(vk::ColorSpaceKHR::SRGB_NONLINEAR)
      .image_extent(extent)
      .image_array_layers(1)
      .image_usage(vk::ImageUsageFlags::COLOR_ATTACHMENT | vk::ImageUsageFlags::TRANSFER_DST)
      .image_sharing_mode(vk::SharingMode::EXCLUSIVE)
      .queue_family_indices(&queue_family_indices)
      .pre_transform(surface_capabilities.current_transform)
      .composite_alpha(vk::CompositeAlphaFlagsKHR::OPAQUE)
      .present_mode(present_mode);
    let swapchain = unsafe {
      swapchain_loader.create_swapchain(&swapchain_create_info, None)
        .map_err(|err| HalaGfxError::new("Failed to create swapchain.", Some(Box::new(err))))?
    };

    let swapchain_images = unsafe {
      swapchain_loader.get_swapchain_images(swapchain)
        .map_err(|err| HalaGfxError::new("Failed to get swapchain images.", Some(Box::new(err))))?
    };
    let mut swapchain_imageviews = Vec::with_capacity(swapchain_images.len());
    for image in swapchain_images.iter() {
      let subresource_range = vk::ImageSubresourceRange::default()
        .aspect_mask(vk::ImageAspectFlags::COLOR)
        .base_mip_level(0)
        .level_count(1)
        .base_array_layer(0)
        .layer_count(1);
      let imageview_create_info = vk::ImageViewCreateInfo::default()
        .image(*image)
        .view_type(vk::ImageViewType::TYPE_2D)
        .format(format)
        .subresource_range(subresource_range);
      let imageview = unsafe {
        logical_device.raw.create_image_view(&imageview_create_info, None)
          .map_err(|err| HalaGfxError::new("Failed to create image view.", Some(Box::new(err))))?
      };
      swapchain_imageviews.push(imageview);
    }

    for (index, &image) in swapchain_images.iter().enumerate() {
      logical_device.set_debug_name(
        image,
        &format!("swapchain_image[{}]", index)
      ).map_err(|err| HalaGfxError::new("Failed to set debug name for swapchain image.", Some(Box::new(err))))?;
    }
    for (index, &imageview) in swapchain_imageviews.iter().enumerate() {
      logical_device.set_debug_name(
        imageview,
        &format!("swapchain_imageview[{}]", index)
      ).map_err(|err| HalaGfxError::new("Failed to set debug name for swapchain image view.", Some(Box::new(err))))?;
    }

    Ok((
      swapchain,
      swapchain_images,
      swapchain_imageviews,
      format.into(),
      vk::ColorSpaceKHR::SRGB_NONLINEAR,
      extent,
      present_mode,
    ))
  }

  /// Create a depth stencil.
  /// param gpu_req: The GPU requirements.
  /// param instance: The instance.
  /// param physical_device: The physical device.
  /// param logical_device: The logical device.
  /// param dims: The dimensions.
  /// return: The depth stencil.
  fn create_depth_stencil(
    gpu_req: &crate::HalaGPURequirements,
    instance: &crate::HalaInstance,
    physical_device: &crate::HalaPhysicalDevice,
    logical_device: &crate::HalaLogicalDevice,
    dims: vk::Extent2D,
  ) -> Result<(HalaFormat, vk::Image, vk::ImageView, vk::DeviceMemory), HalaGfxError>
  {
    if !gpu_req.require_depth && !gpu_req.require_stencil {
      return Ok((HalaFormat::UNDEFINED, vk::Image::null(), vk::ImageView::null(), vk::DeviceMemory::null()));
    }

    let (depth_stencil_format, depth_stencil_image_aspect) = if gpu_req.require_depth && gpu_req.require_stencil {
      (vk::Format::D24_UNORM_S8_UINT, vk::ImageAspectFlags::DEPTH | vk::ImageAspectFlags::STENCIL)
    } else if gpu_req.require_depth {
      (vk::Format::D32_SFLOAT, vk::ImageAspectFlags::DEPTH)
    } else if gpu_req.require_stencil {
      (vk::Format::S8_UINT, vk::ImageAspectFlags::STENCIL)
    } else {
      (vk::Format::UNDEFINED, vk::ImageAspectFlags::empty())
    };

    let props = unsafe {
      instance.raw.get_physical_device_format_properties(physical_device.raw, depth_stencil_format)
    };
    if props.optimal_tiling_features & vk::FormatFeatureFlags::DEPTH_STENCIL_ATTACHMENT == vk::FormatFeatureFlags::empty() {
      return Err(HalaGfxError::new("Failed to find a depth stencil format.", None));
    }

    let extent3d = vk::Extent3D::default()
      .width(dims.width)
      .height(dims.height)
      .depth(1);
    let queue_family_indices = [logical_device.graphics_queue_family_index];
    let depth_stencil_image_info = vk::ImageCreateInfo::default()
      .image_type(vk::ImageType::TYPE_2D)
      .format(depth_stencil_format)
      .extent(extent3d)
      .mip_levels(1)
      .array_layers(1)
      .samples(vk::SampleCountFlags::TYPE_1)
      .tiling(vk::ImageTiling::OPTIMAL)
      .usage(vk::ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT)
      .sharing_mode(vk::SharingMode::EXCLUSIVE)
      .initial_layout(vk::ImageLayout::UNDEFINED)
      .queue_family_indices(&queue_family_indices);
    let depth_stencil_image = unsafe {
      logical_device.raw.create_image(&depth_stencil_image_info, None)
        .map_err(|err| HalaGfxError::new("Failed to create depth stencil image.", Some(Box::new(err))))?
    };

    let memory_requirements = unsafe {
      logical_device.raw.get_image_memory_requirements(depth_stencil_image)
    };
    let memory_type_index = physical_device.find_memory_type_index(
      &memory_requirements,
      vk::MemoryPropertyFlags::DEVICE_LOCAL,
    ).ok_or(HalaGfxError::new("Failed to find memory type index.", None))?;
    let memory_allocate_info = vk::MemoryAllocateInfo::default()
      .allocation_size(memory_requirements.size)
      .memory_type_index(memory_type_index);
    let depth_stencil_memory = unsafe {
      let memory = logical_device.raw.allocate_memory(&memory_allocate_info, None)
        .map_err(|err| HalaGfxError::new("Failed to allocate memory.", Some(Box::new(err))))?;
      logical_device.raw.bind_image_memory(depth_stencil_image, memory, 0)
        .map_err(|err| HalaGfxError::new("Failed to bind image memory.", Some(Box::new(err))))?;
      memory
    };

    let subresource_range = vk::ImageSubresourceRange::default()
      .aspect_mask(depth_stencil_image_aspect)
      .base_mip_level(0)
      .level_count(1)
      .base_array_layer(0)
      .layer_count(1);
    let depth_stencil_image_view_info = vk::ImageViewCreateInfo::default()
      .image(depth_stencil_image)
      .view_type(vk::ImageViewType::TYPE_2D)
      .format(depth_stencil_format)
      .subresource_range(subresource_range);
    let depth_stencil_image_view = unsafe {
      logical_device.raw.create_image_view(&depth_stencil_image_view_info, None)
        .map_err(|err| HalaGfxError::new("Failed to create depth stencil image view.", Some(Box::new(err))))?
    };

    logical_device.set_debug_name(
      depth_stencil_image,
      "depth_stencil_image"
    ).map_err(|err| HalaGfxError::new("Failed to set debug name for depth stencil image.", Some(Box::new(err))))?;
    logical_device.set_debug_name(
      depth_stencil_image_view,
      "depth_stencil_image_view"
    ).map_err(|err| HalaGfxError::new("Failed to set debug name for depth stencil image view.", Some(Box::new(err))))?;
    logical_device.set_debug_name(
      depth_stencil_memory,
      "depth_stencil_memory"
    ).map_err(|err| HalaGfxError::new("Failed to set debug name for depth stencil memory.", Some(Box::new(err))))?;

    Ok((
      depth_stencil_format.into(),
      depth_stencil_image,
      depth_stencil_image_view,
      depth_stencil_memory,
    ))
  }

  /// Create sync objects.
  /// param logical_device: The logical device.
  /// param num_of_images: The number of images.
  /// return: The sync objects.
  #[allow(clippy::type_complexity)]
  fn create_sync_objects(
    logical_device: &crate::HalaLogicalDevice,
    num_of_images: usize,
  ) -> Result<(Vec<vk::Semaphore>, Vec<vk::Semaphore>, Vec<vk::Fence>), HalaGfxError> {
    let mut image_availables = Vec::with_capacity(num_of_images);
    let mut render_finisheds = Vec::with_capacity(num_of_images);
    let mut draw_fences = Vec::with_capacity(num_of_images);
    let semaphore_create_info = vk::SemaphoreCreateInfo::default();
    let fence_create_info = vk::FenceCreateInfo::default()
    .flags(vk::FenceCreateFlags::SIGNALED);
    for _ in 0..num_of_images {
      let semaphore = unsafe {
        logical_device.raw.create_semaphore(&semaphore_create_info, None)
          .map_err(|err| HalaGfxError::new("Failed to create semaphore.", Some(Box::new(err))))?
      };
      image_availables.push(semaphore);
      let semaphore = unsafe {
        logical_device.raw.create_semaphore(&semaphore_create_info, None)
          .map_err(|err| HalaGfxError::new("Failed to create semaphore.", Some(Box::new(err))))?
      };
      render_finisheds.push(semaphore);
      let fence = unsafe {
        logical_device.raw.create_fence(&fence_create_info, None)
          .map_err(|err| HalaGfxError::new("Failed to create fence.", Some(Box::new(err))))?
      };
      draw_fences.push(fence);
    }
    Ok((image_availables, render_finisheds, draw_fences))
  }
}
