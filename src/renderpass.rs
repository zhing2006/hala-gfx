use std::rc::Rc;
use std::cell::RefCell;

use serde::{Serialize, Deserialize, Serializer, Deserializer};
use serde::de::{self, Unexpected, Visitor};

use ash::vk;

use crate::{
  HalaFormat,
  HalaGfxError,
  HalaImageLayout,
  HalaLogicalDevice,
  HalaPipelineStageFlags,
  HalaImageAspectFlags,
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

impl Serialize for HalaSampleCountFlags {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let s = match *self {
      HalaSampleCountFlags::TYPE_1 => "type_1",
      HalaSampleCountFlags::TYPE_2 => "type_2",
      HalaSampleCountFlags::TYPE_4 => "type_4",
      HalaSampleCountFlags::TYPE_8 => "type_8",
      HalaSampleCountFlags::TYPE_16 => "type_16",
      HalaSampleCountFlags::TYPE_32 => "type_32",
      HalaSampleCountFlags::TYPE_64 => "type_64",
      _ => "default",
    };

    serializer.serialize_str(s)
  }
}

impl<'de> Deserialize<'de> for HalaSampleCountFlags {
  fn deserialize<D>(deserializer: D) -> Result<HalaSampleCountFlags, D::Error>
  where
    D: Deserializer<'de>,
  {
    struct HalaSampleCountFlagsVisitor;

    impl<'de> Visitor<'de> for HalaSampleCountFlagsVisitor {
      type Value = HalaSampleCountFlags;

      fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a string of front face")
      }

      fn visit_str<E>(self, value: &str) -> Result<HalaSampleCountFlags, E>
      where
        E: de::Error,
      {
        let val = match value {
          "type_1" => HalaSampleCountFlags::TYPE_1,
          "TYPE_1" => HalaSampleCountFlags::TYPE_1,
          "type_2" => HalaSampleCountFlags::TYPE_2,
          "TYPE_2" => HalaSampleCountFlags::TYPE_2,
          "type_4" => HalaSampleCountFlags::TYPE_4,
          "TYPE_4" => HalaSampleCountFlags::TYPE_4,
          "type_8" => HalaSampleCountFlags::TYPE_8,
          "TYPE_8" => HalaSampleCountFlags::TYPE_8,
          "type_16" => HalaSampleCountFlags::TYPE_16,
          "TYPE_16" => HalaSampleCountFlags::TYPE_16,
          "type_32" => HalaSampleCountFlags::TYPE_32,
          "TYPE_32" => HalaSampleCountFlags::TYPE_32,
          "type_64" => HalaSampleCountFlags::TYPE_64,
          "TYPE_64" => HalaSampleCountFlags::TYPE_64,
          "default" => HalaSampleCountFlags::default(),
          _ => return Err(de::Error::invalid_value(Unexpected::Str(value), &"a sample count flag")),
        };

        Ok(val)
      }
    }

    deserializer.deserialize_str(HalaSampleCountFlagsVisitor)
  }
}

/// The resolve mode flags.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HalaResolveModeFlags(u32);
crate::hala_bitflags_wrapped!(HalaResolveModeFlags, u32);

/// The implementation of the resolve mode flags.
impl HalaResolveModeFlags {
  pub const NONE: Self = Self(vk::ResolveModeFlags::NONE.as_raw());
  pub const SAMPLE_ZERO: Self = Self(vk::ResolveModeFlags::SAMPLE_ZERO.as_raw());
  pub const AVERAGE: Self = Self(vk::ResolveModeFlags::AVERAGE.as_raw());
  pub const MIN: Self = Self(vk::ResolveModeFlags::MIN.as_raw());
  pub const MAX: Self = Self(vk::ResolveModeFlags::MAX.as_raw());
}

impl std::convert::From<vk::ResolveModeFlags> for HalaResolveModeFlags {
  fn from(flags: vk::ResolveModeFlags) -> Self {
    Self(flags.as_raw())
  }
}

