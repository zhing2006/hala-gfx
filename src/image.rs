use ash::vk;

use crate::{
  HalaAccessFlags2,
  HalaBuffer,
  HalaCommandBufferSet,
  HalaFormat,
  HalaGfxError,
  HalaImageLayout,
  HalaLogicalDevice,
  HalaMemoryLocation,
  HalaPipelineStageFlags2,
  HalaSampleCountFlags,
};

/// The image usage flags.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HalaImageUsageFlags(u32);
crate::hala_bitflags_wrapped!(HalaImageUsageFlags, u32);
impl HalaImageUsageFlags {
  pub const TRANSFER_SRC: Self = Self(vk::ImageUsageFlags::TRANSFER_SRC.as_raw());
  pub const TRANSFER_DST: Self = Self(vk::ImageUsageFlags::TRANSFER_DST.as_raw());
  pub const SAMPLED: Self = Self(vk::ImageUsageFlags::SAMPLED.as_raw());
  pub const STORAGE: Self = Self(vk::ImageUsageFlags::STORAGE.as_raw());
  pub const COLOR_ATTACHMENT: Self = Self(vk::ImageUsageFlags::COLOR_ATTACHMENT.as_raw());
  pub const DEPTH_STENCIL_ATTACHMENT: Self = Self(vk::ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT.as_raw());
  pub const TRANSIENT_ATTACHMENT: Self = Self(vk::ImageUsageFlags::TRANSIENT_ATTACHMENT.as_raw());
  pub const INPUT_ATTACHMENT: Self = Self(vk::ImageUsageFlags::INPUT_ATTACHMENT.as_raw());
}

impl std::convert::From<vk::ImageUsageFlags> for HalaImageUsageFlags {
  fn from(v: vk::ImageUsageFlags) -> Self {
    Self(v.as_raw())
  }
}

impl std::convert::From<HalaImageUsageFlags> for vk::ImageUsageFlags {
  fn from(v: HalaImageUsageFlags) -> Self {
    Self::from_raw(v.0)
  }
}

/// The image.
pub struct HalaImage {
  pub(crate) logical_device: std::rc::Rc<std::cell::RefCell<HalaLogicalDevice>>,
  pub raw: vk::Image,
  pub view: vk::ImageView,
  pub extent: vk::Extent3D,
  pub format: HalaFormat,
  pub mip_levels: u32,
  pub mip_views: Vec<vk::ImageView>,
  pub array_layers: u32,
  pub array_views: Vec<vk::ImageView>,
  pub memory_requirements: vk::MemoryRequirements,
  pub allocation: gpu_allocator::vulkan::Allocation,
  pub memory_location: gpu_allocator::MemoryLocation,
  pub size: u64,
  pub(crate) debug_name: String,
}

/// The AsRef trait implementation for the image.
impl AsRef<HalaImage> for HalaImage {
  fn as_ref(&self) -> &HalaImage {
    self
  }
}

/// The Drop trait implementation for the image.
impl Drop for HalaImage {
  fn drop(&mut self) {
    unsafe {
      let mut logical_device = self.logical_device.borrow_mut();
      for mip_view in self.mip_views.iter() {
        logical_device.raw.destroy_image_view(*mip_view, None);
      }
      for array_view in self.array_views.iter() {
        logical_device.raw.destroy_image_view(*array_view, None);
      }
      logical_device.raw.destroy_image_view(self.view, None);
      let allocation = std::mem::take(&mut self.allocation);
      logical_device.gpu_allocator.free(allocation).unwrap();
      logical_device.raw.destroy_image(self.raw, None);
    }
    log::debug!("The HalaImage \"{}\" is dropped.", self.debug_name);
  }
}

/// The implementation of the image.
impl HalaImage {
  /// Create a 2D image with dedicated memory.
  /// param logical_device: The logical device.
  /// param usage: The image usage flags.
  /// param format: The image format.
  /// param width: The image width.
  /// param height: The image height.
  /// param mip_levels: The number of mip levels.
  /// param array_layers: The number of array layers.
  /// param memory_location: The memory location.
  /// param debug_name: The debug name.
  /// return: The image.
  #[allow(clippy::too_many_arguments)]
  pub fn new_2d(
    logical_device: std::rc::Rc<std::cell::RefCell<HalaLogicalDevice>>,
    usage: HalaImageUsageFlags,
    format: HalaFormat,
    width: u32,
    height: u32,
    mip_levels: u32,
    array_layers: u32,
    memory_location: HalaMemoryLocation,
    debug_name: &str,
  ) -> Result<Self, HalaGfxError> {
    Self::new_2d_impl(
      logical_device,
      usage,
      format,
      width,
      height,
      mip_levels,
      array_layers,
      HalaSampleCountFlags::TYPE_1,
      memory_location,
      false,
      debug_name,
    )
  }

