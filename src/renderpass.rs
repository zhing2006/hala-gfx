use std::rc::Rc;
use std::cell::RefCell;

use ash::vk;

use crate::{
  HalaFormat, HalaGfxError, HalaLogicalDevice
};

/// The attachment load operation.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct HalaAttachmentLoadOp(i32);

/// The implementation of the attachment load operation.
impl HalaAttachmentLoadOp {
  pub const LOAD: Self = Self(vk::AttachmentLoadOp::LOAD.as_raw());
  pub const CLEAR: Self = Self(vk::AttachmentLoadOp::CLEAR.as_raw());
  pub const DONT_CARE: Self = Self(vk::AttachmentLoadOp::DONT_CARE.as_raw());
}

impl std::convert::From<vk::AttachmentLoadOp> for HalaAttachmentLoadOp {
  fn from(op: vk::AttachmentLoadOp) -> Self {
    Self(op.as_raw())
  }
}

impl std::convert::From<HalaAttachmentLoadOp> for vk::AttachmentLoadOp {
  fn from(op: HalaAttachmentLoadOp) -> Self {
    vk::AttachmentLoadOp::from_raw(op.0)
  }
}

/// The attachment store operation.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct HalaAttachmentStoreOp(i32);

/// The implementation of the attachment store operation.
impl HalaAttachmentStoreOp {
  pub const STORE: Self = Self(vk::AttachmentStoreOp::STORE.as_raw());
  pub const DONT_CARE: Self = Self(vk::AttachmentStoreOp::DONT_CARE.as_raw());
}

impl std::convert::From<vk::AttachmentStoreOp> for HalaAttachmentStoreOp {
  fn from(op: vk::AttachmentStoreOp) -> Self {
    Self(op.as_raw())
  }
}

impl std::convert::From<HalaAttachmentStoreOp> for vk::AttachmentStoreOp {
  fn from(op: HalaAttachmentStoreOp) -> Self {
    vk::AttachmentStoreOp::from_raw(op.0)
  }
}

/// The sample count flags.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HalaSampleCountFlags(u32);

/// The implementation of the sample count flags.
impl HalaSampleCountFlags {
  pub const TYPE_1: Self = Self(vk::SampleCountFlags::TYPE_1.as_raw());
  pub const TYPE_2: Self = Self(vk::SampleCountFlags::TYPE_2.as_raw());
  pub const TYPE_4: Self = Self(vk::SampleCountFlags::TYPE_4.as_raw());
  pub const TYPE_8: Self = Self(vk::SampleCountFlags::TYPE_8.as_raw());
  pub const TYPE_16: Self = Self(vk::SampleCountFlags::TYPE_16.as_raw());
  pub const TYPE_32: Self = Self(vk::SampleCountFlags::TYPE_32.as_raw());
  pub const TYPE_64: Self = Self(vk::SampleCountFlags::TYPE_64.as_raw());
}

impl std::convert::From<vk::SampleCountFlags> for HalaSampleCountFlags {
  fn from(flags: vk::SampleCountFlags) -> Self {
    Self(flags.as_raw())
  }
}

impl std::convert::From<HalaSampleCountFlags> for vk::SampleCountFlags {
  fn from(flags: HalaSampleCountFlags) -> Self {
    vk::SampleCountFlags::from_raw(flags.0)
  }
}

/// The description of a render pass attachment.
pub struct HalaRenderPassAttachmentDesc {
  pub format: HalaFormat,
  pub load_op: HalaAttachmentLoadOp,
  pub store_op: HalaAttachmentStoreOp,
  pub samples: HalaSampleCountFlags,
}

/// The render pass.
pub struct HalaRenderPass {
  pub(crate) logical_device: Rc<RefCell<HalaLogicalDevice>>,
  pub raw: vk::RenderPass,
  pub color_attachment_descs: Vec<HalaRenderPassAttachmentDesc>,
  pub depth_attachment_desc: Option<HalaRenderPassAttachmentDesc>,
  pub stencil_attachment_desc: Option<HalaRenderPassAttachmentDesc>,
  pub debug_name: String,
}

/// The Drop trait implementation of the render pass.
impl Drop for HalaRenderPass {
  fn drop(&mut self) {
    unsafe {
      self.logical_device.borrow().raw.destroy_render_pass(self.raw, None);
    }
    log::debug!("A HalaRenderPass \"{}\" is dropped.", self.debug_name);
  }
}

/// The implementation of the render pass.
impl HalaRenderPass {

  /// Create a new render pass.
  /// param logical_device: The logical device.
  /// param color_formats: The color formats.
  /// param color_load_ops: The color load operations.
  /// param color_store_ops: The color store operations.
  /// param depth_format: The depth format.
  /// param depth_load_op: The depth load operation.
  /// param depth_store_op: The depth store operation.
  /// param stencil_load_op: The stencil load operation.
  /// param stencil_store_op: The stencil store operation.
  /// param debug_name: The debug name.
  /// return: The render pass.
  pub fn new(
    logical_device: Rc<RefCell<HalaLogicalDevice>>,
    color_formats: &[HalaFormat],
    color_load_ops: &[HalaAttachmentLoadOp],
    color_store_ops: &[HalaAttachmentStoreOp],
    depth_format: Option<HalaFormat>,
    depth_load_op: Option<HalaAttachmentLoadOp>,
    depth_store_op: Option<HalaAttachmentStoreOp>,
    stencil_load_op: Option<HalaAttachmentLoadOp>,
    stencil_store_op: Option<HalaAttachmentStoreOp>,
    debug_name: &str,
  ) -> Result<Self, HalaGfxError> {
    let (
      color_attachment_descs,
      depth_attachment_desc,
      stencil_attachment_desc,
      render_pass,
    ) = Self::create_render_pass(
      &logical_device,
      color_formats,
      color_load_ops,
      color_store_ops,
      depth_format,
      depth_load_op,
      depth_store_op,
      stencil_load_op,
      stencil_store_op,
      debug_name,
    )?;

    log::debug!("A HalaRenderPass is created.");
    Ok(
      Self {
        logical_device,
        raw: render_pass,
        color_attachment_descs,
        depth_attachment_desc,
        stencil_attachment_desc,
        debug_name: debug_name.to_string(),
      }
    )
  }