impl std::convert::From<HalaResolveModeFlags> for vk::ResolveModeFlags {
  fn from(flags: HalaResolveModeFlags) -> Self {
    vk::ResolveModeFlags::from_raw(flags.0)
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

/// The attachment reference.
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct HalaAttachmentReference {
  pub attachment: u32,
  pub layout: HalaImageLayout,
  pub aspect_mask: HalaImageAspectFlags,
}

/// The subpass dependency.
#[repr(C)]
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

/// The subpass description.
pub struct HalaSubpassDescription {
  pub pipeline_bind_point: HalaPipelineBindPoint,
  pub input_attachments: Vec<HalaAttachmentReference>,
  pub color_attachments: Vec<HalaAttachmentReference>,
  pub resolve_attachments: Vec<HalaAttachmentReference>,
  pub depth_stencil_attachment: Option<HalaAttachmentReference>,
  pub preserve_attachments: Vec<u32>,
}

/// The description of a render pass attachment.
#[derive(Clone)]
pub struct HalaRenderPassAttachmentDesc {
  pub format: HalaFormat,
  pub load_op: HalaAttachmentLoadOp,
  pub store_op: HalaAttachmentStoreOp,
  pub stencil_load_op: HalaAttachmentLoadOp,
  pub stencil_store_op: HalaAttachmentStoreOp,
  pub samples: HalaSampleCountFlags,
  pub initial_layout: HalaImageLayout,
  pub final_layout: HalaImageLayout,
}

impl Default for HalaRenderPassAttachmentDesc {
  fn default() -> Self {
    Self {
      format: HalaFormat::UNDEFINED,
      load_op: HalaAttachmentLoadOp::DONT_CARE,
      store_op: HalaAttachmentStoreOp::DONT_CARE,
      stencil_load_op: HalaAttachmentLoadOp::DONT_CARE,
      stencil_store_op: HalaAttachmentStoreOp::DONT_CARE,
      samples: HalaSampleCountFlags::TYPE_1,
      initial_layout: HalaImageLayout::UNDEFINED,
      final_layout: HalaImageLayout::GENERAL,
    }
  }
}

impl HalaRenderPassAttachmentDesc {
  pub fn format(mut self, format: HalaFormat) -> Self {
    self.format = format;
    self
  }

  pub fn load_op(mut self, load_op: HalaAttachmentLoadOp) -> Self {
    self.load_op = load_op;
    self
  }

  pub fn store_op(mut self, store_op: HalaAttachmentStoreOp) -> Self {
    self.store_op = store_op;
    self
  }

  pub fn stencil_load_op(mut self, stencil_load_op: HalaAttachmentLoadOp) -> Self {
    self.stencil_load_op = stencil_load_op;
    self
  }

  pub fn stencil_store_op(mut self, stencil_store_op: HalaAttachmentStoreOp) -> Self {
    self.stencil_store_op = stencil_store_op;
    self
  }

  pub fn samples(mut self, samples: HalaSampleCountFlags) -> Self {
    self.samples = samples;
    self
  }

  pub fn initial_layout(mut self, initial_layout: HalaImageLayout) -> Self {
    self.initial_layout = initial_layout;
    self
  }

  pub fn final_layout(mut self, final_layout: HalaImageLayout) -> Self {
    self.final_layout = final_layout;
    self
  }
}

/// The render pass.
pub struct HalaRenderPass {
  pub(crate) logical_device: Rc<RefCell<HalaLogicalDevice>>,
  pub raw: vk::RenderPass,
  pub color_attachment_descs: Vec<HalaRenderPassAttachmentDesc>,
  pub depth_stencil_attachment_descs: Vec<HalaRenderPassAttachmentDesc>,
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
#[allow(clippy::too_many_arguments)]
impl HalaRenderPass {

  /// Create a new render pass.
  /// param logical_device: The logical device.
  /// param color_attachment_descs: The color attachment descriptions.
  /// param depth_stencil_attachment_descs: The depth and stencil attachment descriptions.
  /// param debug_name: The debug name.
  /// return: The render pass.
  pub fn new(
    logical_device: Rc<RefCell<HalaLogicalDevice>>,
    color_attachment_descs: &[HalaRenderPassAttachmentDesc],
    depth_stencil_attachment_descs: Option<&[HalaRenderPassAttachmentDesc]>,
    debug_name: &str,
  ) -> Result<Self, HalaGfxError> {
    let subpasses = if depth_stencil_attachment_descs.is_some() {
      vec![
        HalaSubpassDescription {
          pipeline_bind_point: HalaPipelineBindPoint::GRAPHICS,
          input_attachments: vec![],
          color_attachments: color_attachment_descs.iter().enumerate().map(|(index, _)| HalaAttachmentReference {
            attachment: index as u32,
            layout: HalaImageLayout::COLOR_ATTACHMENT_OPTIMAL,
            aspect_mask: HalaImageAspectFlags::COLOR,
          }).collect::<Vec<_>>(),
          resolve_attachments: vec![],
          depth_stencil_attachment: Some(HalaAttachmentReference {
            attachment: color_attachment_descs.len() as u32,
            layout: HalaImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
            aspect_mask: HalaImageAspectFlags::DEPTH | HalaImageAspectFlags::STENCIL,
          }),
          preserve_attachments: vec![],
        }
      ]
    } else {
      vec![
        HalaSubpassDescription {
          pipeline_bind_point: HalaPipelineBindPoint::GRAPHICS,
          input_attachments: vec![],
          color_attachments: color_attachment_descs.iter().enumerate().map(|(index, _)| HalaAttachmentReference {
            attachment: index as u32,
            layout: HalaImageLayout::COLOR_ATTACHMENT_OPTIMAL,
            aspect_mask: HalaImageAspectFlags::COLOR,
          }).collect::<Vec<_>>(),
          resolve_attachments: vec![],
          depth_stencil_attachment: None,
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
      depth_stencil_attachment_descs,
      render_pass,
    ) = Self::create_render_pass(
      &logical_device,
      color_attachment_descs,
      depth_stencil_attachment_descs,
      subpasses.as_slice(),
      subpass_deps.as_slice(),
      debug_name,
    )?;

    log::debug!("A HalaRenderPass \"{}\" is created.", debug_name);
    Ok(
      Self {
        logical_device,
        raw: render_pass,
        color_attachment_descs,
        depth_stencil_attachment_descs,
        debug_name: debug_name.to_string(),
      }
    )
  }

  /// Create a new render pass with subpasses.
  /// param logical_device: The logical device.
  /// param color_attachment_descs: The color attachment descriptions.
  /// param depth_stencil_attachment_descs: The depth and stencil attachment descriptions.
  /// param subpasses: The subpasses.
  /// param subpass_deps: The subpass dependencies.
  /// param debug_name: The debug name.
  /// return: The render pass.
  pub fn with_subpasses(
    logical_device: Rc<RefCell<HalaLogicalDevice>>,
    color_attachment_descs: &[HalaRenderPassAttachmentDesc],
    depth_stencil_attachment_descs: Option<&[HalaRenderPassAttachmentDesc]>,
    subpasses: &[HalaSubpassDescription],
    subpass_deps: &[HalaSubpassDependency],
    debug_name: &str,
  ) -> Result<Self, HalaGfxError> {
    let (
      color_attachment_descs,
      depth_stencil_attachment_descs,
      render_pass,
    ) = Self::create_render_pass(
      &logical_device,
      color_attachment_descs,
      depth_stencil_attachment_descs,
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
        depth_stencil_attachment_descs,
        debug_name: debug_name.to_string(),
      }
    )
  }

  /// Create a render pass.
  /// param logical_device: The logical device.
  /// param color_attachment_descs: The color attachment descriptions.
  /// param depth_stencil_attachment_descs: The depth and stencil attachment descriptions.
  /// param subpasses: The subpasses.
  /// param subpass_deps: The subpass dependencies.
  /// param debug_name: The debug name.
  /// return: The render pass.
  #[allow(clippy::too_many_arguments)]
  fn create_render_pass(
    logical_device: &Rc<RefCell<HalaLogicalDevice>>,
    color_attachment_descs: &[HalaRenderPassAttachmentDesc],
    depth_stencil_attachment_descs: Option<&[HalaRenderPassAttachmentDesc]>,
    subpasses: &[HalaSubpassDescription],
    subpass_deps: &[HalaSubpassDependency],
    debug_name: &str,
  ) -> Result<(
    Vec<HalaRenderPassAttachmentDesc>,
    Vec<HalaRenderPassAttachmentDesc>,
    vk::RenderPass,
  ), HalaGfxError> {
    let attachments = color_attachment_descs.iter().map(|desc| {
      vk::AttachmentDescription2::default()
        .format(desc.format.into())
        .samples(desc.samples.into())
        .load_op(desc.load_op.into())
        .store_op(desc.store_op.into())
        .stencil_load_op(vk::AttachmentLoadOp::DONT_CARE)
        .stencil_store_op(vk::AttachmentStoreOp::DONT_CARE)
        .initial_layout(desc.initial_layout.into())
        .final_layout(desc.final_layout.into())
    }).chain(depth_stencil_attachment_descs.iter().flat_map(|&descs| {
      descs.iter().map(|desc| {
        vk::AttachmentDescription2::default()
          .format(desc.format.into())
          .samples(desc.samples.into())
          .load_op(desc.load_op.into())
          .store_op(desc.store_op.into())
          .stencil_load_op(desc.stencil_load_op.into())
          .stencil_store_op(desc.stencil_store_op.into())
          .initial_layout(desc.initial_layout.into())
          .final_layout(desc.final_layout.into())
      })
    })).collect::<Vec<_>>();

    let attachment_ref_list = subpasses.iter().map(|desc| {
      let input_attachment_refs = desc.input_attachments.iter().map(|ref_| {
        vk::AttachmentReference2::default()
          .attachment(ref_.attachment)
          .layout(ref_.layout.into())
          .aspect_mask(ref_.aspect_mask.into())
      }).collect::<Vec<_>>();
      let color_attachment_refs = desc.color_attachments.iter().map(|ref_| {
        vk::AttachmentReference2::default()
          .attachment(ref_.attachment)
          .layout(ref_.layout.into())
          .aspect_mask(ref_.aspect_mask.into())
      }).collect::<Vec<_>>();
      let resolve_attachment_refs = desc.resolve_attachments.iter().map(|ref_| {
        vk::AttachmentReference2::default()
          .attachment(ref_.attachment)
          .layout(ref_.layout.into())
          .aspect_mask(ref_.aspect_mask.into())
      }).collect::<Vec<_>>();
      let depth_stencil_attachment_ref = desc.depth_stencil_attachment.map_or(
        vk::AttachmentReference2::default(),
        |ref_| {
          vk::AttachmentReference2::default()
            .attachment(ref_.attachment)
            .layout(ref_.layout.into())
            .aspect_mask(ref_.aspect_mask.into())
        }
      );

      (input_attachment_refs, color_attachment_refs, resolve_attachment_refs, depth_stencil_attachment_ref)
    }).collect::<Vec<_>>();

    let vk_subpasses = subpasses.iter().zip(attachment_ref_list.iter()).map(|
      (desc, (input_attachment_refs, color_attachment_refs, resolve_attachment_refs, depth_stencil_attachment_ref))
    | {
        let vk_subpass = vk::SubpassDescription2::default()
          .pipeline_bind_point(desc.pipeline_bind_point.into());
        let vk_subpass = if !input_attachment_refs.is_empty() {
          vk_subpass.input_attachments(input_attachment_refs.as_slice())
        } else {
          vk_subpass
        };
        let vk_subpass = if !resolve_attachment_refs.is_empty() {
          vk_subpass.resolve_attachments(resolve_attachment_refs.as_slice())
        } else {
          vk_subpass
        };
        let vk_subpass = if !color_attachment_refs.is_empty() {
          vk_subpass.color_attachments(color_attachment_refs.as_slice())
        } else {
          vk_subpass
        };
        let vk_subpass = if !desc.preserve_attachments.is_empty() {
          vk_subpass.preserve_attachments(desc.preserve_attachments.as_slice())
        } else {
          vk_subpass
        };
        let vk_subpass = if desc.depth_stencil_attachment.is_some() {
          vk_subpass.depth_stencil_attachment(depth_stencil_attachment_ref)
        } else {
          vk_subpass
        };
        vk_subpass
    }).collect::<Vec<_>>();

    let vk_subpass_deps = subpass_deps.iter().map(|dep| {
      vk::SubpassDependency2::default()
        .src_subpass(dep.src_subpass)
        .dst_subpass(dep.dst_subpass)
        .src_stage_mask(dep.src_stage_mask.into())
        .dst_stage_mask(dep.dst_stage_mask.into())
        .src_access_mask(dep.src_access_mask.into())
        .dst_access_mask(dep.dst_access_mask.into())
        .dependency_flags(dep.dependency_flags.into())
        .view_offset(0)
    }).collect::<Vec<_>>();

    let render_pass_create_info = vk::RenderPassCreateInfo2::default()
      .attachments(attachments.as_slice())
      .subpasses(vk_subpasses.as_slice())
      .dependencies(vk_subpass_deps.as_slice());
    let render_pass = unsafe {
      logical_device.borrow().raw.create_render_pass2(&render_pass_create_info, None)
        .map_err(|err| HalaGfxError::new("Failed to create render pass.", Some(Box::new(err))))?
    };
    logical_device.borrow().set_debug_name(
      render_pass,
      debug_name,
    ).map_err(|err| HalaGfxError::new("Failed to set debug name for render pass.", Some(Box::new(err))))?;

    Ok((color_attachment_descs.to_vec(), depth_stencil_attachment_descs.unwrap_or_default().to_vec(), render_pass))
  }

}