  /// Create a 2D image with managed memory.
  /// param logical_device: The logical device.
  /// param usage: The image usage flags.
  /// param format: The image format.
  /// param width: The image width.
  /// param height: The image height.
  /// param mip_levels: The number of mip levels.
  /// param array_layers: The number of array layers.
  /// param memory_location: The memory location.
  /// param debug_name: The debug name.
  /// return: The image.
  #[allow(clippy::too_many_arguments)]
  pub fn new_2d_managed(
    logical_device: std::rc::Rc<std::cell::RefCell<HalaLogicalDevice>>,
    usage: HalaImageUsageFlags,
    format: HalaFormat,
    width: u32,
    height: u32,
    mip_levels: u32,
    array_layers: u32,
    memory_location: HalaMemoryLocation,
    debug_name: &str,
  ) -> Result<Self, HalaGfxError> {
    Self::new_2d_impl(
      logical_device,
      usage,
      format,
      width,
      height,
      mip_levels,
      array_layers,
      HalaSampleCountFlags::TYPE_1,
      memory_location,
      true,
      debug_name,
    )
  }

  /// Create a 2D multisample image with dedicated memory.
  /// param logical_device: The logical device.
  /// param usage: The image usage flags.
  /// param format: The image format.
  /// param width: The image width.
  /// param height: The image height.
  /// param mip_levels: The number of mip levels.
  /// param array_layers: The number of array layers.
  /// param samples: The number of samples.
  /// param memory_location: The memory location.
  /// param debug_name: The debug name.
  /// return: The image.
  #[allow(clippy::too_many_arguments)]
  pub fn with_2d_multisample(
    logical_device: std::rc::Rc<std::cell::RefCell<HalaLogicalDevice>>,
    usage: HalaImageUsageFlags,
    format: HalaFormat,
    width: u32,
    height: u32,
    mip_levels: u32,
    array_layers: u32,
    samples: HalaSampleCountFlags,
    memory_location: HalaMemoryLocation,
    debug_name: &str,
  ) -> Result<Self, HalaGfxError> {
    Self::new_2d_impl(
      logical_device,
      usage,
      format,
      width,
      height,
      mip_levels,
      array_layers,
      samples,
      memory_location,
      false,
      debug_name,
    )
  }

  /// Create a 2D multisample image with managed memory.
  /// param logical_device: The logical device.
  /// param usage: The image usage flags.
  /// param format: The image format.
  /// param width: The image width.
  /// param height: The image height.
  /// param mip_levels: The number of mip levels.
  /// param array_layers: The number of array layers.
  /// param samples: The number of samples.
  /// param memory_location: The memory location.
  /// param debug_name: The debug name.
  /// return: The image.
  #[allow(clippy::too_many_arguments)]
  pub fn with_2d_multisample_managed(
    logical_device: std::rc::Rc<std::cell::RefCell<HalaLogicalDevice>>,
    usage: HalaImageUsageFlags,
    format: HalaFormat,
    width: u32,
    height: u32,
    mip_levels: u32,
    array_layers: u32,
    samples: HalaSampleCountFlags,
    memory_location: HalaMemoryLocation,
    debug_name: &str,
  ) -> Result<Self, HalaGfxError> {
    Self::new_2d_impl(
      logical_device,
      usage,
      format,
      width,
      height,
      mip_levels,
      array_layers,
      samples,
      memory_location,
      true,
      debug_name,
    )
  }

