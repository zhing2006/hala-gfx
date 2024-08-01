use std::rc::Rc;
use std::cell::RefCell;

use ash::vk;

use crate::{
  HalaBuffer, HalaCommandPools, HalaFormat, HalaFrameBufferSet, HalaGfxError, HalaImage, HalaImageBarrierInfo, HalaImageLayout, HalaLogicalDevice, HalaPipelineStageFlags2, HalaQueryPool, HalaRenderPass, HalaSwapchain
};

/// The command buffer type.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct HalaCommandBufferType(i32);
impl HalaCommandBufferType {
  #[inline]
  pub const fn from_raw(x: i32) -> Self {
      Self(x)
  }
  #[inline]
  pub const fn as_raw(self) -> i32 {
      self.0
  }
}
impl HalaCommandBufferType {
  pub const GRAPHICS: Self = Self(0);
  pub const TRANSFER: Self = Self(1);
  pub const COMPUTE: Self = Self(2);
}

impl std::fmt::Debug for HalaCommandBufferType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self.0 {
      0 => write!(f, "GRAPHICS"),
      1 => write!(f, "TRANSFER"),
      2 => write!(f, "COMPUTE"),
      _ => write!(f, "UNKNOWN"),
    }
  }
}

/// The command buffer level.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct HalaCommandBufferLevel(i32);
impl HalaCommandBufferLevel {
  pub const PRIMARY: Self = Self(vk::CommandBufferLevel::PRIMARY.as_raw());
  pub const SECONDARY: Self = Self(vk::CommandBufferLevel::SECONDARY.as_raw());
}

impl std::convert::From<vk::CommandBufferLevel> for HalaCommandBufferLevel {
  fn from(level: vk::CommandBufferLevel) -> Self {
    Self(level.as_raw())
  }
}

impl std::convert::From<HalaCommandBufferLevel> for vk::CommandBufferLevel {
  fn from(level: HalaCommandBufferLevel) -> Self {
    unsafe { std::mem::transmute(level.0) }
  }
}

/// The command buffer usage flags.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HalaCommandBufferUsageFlags(u32);
crate::hala_bitflags_wrapped!(HalaCommandBufferUsageFlags, u32);
impl HalaCommandBufferUsageFlags {
  pub const ONE_TIME_SUBMIT: Self = Self(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT.as_raw());
  pub const RENDER_PASS_CONTINUE: Self = Self(vk::CommandBufferUsageFlags::RENDER_PASS_CONTINUE.as_raw());
  pub const SIMULTANEOUS_USE: Self = Self(vk::CommandBufferUsageFlags::SIMULTANEOUS_USE.as_raw());
}

impl std::convert::From<vk::CommandBufferUsageFlags> for HalaCommandBufferUsageFlags {
  fn from(flags: vk::CommandBufferUsageFlags) -> Self {
    Self(flags.as_raw())
  }
}

impl std::convert::From<HalaCommandBufferUsageFlags> for vk::CommandBufferUsageFlags {
  fn from(flags: HalaCommandBufferUsageFlags) -> Self {
    unsafe { std::mem::transmute(flags.0) }
  }
}

/// The subpass contents.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct HalaSubpassContents(i32);
impl HalaSubpassContents {
  pub const INLINE: Self = Self(vk::SubpassContents::INLINE.as_raw());
  pub const SECONDARY_COMMAND_BUFFERS: Self = Self(vk::SubpassContents::SECONDARY_COMMAND_BUFFERS.as_raw());
  pub const SECONDARY_COMMAND_BUFFERS_INLINE: Self = Self(1000451000);
}

impl std::convert::From<vk::SubpassContents> for HalaSubpassContents {
  fn from(contents: vk::SubpassContents) -> Self {
    Self(contents.as_raw())
  }
}

impl std::convert::From<HalaSubpassContents> for vk::SubpassContents {
  fn from(contents: HalaSubpassContents) -> Self {
    unsafe { std::mem::transmute(contents.0) }
  }
}

/// The command buffer set.
pub struct HalaCommandBufferSet {
  pub(crate) logical_device: Rc<RefCell<HalaLogicalDevice>>,
  pub(crate) command_pools: Rc<RefCell<HalaCommandPools>>,
  pub raw: Vec<vk::CommandBuffer>,
  pub command_buffer_type: HalaCommandBufferType,

  pub(crate) debug_name: String,
}

/// The Drop trait implementation of the command buffer set.
impl Drop for HalaCommandBufferSet {
  fn drop(&mut self) {
    let logical_device = self.logical_device.borrow();
    let command_pools = self.command_pools.borrow();
    unsafe {
      match self.command_buffer_type {
        HalaCommandBufferType::GRAPHICS => logical_device.raw.free_command_buffers(command_pools.graphics, &self.raw),
        HalaCommandBufferType::TRANSFER => logical_device.raw.free_command_buffers(command_pools.transfer, &self.raw),
        HalaCommandBufferType::COMPUTE => logical_device.raw.free_command_buffers(command_pools.compute, &self.raw),
        _ => (),
      }
    }
    log::debug!("A HalaCommandBufferSet[{:?}] \"{}\" with {} buffer(s) is dropped.", self.command_buffer_type, self.debug_name, self.raw.len());
  }
}

/// The implementation of the command buffer set.
impl HalaCommandBufferSet {

  /// Create a new command buffer set.
  /// param logical_device: The logical device.
  /// param command_pools: The command pools.
  /// param buffer_type: The buffer type.
  /// param buffer_level: The buffer level.
  /// param count: The count of the command buffers.
  /// param debug_name: The debug name.
  /// return: The command buffer set.
  pub fn new(
    logical_device: Rc<RefCell<HalaLogicalDevice>>,
    command_pools: Rc<RefCell<HalaCommandPools>>,
    buffer_type: HalaCommandBufferType,
    buffer_level: HalaCommandBufferLevel,
    count: usize,
    debug_name: &str,
  ) -> Result<Self, HalaGfxError> {
    let command_buffers = {
      let logical_device = logical_device.borrow();
      let command_pools = command_pools.borrow();
      let create_info = vk::CommandBufferAllocateInfo::default()
        .command_pool(match buffer_type {
          HalaCommandBufferType::GRAPHICS => command_pools.graphics,
          HalaCommandBufferType::TRANSFER => command_pools.transfer,
          HalaCommandBufferType::COMPUTE => command_pools.compute,
          _ => command_pools.graphics,
        })
        .level(buffer_level.into())
        .command_buffer_count(count as u32);

      unsafe {
        logical_device.raw.allocate_command_buffers(&create_info)
          .map_err(|err| HalaGfxError::new("Failed to allocate command buffers.", Some(Box::new(err))))?
      }
    };
    for (index, &command_buffer) in command_buffers.iter().enumerate() {
      logical_device.borrow().set_debug_name(
        command_buffer,
        &format!("{}[{}]", debug_name, index)
      ).map_err(|err| HalaGfxError::new("Failed to set debug name for the command buffer.", Some(Box::new(err))))?;
    }

    log::debug!("A HalaCommandBufferSet[{:?}] \"{}\" with {} buffer(s) is created.", buffer_type, debug_name, count);
    let command_buffer_set = Self {
      logical_device,
      command_pools,
      raw: command_buffers,
      command_buffer_type: buffer_type,
      debug_name: debug_name.to_string(),
    };
    Ok(command_buffer_set)
  }

