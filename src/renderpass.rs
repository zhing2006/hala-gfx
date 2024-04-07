use std::rc::Rc;
use std::cell::RefCell;

use ash::vk;

use crate::{
  HalaGfxError,
  HalaLogicalDevice,
};

/// The description of a render pass attachment.
#[derive(Default, Debug, Clone, Copy)]
pub struct HalaRenderPassAttachmentDesc {
  pub format: vk::Format,
  pub load_op: vk::AttachmentLoadOp,
  pub store_op: vk::AttachmentStoreOp,
  pub samples: vk::SampleCountFlags,
}

/// The render pass.
pub struct HalaRenderPass {
  pub(crate) logical_device: Rc<RefCell<HalaLogicalDevice>>,
  pub raw: vk::RenderPass,
  pub color_attachments: Vec<HalaRenderPassAttachmentDesc>,
  pub depth_attachment: Option<HalaRenderPassAttachmentDesc>,
}

/// The Drop trait implementation of the render pass.
impl Drop for HalaRenderPass {
  fn drop(&mut self) {
    unsafe {
      self.logical_device.borrow().raw.destroy_render_pass(self.raw, None);
    }
    log::debug!("A HalaRenderPass is dropped.");
  }
}

/// The implementation of the render pass.
impl HalaRenderPass {
  /// Create a new render pass.
  /// param logical_device: The logical device.
  /// param color_formats: The color formats.
  /// param color_clear_flags: The color clear flags.
  /// param depth_format: The depth format.
  /// param debug_name: The debug name.
  /// return: The render pass.
  pub fn new(
    logical_device: Rc<RefCell<HalaLogicalDevice>>,
    color_formats: &[vk::Format],
    color_clear_flags: &[bool],
    depth_format: vk::Format,
    depth_clear_flag: bool,
    debug_name: &str,
  ) -> Result<Self, HalaGfxError> {
    let (
      color_attachment_descs,
      depth_attachment_desc,
      render_pass
    ) = Self::create_render_pass(
      &logical_device,
      color_formats,
      color_clear_flags,
      depth_format,
      depth_clear_flag,
      debug_name,
    )?;

    log::debug!("A HalaRenderPass is created.");
    Ok(
      Self {
        logical_device,
        raw: render_pass,
        color_attachments: color_attachment_descs,
        depth_attachment: depth_attachment_desc,
      }
    )
  }

  /// Create a render pass.
  /// param logical_device: The logical device.
  /// param color_formats: The color formats.
  /// param color_clear_flags: The color clear flags.
  /// param depth_format: The depth format.
  /// param depth_clear_flag: The depth clear flag.
  /// param debug_name: The debug name.
  /// return: The render pass.
  fn create_render_pass(
    logical_device: &Rc<RefCell<HalaLogicalDevice>>,
    color_formats: &[vk::Format],
    color_clear_flags: &[bool],
    depth_format: vk::Format,
    depth_clear_flag: bool,
    debug_name: &str,
  ) -> Result<(
    Vec<HalaRenderPassAttachmentDesc>,
    Option<HalaRenderPassAttachmentDesc>,
    vk::RenderPass), HalaGfxError>
  {
    let color_attachment_descs = color_formats.iter().zip(color_clear_flags.iter())
      .map(|(&color_format, &clear_flag)| {
        HalaRenderPassAttachmentDesc {
          format: color_format,
          load_op: if clear_flag { vk::AttachmentLoadOp::CLEAR } else { vk::AttachmentLoadOp::DONT_CARE },
          store_op: vk::AttachmentStoreOp::STORE,
          samples: vk::SampleCountFlags::TYPE_1,
        }
      }
    ).collect::<Vec<_>>();
    let depth_attachment_desc = if depth_format != vk::Format::UNDEFINED {
      Some(HalaRenderPassAttachmentDesc {
        format: depth_format,
        load_op: if depth_clear_flag { vk::AttachmentLoadOp::CLEAR } else { vk::AttachmentLoadOp::DONT_CARE },
        store_op: vk::AttachmentStoreOp::DONT_CARE,
        samples: vk::SampleCountFlags::TYPE_1,
      })
    } else {
      None
    };
    let attachments = color_attachment_descs.iter().map(|desc| {
      vk::AttachmentDescription::default()
        .format(desc.format)
        .samples(desc.samples)
        .load_op(desc.load_op)
        .store_op(desc.store_op)
        .stencil_load_op(vk::AttachmentLoadOp::DONT_CARE)
        .stencil_store_op(vk::AttachmentStoreOp::DONT_CARE)
        .initial_layout(vk::ImageLayout::UNDEFINED)
        .final_layout(vk::ImageLayout::PRESENT_SRC_KHR)
    }).chain(depth_attachment_desc.iter().map(|desc| {
      vk::AttachmentDescription::default()
        .format(desc.format)
        .samples(desc.samples)
        .load_op(desc.load_op)
        .store_op(desc.store_op)
        .stencil_load_op(vk::AttachmentLoadOp::CLEAR)
        .stencil_store_op(vk::AttachmentStoreOp::DONT_CARE)
        .initial_layout(vk::ImageLayout::UNDEFINED)
        .final_layout(vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL)
    })).collect::<Vec<_>>();
    let color_attachment_refs = color_attachment_descs.iter().enumerate().map(|(i, _)| {
      vk::AttachmentReference {
        attachment: i as u32,
        layout: vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
      }
    }).collect::<Vec<_>>();
    let depth_attachment_ref = vk::AttachmentReference {
      attachment: color_attachment_descs.len() as u32,
      layout: vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
    };
    let subpasses = if depth_attachment_desc.is_some() {
      [
        vk::SubpassDescription::default()
          .pipeline_bind_point(vk::PipelineBindPoint::GRAPHICS)
          .color_attachments(&color_attachment_refs)
          .depth_stencil_attachment(&depth_attachment_ref)
      ]
    } else {
      [
        vk::SubpassDescription::default()
          .pipeline_bind_point(vk::PipelineBindPoint::GRAPHICS)
          .color_attachments(&color_attachment_refs)
      ]
    };
    let subpass_deps = [vk::SubpassDependency::default()
      .src_subpass(vk::SUBPASS_EXTERNAL)
      .dst_subpass(0)
      .src_stage_mask(vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
      .dst_stage_mask(vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
      .src_access_mask(vk::AccessFlags::empty())
      .dst_access_mask(vk::AccessFlags::COLOR_ATTACHMENT_READ | vk::AccessFlags::COLOR_ATTACHMENT_WRITE)];
    let render_pass_create_info = vk::RenderPassCreateInfo::default()
      .attachments(&attachments)
      .subpasses(&subpasses)
      .dependencies(&subpass_deps);
    let render_pass = unsafe {
      logical_device.borrow().raw.create_render_pass(&render_pass_create_info, None)
        .map_err(|err| HalaGfxError::new("Failed to create render pass.", Some(Box::new(err))))?
    };
    logical_device.borrow().set_debug_name(
      render_pass,
      debug_name,
    ).map_err(|err| HalaGfxError::new("Failed to set debug name for render pass.", Some(Box::new(err))))?;
    Ok((color_attachment_descs, depth_attachment_desc, render_pass))
  }
}