  /// Create a 2D image.
  /// param logical_device: The logical device.
  /// param usage: The image usage flags.
  /// param format: The image format.
  /// param width: The image width.
  /// param height: The image height.
  /// param mip_levels: The number of mip levels.
  /// param array_layers: The number of array layers.
  /// param samples: The number of samples.
  /// param memory_location: The memory location.
  /// param use_managed_memory: Whether to use managed memory.
  /// param debug_name: The debug name.
  /// return: The image.
  #[allow(clippy::too_many_arguments)]
  fn new_2d_impl(
    logical_device: std::rc::Rc<std::cell::RefCell<HalaLogicalDevice>>,
    usage: HalaImageUsageFlags,
    format: HalaFormat,
    width: u32,
    height: u32,
    mip_levels: u32,
    array_layers: u32,
    samples: HalaSampleCountFlags,
    memory_location: HalaMemoryLocation,
    use_managed_memory: bool,
    debug_name: &str,
  ) -> Result<Self, HalaGfxError> {
    let image_info = vk::ImageCreateInfo::default()
      .image_type(vk::ImageType::TYPE_2D)
      .format(format.into())
      .extent(vk::Extent3D {
        width,
        height,
        depth: 1,
      })
      .mip_levels(mip_levels)
      .array_layers(array_layers)
      .samples(samples.into())
      .tiling(vk::ImageTiling::OPTIMAL)
      .usage(usage.into())
      .sharing_mode(vk::SharingMode::EXCLUSIVE)
      .initial_layout(vk::ImageLayout::UNDEFINED);

    let (image, memory_requirements, allocation) = Self::create_and_allocate(
      &logical_device,
      image_info,
      memory_location,
      use_managed_memory,
      debug_name,
    )?;

    let (view, mip_views, array_views) = Self::create_view(
      &logical_device,
      image,
      vk::ImageViewType::TYPE_2D,
      format.into(),
      mip_levels,
      array_layers,
      debug_name,
    )?;

    log::debug!("A HalaImage \"{}\" with resolution [{} x {}], format {} is created.", debug_name, width, height, format);
    Ok(Self {
      logical_device,
      raw: image,
      view,
      extent: vk::Extent3D {
        width,
        height,
        depth: 1,
      },
      format,
      mip_levels,
      mip_views,
      array_layers,
      array_views,
      memory_requirements,
      allocation,
      memory_location: memory_location.into(),
      size: memory_requirements.size,
      debug_name: debug_name.to_string(),
    })
  }

  /// Create a 3D image with dedicated memory.
  /// param logical_device: The logical device.
  /// param usage: The image usage flags.
  /// param format: The image format.
  /// param width: The image width.
  /// param height: The image height.
  /// param depth: The image depth.
  /// param memory_location: The memory location.
  /// param debug_name: The debug name.
  /// return: The image.
  #[allow(clippy::too_many_arguments)]
  pub fn new_3d(
    logical_device: std::rc::Rc<std::cell::RefCell<HalaLogicalDevice>>,
    usage: HalaImageUsageFlags,
    format: HalaFormat,
    width: u32,
    height: u32,
    depth: u32,
    memory_location: HalaMemoryLocation,
    debug_name: &str,
  ) -> Result<Self, HalaGfxError> {
    Self::new_3d_impl(
      logical_device,
      usage,
      format,
      width,
      height,
      depth,
      memory_location,
      false,
      debug_name,
    )
  }

  /// Create a 3D image with managed memory.
  /// param logical_device: The logical device.
  /// param usage: The image usage flags.
  /// param format: The image format.
  /// param width: The image width.
  /// param height: The image height.
  /// param depth: The image depth.
  /// param memory_location: The memory location.
  /// param debug_name: The debug name.
  /// return: The image.
  #[allow(clippy::too_many_arguments)]
  pub fn new_3d_managed(
    logical_device: std::rc::Rc<std::cell::RefCell<HalaLogicalDevice>>,
    usage: HalaImageUsageFlags,
    format: HalaFormat,
    width: u32,
    height: u32,
    depth: u32,
    memory_location: HalaMemoryLocation,
    debug_name: &str,
  ) -> Result<Self, HalaGfxError> {
    Self::new_3d_impl(
      logical_device,
      usage,
      format,
      width,
      height,
      depth,
      memory_location,
      true,
      debug_name,
    )
  }