  /// Reset the command buffer.
  /// param index: The index of the command buffer.
  /// param release_resources: Whether to release the resources.
  /// return: The result.
  pub fn reset(&self, index: usize, release_resources: bool) -> Result<(), HalaGfxError> {
    let logical_device = self.logical_device.borrow();
    unsafe {
      logical_device.raw.reset_command_buffer(
        self.raw[index],
        if release_resources { vk::CommandBufferResetFlags::RELEASE_RESOURCES } else { vk::CommandBufferResetFlags::empty() }
      ).map_err(|err| HalaGfxError::new("Failed to reset the command buffer.", Some(Box::new(err))))?;
    }
    Ok(())
  }

  /// Begin the command buffer.
  /// param index: The index of the command buffer.
  /// param usage_flags: The usage flags.
  /// return: The result.
  pub fn begin(&self, index: usize, usage_flags: HalaCommandBufferUsageFlags) -> Result<(), HalaGfxError> {
    let logical_device = self.logical_device.borrow();
    let begin_info = vk::CommandBufferBeginInfo::default()
      .flags(usage_flags.into());
    unsafe {
      logical_device.raw.begin_command_buffer(self.raw[index], &begin_info)
        .map_err(|err| HalaGfxError::new("Failed to begin the command buffer.", Some(Box::new(err))))?;
    }
    Ok(())
  }

  /// End the command buffer.
  /// param index: The index of the command buffer.
  /// return: The result.
  pub fn end(&self, index: usize) -> Result<(), HalaGfxError> {
    let logical_device = self.logical_device.borrow();
    unsafe {
      logical_device.raw.end_command_buffer(self.raw[index])
        .map_err(|err| HalaGfxError::new("Failed to end the command buffer.", Some(Box::new(err))))?;
    }
    Ok(())
  }

  /// Reset the query pool.
  /// param index: The index of the command buffer.
  /// param query_pool: The query pool.
  /// param first_query: The first query.
  /// param query_count: The query count.
  pub fn reset_query_pool(&self, index: usize, query_pool: &HalaQueryPool, first_query: u32, query_count: u32) {
    let logical_device = self.logical_device.borrow();
    unsafe {
      logical_device.raw.cmd_reset_query_pool(self.raw[index], query_pool.raw, first_query, query_count);
    }
  }

  /// Write the timestamp.
  /// param index: The index of the command buffer.
  /// param stage_flags: The pipeline stage flags.
  /// param query_pool: The query pool.
  /// param query: The query.
  pub fn write_timestamp(&self, index: usize, stage_flags: HalaPipelineStageFlags2, query_pool: &HalaQueryPool, query: u32) {
    let logical_device = self.logical_device.borrow();
    unsafe {
      logical_device.raw.cmd_write_timestamp2(
        self.raw[index],
        stage_flags.into(),
        query_pool.raw,
        query);
    }
  }

  /// Begin the render pass.
  /// param index: The index of the command buffer.
  /// param render_pass: The render pass.
  /// param framebuffers: The framebuffers.
  /// param render_area: The render area(x, y, width, height).
  /// param clear_values: The clear values(color, depth, stencil).
  /// param subpass_contents: The subpass contents.
  pub fn begin_render_pass(
    &self,
    index: usize,
    render_pass: &HalaRenderPass,
    framebuffers: &HalaFrameBufferSet,
    render_area: (i32, i32, u32, u32),
    clear_values: ([f32; 4], f32, u32),
    subpass_contents: HalaSubpassContents,
  ) {
    let (clear_color, depth_clear_value, stencil_clear_value) = clear_values;
    let mut clear_values = vec![
      vk::ClearValue {
        color: vk::ClearColorValue {
          float32: [clear_color[0], clear_color[1], clear_color[2], clear_color[3]],
        },
      }
    ];
    if !render_pass.depth_attachment_descs.is_empty() {
      clear_values.push(
        vk::ClearValue {
          depth_stencil: vk::ClearDepthStencilValue {
            depth: depth_clear_value,
            stencil: stencil_clear_value,
          },
        }
      );
    }
    let render_pass_begin_info = vk::RenderPassBeginInfo::default()
      .render_pass(render_pass.raw)
      .framebuffer(framebuffers.raw[index])
      .render_area(vk::Rect2D {
        offset: vk::Offset2D { x: render_area.0, y: render_area.1 },
        extent: vk::Extent2D { width: render_area.2, height: render_area.3 },
      })
      .clear_values(&clear_values);

    unsafe {
      let logical_device = self.logical_device.borrow();
      logical_device.raw.cmd_begin_render_pass(self.raw[index], &render_pass_begin_info, subpass_contents.into());
    }
  }

  /// End the render pass.
  /// param index: The index of the command buffer.
  pub fn end_render_pass(&self, index: usize) {
    let logical_device = self.logical_device.borrow();
    unsafe {
      logical_device.raw.cmd_end_render_pass(self.raw[index]);
    }
  }

  /// Begin rendering.
  /// param index: The index of the command buffer.
  /// param swapchain: The swapchain.
  /// param render_area: The render area(x, y, width, height).
  /// param color_clear_value: The color clear value.
  /// param depth_clear_value: The depth clear value.
  /// param stencil_clear_value: The stencil clear value.
  pub fn begin_rendering(
    &self,
    index: usize,
    swapchain: &HalaSwapchain,
    render_area: (i32, i32, u32, u32),
    color_clear_value: Option<[f32; 4]>,
    depth_clear_value: Option<f32>,
    stencil_clear_value: Option<u32>,
  ) {
    let has_depth = swapchain.depth_stencil_format != HalaFormat::UNDEFINED;
    let has_stencil = swapchain.has_stencil;

    let color_attachment_info = vk::RenderingAttachmentInfo::default()
      .image_view(swapchain.image_views[index])
      .image_layout(vk::ImageLayout::ATTACHMENT_OPTIMAL)
      .load_op(if color_clear_value.is_some() { vk::AttachmentLoadOp::CLEAR } else { vk::AttachmentLoadOp::DONT_CARE })
      .store_op(vk::AttachmentStoreOp::STORE)
      .clear_value(vk::ClearValue {
        color: vk::ClearColorValue {
          float32: color_clear_value.unwrap_or([0f32; 4]),
        },
      });
    let depth_attachment_info = vk::RenderingAttachmentInfo::default()
      .image_view(swapchain.depth_stencil_image_view)
      .image_layout(vk::ImageLayout::ATTACHMENT_OPTIMAL)
      .load_op(if depth_clear_value.is_some() { vk::AttachmentLoadOp::CLEAR } else { vk::AttachmentLoadOp::DONT_CARE })
      .store_op(vk::AttachmentStoreOp::DONT_CARE)
      .clear_value(vk::ClearValue {
        depth_stencil: vk::ClearDepthStencilValue {
          depth: depth_clear_value.unwrap_or(1.0),
          stencil: stencil_clear_value.unwrap_or(0),
        },
      });
    let stencil_attachment_info = vk::RenderingAttachmentInfo::default()
      .image_view(swapchain.depth_stencil_image_view)
      .image_layout(vk::ImageLayout::ATTACHMENT_OPTIMAL)
      .load_op(if stencil_clear_value.is_some() { vk::AttachmentLoadOp::CLEAR } else { vk::AttachmentLoadOp::DONT_CARE })
      .store_op(vk::AttachmentStoreOp::DONT_CARE)
      .clear_value(vk::ClearValue {
        depth_stencil: vk::ClearDepthStencilValue {
          depth: depth_clear_value.unwrap_or(1.0),
          stencil: stencil_clear_value.unwrap_or(0),
        },
      });

    let rendering_info = vk::RenderingInfo::default()
      .render_area(vk::Rect2D {
        offset: vk::Offset2D { x: render_area.0, y: render_area.1 },
        extent: vk::Extent2D { width: render_area.2, height: render_area.3 },
      })
      .layer_count(1)
      .color_attachments(std::slice::from_ref(&color_attachment_info));
    let rendering_info = if has_depth {
      rendering_info.depth_attachment(&depth_attachment_info)
    } else {
      rendering_info
    };
    let rendering_info = if has_stencil {
      rendering_info.stencil_attachment(&stencil_attachment_info)
    } else {
      rendering_info
    };

    unsafe {
      let logical_device = self.logical_device.borrow();
      logical_device.raw.cmd_begin_rendering(self.raw[index], &rendering_info);
    }
  }

