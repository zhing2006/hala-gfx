use std::rc::Rc;
use std::cell::RefCell;

use ash::vk;

use crate::{
  HalaGfxError,
  HalaLogicalDevice,
  HalaRenderPass,
};

/// The frame buffer set.
pub struct HalaFrameBufferSet {
  pub(crate) logical_device: Rc<RefCell<HalaLogicalDevice>>,
  pub raw: Vec<vk::Framebuffer>,
}

/// The Drop trait implementation for frame buffer set.
impl Drop for HalaFrameBufferSet {
  fn drop(&mut self) {
    unsafe {
      for framebuffer in self.raw.iter() {
        self.logical_device.borrow().raw.destroy_framebuffer(*framebuffer, None);
      }
    }
    log::debug!("A HalaFrameBufferSet is dropped.");
  }
}

/// The implementation of frame buffer set.
impl HalaFrameBufferSet {
  /// Create a new frame buffer set.
  /// param logical_device: The logical device.
  /// param render_pass: The render pass.
  /// param image_views: The image views.
  /// param extent: The extent.
  /// param depth_stencil_image_view: The depth stencil image view.
  /// return: The frame buffer set.
  pub fn new(
    logical_device: Rc<RefCell<HalaLogicalDevice>>,
    render_pass: &HalaRenderPass,
    image_views: &[vk::ImageView],
    extent: vk::Extent2D,
    depth_stencil_image_view: vk::ImageView,
  ) -> Result<Self, crate::HalaGfxError> {
    let framebuffers = image_views.iter().map(|image_view| {
      let attachments = if depth_stencil_image_view != vk::ImageView::null() { vec![*image_view, depth_stencil_image_view] } else { vec![*image_view] };
      let framebuffer_create_info = vk::FramebufferCreateInfo::default()
        .render_pass(render_pass.raw)
        .attachments(&attachments)
        .width(extent.width)
        .height(extent.height)
        .layers(1);
      unsafe {
        logical_device.borrow().raw.create_framebuffer(&framebuffer_create_info, None)
      }
    }).collect::<Result<Vec<_>, _>>()
      .map_err(|err| HalaGfxError::new("Failed to create framebuffer.", Some(Box::new(err))))?;
    {
      let logical_device = logical_device.borrow();
      for (index, &framebuffer) in framebuffers.iter().enumerate() {
        logical_device.set_debug_name(
          framebuffer,
          &format!("framebuffer_{}", index))
          .map_err(|err| HalaGfxError::new("Failed to set debug name for framebuffer.", Some(Box::new(err))))?;
      }
    }

    log::debug!("A HalaFrameBufferSet is created.");
    Ok(
      Self {
        logical_device,
        raw: framebuffers,
      }
    )
  }
}