  /// Create a 3D image.
  /// param logical_device: The logical device.
  /// param usage: The image usage flags.
  /// param format: The image format.
  /// param width: The image width.
  /// param height: The image height.
  /// param depth: The image depth.
  /// param memory_location: The memory location.
  /// param use_managed_memory: Whether to use managed memory.
  /// param debug_name: The debug name.
  /// return: The image.
  #[allow(clippy::too_many_arguments)]
  fn new_3d_impl(
    logical_device: std::rc::Rc<std::cell::RefCell<HalaLogicalDevice>>,
    usage: HalaImageUsageFlags,
    format: HalaFormat,
    width: u32,
    height: u32,
    depth: u32,
    memory_location: HalaMemoryLocation,
    use_managed_memory: bool,
    debug_name: &str,
  ) -> Result<Self, HalaGfxError> {
    let image_info = vk::ImageCreateInfo::default()
      .image_type(vk::ImageType::TYPE_3D)
      .format(format.into())
      .extent(vk::Extent3D {
        width,
        height,
        depth,
      })
      .mip_levels(1)
      .array_layers(1)
      .samples(vk::SampleCountFlags::TYPE_1)
      .tiling(vk::ImageTiling::OPTIMAL)
      .usage(usage.into())
      .sharing_mode(vk::SharingMode::EXCLUSIVE)
      .initial_layout(vk::ImageLayout::UNDEFINED);

    let (image, memory_requirements, allocation) = Self::create_and_allocate(
      &logical_device,
      image_info,
      memory_location,
      use_managed_memory,
      debug_name,
    )?;

    let (view, mip_views, array_views) = Self::create_view(
      &logical_device,
      image,
      vk::ImageViewType::TYPE_3D,
      format.into(),
      1,
      1,
      debug_name,
    )?;

    log::debug!("A HalaImage \"{}\" is created.", debug_name);
    Ok(Self {
      logical_device,
      raw: image,
      view,
      extent: vk::Extent3D {
        width,
        height,
        depth,
      },
      format,
      mip_levels: 1,
      mip_views,
      array_layers: 1,
      array_views,
      memory_requirements,
      allocation,
      memory_location: memory_location.into(),
      size: memory_requirements.size,
      debug_name: debug_name.to_string(),
    })
  }

  /// Create and allocate an image.
  /// param logical_device: The logical device.
  /// param image_info: The image create info.
  /// param memory_location: The memory location.
  /// param use_managed_memory: Whether to use managed memory.
  /// param debug_name: The debug name.
  /// return: The result(image, memory requirements, allocation).
  fn create_and_allocate(
    logical_device: &std::rc::Rc<std::cell::RefCell<HalaLogicalDevice>>,
    image_info: vk::ImageCreateInfo<'_>,
    memory_location: HalaMemoryLocation,
    use_managed_memory: bool,
    debug_name: &str,
  ) -> Result<(vk::Image, vk::MemoryRequirements, gpu_allocator::vulkan::Allocation), HalaGfxError> {
    let (image,memory_requirements) = unsafe {
      let logical_device = logical_device.borrow();
      let image = logical_device.raw.create_image(&image_info, None)
        .map_err(|err| HalaGfxError::new("Failed to create image.", Some(Box::new(err))))?;
      logical_device.set_debug_name(
        image,
        debug_name,
      ).map_err(|err| HalaGfxError::new("Failed to set debug name for image.", Some(Box::new(err))))?;
      (image, logical_device.raw.get_image_memory_requirements(image))
    };

    let allocation = logical_device.borrow_mut().gpu_allocator
      .allocate(
        &gpu_allocator::vulkan::AllocationCreateDesc {
          name: debug_name,
          requirements: memory_requirements,
          location: memory_location.into(),
          linear: true,
          allocation_scheme: if use_managed_memory { gpu_allocator::vulkan::AllocationScheme::GpuAllocatorManaged } else { gpu_allocator::vulkan::AllocationScheme::DedicatedImage(image) },
        }
      ).map_err(|err| HalaGfxError::new("Failed to allocate image.", Some(Box::new(err))))?;
    unsafe {
      let logical_device = logical_device.borrow();
      logical_device.raw.bind_image_memory(image, allocation.memory(), allocation.offset())
        .map_err(|err| HalaGfxError::new("Failed to bind image memory.", Some(Box::new(err))))?;
    }

    Ok((image, memory_requirements, allocation))
  }