  /// Begin rendering with the specified render targets.
  /// param index: The index of the command buffer.
  /// param color_images: The color images.
  /// param depth_image: The depth image.
  /// param render_area: The render area(x, y, width, height).
  /// param color_clear_values: The color clear values.
  /// param depth_clear_value: The depth clear value.
  /// param stencil_clear_value: The stencil clear value.
  /// return: The result.
  #[allow(clippy::too_many_arguments)]
  pub fn begin_rendering_with_rt<T>(
    &self,
    index: usize,
    color_images: &[T],
    depth_image: Option<T>,
    render_area: (i32, i32, u32, u32),
    color_clear_values: &[Option<[f32; 4]>],
    depth_clear_value: Option<f32>,
    stencil_clear_value: Option<u32>,
  )
    where T: AsRef<HalaImage>
  {
    let has_depth = depth_image.is_some();
    let has_stencil = depth_image.as_ref().map_or(false, |image| image.as_ref().format == HalaFormat::D16_UNORM_S8_UINT || image.as_ref().format == HalaFormat::D24_UNORM_S8_UINT || image.as_ref().format == HalaFormat::D32_SFLOAT_S8_UINT);

    let color_attachment_info = color_images.iter().zip(color_clear_values).map(|(image, clear_value)| {
      vk::RenderingAttachmentInfo::default()
        .image_view(image.as_ref().view)
        .image_layout(vk::ImageLayout::ATTACHMENT_OPTIMAL)
        .load_op(if clear_value.is_some() { vk::AttachmentLoadOp::CLEAR } else { vk::AttachmentLoadOp::DONT_CARE })
        .store_op(vk::AttachmentStoreOp::STORE)
        .clear_value(vk::ClearValue {
          color: vk::ClearColorValue {
            float32: clear_value.unwrap_or([0f32; 4]),
          },
        })
    }).collect::<Vec<_>>();
    let depth_image_view = depth_image.as_ref().map_or(vk::ImageView::null(), |image| image.as_ref().view);
    let depth_attachment_info = vk::RenderingAttachmentInfo::default()
      .image_view(depth_image_view)
      .image_layout(vk::ImageLayout::ATTACHMENT_OPTIMAL)
      .load_op(if depth_clear_value.is_some() { vk::AttachmentLoadOp::CLEAR } else { vk::AttachmentLoadOp::DONT_CARE })
      .store_op(vk::AttachmentStoreOp::DONT_CARE)
      .clear_value(vk::ClearValue {
        depth_stencil: vk::ClearDepthStencilValue {
          depth: depth_clear_value.unwrap_or(1.0),
          stencil: stencil_clear_value.unwrap_or(0),
        },
      });
    let stencil_attachment_info = vk::RenderingAttachmentInfo::default()
      .image_view(depth_image_view)
      .image_layout(vk::ImageLayout::ATTACHMENT_OPTIMAL)
      .load_op(if stencil_clear_value.is_some() { vk::AttachmentLoadOp::CLEAR } else { vk::AttachmentLoadOp::DONT_CARE })
      .store_op(vk::AttachmentStoreOp::DONT_CARE)
      .clear_value(vk::ClearValue {
        depth_stencil: vk::ClearDepthStencilValue {
          depth: depth_clear_value.unwrap_or(1.0),
          stencil: stencil_clear_value.unwrap_or(0),
        },
      });

    let rendering_info = vk::RenderingInfo::default()
      .render_area(vk::Rect2D {
        offset: vk::Offset2D { x: render_area.0, y: render_area.1 },
        extent: vk::Extent2D { width: render_area.2, height: render_area.3 },
      })
      .layer_count(1)
      .color_attachments(color_attachment_info.as_slice());
    let rendering_info = if has_depth {
      rendering_info.depth_attachment(&depth_attachment_info)
    } else {
      rendering_info
    };
    let rendering_info = if has_stencil {
      rendering_info.stencil_attachment(&stencil_attachment_info)
    } else {
      rendering_info
    };

    unsafe {
      let logical_device = self.logical_device.borrow();
      logical_device.raw.cmd_begin_rendering(self.raw[index], &rendering_info);
    }
  }

  /// End rendering.
  /// param index: The index of the command buffer.
  pub fn end_rendering(&self, index: usize) {
    let logical_device = self.logical_device.borrow();
    unsafe {
      logical_device.raw.cmd_end_rendering(self.raw[index]);
    }
  }

  /// Set the viewports.
  /// param index: The index of the command buffer.
  /// param first_viewport: The first viewport.
  /// param viewports: The viewports(x, y, width, height, min_depth, max_depth).
  pub fn set_viewport(
    &self,
    index: usize,
    first_viewport: u32,
    viewports: &[(f32, f32, f32, f32, f32, f32)],
  ) {
    let logical_device = self.logical_device.borrow();
    let viewports = viewports.iter().map(|(x, y, width, height, min_depth, max_depth)| {
      vk::Viewport {
        x: *x,
        y: *y,
        width: *width,
        height: *height,
        min_depth: *min_depth,
        max_depth: *max_depth,
      }
    }).collect::<Vec<_>>();
    unsafe {
      logical_device.raw.cmd_set_viewport(self.raw[index], first_viewport, viewports.as_slice());
    }
  }

  /// Set the scissors.
  /// param index: The index of the command buffer.
  /// param first_scissor: The first scissor.
  /// param scissors: The scissors(x, y, width, height).
  pub fn set_scissor(
    &self,
    index: usize,
    first_scissor: u32,
    scissors: &[(i32, i32, u32, u32)],
  ) {
    let logical_device = self.logical_device.borrow();
    let scissors = scissors.iter().map(|(x, y, width, height)| {
      vk::Rect2D {
        offset: vk::Offset2D { x: *x, y: *y },
        extent: vk::Extent2D { width: *width, height: *height },
      }
    }).collect::<Vec<_>>();
    unsafe {
      logical_device.raw.cmd_set_scissor(self.raw[index], first_scissor, scissors.as_slice());
    }
  }

