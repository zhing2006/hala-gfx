use std::rc::Rc;
use std::cell::RefCell;

use ash::vk;

use crate::{
  HalaFormat,
  HalaGfxError,
  HalaImageLayout,
  HalaLogicalDevice,
  HalaPipelineStageFlags,
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
crate::hala_bitflags_wrapped!(HalaSampleCountFlags, u32);

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

/// The pipeline bind point.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct HalaPipelineBindPoint(i32);

/// The implementation of the pipeline bind point.
impl HalaPipelineBindPoint {
  pub const GRAPHICS: Self = Self(vk::PipelineBindPoint::GRAPHICS.as_raw());
  pub const COMPUTE: Self = Self(vk::PipelineBindPoint::COMPUTE.as_raw());
  pub const RAY_TRACING: Self = Self(vk::PipelineBindPoint::RAY_TRACING_KHR.as_raw());
}

impl std::convert::From<vk::PipelineBindPoint> for HalaPipelineBindPoint {
  fn from(bind_point: vk::PipelineBindPoint) -> Self {
    Self(bind_point.as_raw())
  }
}

impl std::convert::From<HalaPipelineBindPoint> for vk::PipelineBindPoint {
  fn from(bind_point: HalaPipelineBindPoint) -> Self {
    vk::PipelineBindPoint::from_raw(bind_point.0)
  }
}

/// The dependency flags.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HalaDependencyFlags(u32);
crate::hala_bitflags_wrapped!(HalaDependencyFlags, u32);
impl HalaDependencyFlags {
  pub const BY_REGION: Self = Self(vk::DependencyFlags::BY_REGION.as_raw());
  pub const DEVICE_GROUP: Self = Self(vk::DependencyFlags::DEVICE_GROUP.as_raw());
  pub const VIEW_LOCAL: Self = Self(vk::DependencyFlags::VIEW_LOCAL.as_raw());
  pub const FEEDBACK_LOOP: Self = Self(vk::DependencyFlags::FEEDBACK_LOOP_EXT.as_raw());
}

impl std::convert::From<vk::DependencyFlags> for HalaDependencyFlags {
  fn from(flags: vk::DependencyFlags) -> Self {
    Self(flags.as_raw())
  }
}

impl std::convert::From<HalaDependencyFlags> for vk::DependencyFlags {
  fn from(flags: HalaDependencyFlags) -> Self {
    vk::DependencyFlags::from_raw(flags.0)
  }
}

/// The access flags.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HalaAccessFlags(u32);
crate::hala_bitflags_wrapped!(HalaAccessFlags, u32);

/// The implementation of the access flags.
impl HalaAccessFlags {
  pub const INDIRECT_COMMAND_READ: Self = Self(vk::AccessFlags::INDIRECT_COMMAND_READ.as_raw());
  pub const INDEX_READ: Self = Self(vk::AccessFlags::INDEX_READ.as_raw());
  pub const VERTEX_ATTRIBUTE_READ: Self = Self(vk::AccessFlags::VERTEX_ATTRIBUTE_READ.as_raw());
  pub const UNIFORM_READ: Self = Self(vk::AccessFlags::UNIFORM_READ.as_raw());
  pub const INPUT_ATTACHMENT_READ: Self = Self(vk::AccessFlags::INPUT_ATTACHMENT_READ.as_raw());
  pub const SHADER_READ: Self = Self(vk::AccessFlags::SHADER_READ.as_raw());
  pub const SHADER_WRITE: Self = Self(vk::AccessFlags::SHADER_WRITE.as_raw());
  pub const COLOR_ATTACHMENT_READ: Self = Self(vk::AccessFlags::COLOR_ATTACHMENT_READ.as_raw());
  pub const COLOR_ATTACHMENT_WRITE: Self = Self(vk::AccessFlags::COLOR_ATTACHMENT_WRITE.as_raw());
  pub const DEPTH_STENCIL_ATTACHMENT_READ: Self = Self(vk::AccessFlags::DEPTH_STENCIL_ATTACHMENT_READ.as_raw());
  pub const DEPTH_STENCIL_ATTACHMENT_WRITE: Self = Self(vk::AccessFlags::DEPTH_STENCIL_ATTACHMENT_WRITE.as_raw());
  pub const TRANSFER_READ: Self = Self(vk::AccessFlags::TRANSFER_READ.as_raw());
  pub const TRANSFER_WRITE: Self = Self(vk::AccessFlags::TRANSFER_WRITE.as_raw());
  pub const HOST_READ: Self = Self(vk::AccessFlags::HOST_READ.as_raw());
  pub const HOST_WRITE: Self = Self(vk::AccessFlags::HOST_WRITE.as_raw());
  pub const MEMORY_READ: Self = Self(vk::AccessFlags::MEMORY_READ.as_raw());
  pub const MEMORY_WRITE: Self = Self(vk::AccessFlags::MEMORY_WRITE.as_raw());
  pub const TRANSFORM_FEEDBACK_WRITE: Self = Self(vk::AccessFlags::TRANSFORM_FEEDBACK_WRITE_EXT.as_raw());
  pub const TRANSFORM_FEEDBACK_COUNTER_READ: Self = Self(vk::AccessFlags::TRANSFORM_FEEDBACK_COUNTER_READ_EXT.as_raw());
  pub const TRANSFORM_FEEDBACK_COUNTER_WRITE: Self = Self(vk::AccessFlags::TRANSFORM_FEEDBACK_COUNTER_WRITE_EXT.as_raw());
  pub const CONDITIONAL_RENDERING_READ: Self = Self(vk::AccessFlags::CONDITIONAL_RENDERING_READ_EXT.as_raw());
  pub const COLOR_ATTACHMENT_READ_NONCOHERENT: Self = Self(vk::AccessFlags::COLOR_ATTACHMENT_READ_NONCOHERENT_EXT.as_raw());
  pub const ACCELERATION_STRUCTURE_READ: Self = Self(vk::AccessFlags::ACCELERATION_STRUCTURE_READ_KHR.as_raw());
  pub const ACCELERATION_STRUCTURE_WRITE: Self = Self(vk::AccessFlags::ACCELERATION_STRUCTURE_WRITE_KHR.as_raw());
  pub const FRAGMENT_DENSITY_MAP_READ: Self = Self(vk::AccessFlags::FRAGMENT_DENSITY_MAP_READ_EXT.as_raw());
  pub const FRAGMENT_SHADING_RATE_ATTACHMENT_READ: Self = Self(vk::AccessFlags::FRAGMENT_SHADING_RATE_ATTACHMENT_READ_KHR.as_raw());
  pub const COMMAND_PREPROCESS_READ: Self = Self(vk::AccessFlags::COMMAND_PREPROCESS_READ_NV.as_raw());
  pub const COMMAND_PREPROCESS_WRITE: Self = Self(vk::AccessFlags::COMMAND_PREPROCESS_WRITE_NV.as_raw());
  pub const NONE: Self = Self(vk::AccessFlags::NONE.as_raw());
}

impl std::convert::From<vk::AccessFlags> for HalaAccessFlags {
  fn from(flags: vk::AccessFlags) -> Self {
    Self(flags.as_raw())
  }
}

impl std::convert::From<HalaAccessFlags> for vk::AccessFlags {
  fn from(flags: HalaAccessFlags) -> Self {
    vk::AccessFlags::from_raw(flags.0)
  }
}

/// The subpass dependency.
#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Copy, Clone, Default)]
pub struct HalaSubpassDependency {
  pub src_subpass: u32,
  pub dst_subpass: u32,
  pub src_stage_mask: HalaPipelineStageFlags,
  pub dst_stage_mask: HalaPipelineStageFlags,
  pub src_access_mask: HalaAccessFlags,
  pub dst_access_mask: HalaAccessFlags,
  pub dependency_flags: HalaDependencyFlags,
}

impl std::convert::From<vk::SubpassDependency> for HalaSubpassDependency {
  fn from(dep: vk::SubpassDependency) -> Self {
    Self {
      src_subpass: dep.src_subpass,
      dst_subpass: dep.dst_subpass,
      src_stage_mask: dep.src_stage_mask.into(),
      dst_stage_mask: dep.dst_stage_mask.into(),
      src_access_mask: dep.src_access_mask.into(),
      dst_access_mask: dep.dst_access_mask.into(),
      dependency_flags: dep.dependency_flags.into(),
    }
  }
}

impl std::convert::From<HalaSubpassDependency> for vk::SubpassDependency {
  fn from(dep: HalaSubpassDependency) -> Self {
    Self {
      src_subpass: dep.src_subpass,
      dst_subpass: dep.dst_subpass,
      src_stage_mask: dep.src_stage_mask.into(),
      dst_stage_mask: dep.dst_stage_mask.into(),
      src_access_mask: dep.src_access_mask.into(),
      dst_access_mask: dep.dst_access_mask.into(),
      dependency_flags: dep.dependency_flags.into(),
    }
  }
}

/// The subpass description.
pub struct HalaSubpassDescription {
  pub pipeline_bind_point: HalaPipelineBindPoint,
  pub input_attachment_layouts: Vec<HalaImageLayout>,
  pub color_attachment_layouts: Vec<HalaImageLayout>,
  pub resolve_attachment_layouts: Vec<HalaImageLayout>,
  pub depth_stencil_attachment_layout: Option<HalaImageLayout>,
  pub preserve_attachments: Vec<u32>,
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
    let subpasses = if depth_format.is_some() {
      vec![
        HalaSubpassDescription {
          pipeline_bind_point: HalaPipelineBindPoint::GRAPHICS,
          input_attachment_layouts: vec![],
          color_attachment_layouts: vec![HalaImageLayout::COLOR_ATTACHMENT_OPTIMAL],
          resolve_attachment_layouts: vec![],
          depth_stencil_attachment_layout: Some(HalaImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL),
          preserve_attachments: vec![],
        }
      ]
    } else {
      vec![
        HalaSubpassDescription {
          pipeline_bind_point: HalaPipelineBindPoint::GRAPHICS,
          input_attachment_layouts: vec![],
          color_attachment_layouts: vec![HalaImageLayout::COLOR_ATTACHMENT_OPTIMAL],
          resolve_attachment_layouts: vec![],
          depth_stencil_attachment_layout: None,
          preserve_attachments: vec![],
        }
      ]
    };
    let subpass_deps = vec![
      HalaSubpassDependency {
        src_subpass: vk::SUBPASS_EXTERNAL,
        dst_subpass: 0,
        src_stage_mask: HalaPipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
        dst_stage_mask: HalaPipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
        src_access_mask: HalaAccessFlags::empty(),
        dst_access_mask: HalaAccessFlags::COLOR_ATTACHMENT_READ | HalaAccessFlags::COLOR_ATTACHMENT_WRITE,
        dependency_flags: HalaDependencyFlags::empty(),
      }
    ];

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
      subpasses,
      subpass_deps,
      debug_name,
    )?;

    log::debug!("A HalaRenderPass \"{}\" is created.", debug_name);
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

  /// Create a new render pass with subpasses.
  /// param logical_device: The logical device.
  /// param color_formats: The color formats.
  /// param color_load_ops: The color load operations.
  /// param color_store_ops: The color store operations.
  /// param depth_format: The depth format.
  /// param depth_load_op: The depth load operation.
  /// param depth_store_op: The depth store operation.
  /// param stencil_load_op: The stencil load operation.
  /// param stencil_store_op: The stencil store operation.
  /// param subpasses: The subpasses.
  /// param subpass_deps: The subpass dependencies.
  /// param debug_name: The debug name.
  /// return: The render pass.
  pub fn with_subpasses(
    logical_device: Rc<RefCell<HalaLogicalDevice>>,
    color_formats: &[HalaFormat],
    color_load_ops: &[HalaAttachmentLoadOp],
    color_store_ops: &[HalaAttachmentStoreOp],
    depth_format: Option<HalaFormat>,
    depth_load_op: Option<HalaAttachmentLoadOp>,
    depth_store_op: Option<HalaAttachmentStoreOp>,
    stencil_load_op: Option<HalaAttachmentLoadOp>,
    stencil_store_op: Option<HalaAttachmentStoreOp>,
    subpasses: Vec<HalaSubpassDescription>,
    subpass_deps: Vec<HalaSubpassDependency>,
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
      subpasses,
      subpass_deps,
      debug_name,
    )?;

    log::debug!("A HalaRenderPass \"{}\" is created.", debug_name);
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
  /// param subpasses: The subpasses.
  /// param subpass_deps: The subpass dependencies.
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
    subpasses: Vec<HalaSubpassDescription>,
    subpass_deps: Vec<HalaSubpassDependency>,
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

    let attachment_ref_list = subpasses.iter().map(|desc| {
      let input_attachment_refs = desc.input_attachment_layouts.iter().enumerate().map(|(i, &layout)| {
        vk::AttachmentReference {
          attachment: i as u32,
          layout: layout.into(),
        }
      }).collect::<Vec<_>>();
      let color_attachment_refs = desc.color_attachment_layouts.iter().enumerate().map(|(i, &layout)| {
        vk::AttachmentReference {
          attachment: i as u32,
          layout: layout.into(),
        }
      }).collect::<Vec<_>>();
      let resolve_attachment_refs = desc.resolve_attachment_layouts.iter().enumerate().map(|(i, &layout)| {
        vk::AttachmentReference {
          attachment: i as u32,
          layout: layout.into(),
        }
      }).collect::<Vec<_>>();
      let depth_stencil_attachment_ref = desc.depth_stencil_attachment_layout.map_or(
        vk::AttachmentReference {
          attachment: vk::ATTACHMENT_UNUSED,
          layout: vk::ImageLayout::UNDEFINED,
        },
        |layout| {
          vk::AttachmentReference {
            attachment: color_attachment_descs.len() as u32,
            layout: layout.into(),
          }
        },
      );

      (input_attachment_refs, color_attachment_refs, resolve_attachment_refs, depth_stencil_attachment_ref)
    }).collect::<Vec<_>>();

    let vk_subpasses = subpasses.iter().zip(attachment_ref_list.iter()).map(|
      (desc, (input_attachment_refs, color_attachment_refs, resolve_attachment_refs, depth_stencil_attachment_ref))
    | {
      if desc.depth_stencil_attachment_layout.is_some() {
        vk::SubpassDescription::default()
          .pipeline_bind_point(desc.pipeline_bind_point.into())
          .color_attachments(color_attachment_refs.as_slice())
          .input_attachments(input_attachment_refs.as_slice())
          .resolve_attachments(resolve_attachment_refs.as_slice())
          .preserve_attachments(desc.preserve_attachments.as_slice())
          .depth_stencil_attachment(&depth_stencil_attachment_ref)
      } else {
        vk::SubpassDescription::default()
          .pipeline_bind_point(desc.pipeline_bind_point.into())
          .color_attachments(color_attachment_refs.as_slice())
          .input_attachments(input_attachment_refs.as_slice())
          .resolve_attachments(resolve_attachment_refs.as_slice())
          .preserve_attachments(desc.preserve_attachments.as_slice())
      }
    }).collect::<Vec<_>>();

    let vk_subpass_deps = subpass_deps.iter().map(|dep| {
      vk::SubpassDependency::default()
        .src_subpass(dep.src_subpass)
        .dst_subpass(dep.dst_subpass)
        .src_stage_mask(dep.src_stage_mask.into())
        .dst_stage_mask(dep.dst_stage_mask.into())
        .src_access_mask(dep.src_access_mask.into())
        .dst_access_mask(dep.dst_access_mask.into())
        .dependency_flags(dep.dependency_flags.into())
    }).collect::<Vec<_>>();

    let render_pass_create_info = vk::RenderPassCreateInfo::default()
      .attachments(attachments.as_slice())
      .subpasses(vk_subpasses.as_slice())
      .dependencies(vk_subpass_deps.as_slice());
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