  /// Create an image view.
  /// param logical_device: The logical device.
  /// param image: The image.
  /// param view_type: The image view type.
  /// param format: The image format.
  /// param mip_levels: The number of mip levels.
  /// param array_layers: The number of array layers.
  /// param debug_name: The debug name.
  /// return: The image view.
  fn create_view(
    logical_device: &std::rc::Rc<std::cell::RefCell<HalaLogicalDevice>>,
    image: vk::Image,
    view_type: vk::ImageViewType,
    format: vk::Format,
    mip_levels: u32,
    array_layers: u32,
    debug_name: &str,
  ) -> Result<
    (
      vk::ImageView,
      Vec<vk::ImageView>,
      Vec<vk::ImageView>,
    ),
    HalaGfxError
  > {
    let view_info = vk::ImageViewCreateInfo::default()
      .image(image)
      .view_type(view_type)
      .format(format)
      .subresource_range(vk::ImageSubresourceRange {
        aspect_mask: if format == vk::Format::D16_UNORM || format == vk::Format::D32_SFLOAT || format == vk::Format::D24_UNORM_S8_UINT { vk::ImageAspectFlags::DEPTH } else { vk::ImageAspectFlags::COLOR },
        base_mip_level: 0,
        level_count: mip_levels,
        base_array_layer: 0,
        layer_count: array_layers,
      });

    let view = unsafe {
      let logical_device = logical_device.borrow();
      let view = logical_device.raw.create_image_view(&view_info, None)
        .map_err(|err| HalaGfxError::new("Failed to create image view.", Some(Box::new(err))))?;
      logical_device.set_debug_name(
        view,
        &format!("{}_view", debug_name),
      ).map_err(|err| HalaGfxError::new("Failed to set debug name for image view.", Some(Box::new(err))))?;
      view
    };

    let mut mip_views = Vec::new();
    for mip_level in 0..mip_levels {
      let mip_view_info = vk::ImageViewCreateInfo::default()
        .image(image)
        .view_type(view_type)
        .format(format)
        .subresource_range(vk::ImageSubresourceRange {
          aspect_mask: if format == vk::Format::D16_UNORM || format == vk::Format::D32_SFLOAT || format == vk::Format::D24_UNORM_S8_UINT { vk::ImageAspectFlags::DEPTH } else { vk::ImageAspectFlags::COLOR },
          base_mip_level: mip_level,
          level_count: 1,
          base_array_layer: 0,
          layer_count: array_layers,
        });

      let mip_view = unsafe {
        let logical_device = logical_device.borrow();
        let mip_view = logical_device.raw.create_image_view(&mip_view_info, None)
          .map_err(|err| HalaGfxError::new("Failed to create mip view.", Some(Box::new(err))))?;
        logical_device.set_debug_name(
          mip_view,
          &format!("{}_mip_view_{}", debug_name, mip_level),
        ).map_err(|err| HalaGfxError::new("Failed to set debug name for mip view.", Some(Box::new(err))))?;
        mip_view
      };
      mip_views.push(mip_view);
    }

    let mut array_views = Vec::new();
    for array_layer in 0..array_layers {
      let array_view_info = vk::ImageViewCreateInfo::default()
        .image(image)
        .view_type(view_type)
        .format(format)
        .subresource_range(vk::ImageSubresourceRange {
          aspect_mask: if format == vk::Format::D16_UNORM || format == vk::Format::D32_SFLOAT || format == vk::Format::D24_UNORM_S8_UINT { vk::ImageAspectFlags::DEPTH } else { vk::ImageAspectFlags::COLOR },
          base_mip_level: 0,
          level_count: mip_levels,
          base_array_layer: array_layer,
          layer_count: 1,
        });

      let array_view = unsafe {
        let logical_device = logical_device.borrow();
        let array_view = logical_device.raw.create_image_view(&array_view_info, None)
          .map_err(|err| HalaGfxError::new("Failed to create array view.", Some(Box::new(err))))?;
        logical_device.set_debug_name(
          array_view,
          &format!("{}_array_view_{}", debug_name, array_layer),
        ).map_err(|err| HalaGfxError::new("Failed to set debug name for array view.", Some(Box::new(err))))?;
        array_view
      };
      array_views.push(array_view);
    }

    Ok((view, mip_views, array_views))
  }