  /// Enable the depth test.
  /// param index: The index of the command buffer.
  /// param enable: Whether to enable the depth test.
  pub fn enable_depth_test(&self, index: usize, enable: bool) {
    let logical_device = self.logical_device.borrow();
    unsafe {
      logical_device.raw.cmd_set_depth_test_enable(self.raw[index], enable)
    }
  }

  /// Enable the depth write.
  /// param index: The index of the command buffer.
  /// param enable: Whether to enable the depth write.
  pub fn enable_depth_write(&self, index: usize, enable: bool) {
    let logical_device = self.logical_device.borrow();
    unsafe {
      logical_device.raw.cmd_set_depth_write_enable(self.raw[index], enable)
    }
  }

  /// Set the depth compare op.
  /// param index: The index of the command buffer.
  /// param op: The depth compare op.
  pub fn set_depth_compare_op(&self, index: usize, op: crate::HalaCompareOp) {
    let logical_device = self.logical_device.borrow();
    unsafe {
      logical_device.raw.cmd_set_depth_compare_op(self.raw[index], op.into())
    }
  }

  /// Enable the depth bounds test.
  /// param index: The index of the command buffer.
  /// param enable: Whether to enable the depth bounds test.
  pub fn enable_stencil_test(&self, index: usize, enable: bool) {
    let logical_device = self.logical_device.borrow();
    unsafe {
      logical_device.raw.cmd_set_stencil_test_enable(self.raw[index], enable)
    }
  }

  /// Set the stencil opertions.
  /// param index: The index of the command buffer.
  /// param face_mask: The face flags.
  /// param fail_op: The fail operation.
  /// param pass_op: The pass operation.
  /// param depth_fail_op: The depth fail operation.
  /// param compare_op: The compare operation.
  pub fn set_stencil_op(
    &self,
    index: usize,
    face_mask: crate::HalaStencilFaceFlags,
    fail_op: crate::HalaStencilOp,
    pass_op: crate::HalaStencilOp,
    depth_fail_op: crate::HalaStencilOp,
    compare_op: crate::HalaCompareOp,
  ) {
    let logical_device = self.logical_device.borrow();
    unsafe {
      logical_device.raw.cmd_set_stencil_op(
        self.raw[index],
        face_mask.into(),
        fail_op.into(),
        pass_op.into(),
        depth_fail_op.into(),
        compare_op.into(),
      );
    }
  }

  /// Set the stencil compare mask.
  /// param index: The index of the command buffer.
  /// param face_mask: The face flags.
  /// param compare_mask: The compare mask.
  pub fn set_stencil_compare_mask(&self, index: usize, face_mask: crate::HalaStencilFaceFlags, compare_mask: u32) {
    let logical_device = self.logical_device.borrow();
    unsafe {
      logical_device.raw.cmd_set_stencil_compare_mask(self.raw[index], face_mask.into(), compare_mask);
    }
  }

  /// Set the stencil write mask.
  /// param index: The index of the command buffer.
  /// param face_mask: The face flags.
  /// param write_mask: The write mask.
  pub fn set_stencil_write_mask(&self, index: usize, face_mask: crate::HalaStencilFaceFlags, write_mask: u32) {
    let logical_device = self.logical_device.borrow();
    unsafe {
      logical_device.raw.cmd_set_stencil_write_mask(self.raw[index], face_mask.into(), write_mask);
    }
  }

  /// Set the stencil reference.
  /// param index: The index of the command buffer.
  /// param face_mask: The face flags.
  /// param reference: The reference.
  pub fn set_stencil_reference(&self, index: usize, face_mask: crate::HalaStencilFaceFlags, reference: u32) {
    let logical_device = self.logical_device.borrow();
    unsafe {
      logical_device.raw.cmd_set_stencil_reference(self.raw[index], face_mask.into(), reference);
    }
  }

  /// Push constants.
  /// param index: The index of the command buffer.
  /// param pipeline_layout: The pipeline layout.
  /// param shader_stage: The shader stage.
  /// param offset: The offset in bytes.
  /// param data: The data.
  pub fn push_constants(
    &self,
    index: usize,
    pipeline_layout: vk::PipelineLayout,
    shader_stage: crate::HalaShaderStageFlags,
    offset: u32,
    data: &[u8],
  ) {
    let logical_device = self.logical_device.borrow();
    unsafe {
      logical_device.raw.cmd_push_constants(
        self.raw[index],
        pipeline_layout,
        shader_stage.into(),
        offset,
        data,
      )
    }
  }

  /// Push constants as f32.
  /// param index: The index of the command buffer.
  /// param pipeline_layout: The pipeline layout.
  /// param shader_stage: The shader stage.
  /// param offset: The offset in bytes.
  /// param data: The data.
  pub fn push_constants_f32(
    &self,
    index: usize,
    pipeline_layout: vk::PipelineLayout,
    shader_stage: crate::HalaShaderStageFlags,
    offset: u32,
    data: &[f32],
  ) {
    let logical_device = self.logical_device.borrow();
    unsafe {
      let data = std::slice::from_raw_parts(data.as_ptr() as *const u8, std::mem::size_of_val(data));
      logical_device.raw.cmd_push_constants(
        self.raw[index],
        pipeline_layout,
        shader_stage.into(),
        offset,
        data,
      )
    }
  }

  /// Draw.
  /// param index: The index of the command buffer.
  /// param vertex_count: The vertex count.
  /// param instance_count: The instance count.
  /// param first_vertex: The first vertex.
  /// param first_instance: The first instance.
  pub fn draw(
    &self,
    index: usize,
    vertex_count: u32,
    instance_count: u32,
    first_vertex: u32,
    first_instance: u32,
  ) {
    let logical_device = self.logical_device.borrow();
    unsafe {
      logical_device.raw.cmd_draw(self.raw[index], vertex_count, instance_count, first_vertex, first_instance);
    }
  }

  /// Draw indexed.
  /// param index: The index of the command buffer.
  /// param index_count: The index count.
  /// param instance_count: The instance count.
  /// param first_index: The first index.
  /// param vertex_offset: The vertex offset.
  /// param first_instance: The first instance.
  pub fn draw_indexed(
    &self,
    index: usize,
    index_count: u32,
    instance_count: u32,
    first_index: u32,
    vertex_offset: i32,
    first_instance: u32,
  ) {
    let logical_device = self.logical_device.borrow();
    unsafe {
      logical_device.raw.cmd_draw_indexed(
        self.raw[index],
        index_count,
        instance_count,
        first_index,
        vertex_offset,
        first_instance);
    }
  }

  /// Draw indirect.
  /// param index: The index of the command buffer.
  /// param buffer: The buffer.
  /// param offset: The offset.
  /// param draw_count: The draw count.
  /// param stride: The stride.
  pub fn draw_indirect(
    &self,
    index: usize,
    buffer: &HalaBuffer,
    offset: u64,
    draw_count: u32,
    stride: u32,
  ) {
    let logical_device = self.logical_device.borrow();
    unsafe {
      logical_device.raw.cmd_draw_indirect(
        self.raw[index],
        buffer.raw,
        offset,
        draw_count,
        stride);
    }
  }