  /// Create a render pass.
  /// param logical_device: The logical device.
  /// param color_formats: The color formats.
  /// param color_load_ops: The color load operations.
  /// param color_store_ops: The color store operations.
  /// param depth_format: The depth format.
  /// param depth_load_op: The depth load operation.
  /// param depth_store_op: The depth store operation.
  /// param stencil_load_op: The stencil load operation.
  /// param stencil_store_op: The stencil store operation.
  /// param debug_name: The debug name.
  /// return: The render pass.
  fn create_render_pass(
    logical_device: &Rc<RefCell<HalaLogicalDevice>>,
    color_formats: &[HalaFormat],
    color_load_ops: &[HalaAttachmentLoadOp],
    color_store_ops: &[HalaAttachmentStoreOp],
    depth_format: Option<HalaFormat>,
    depth_load_op: Option<HalaAttachmentLoadOp>,
    depth_store_op: Option<HalaAttachmentStoreOp>,
    stencil_load_op: Option<HalaAttachmentLoadOp>,
    stencil_store_op: Option<HalaAttachmentStoreOp>,
    debug_name: &str,
  ) -> Result<(
    Vec<HalaRenderPassAttachmentDesc>,
    Option<HalaRenderPassAttachmentDesc>,
    Option<HalaRenderPassAttachmentDesc>,
    vk::RenderPass,
  ), HalaGfxError> {
    let color_attachment_descs = color_formats.iter().zip(color_load_ops.iter()).zip(color_store_ops.iter())
      .map(|((&format, &load_op), &store_op)| {
        HalaRenderPassAttachmentDesc {
          format: format,
          load_op: load_op,
          store_op: store_op,
          samples: HalaSampleCountFlags::TYPE_1,
        }
      }
    ).collect::<Vec<_>>();
    let depth_attachment_desc = if depth_format.is_some() {
      let depth_format = depth_format.ok_or(HalaGfxError::new("The depth format is not specified.", None))?;
      let depth_store_op = depth_store_op.ok_or(HalaGfxError::new("The depth store operation is not specified.", None))?;
      let depth_load_op = depth_load_op.ok_or(HalaGfxError::new("The depth load operation is not specified.", None))?;
      Some(HalaRenderPassAttachmentDesc {
        format: depth_format,
        load_op: depth_load_op,
        store_op: depth_store_op,
        samples: HalaSampleCountFlags::TYPE_1,
      })
    } else {
      None
    };
    let stencil_attachment_desc = if depth_format.is_some() && stencil_load_op.is_some() {
      let depth_format = depth_format.ok_or(HalaGfxError::new("The depth format is not specified.", None))?;
      let stencil_load_op = stencil_load_op.ok_or(HalaGfxError::new("The stencil load operation is not specified.", None))?;
      let stencil_store_op = stencil_store_op.ok_or(HalaGfxError::new("The stencil store operation is not specified.", None))?;
      Some(HalaRenderPassAttachmentDesc {
        format: depth_format,
        load_op: stencil_load_op,
        store_op: stencil_store_op,
        samples: HalaSampleCountFlags::TYPE_1,
      })
    } else {
      None
    };

    let attachments = color_attachment_descs.iter().map(|desc| {
      vk::AttachmentDescription::default()
        .format(desc.format.into())
        .samples(desc.samples.into())
        .load_op(desc.load_op.into())
        .store_op(desc.store_op.into())
        .stencil_load_op(vk::AttachmentLoadOp::DONT_CARE)
        .stencil_store_op(vk::AttachmentStoreOp::DONT_CARE)
        .initial_layout(vk::ImageLayout::UNDEFINED)
        .final_layout(vk::ImageLayout::PRESENT_SRC_KHR)
    }).chain(depth_attachment_desc.iter().map(|desc| {
      let vk_desc = vk::AttachmentDescription::default()
        .format(desc.format.into())
        .samples(desc.samples.into())
        .load_op(desc.load_op.into())
        .store_op(desc.store_op.into())
        .initial_layout(vk::ImageLayout::UNDEFINED)
        .final_layout(vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL);
      if let Some(stencil_attachment_desc) = stencil_attachment_desc.as_ref() {
        vk_desc.stencil_load_op(stencil_attachment_desc.load_op.into())
          .stencil_store_op(stencil_attachment_desc.store_op.into())
      } else {
        vk_desc.stencil_load_op(vk::AttachmentLoadOp::CLEAR)
          .stencil_store_op(vk::AttachmentStoreOp::DONT_CARE)
      }
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

    Ok((color_attachment_descs, depth_attachment_desc, stencil_attachment_desc, render_pass))
  }

}