  /// Generate mipmaps for the image.
  /// param command_buffers: The command buffer set.
  /// return: The result.
  pub fn gen_mipmaps(
    &self,
    command_buffers: &HalaCommandBufferSet,
  ) -> Result<(), HalaGfxError> {
    unsafe {
      let logical_device = self.logical_device.borrow();
      let queue = match command_buffers.command_buffer_type {
        crate::HalaCommandBufferType::GRAPHICS => logical_device.get_graphics_queue(0),
        crate::HalaCommandBufferType::TRANSFER => logical_device.get_transfer_queue(0),
        crate::HalaCommandBufferType::COMPUTE => logical_device.get_compute_queue(0),
        _ => logical_device.get_graphics_queue(0),
      };
      logical_device.execute_and_submit(
        command_buffers,
        0,
        |logical_device, command_buffers, index| {
          for mip_level in 1..self.mip_levels {
            let mip_width = std::cmp::max(1, self.extent.width >> (mip_level - 1));
            let mip_height = std::cmp::max(1, self.extent.height >> (mip_level - 1));

            let input_barriers = [
              vk::ImageMemoryBarrier2::default()
                .src_stage_mask(vk::PipelineStageFlags2::TRANSFER)
                .src_access_mask(vk::AccessFlags2::TRANSFER_WRITE)
                .dst_stage_mask(vk::PipelineStageFlags2::TRANSFER)
                .dst_access_mask(vk::AccessFlags2::TRANSFER_READ)
                .old_layout(vk::ImageLayout::UNDEFINED)
                .new_layout(vk::ImageLayout::TRANSFER_SRC_OPTIMAL)
                .image(self.raw)
                .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
                .dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
                .subresource_range(
                  vk::ImageSubresourceRange::default()
                    .aspect_mask(vk::ImageAspectFlags::COLOR)
                    .base_mip_level(mip_level - 1)
                    .level_count(1)
                    .base_array_layer(0)
                    .layer_count(1)
                ),
              vk::ImageMemoryBarrier2::default()
                .src_stage_mask(vk::PipelineStageFlags2::NONE)
                .src_access_mask(vk::AccessFlags2::NONE)
                .dst_stage_mask(vk::PipelineStageFlags2::TRANSFER)
                .dst_access_mask(vk::AccessFlags2::TRANSFER_WRITE)
                .old_layout(vk::ImageLayout::UNDEFINED)
                .new_layout(vk::ImageLayout::TRANSFER_DST_OPTIMAL)
                .image(self.raw)
                .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
                .dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
                .subresource_range(
                  vk::ImageSubresourceRange::default()
                    .aspect_mask(vk::ImageAspectFlags::COLOR)
                    .base_mip_level(mip_level)
                    .level_count(1)
                    .base_array_layer(0)
                    .layer_count(1)
                ),
            ];

            let input_dependency_info = vk::DependencyInfoKHR::default()
              .image_memory_barriers(&input_barriers);
            logical_device.raw.cmd_pipeline_barrier2(
              command_buffers.raw[index],
              &input_dependency_info,
            );

            logical_device.raw.cmd_pipeline_barrier2(command_buffers.raw[index], &input_dependency_info);

            let blit = vk::ImageBlit2::default()
              .src_offsets([
                vk::Offset3D::default(),
                vk::Offset3D {
                  x: mip_width as i32,
                  y: mip_height as i32,
                  z: 1,
                },
              ])
              .src_subresource(vk::ImageSubresourceLayers::default()
                .aspect_mask(vk::ImageAspectFlags::COLOR)
                .mip_level(mip_level - 1)
                .base_array_layer(0)
                .layer_count(1)
              )
              .dst_offsets([
                vk::Offset3D::default(),
                vk::Offset3D {
                  x: if mip_width > 1 { mip_width / 2 } else { 1 } as i32,
                  y: if mip_height > 1 { mip_height / 2 } else { 1 } as i32,
                  z: 1,
                },
              ])
              .dst_subresource(vk::ImageSubresourceLayers::default()
                .aspect_mask(vk::ImageAspectFlags::COLOR)
                .mip_level(mip_level)
                .base_array_layer(0)
                .layer_count(1)
              );

            let blit_info = vk::BlitImageInfo2::default()
              .src_image(self.raw)
              .src_image_layout(vk::ImageLayout::TRANSFER_SRC_OPTIMAL)
              .dst_image(self.raw)
              .dst_image_layout(vk::ImageLayout::TRANSFER_DST_OPTIMAL)
              .regions(std::slice::from_ref(&blit));

            logical_device.raw.cmd_blit_image2(command_buffers.raw[index], &blit_info);

            let output_barriers = [
              vk::ImageMemoryBarrier2::default()
                .src_stage_mask(vk::PipelineStageFlags2::TRANSFER)
                .src_access_mask(vk::AccessFlags2::TRANSFER_WRITE)
                .dst_stage_mask(vk::PipelineStageFlags2::ALL_GRAPHICS)
                .dst_access_mask(vk::AccessFlags2::SHADER_READ)
                .old_layout(vk::ImageLayout::UNDEFINED)
                .new_layout(vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL)
                .image(self.raw)
                .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
                .dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
                .subresource_range(
                  vk::ImageSubresourceRange::default()
                    .aspect_mask(vk::ImageAspectFlags::COLOR)
                    .base_mip_level(mip_level - 1)
                    .level_count(1)
                    .base_array_layer(0)
                    .layer_count(1)
                ),
            ];

            let output_dependency_info = vk::DependencyInfoKHR::default()
              .image_memory_barriers(&output_barriers);
            logical_device.raw.cmd_pipeline_barrier2(
              command_buffers.raw[index],
              &output_dependency_info,
            );

            logical_device.raw.cmd_pipeline_barrier2(command_buffers.raw[index], &output_dependency_info);
          }

          let output_barrier = vk::ImageMemoryBarrier2::default()
            .src_stage_mask(vk::PipelineStageFlags2::TRANSFER)
            .src_access_mask(vk::AccessFlags2::TRANSFER_WRITE)
            .dst_stage_mask(vk::PipelineStageFlags2::ALL_GRAPHICS)
            .dst_access_mask(vk::AccessFlags2::SHADER_READ)
            .old_layout(vk::ImageLayout::UNDEFINED)
            .new_layout(vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL)
            .image(self.raw)
            .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
            .dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
            .subresource_range(
              vk::ImageSubresourceRange::default()
                .aspect_mask(vk::ImageAspectFlags::COLOR)
                .base_mip_level(self.mip_levels - 1)
                .level_count(1)
                .base_array_layer(0)
                .layer_count(1)
            );

          let output_dependency_info = vk::DependencyInfoKHR::default()
            .image_memory_barriers(std::slice::from_ref(&output_barrier));
          logical_device.raw.cmd_pipeline_barrier2(
            command_buffers.raw[index],
            &output_dependency_info,
          );

          logical_device.raw.cmd_pipeline_barrier2(command_buffers.raw[index], &output_dependency_info);
        },
        queue,
      )?;
    }

    Ok(())
  }