  /// Draw indexed indirect.
  /// param index: The index of the command buffer.
  /// param buffer: The buffer.
  /// param offset: The offset.
  /// param draw_count: The draw count.
  /// param stride: The stride.
  pub fn draw_indexed_indirect(
    &self,
    index: usize,
    buffer: &HalaBuffer,
    offset: u64,
    draw_count: u32,
    stride: u32,
  ) {
    let logical_device = self.logical_device.borrow();
    unsafe {
      logical_device.raw.cmd_draw_indexed_indirect(
        self.raw[index],
        buffer.raw,
        offset,
        draw_count,
        stride);
    }
  }

  /// Draw indirect count.
  /// param index: The index of the command buffer.
  /// param buffer: The buffer.
  /// param offset: The offset.
  /// param count_buffer: The count buffer.
  /// param count_buffer_offset: The count buffer offset.
  /// param max_draw_count: The max draw count.
  /// param stride: The stride.
  #[allow(clippy::too_many_arguments)]
  pub fn draw_indirect_count(
    &self,
    index: usize,
    buffer: &HalaBuffer,
    offset: u64,
    count_buffer: &HalaBuffer,
    count_buffer_offset: u64,
    max_draw_count: u32,
    stride: u32,
  ) {
    let logical_device = self.logical_device.borrow();
    unsafe {
      logical_device.raw.cmd_draw_indirect_count(
        self.raw[index],
        buffer.raw,
        offset,
        count_buffer.raw,
        count_buffer_offset,
        max_draw_count,
        stride);
    }
  }

  /// Draw indexed indirect count.
  /// param index: The index of the command buffer.
  /// param buffer: The buffer.
  /// param offset: The offset.
  /// param count_buffer: The count buffer.
  /// param count_buffer_offset: The count buffer offset.
  /// param max_draw_count: The max draw count.
  /// param stride: The stride.
  #[allow(clippy::too_many_arguments)]
  pub fn draw_indexed_indirect_count(
    &self,
    index: usize,
    buffer: &HalaBuffer,
    offset: u64,
    count_buffer: &HalaBuffer,
    count_buffer_offset: u64,
    max_draw_count: u32,
    stride: u32,
  ) {
    let logical_device = self.logical_device.borrow();
    unsafe {
      logical_device.raw.cmd_draw_indexed_indirect_count(
        self.raw[index],
        buffer.raw,
        offset,
        count_buffer.raw,
        count_buffer_offset,
        max_draw_count,
        stride);
    }
  }

  /// Draw mesh tasks.
  /// param index: The index of the command buffer.
  /// param group_count_x: The group count x.
  /// param group_count_y: The group count y.
  /// param group_count_z: The group count z.
  pub fn draw_mesh_tasks(
    &self,
    index: usize,
    group_count_x: u32,
    group_count_y: u32,
    group_count_z: u32,
  ) {
    let logical_device = self.logical_device.borrow();
    unsafe {
      logical_device.mesh_shader_loader.cmd_draw_mesh_tasks(self.raw[index], group_count_x, group_count_y, group_count_z);
    }
  }

  /// Draw mesh tasks indirect.
  /// param index: The index of the command buffer.
  /// param buffer: The buffer.
  /// param offset: The offset.
  /// param draw_count: The draw count.
  /// param stride: The stride.
  pub fn draw_mesh_tasks_indirect(
    &self,
    index: usize,
    buffer: &HalaBuffer,
    offset: u64,
    draw_count: u32,
    stride: u32,
  ) {
    let logical_device = self.logical_device.borrow();
    unsafe {
      logical_device.mesh_shader_loader.cmd_draw_mesh_tasks_indirect(self.raw[index], buffer.raw, offset, draw_count, stride);
    }
  }

  /// Draw mesh tasks indirect count.
  /// param index: The index of the command buffer.
  /// param buffer: The buffer.
  /// param offset: The offset.
  /// param count_buffer: The count buffer.
  /// param count_buffer_offset: The count buffer offset.
  /// param max_draw_count: The max draw count.
  /// param stride: The stride.
  #[allow(clippy::too_many_arguments)]
  pub fn draw_mesh_tasks_indirect_count(
    &self,
    index: usize,
    buffer: &HalaBuffer,
    offset: u64,
    count_buffer: &HalaBuffer,
    count_buffer_offset: u64,
    max_draw_count: u32,
    stride: u32,
  ) {
    let logical_device = self.logical_device.borrow();
    unsafe {
      logical_device.mesh_shader_loader.cmd_draw_mesh_tasks_indirect_count(self.raw[index], buffer.raw, offset, count_buffer.raw, count_buffer_offset, max_draw_count, stride);
    }
  }

  /// Dispatch compute.
  /// param index: The index of the command buffer.
  /// param group_count_x: The group count x.
  /// param group_count_y: The group count y.
  /// param group_count_z: The group count z.
  pub fn dispatch(&self, index: usize, group_count_x: u32, group_count_y: u32, group_count_z: u32) {
    let logical_device = self.logical_device.borrow();
    unsafe {
      logical_device.raw.cmd_dispatch(self.raw[index], group_count_x, group_count_y, group_count_z);
    }
  }

  /// Dispatch indirect.
  /// param index: The index of the command buffer.
  /// param buffer: The buffer.
  /// param offset: The offset.
  pub fn dispatch_indirect(&self, index: usize, buffer: &HalaBuffer, offset: u64) {
    let logical_device = self.logical_device.borrow();
    unsafe {
      logical_device.raw.cmd_dispatch_indirect(self.raw[index], buffer.raw, offset);
    }
  }

  /// Bind the graphics pipeline.
  /// param index: The index of the command buffer.
  /// param pipeline: The graphics pipeline.
  pub fn bind_graphics_pipeline(&self, index: usize, pipeline: &crate::HalaGraphicsPipeline) {
    let logical_device = self.logical_device.borrow();
    unsafe {
      logical_device.raw.cmd_bind_pipeline(
        self.raw[index],
        vk::PipelineBindPoint::GRAPHICS,
        pipeline.raw);
    }
  }

  /// Bind the ray tracing pipeline.
  /// param index: The index of the command buffer.
  /// param pipeline: The ray tracing pipeline.
  pub fn bind_ray_tracing_pipeline(&self, index: usize, pipeline: &crate::HalaRayTracingPipeline) {
    let logical_device = self.logical_device.borrow();
    unsafe {
      logical_device.raw.cmd_bind_pipeline(
        self.raw[index],
        vk::PipelineBindPoint::RAY_TRACING_KHR,
        pipeline.raw);
    }
  }

  /// Bind the compute pipeline.
  /// param index: The index of the command buffer.
  /// param pipeline: The compute pipeline.
  pub fn bind_compute_pipeline(&self, index: usize, pipeline: &crate::HalaComputePipeline) {
    let logical_device = self.logical_device.borrow();
    unsafe {
      logical_device.raw.cmd_bind_pipeline(
        self.raw[index],
        vk::PipelineBindPoint::COMPUTE,
        pipeline.raw);
    }
  }