  /// Upload data to the gpu image with a staging buffer.
  /// This is expensive and should not be done in a hot loop.
  /// param data: The data to be uploaded.
  /// param dst_stage_mask: The destination stage mask.
  /// param dst_access_mask: The destination access mask.
  /// param dst_layout: The destination layout.
  /// param staging_buffer: The staging buffer.
  /// param command_buffers: The transfer command buffer set.
  /// return: The result.
  pub fn update_gpu_memory_with_buffer<T: Copy>(
    &self,
    data: &[T],
    dst_stage_mask: HalaPipelineStageFlags2,
    dst_access_mask: HalaAccessFlags2,
    dst_layout: HalaImageLayout,
    staging_buffer: &HalaBuffer,
    command_buffers: &HalaCommandBufferSet,
  ) -> Result<(), HalaGfxError> {
    let src = data.as_ptr() as *const u8;
    let src_size = std::mem::size_of_val(data);
    self.update_gpu_memory_with_buffer_raw(src, src_size, dst_stage_mask, dst_access_mask, dst_layout, staging_buffer, command_buffers)?;

    Ok(())
  }

  /// Upload raw data to the gpu image with a staging buffer.
  /// This is expensive and should not be done in a hot loop.
  /// param data: The data to be uploaded.
  /// param size: The size of the data.
  /// param dst_stage_mask: The destination stage mask.
  /// param dst_access_mask: The destination access mask.
  /// param dst_layout: The destination layout.
  /// param staging_buffer: The staging buffer.
  /// param command_buffers: The transfer command buffer set.
  /// return: The result.
  pub fn update_gpu_memory_with_buffer_raw(
    &self,
    data: *const u8,
    size: usize,
    dst_stage_mask: HalaPipelineStageFlags2,
    dst_access_mask: HalaAccessFlags2,
    dst_layout: HalaImageLayout,
    staging_buffer: &HalaBuffer,
    command_buffers: &HalaCommandBufferSet,
  ) -> Result<(), HalaGfxError> {
    if self.memory_location == gpu_allocator::MemoryLocation::GpuOnly {
      let src = data;
      let src_bytes = size;

      let dst = staging_buffer.allocation.mapped_ptr().unwrap().as_ptr() as *mut u8;
      let dst_bytes = staging_buffer.size as usize;
      unsafe { std::ptr::copy_nonoverlapping(src, dst, std::cmp::min(src_bytes, dst_bytes)) };

      unsafe {
        let logical_device = self.logical_device.borrow();
        let queue = match command_buffers.command_buffer_type {
          crate::HalaCommandBufferType::GRAPHICS => logical_device.get_graphics_queue(0),
          crate::HalaCommandBufferType::TRANSFER => logical_device.get_transfer_queue(0),
          crate::HalaCommandBufferType::COMPUTE => logical_device.get_compute_queue(0),
          _ => logical_device.get_graphics_queue(0),
        };
        logical_device.execute_and_submit(
          command_buffers,
          0,
          |logical_device, command_buffers, index| {
            let input_barrier = vk::ImageMemoryBarrier2::default()
              .src_stage_mask(vk::PipelineStageFlags2::NONE)
              .src_access_mask(vk::AccessFlags2::NONE)
              .dst_stage_mask(vk::PipelineStageFlags2::TRANSFER)
              .dst_access_mask(vk::AccessFlags2::TRANSFER_WRITE)
              .old_layout(vk::ImageLayout::UNDEFINED)
              .new_layout(vk::ImageLayout::TRANSFER_DST_OPTIMAL)
              .image(self.raw)
              .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
              .dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
              .subresource_range(
                vk::ImageSubresourceRange::default()
                  .aspect_mask(vk::ImageAspectFlags::COLOR)
                  .base_mip_level(0)
                  .level_count(1)
                  .base_array_layer(0)
                  .layer_count(1)
              );

            let dependency_info = vk::DependencyInfoKHR::default()
              .image_memory_barriers(std::slice::from_ref(&input_barrier));
            logical_device.raw.cmd_pipeline_barrier2(
              command_buffers.raw[index],
              &dependency_info,
            );

            let region = vk::BufferImageCopy2::default()
              .image_subresource(vk::ImageSubresourceLayers::default()
                .aspect_mask(vk::ImageAspectFlags::COLOR)
                .mip_level(0)
                .base_array_layer(0)
                .layer_count(1)
              )
              .image_extent(self.extent);
            let copy_buffer_to_image_info = vk::CopyBufferToImageInfo2::default()
              .src_buffer(staging_buffer.raw)
              .dst_image(self.raw)
              .dst_image_layout(vk::ImageLayout::TRANSFER_DST_OPTIMAL)
              .regions(std::slice::from_ref(&region));

            logical_device.raw.cmd_copy_buffer_to_image2(
              command_buffers.raw[index],
              &copy_buffer_to_image_info,
            );

            let output_barrier = vk::ImageMemoryBarrier2::default()
              .src_stage_mask(vk::PipelineStageFlags2::TRANSFER)
              .src_access_mask(vk::AccessFlags2::TRANSFER_WRITE)
              .dst_stage_mask(dst_stage_mask.into())
              .dst_access_mask(dst_access_mask.into())
              .old_layout(vk::ImageLayout::TRANSFER_DST_OPTIMAL)
              .new_layout(dst_layout.into())
              .image(self.raw)
              .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
              .dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
              .subresource_range(
                vk::ImageSubresourceRange::default()
                  .aspect_mask(vk::ImageAspectFlags::COLOR)
                  .base_mip_level(0)
                  .level_count(1)
                  .base_array_layer(0)
                  .layer_count(1)
              );

            let dependency_info = vk::DependencyInfoKHR::default()
              .image_memory_barriers(std::slice::from_ref(&output_barrier));
            logical_device.raw.cmd_pipeline_barrier2(
              command_buffers.raw[index],
              &dependency_info,
            );
          },
          queue,
        )?;
      }
    } else {
      return Err(HalaGfxError::new("Cannot update GPU memory of a non GPU only buffer.", None));
    }

    Ok(())
  }}