  /// Bind the graphics descriptor sets.
  /// param index: The index of the command buffer.
  /// param pipeline: The graphics pipeline.
  /// param first_set: The first set.
  /// param descriptor_sets: The descriptor sets.
  /// param dynamic_offsets: The dynamic offsets.
  pub fn bind_graphics_descriptor_sets<DS>(
    &self,
    index: usize,
    pipeline: &crate::HalaGraphicsPipeline,
    first_set: u32,
    descriptor_sets: &[DS],
    dynamic_offsets: &[u32],
  )
    where DS: AsRef<crate::HalaDescriptorSet>
  {
    let logical_device = self.logical_device.borrow();
    let descriptor_sets: Vec<vk::DescriptorSet> = descriptor_sets.iter().map(|set| {
      let set = set.as_ref();
      if set.is_static {
        set.raw[0]
      } else {
        set.raw[index]
      }
    }).collect();
    unsafe {
      logical_device.raw.cmd_bind_descriptor_sets(
        self.raw[index],
        vk::PipelineBindPoint::GRAPHICS,
        pipeline.layout,
        first_set,
        &descriptor_sets,
        dynamic_offsets,
      );
    }
  }

  /// Bind the ray tracing descriptor sets.
  /// param index: The index of the command buffer.
  /// param pipeline: The ray tracing pipeline.
  /// param first_set: The first set.
  /// param descriptor_sets: The descriptor sets.
  /// param dynamic_offsets: The dynamic offsets.
  pub fn bind_ray_tracing_descriptor_sets<DS>(
    &self,
    index: usize,
    pipeline: &crate::HalaRayTracingPipeline,
    first_set: u32,
    descriptor_sets: &[DS],
    dynamic_offsets: &[u32],
  )
    where DS: AsRef<crate::HalaDescriptorSet>
  {
    let logical_device = self.logical_device.borrow();
    let descriptor_sets: Vec<vk::DescriptorSet> = descriptor_sets.iter().map(|set| {
      let set = set.as_ref();
      if set.is_static {
        set.raw[0]
      } else {
        set.raw[index]
      }
    }).collect();
    unsafe {
      logical_device.raw.cmd_bind_descriptor_sets(
        self.raw[index],
        vk::PipelineBindPoint::RAY_TRACING_KHR,
        pipeline.layout,
        first_set,
        &descriptor_sets,
        dynamic_offsets,
      );
    }
  }

  /// Bind the compute descriptor sets.
  /// param index: The index of the command buffer.
  /// param pipeline: The compute pipeline.
  /// param first_set: The first set.
  /// param descriptor_sets: The descriptor sets.
  /// param dynamic_offsets: The dynamic offsets.
  pub fn bind_compute_descriptor_sets<DS>(
    &self,
    index: usize,
    pipeline: &crate::HalaComputePipeline,
    first_set: u32,
    descriptor_sets: &[DS],
    dynamic_offsets: &[u32],
  )
    where DS: AsRef<crate::HalaDescriptorSet>
  {
    let logical_device = self.logical_device.borrow();
    let descriptor_sets: Vec<vk::DescriptorSet> = descriptor_sets.iter().map(|set| {
      let set = set.as_ref();
      if set.is_static {
        set.raw[0]
      } else {
        set.raw[index]
      }
    }).collect();
    unsafe {
      logical_device.raw.cmd_bind_descriptor_sets(
        self.raw[index],
        vk::PipelineBindPoint::COMPUTE,
        pipeline.layout,
        first_set,
        &descriptor_sets,
        dynamic_offsets,
      );
    }
  }

  /// Bind the vertex buffers.
  /// param index: The index of the command buffer.
  /// param first_binding: The first binding.
  /// param buffers: The buffers.
  /// param offsets: The offsets.
  pub fn bind_vertex_buffers<B>(
    &self,
    index: usize,
    first_binding: u32,
    buffers: &[B],
    offsets: &[u64],
  )
    where B: AsRef<crate::HalaBuffer>
  {
    let logical_device = self.logical_device.borrow();
    let buffers: Vec<vk::Buffer> = buffers.iter().map(|buffer| buffer.as_ref().raw).collect();
    unsafe {
      logical_device.raw.cmd_bind_vertex_buffers(
        self.raw[index],
        first_binding,
        &buffers,
        offsets,
      );
    }
  }

  /// Bind the index buffers.
  /// param index: The index of the command buffer.
  /// param buffers: The buffers.
  /// param offsets: The offsets.
  /// param index_type: The index type.
  pub fn bind_index_buffers<B>(
    &self,
    index: usize,
    buffers: &[B],
    offsets: &[u64],
    index_type: crate::HalaIndexType,
  )
    where B: AsRef<crate::HalaBuffer>
  {
    let logical_device = self.logical_device.borrow();
    let buffers: Vec<vk::Buffer> = buffers.iter().map(|buffer| buffer.as_ref().raw).collect();
    unsafe {
      logical_device.raw.cmd_bind_index_buffer(
        self.raw[index],
        buffers[0],
        offsets[0],
        index_type.into(),
      );
    }
  }

  /// Trace rays.
  /// param index: The index of the command buffer.
  /// param sbt: The shader binding table.
  /// param width: The width.
  /// param height: The height.
  /// param depth: The depth.
  pub fn trace_rays(
    &self,
    index: usize,
    sbt: &crate::HalaShaderBindingTable,
    width: u32,
    height: u32,
    depth: u32,
  ) {
    let logical_device = self.logical_device.borrow();
    unsafe {
      logical_device.ray_tracing_pipeline_loader.cmd_trace_rays(
        self.raw[index],
        &sbt.raygen_region,
        &sbt.miss_region,
        &sbt.hit_region,
        &sbt.callable_region,
        width,
        height,
        depth,
      );
    }
  }

  /// Trace rays indirect.
  /// param index: The index of the command buffer.
  /// param sbts: The shader binding tables.
  /// param indirect_device_address: The indirect device address.
  pub fn trace_rays_indirect(
    &self,
    index: usize,
    sbt: &crate::HalaShaderBindingTable,
    indirect_device_address: u64,
  ) {
    let logical_device = self.logical_device.borrow();
    unsafe {
      logical_device.ray_tracing_pipeline_loader.cmd_trace_rays_indirect(
        self.raw[index],
        &sbt.raygen_region,
        &sbt.miss_region,
        &sbt.hit_region,
        &sbt.callable_region,
        indirect_device_address,
      );
    }
  }

  /// Set the ray tracing pipeline stack size.
  /// param index: The index of the command buffer.
  /// param pipeline_stack_size: The pipeline stack size.
  pub fn set_ray_tracing_pipeline_stack_size(
    &self,
    index: usize,
    pipeline_stack_size: u32,
  ) {
    let logical_device = self.logical_device.borrow();
    unsafe {
      logical_device.ray_tracing_pipeline_loader.cmd_set_ray_tracing_pipeline_stack_size(
        self.raw[index],
        pipeline_stack_size,
      );
    }
  }

  /// Set swapchain image barriers.
  /// param index: The index of the command buffer.
  /// param swapchain: The swapchain.
  /// param color_barrier_info: The color barrier info.
  /// param depth_stencil_barrier_info: The depth stencil barrier info.
  pub fn set_swapchain_image_barrier(
    &self,
    index: usize,
    swapchain: &HalaSwapchain,
    color_barrier_info: &HalaImageBarrierInfo,
    depth_stencil_barrier_info: &HalaImageBarrierInfo,
  ) {
    let color_barrier = vk::ImageMemoryBarrier2::default()
      .src_stage_mask(color_barrier_info.src_stage_mask.into())
      .src_access_mask(color_barrier_info.src_access_mask.into())
      .dst_stage_mask(color_barrier_info.dst_stage_mask.into())
      .dst_access_mask(color_barrier_info.dst_access_mask.into())
      .old_layout(color_barrier_info.old_layout.into())
      .new_layout(color_barrier_info.new_layout.into())
      .src_queue_family_index(color_barrier_info.src_queue_family_index)
      .dst_queue_family_index(color_barrier_info.dst_queue_family_index)
      .image(swapchain.images[index])
      .subresource_range(
        vk::ImageSubresourceRange::default()
          .aspect_mask(color_barrier_info.aspect_mask.into())
          .base_mip_level(color_barrier_info.base_mip_level)
          .level_count(color_barrier_info.level_count)
          .base_array_layer(color_barrier_info.base_array_layer)
          .layer_count(color_barrier_info.layer_count)
      );
    let depth_stencil_barrier = vk::ImageMemoryBarrier2::default()
      .src_stage_mask(depth_stencil_barrier_info.src_stage_mask.into())
      .src_access_mask(depth_stencil_barrier_info.src_access_mask.into())
      .dst_stage_mask(depth_stencil_barrier_info.dst_stage_mask.into())
      .dst_access_mask(depth_stencil_barrier_info.dst_access_mask.into())
      .old_layout(depth_stencil_barrier_info.old_layout.into())
      .new_layout(depth_stencil_barrier_info.new_layout.into())
      .src_queue_family_index(depth_stencil_barrier_info.src_queue_family_index)
      .dst_queue_family_index(depth_stencil_barrier_info.dst_queue_family_index)
      .image(swapchain.depth_stencil_image)
      .subresource_range(
        vk::ImageSubresourceRange::default()
          .aspect_mask(depth_stencil_barrier_info.aspect_mask.into())
          .base_mip_level(depth_stencil_barrier_info.base_mip_level)
          .level_count(depth_stencil_barrier_info.level_count)
          .base_array_layer(depth_stencil_barrier_info.base_array_layer)
          .layer_count(depth_stencil_barrier_info.layer_count)
      );
    let barriers = [color_barrier, depth_stencil_barrier];

    let dependency_info = if swapchain.depth_stencil_format == HalaFormat::UNDEFINED {
      vk::DependencyInfoKHR::default()
        .image_memory_barriers(std::slice::from_ref(&color_barrier))
    } else {
      vk::DependencyInfoKHR::default()
        .image_memory_barriers(&barriers)
    };

    unsafe {
      let logical_device = self.logical_device.borrow();
      logical_device.raw.cmd_pipeline_barrier2(
        self.raw[index],
        &dependency_info,
      );
    }
  }

  /// Set image barriers.
  /// param index: The index of the command buffer.
  /// param barriers: The barriers.
  pub fn set_image_barriers<IBI>(
    &self,
    index: usize,
    barriers: &[IBI],
  )
    where IBI: AsRef<crate::HalaImageBarrierInfo>
  {
    let barriers = barriers.iter().map(
      |barrier_info| {
        let barrier_info = barrier_info.as_ref();
        vk::ImageMemoryBarrier2::default()
          .src_stage_mask(barrier_info.src_stage_mask.into())
          .src_access_mask(barrier_info.src_access_mask.into())
          .dst_stage_mask(barrier_info.dst_stage_mask.into())
          .dst_access_mask(barrier_info.dst_access_mask.into())
          .old_layout(barrier_info.old_layout.into())
          .new_layout(barrier_info.new_layout.into())
          .src_queue_family_index(barrier_info.src_queue_family_index)
          .dst_queue_family_index(barrier_info.dst_queue_family_index)
          .image(barrier_info.image)
          .subresource_range(
            vk::ImageSubresourceRange::default()
              .aspect_mask(barrier_info.aspect_mask.into())
              .base_mip_level(barrier_info.base_mip_level)
              .level_count(barrier_info.level_count)
              .base_array_layer(barrier_info.base_array_layer)
              .layer_count(barrier_info.layer_count)
          )
      }
    ).collect::<Vec<_>>();

    let dependency_info = vk::DependencyInfoKHR::default()
      .image_memory_barriers(barriers.as_slice());

    unsafe {
      let logical_device = self.logical_device.borrow();
      logical_device.raw.cmd_pipeline_barrier2(
        self.raw[index],
        &dependency_info,
      );
    }
  }

  /// Set buffer barriers.
  /// param index: The index of the command buffer.
  /// param barriers: The barriers.
  pub fn set_buffer_barriers<BBI>(
    &self,
    index: usize,
    barriers: &[BBI],
  )
    where BBI: AsRef<crate::HalaBufferBarrierInfo>
  {
    let barriers = barriers.iter().map(
      |barrier_info| {
        let barrier_info = barrier_info.as_ref();
        vk::BufferMemoryBarrier2KHR::default()
          .src_stage_mask(barrier_info.src_stage_mask.into())
          .src_access_mask(barrier_info.src_access_mask.into())
          .dst_stage_mask(barrier_info.dst_stage_mask.into())
          .dst_access_mask(barrier_info.dst_access_mask.into())
          .src_queue_family_index(barrier_info.src_queue_family_index)
          .dst_queue_family_index(barrier_info.dst_queue_family_index)
          .buffer(barrier_info.buffer)
          .offset(barrier_info.offset)
          .size(barrier_info.size)
      }
    ).collect::<Vec<_>>();

    let dependency_info = vk::DependencyInfoKHR::default()
      .buffer_memory_barriers(barriers.as_slice());

    unsafe {
      let logical_device = self.logical_device.borrow();
      logical_device.raw.cmd_pipeline_barrier2(
        self.raw[index],
        &dependency_info,
      );
    }
  }

  /// Set memory barriers.
  /// param index: The index of the command buffer.
  /// param barriers: The barriers.
  pub fn set_memory_barriers<MBI>(
    &self,
    index: usize,
    barriers: &[MBI],
  )
    where MBI: AsRef<crate::HalaMemoryBarrierInfo>
  {
    let barriers = barriers.iter().map(
      |barrier_info| {
        let barrier_info = barrier_info.as_ref();
        vk::MemoryBarrier2KHR::default()
          .src_stage_mask(barrier_info.src_stage_mask.into())
          .src_access_mask(barrier_info.src_access_mask.into())
          .dst_stage_mask(barrier_info.dst_stage_mask.into())
          .dst_access_mask(barrier_info.dst_access_mask.into())
      }
    ).collect::<Vec<_>>();

    let dependency_info = vk::DependencyInfoKHR::default()
      .memory_barriers(barriers.as_slice());

    unsafe {
      let logical_device = self.logical_device.borrow();
      logical_device.raw.cmd_pipeline_barrier2(
        self.raw[index],
        &dependency_info,
      );
    }
  }

  /// Copy image to swapchain.
  /// param index: The index of the command buffer.
  /// param src_image: The source image.
  /// param src_image_layout: The source image layout.
  /// param dst_swapchain: The destination swapchain.
  /// param dst_swapchain_layout: The destination swapchain layout.
  /// param dst_swapchain_index: The destination swapchain index.
  pub fn copy_image_2_swapchain(
    &self,
    index: usize,
    src_image: &HalaImage,
    src_image_layout: crate::HalaImageLayout,
    dst_swapchain: &HalaSwapchain,
    dst_swapchain_layout: crate::HalaImageLayout,
    dst_swapchain_index: usize,
  ) {
    let region = vk::ImageCopy2::default()
      .src_subresource(
        vk::ImageSubresourceLayers::default()
          .aspect_mask(vk::ImageAspectFlags::COLOR)
          .mip_level(0)
          .base_array_layer(0)
          .layer_count(1)
      )
      .dst_subresource(
        vk::ImageSubresourceLayers::default()
          .aspect_mask(vk::ImageAspectFlags::COLOR)
          .mip_level(0)
          .base_array_layer(0)
          .layer_count(1)
      )
      .extent(vk::Extent3D { width: src_image.extent.width, height: src_image.extent.height, depth: 1 });
    let copy_image_info = vk::CopyImageInfo2::default()
      .src_image(src_image.raw)
      .src_image_layout(src_image_layout.into())
      .dst_image(dst_swapchain.images[dst_swapchain_index])
      .dst_image_layout(dst_swapchain_layout.into())
      .regions(std::slice::from_ref(&region));

    unsafe {
      let logical_device = self.logical_device.borrow();
      logical_device.raw.cmd_copy_image2(
        self.raw[index],
        &copy_image_info,
      );
    }
  }

  /// Copy image to image.
  /// param index: The index of the command buffer.
  /// param src_image: The source image.
  /// param src_image_layout: The source image layout.
  /// param dst_image: The destination image.
  /// param dst_image_layout: The destination image layout.
  /// param regions: The regions.
  pub fn copy_image_2_image(
    &self,
    index: usize,
    src_image: &HalaImage,
    src_image_layout: HalaImageLayout,
    dst_image: &HalaImage,
    dst_image_layout: HalaImageLayout,
  ) {
    let region = vk::ImageCopy2::default()
      .src_subresource(
        vk::ImageSubresourceLayers::default()
          .aspect_mask(vk::ImageAspectFlags::COLOR)
          .mip_level(0)
          .base_array_layer(0)
          .layer_count(1)
      )
      .dst_subresource(
        vk::ImageSubresourceLayers::default()
          .aspect_mask(vk::ImageAspectFlags::COLOR)
          .mip_level(0)
          .base_array_layer(0)
          .layer_count(1)
      )
      .extent(vk::Extent3D { width: src_image.extent.width, height: src_image.extent.height, depth: 1 });
    let copy_image_info = vk::CopyImageInfo2::default()
      .src_image(src_image.raw)
      .src_image_layout(src_image_layout.into())
      .dst_image(dst_image.raw)
      .dst_image_layout(dst_image_layout.into())
      .regions(std::slice::from_ref(&region));

    unsafe {
      let logical_device = self.logical_device.borrow();
      logical_device.raw.cmd_copy_image2(
        self.raw[index],
        &copy_image_info,
      );
    }
  }

  /// Copy buffer to image.
  /// param index: The index of the command buffer.
  /// param src_buffer: The source buffer.
  /// param dst_image: The destination image.
  /// param dst_image_layout: The destination image layout.
  pub fn copy_buffer_2_image(
    &self,
    index: usize,
    src_buffer: &HalaBuffer,
    dst_image: &HalaImage,
    dst_image_layout: HalaImageLayout,
  ) {
    let region = vk::BufferImageCopy2::default()
      .image_subresource(vk::ImageSubresourceLayers::default()
        .aspect_mask(vk::ImageAspectFlags::COLOR)
        .mip_level(0)
        .base_array_layer(0)
        .layer_count(1)
      )
      .image_extent(dst_image.extent);
    let copy_buffer_to_image_info = vk::CopyBufferToImageInfo2::default()
      .src_buffer(src_buffer.raw)
      .dst_image(dst_image.raw)
      .dst_image_layout(dst_image_layout.into())
      .regions(std::slice::from_ref(&region));

    unsafe {
      let logical_device = self.logical_device.borrow();
      logical_device.raw.cmd_copy_buffer_to_image2(
        self.raw[index],
        &copy_buffer_to_image_info,
      );
    }
  }

  /// Copy image to buffer.
  /// param index: The index of the command buffer.
  /// param src_image: The source image.
  /// param src_image_layout: The source image layout.
  /// param dst_buffer: The destination buffer.
  pub fn copy_image_2_buffer(
    &self,
    index: usize,
    src_image: &HalaImage,
    src_image_layout: HalaImageLayout,
    dst_buffer: &HalaBuffer,
  ) {
    let region = vk::BufferImageCopy2::default()
      .image_subresource(vk::ImageSubresourceLayers::default()
        .aspect_mask(vk::ImageAspectFlags::COLOR)
        .mip_level(0)
        .base_array_layer(0)
        .layer_count(1)
      )
      .image_extent(src_image.extent);
    let copy_image_to_buffer_info = vk::CopyImageToBufferInfo2::default()
      .src_image(src_image.raw)
      .src_image_layout(src_image_layout.into())
      .dst_buffer(dst_buffer.raw)
      .regions(std::slice::from_ref(&region));

    unsafe {
      let logical_device = self.logical_device.borrow();
      logical_device.raw.cmd_copy_image_to_buffer2(
        self.raw[index],
        &copy_image_to_buffer_info,
      );
    }
  }

  /// Copy buffer to buffer.
  /// param index: The index of the command buffer.
  /// param src_buffer: The source buffer.
  /// param src_offset: The source offset.
  /// param dst_buffer: The destination buffer.
  /// param dst_offset: The destination offset.
  pub fn copy_buffer_2_buffer(
    &self,
    index: usize,
    src_buffer: &HalaBuffer,
    src_offset: u64,
    dst_buffer: &HalaBuffer,
    dst_offset: u64,
  ) {
    let region = vk::BufferCopy2::default()
      .size(src_buffer.size)
      .src_offset(src_offset)
      .dst_offset(dst_offset);
    let copy_buffer_info = vk::CopyBufferInfo2::default()
      .src_buffer(src_buffer.raw)
      .dst_buffer(dst_buffer.raw)
      .regions(std::slice::from_ref(&region));

    unsafe {
      let logical_device = self.logical_device.borrow();
      logical_device.raw.cmd_copy_buffer2(
        self.raw[index],
        &copy_buffer_info,
      );
    }
  }

  /// Begin a debug label.
  /// param index: The index of the command buffer.
  /// param name: The name of the label.
  /// param color: The color of the label.
  pub fn begin_debug_label(
    &self,
    index: usize,
    name: &str,
    color: [f32; 4],
  ) {
    let name = std::ffi::CString::new(name).unwrap();
    let logical_device = self.logical_device.borrow();
    let label = vk::DebugUtilsLabelEXT::default()
      .label_name(&name)
      .color(color);
    unsafe {
      if let Some(debug_utils_loader) = &logical_device.debug_utils_loader {
        debug_utils_loader.cmd_begin_debug_utils_label(
          self.raw[index],
          &label,
        )
      }
    }
  }

  /// End a debug label.
  /// param index: The index of the command buffer.
  pub fn end_debug_label(&self, index: usize) {
    let logical_device = self.logical_device.borrow();
    unsafe {
      if let Some(debug_utils_loader) = &logical_device.debug_utils_loader {
        debug_utils_loader.cmd_end_debug_utils_label(
          self.raw[index],
        )
      }
    }
  }

}
