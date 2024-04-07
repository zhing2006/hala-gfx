use std::rc::Rc;
use std::cell::RefCell;

use ash::vk;

use crate::{
  HalaGfxError,
  HalaLogicalDevice,
  HalaSwapchain,
  HalaFormat,
  HalaShaderStageFlags,
  HalaShader,
  HalaPipelineCache,
  HalaDescriptorSetLayout,
};

/// The pipeline stage flags.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HalaPipelineStageFlags(u32);
crate::hala_bitflags_wrapped!(HalaPipelineStageFlags, u32);
impl HalaPipelineStageFlags {
  pub const TOP_OF_PIPE: Self = Self(vk::PipelineStageFlags::TOP_OF_PIPE.as_raw());
  pub const DRAW_INDIRECT: Self = Self(vk::PipelineStageFlags::DRAW_INDIRECT.as_raw());
  pub const VERTEX_INPUT: Self = Self(vk::PipelineStageFlags::VERTEX_INPUT.as_raw());
  pub const VERTEX_SHADER: Self = Self(vk::PipelineStageFlags::VERTEX_SHADER.as_raw());
  pub const TESSELLATION_CONTROL_SHADER: Self = Self(vk::PipelineStageFlags::TESSELLATION_CONTROL_SHADER.as_raw());
  pub const TESSELLATION_EVALUATION_SHADER: Self = Self(vk::PipelineStageFlags::TESSELLATION_EVALUATION_SHADER.as_raw());
  pub const GEOMETRY_SHADER: Self = Self(vk::PipelineStageFlags::GEOMETRY_SHADER.as_raw());
  pub const FRAGMENT_SHADER: Self = Self(vk::PipelineStageFlags::FRAGMENT_SHADER.as_raw());
  pub const EARLY_FRAGMENT_TESTS: Self = Self(vk::PipelineStageFlags::EARLY_FRAGMENT_TESTS.as_raw());
  pub const LATE_FRAGMENT_TESTS: Self = Self(vk::PipelineStageFlags::LATE_FRAGMENT_TESTS.as_raw());
  pub const COLOR_ATTACHMENT_OUTPUT: Self = Self(vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT.as_raw());
  pub const COMPUTE_SHADER: Self = Self(vk::PipelineStageFlags::COMPUTE_SHADER.as_raw());
  pub const TRANSFER: Self = Self(vk::PipelineStageFlags::TRANSFER.as_raw());
  pub const BOTTOM_OF_PIPE: Self = Self(vk::PipelineStageFlags::BOTTOM_OF_PIPE.as_raw());
  pub const HOST: Self = Self(vk::PipelineStageFlags::HOST.as_raw());
  pub const ALL_GRAPHICS: Self = Self(vk::PipelineStageFlags::ALL_GRAPHICS.as_raw());
  pub const ALL_COMMANDS: Self = Self(vk::PipelineStageFlags::ALL_COMMANDS.as_raw());
  pub const TRANSFORM_FEEDBACK_EXT: Self = Self(vk::PipelineStageFlags::TRANSFORM_FEEDBACK_EXT.as_raw());
  pub const CONDITIONAL_RENDERING_EXT: Self = Self(vk::PipelineStageFlags::CONDITIONAL_RENDERING_EXT.as_raw());
  pub const RAY_TRACING_SHADER_KHR: Self = Self(vk::PipelineStageFlags::RAY_TRACING_SHADER_KHR.as_raw());
  pub const ACCELERATION_STRUCTURE_BUILD_KHR: Self = Self(vk::PipelineStageFlags::ACCELERATION_STRUCTURE_BUILD_KHR.as_raw());
}

impl std::convert::From<vk::PipelineStageFlags> for HalaPipelineStageFlags {
  fn from(flags: vk::PipelineStageFlags) -> Self {
    Self(flags.as_raw())
  }
}

impl std::convert::From<HalaPipelineStageFlags> for vk::PipelineStageFlags {
  fn from(flags: HalaPipelineStageFlags) -> Self {
    vk::PipelineStageFlags::from_raw(flags.0)
  }
}

/// The pipeline stage flags2.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HalaPipelineStageFlags2(u64);
crate::hala_bitflags_wrapped!(HalaPipelineStageFlags2, u64);

impl HalaPipelineStageFlags2 {
  pub const NONE: Self = Self(vk::PipelineStageFlags2::NONE.as_raw());
  pub const TOP_OF_PIPE: Self = Self(vk::PipelineStageFlags2::TOP_OF_PIPE.as_raw());
  pub const DRAW_INDIRECT: Self = Self(vk::PipelineStageFlags2::DRAW_INDIRECT.as_raw());
  pub const VERTEX_INPUT: Self = Self(vk::PipelineStageFlags2::VERTEX_INPUT.as_raw());
  pub const VERTEX_SHADER: Self = Self(vk::PipelineStageFlags2::VERTEX_SHADER.as_raw());
  pub const TESSELLATION_CONTROL_SHADER: Self = Self(vk::PipelineStageFlags2::TESSELLATION_CONTROL_SHADER.as_raw());
  pub const TESSELLATION_EVALUATION_SHADER: Self = Self(vk::PipelineStageFlags2::TESSELLATION_EVALUATION_SHADER.as_raw());
  pub const GEOMETRY_SHADER: Self = Self(vk::PipelineStageFlags2::GEOMETRY_SHADER.as_raw());
  pub const FRAGMENT_SHADER: Self = Self(vk::PipelineStageFlags2::FRAGMENT_SHADER.as_raw());
  pub const EARLY_FRAGMENT_TESTS: Self = Self(vk::PipelineStageFlags2::EARLY_FRAGMENT_TESTS.as_raw());
  pub const LATE_FRAGMENT_TESTS: Self = Self(vk::PipelineStageFlags2::LATE_FRAGMENT_TESTS.as_raw());
  pub const COLOR_ATTACHMENT_OUTPUT: Self = Self(vk::PipelineStageFlags2::COLOR_ATTACHMENT_OUTPUT.as_raw());
  pub const COMPUTE_SHADER: Self = Self(vk::PipelineStageFlags2::COMPUTE_SHADER.as_raw());
  pub const ALL_TRANSFER: Self = Self(vk::PipelineStageFlags2::ALL_TRANSFER.as_raw());
  pub const TRANSFER: Self = Self(vk::PipelineStageFlags2::TRANSFER.as_raw());
  pub const BOTTOM_OF_PIPE: Self = Self(vk::PipelineStageFlags2::BOTTOM_OF_PIPE.as_raw());
  pub const HOST: Self = Self(vk::PipelineStageFlags2::HOST.as_raw());
  pub const ALL_GRAPHICS: Self = Self(vk::PipelineStageFlags2::ALL_GRAPHICS.as_raw());
  pub const ALL_COMMANDS: Self = Self(vk::PipelineStageFlags2::ALL_COMMANDS.as_raw());
  pub const COPY: Self = Self(vk::PipelineStageFlags2::COPY.as_raw());
  pub const RESOLVE: Self = Self(vk::PipelineStageFlags2::RESOLVE.as_raw());
  pub const BLIT: Self = Self(vk::PipelineStageFlags2::BLIT.as_raw());
  pub const CLEAR: Self = Self(vk::PipelineStageFlags2::CLEAR.as_raw());
  pub const INDEX_INPUT: Self = Self(vk::PipelineStageFlags2::INDEX_INPUT.as_raw());
  pub const VERTEX_ATTRIBUTE_INPUT: Self = Self(vk::PipelineStageFlags2::VERTEX_ATTRIBUTE_INPUT.as_raw());
  pub const PRE_RASTERIZATION_SHADERS: Self = Self(vk::PipelineStageFlags2::PRE_RASTERIZATION_SHADERS.as_raw());
  pub const TRANSFORM_FEEDBACK_EXT: Self = Self(vk::PipelineStageFlags2::TRANSFORM_FEEDBACK_EXT.as_raw());
  pub const CONDITIONAL_RENDERING_EXT: Self = Self(vk::PipelineStageFlags2::CONDITIONAL_RENDERING_EXT.as_raw());
  pub const COMMAND_PREPROCESS_NV: Self = Self(vk::PipelineStageFlags2::COMMAND_PREPROCESS_NV.as_raw());
  pub const FRAGMENT_SHADING_RATE_ATTACHMENT: Self = Self(vk::PipelineStageFlags2::FRAGMENT_SHADING_RATE_ATTACHMENT_KHR.as_raw());
  pub const SHADING_RATE_IMAGE_NV: Self = Self(vk::PipelineStageFlags2::SHADING_RATE_IMAGE_NV.as_raw());
  pub const ACCELERATION_STRUCTURE_BUILD: Self = Self(vk::PipelineStageFlags2::ACCELERATION_STRUCTURE_BUILD_KHR.as_raw());
  pub const RAY_TRACING_SHADER: Self = Self(vk::PipelineStageFlags2::RAY_TRACING_SHADER_KHR.as_raw());
  pub const FRAGMENT_DENSITY_PROCESS_EXT: Self = Self(vk::PipelineStageFlags2::FRAGMENT_DENSITY_PROCESS_EXT.as_raw());
  pub const TASK_SHADER_EXT: Self = Self(vk::PipelineStageFlags2::TASK_SHADER_EXT.as_raw());
  pub const MESH_SHADER_EXT: Self = Self(vk::PipelineStageFlags2::MESH_SHADER_EXT.as_raw());
}

impl std::convert::From<vk::PipelineStageFlags2> for HalaPipelineStageFlags2 {
  fn from(flags: vk::PipelineStageFlags2) -> Self {
    Self(flags.as_raw())
  }
}

impl std::convert::From<HalaPipelineStageFlags2> for vk::PipelineStageFlags2 {
  fn from(flags: HalaPipelineStageFlags2) -> Self {
    vk::PipelineStageFlags2::from_raw(flags.0)
  }
}

/// The vertex input rate.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct HalaVertexInputRate(i32);
impl HalaVertexInputRate {
  pub const VERTEX: Self = Self(vk::VertexInputRate::VERTEX.as_raw());
  pub const INSTANCE: Self = Self(vk::VertexInputRate::INSTANCE.as_raw());
}

impl std::convert::From<vk::VertexInputRate> for HalaVertexInputRate {
  fn from(val: vk::VertexInputRate) -> Self {
    Self(val.as_raw())
  }
}

impl std::convert::From<HalaVertexInputRate> for vk::VertexInputRate {
  fn from(val: HalaVertexInputRate) -> Self {
    vk::VertexInputRate::from_raw(val.0)
  }
}

/// The primitive topology.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct HalaPrimitiveTopology(i32);
impl HalaPrimitiveTopology {
  pub const POINT_LIST: Self = Self(vk::PrimitiveTopology::POINT_LIST.as_raw());
  pub const LINE_LIST: Self = Self(vk::PrimitiveTopology::LINE_LIST.as_raw());
  pub const LINE_STRIP: Self = Self(vk::PrimitiveTopology::LINE_STRIP.as_raw());
  pub const TRIANGLE_LIST: Self = Self(vk::PrimitiveTopology::TRIANGLE_LIST.as_raw());
  pub const TRIANGLE_STRIP: Self = Self(vk::PrimitiveTopology::TRIANGLE_STRIP.as_raw());
  pub const TRIANGLE_FAN: Self = Self(vk::PrimitiveTopology::TRIANGLE_FAN.as_raw());
  pub const LINE_LIST_WITH_ADJACENCY: Self = Self(vk::PrimitiveTopology::LINE_LIST_WITH_ADJACENCY.as_raw());
  pub const LINE_STRIP_WITH_ADJACENCY: Self = Self(vk::PrimitiveTopology::LINE_STRIP_WITH_ADJACENCY.as_raw());
  pub const TRIANGLE_LIST_WITH_ADJACENCY: Self = Self(vk::PrimitiveTopology::TRIANGLE_LIST_WITH_ADJACENCY.as_raw());
  pub const TRIANGLE_STRIP_WITH_ADJACENCY: Self = Self(vk::PrimitiveTopology::TRIANGLE_STRIP_WITH_ADJACENCY.as_raw());
  pub const PATCH_LIST: Self = Self(vk::PrimitiveTopology::PATCH_LIST.as_raw());
}

impl std::convert::From<vk::PrimitiveTopology> for HalaPrimitiveTopology {
  fn from(val: vk::PrimitiveTopology) -> Self {
    Self(val.as_raw())
  }
}

impl std::convert::From<HalaPrimitiveTopology> for vk::PrimitiveTopology {
  fn from(val: HalaPrimitiveTopology) -> Self {
    vk::PrimitiveTopology::from_raw(val.0)
  }
}

/// The blend factor.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct HalaBlendFactor(i32);
impl HalaBlendFactor {
  pub const ZERO: Self = Self(vk::BlendFactor::ZERO.as_raw());
  pub const ONE: Self = Self(vk::BlendFactor::ONE.as_raw());
  pub const SRC_COLOR: Self = Self(vk::BlendFactor::SRC_COLOR.as_raw());
  pub const ONE_MINUS_SRC_COLOR: Self = Self(vk::BlendFactor::ONE_MINUS_SRC_COLOR.as_raw());
  pub const DST_COLOR: Self = Self(vk::BlendFactor::DST_COLOR.as_raw());
  pub const ONE_MINUS_DST_COLOR: Self = Self(vk::BlendFactor::ONE_MINUS_DST_COLOR.as_raw());
  pub const SRC_ALPHA: Self = Self(vk::BlendFactor::SRC_ALPHA.as_raw());
  pub const ONE_MINUS_SRC_ALPHA: Self = Self(vk::BlendFactor::ONE_MINUS_SRC_ALPHA.as_raw());
  pub const DST_ALPHA: Self = Self(vk::BlendFactor::DST_ALPHA.as_raw());
  pub const ONE_MINUS_DST_ALPHA: Self = Self(vk::BlendFactor::ONE_MINUS_DST_ALPHA.as_raw());
  pub const CONSTANT_COLOR: Self = Self(vk::BlendFactor::CONSTANT_COLOR.as_raw());
  pub const ONE_MINUS_CONSTANT_COLOR: Self = Self(vk::BlendFactor::ONE_MINUS_CONSTANT_COLOR.as_raw());
  pub const CONSTANT_ALPHA: Self = Self(vk::BlendFactor::CONSTANT_ALPHA.as_raw());
  pub const ONE_MINUS_CONSTANT_ALPHA: Self = Self(vk::BlendFactor::ONE_MINUS_CONSTANT_ALPHA.as_raw());
  pub const SRC_ALPHA_SATURATE: Self = Self(vk::BlendFactor::SRC_ALPHA_SATURATE.as_raw());
  pub const SRC1_COLOR: Self = Self(vk::BlendFactor::SRC1_COLOR.as_raw());
  pub const ONE_MINUS_SRC1_COLOR: Self = Self(vk::BlendFactor::ONE_MINUS_SRC1_COLOR.as_raw());
  pub const SRC1_ALPHA: Self = Self(vk::BlendFactor::SRC1_ALPHA.as_raw());
  pub const ONE_MINUS_SRC1_ALPHA: Self = Self(vk::BlendFactor::ONE_MINUS_SRC1_ALPHA.as_raw());
}

impl std::convert::From<vk::BlendFactor> for HalaBlendFactor {
  fn from(val: vk::BlendFactor) -> Self {
    Self(val.as_raw())
  }
}

impl std::convert::From<HalaBlendFactor> for vk::BlendFactor {
  fn from(val: HalaBlendFactor) -> Self {
    vk::BlendFactor::from_raw(val.0)
  }
}

/// The blend operation.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct HalaBlendOp(i32);
impl HalaBlendOp {
  pub const ADD: Self = Self(vk::BlendOp::ADD.as_raw());
  pub const SUBTRACT: Self = Self(vk::BlendOp::SUBTRACT.as_raw());
  pub const REVERSE_SUBTRACT: Self = Self(vk::BlendOp::REVERSE_SUBTRACT.as_raw());
  pub const MIN: Self = Self(vk::BlendOp::MIN.as_raw());
  pub const MAX: Self = Self(vk::BlendOp::MAX.as_raw());
}

impl std::convert::From<vk::BlendOp> for HalaBlendOp {
  fn from(val: vk::BlendOp) -> Self {
    Self(val.as_raw())
  }
}

impl std::convert::From<HalaBlendOp> for vk::BlendOp {
  fn from(val: HalaBlendOp) -> Self {
    vk::BlendOp::from_raw(val.0)
  }
}

/// The front face.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct HalaFrontFace(i32);
impl HalaFrontFace {
  pub const COUNTER_CLOCKWISE: Self = Self(vk::FrontFace::COUNTER_CLOCKWISE.as_raw());
  pub const CLOCKWISE: Self = Self(vk::FrontFace::CLOCKWISE.as_raw());
}

impl std::convert::From<vk::FrontFace> for HalaFrontFace {
  fn from(val: vk::FrontFace) -> Self {
    Self(val.as_raw())
  }
}

impl std::convert::From<HalaFrontFace> for vk::FrontFace {
  fn from(val: HalaFrontFace) -> Self {
    vk::FrontFace::from_raw(val.0)
  }
}

/// The cull mode.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HalaCullModeFlags(u32);
crate::hala_bitflags_wrapped!(HalaCullModeFlags, u32);
impl HalaCullModeFlags {
  pub const NONE: Self = Self(vk::CullModeFlags::NONE.as_raw());
  pub const FRONT: Self = Self(vk::CullModeFlags::FRONT.as_raw());
  pub const BACK: Self = Self(vk::CullModeFlags::BACK.as_raw());
  pub const FRONT_AND_BACK: Self = Self(vk::CullModeFlags::FRONT_AND_BACK.as_raw());
}

impl std::convert::From<vk::CullModeFlags> for HalaCullModeFlags {
  fn from(val: vk::CullModeFlags) -> Self {
    Self(val.as_raw())
  }
}

impl std::convert::From<HalaCullModeFlags> for vk::CullModeFlags {
  fn from(val: HalaCullModeFlags) -> Self {
    vk::CullModeFlags::from_raw(val.0)
  }
}

/// The polygon mode.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct HalaPolygonMode(i32);
impl HalaPolygonMode {
  pub const FILL: Self = Self(vk::PolygonMode::FILL.as_raw());
  pub const LINE: Self = Self(vk::PolygonMode::LINE.as_raw());
  pub const POINT: Self = Self(vk::PolygonMode::POINT.as_raw());
}

impl std::convert::From<vk::PolygonMode> for HalaPolygonMode {
  fn from(val: vk::PolygonMode) -> Self {
    Self(val.as_raw())
  }
}

impl std::convert::From<HalaPolygonMode> for vk::PolygonMode {
  fn from(val: HalaPolygonMode) -> Self {
    vk::PolygonMode::from_raw(val.0)
  }
}

/// The compare operation.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct HalaCompareOp(i32);
impl HalaCompareOp {
  pub const NEVER: Self = Self(vk::CompareOp::NEVER.as_raw());
  pub const LESS: Self = Self(vk::CompareOp::LESS.as_raw());
  pub const EQUAL: Self = Self(vk::CompareOp::EQUAL.as_raw());
  pub const LESS_OR_EQUAL: Self = Self(vk::CompareOp::LESS_OR_EQUAL.as_raw());
  pub const GREATER: Self = Self(vk::CompareOp::GREATER.as_raw());
  pub const NOT_EQUAL: Self = Self(vk::CompareOp::NOT_EQUAL.as_raw());
  pub const GREATER_OR_EQUAL: Self = Self(vk::CompareOp::GREATER_OR_EQUAL.as_raw());
  pub const ALWAYS: Self = Self(vk::CompareOp::ALWAYS.as_raw());
}

impl std::convert::From<vk::CompareOp> for HalaCompareOp {
  fn from(val: vk::CompareOp) -> Self {
    Self(val.as_raw())
  }
}

impl std::convert::From<HalaCompareOp> for vk::CompareOp {
  fn from(val: HalaCompareOp) -> Self {
    vk::CompareOp::from_raw(val.0)
  }
}

/// The stencil face flags.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HalaStencilFaceFlags(u32);
crate::hala_bitflags_wrapped!(HalaStencilFaceFlags, u32);
impl HalaStencilFaceFlags {
  pub const FRONT: Self = Self(vk::StencilFaceFlags::FRONT.as_raw());
  pub const BACK: Self = Self(vk::StencilFaceFlags::BACK.as_raw());
  pub const FRONT_AND_BACK: Self = Self(vk::StencilFaceFlags::FRONT_AND_BACK.as_raw());
}

impl std::convert::From<vk::StencilFaceFlags> for HalaStencilFaceFlags {
  fn from(flags: vk::StencilFaceFlags) -> Self {
    Self(flags.as_raw())
  }
}

impl std::convert::From<HalaStencilFaceFlags> for vk::StencilFaceFlags {
  fn from(flags: HalaStencilFaceFlags) -> Self {
    vk::StencilFaceFlags::from_raw(flags.0)
  }
}

/// The stencil operation.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct HalaStencilOp(i32);
impl HalaStencilOp {
  pub const KEEP: Self = Self(vk::StencilOp::KEEP.as_raw());
  pub const ZERO: Self = Self(vk::StencilOp::ZERO.as_raw());
  pub const REPLACE: Self = Self(vk::StencilOp::REPLACE.as_raw());
  pub const INCREMENT_AND_CLAMP: Self = Self(vk::StencilOp::INCREMENT_AND_CLAMP.as_raw());
  pub const DECREMENT_AND_CLAMP: Self = Self(vk::StencilOp::DECREMENT_AND_CLAMP.as_raw());
  pub const INVERT: Self = Self(vk::StencilOp::INVERT.as_raw());
  pub const INCREMENT_AND_WRAP: Self = Self(vk::StencilOp::INCREMENT_AND_WRAP.as_raw());
  pub const DECREMENT_AND_WRAP: Self = Self(vk::StencilOp::DECREMENT_AND_WRAP.as_raw());
}

impl std::convert::From<vk::StencilOp> for HalaStencilOp {
  fn from(val: vk::StencilOp) -> Self {
    Self(val.as_raw())
  }
}

impl std::convert::From<HalaStencilOp> for vk::StencilOp {
  fn from(val: HalaStencilOp) -> Self {
    vk::StencilOp::from_raw(val.0)
  }
}

/// The vertex input binding description.
#[derive(Copy, Clone, Default)]
pub struct HalaVertexInputAttributeDescription {
  pub location: u32,
  pub binding: u32,
  pub format: HalaFormat,
  pub offset: u32,
}

impl AsRef<HalaVertexInputAttributeDescription> for HalaVertexInputAttributeDescription {
  fn as_ref(&self) -> &Self {
    self
  }
}

impl std::convert::From<vk::VertexInputAttributeDescription> for HalaVertexInputAttributeDescription {
  fn from(val: vk::VertexInputAttributeDescription) -> Self {
    Self::from(&val)
  }
}

impl std::convert::From<&vk::VertexInputAttributeDescription> for HalaVertexInputAttributeDescription {
  fn from(val: &vk::VertexInputAttributeDescription) -> Self {
    Self {
      location: val.location,
      binding: val.binding,
      format: HalaFormat::from(val.format),
      offset: val.offset,
    }
  }
}

impl std::convert::From<HalaVertexInputAttributeDescription> for vk::VertexInputAttributeDescription {
  fn from(val: HalaVertexInputAttributeDescription) -> Self {
    Self::from(&val)
  }
}

impl std::convert::From<&HalaVertexInputAttributeDescription> for vk::VertexInputAttributeDescription {
  fn from(val: &HalaVertexInputAttributeDescription) -> Self {
    Self {
      location: val.location,
      binding: val.binding,
      format: vk::Format::from(val.format),
      offset: val.offset,
    }
  }
}

/// The vertex input binding description.
#[derive(Copy, Clone, Default)]
pub struct HalaVertexInputBindingDescription {
  pub binding: u32,
  pub stride: u32,
  pub input_rate: HalaVertexInputRate,
}

impl AsRef<HalaVertexInputBindingDescription> for HalaVertexInputBindingDescription {
  fn as_ref(&self) -> &Self {
    self
  }
}

impl std::convert::From<vk::VertexInputBindingDescription> for HalaVertexInputBindingDescription {
  fn from(val: vk::VertexInputBindingDescription) -> Self {
    Self::from(&val)
  }
}

impl std::convert::From<&vk::VertexInputBindingDescription> for HalaVertexInputBindingDescription {
  fn from(val: &vk::VertexInputBindingDescription) -> Self {
    Self {
      binding: val.binding,
      stride: val.stride,
      input_rate: HalaVertexInputRate::from(val.input_rate),
    }
  }
}

impl std::convert::From<HalaVertexInputBindingDescription> for vk::VertexInputBindingDescription {
  fn from(val: HalaVertexInputBindingDescription) -> Self {
    Self::from(&val)
  }
}

impl std::convert::From<&HalaVertexInputBindingDescription> for vk::VertexInputBindingDescription {
  fn from(val: &HalaVertexInputBindingDescription) -> Self {
    Self {
      binding: val.binding,
      stride: val.stride,
      input_rate: vk::VertexInputRate::from(val.input_rate),
    }
  }
}

/// The push constant range.
#[derive(Copy, Clone, Default)]
pub struct HalaPushConstantRange {
  pub stage_flags: HalaShaderStageFlags,
  pub offset: u32,
  pub size: u32,
}

impl AsRef<HalaPushConstantRange> for HalaPushConstantRange {
  fn as_ref(&self) -> &Self {
    self
  }
}

impl std::convert::From<vk::PushConstantRange> for HalaPushConstantRange {
  fn from(val: vk::PushConstantRange) -> Self {
    Self::from(&val)
  }
}

impl std::convert::From<&vk::PushConstantRange> for HalaPushConstantRange {
  fn from(val: &vk::PushConstantRange) -> Self {
    Self {
      stage_flags: val.stage_flags.into(),
      offset: val.offset,
      size: val.size,
    }
  }
}

impl std::convert::From<HalaPushConstantRange> for vk::PushConstantRange {
  fn from(val: HalaPushConstantRange) -> Self {
    Self::from(&val)
  }
}

impl std::convert::From<&HalaPushConstantRange> for vk::PushConstantRange {
  fn from(val: &HalaPushConstantRange) -> Self {
    Self {
      stage_flags: val.stage_flags.into(),
      offset: val.offset,
      size: val.size,
    }
  }
}

/// The pipeline base.
pub(crate) struct HalaPipelineBase;
impl HalaPipelineBase {
  /// Create a pipeline layout.
  /// param logical_device: The logical device.
  /// param push_constant_ranges: The push constant ranges.
  /// param descriptor_set_layouts: The descriptor set layouts.
  /// param debug_name: The debug name.
  /// return: The pipeline layout.
  pub(crate) fn create_pipeline_layout<PCR, DSL>(
    logical_device: &Rc<RefCell<HalaLogicalDevice>>,
    push_constant_ranges: &[PCR],
    descriptor_set_layouts: &[DSL],
    debug_name: &str,
  ) -> Result<vk::PipelineLayout, HalaGfxError>
    where PCR: AsRef<HalaPushConstantRange>,
          DSL: AsRef<HalaDescriptorSetLayout>
  {
    let ranges = push_constant_ranges
      .iter()
      .map(|pcr| pcr.as_ref().into())
      .collect::<Vec<_>>();

    let layouts = descriptor_set_layouts
      .iter()
      .map(|dsl| dsl.as_ref().raw)
      .collect::<Vec<_>>();

    let pipeline_layout_info = vk::PipelineLayoutCreateInfo::default()
      .push_constant_ranges(ranges.as_slice())
      .set_layouts(layouts.as_slice());

    let pipeline_layout = unsafe {
      logical_device.borrow().raw.create_pipeline_layout(&pipeline_layout_info, None)
        .map_err(|err| HalaGfxError::new("Failed to create pipeline layout", Some(Box::new(err))))?
    };
    logical_device.borrow().set_debug_name(
      pipeline_layout,
      &format!("{}_layout", debug_name),
    ).map_err(|err| HalaGfxError::new("Failed to set debug name for pipeline layout.", Some(Box::new(err))))?;

    Ok(pipeline_layout)
  }
}

/// The graphics pipeline.
pub struct HalaGraphicsPipeline {
  pub(crate) logical_device: Rc<RefCell<HalaLogicalDevice>>,
  pub raw: vk::Pipeline,
  pub layout: vk::PipelineLayout,

  pub(crate) debug_name: String,
}

/// The Drop trait implementation for graphics pipeline.
impl Drop for HalaGraphicsPipeline {
  fn drop(&mut self) {
    unsafe {
      self.logical_device.borrow().raw.destroy_pipeline(self.raw, None);
      self.logical_device.borrow().raw.destroy_pipeline_layout(self.layout, None);
    }
    log::debug!("A HalaGraphicsPipeline \"{}\" is dropped.", self.debug_name);
  }
}

/// The implementation of graphics pipeline.
/// param logical_device: The logical device.
/// param swapchain: The swapchain.
/// param descriptor_set_layouts: The descriptor set layouts.
/// param vertex_attribute_descriptions: The vertex attribute descriptions.
/// param vertex_binding_descriptions: The vertex binding descriptions.
/// param push_constant_ranges: The push constant ranges.
/// param primitive_topology: The primitive topology.
/// param color_blend: The color blend(source, destination, operation).
/// param alpha_blend: The alpha blend(source, destination, operation).
/// param rasterizer_info: The rasterizer info(line width, front face, cull mode, polygon mode)
/// param depth_info: The depth info(test enable, write enable, compare operation).
/// param shaders: The shaders.
/// param renderpass: The renderpass.
/// param pipeline_cache: The pipeline cache.
/// param debug_name: The debug name.
/// return: The graphics pipeline.
#[allow(clippy::too_many_arguments)]
impl HalaGraphicsPipeline {
  pub fn new<DSL, VIAD, VIBD, PCR>(
    logical_device: Rc<RefCell<HalaLogicalDevice>>,
    swapchain: &HalaSwapchain,
    descriptor_set_layouts: &[DSL],
    vertex_attribute_descriptions: &[VIAD],
    vertex_binding_descriptions: &[VIBD],
    push_constant_ranges: &[PCR],
    primitive_topology: HalaPrimitiveTopology,
    color_blend: (HalaBlendFactor, HalaBlendFactor, HalaBlendOp),
    alpha_blend: (HalaBlendFactor, HalaBlendFactor, HalaBlendOp),
    rasterizer_info: (f32, HalaFrontFace, HalaCullModeFlags, HalaPolygonMode),
    depth_info: (bool, bool, HalaCompareOp),
    shaders: &[HalaShader],
    pipeline_cache: Option<&HalaPipelineCache>,
    debug_name: &str,
  ) -> Result<Self, HalaGfxError>
    where DSL: AsRef<HalaDescriptorSetLayout>,
          VIAD: AsRef<HalaVertexInputAttributeDescription>,
          VIBD: AsRef<HalaVertexInputBindingDescription>,
          PCR: AsRef<HalaPushConstantRange>
  {
    let pipeline_layout = HalaPipelineBase::create_pipeline_layout(
      &logical_device,
      push_constant_ranges,
      descriptor_set_layouts,
      debug_name
    )?;

    let graphics_pipeline = Self::create_pipeline(
      &logical_device,
      swapchain,
      vertex_attribute_descriptions,
      vertex_binding_descriptions,
      primitive_topology,
      color_blend,
      alpha_blend,
      rasterizer_info,
      depth_info,
      shaders,
      pipeline_cache,
      pipeline_layout,
      debug_name
    )?;

    log::debug!("A HalaGraphicsPipeline \"{}\" is created.", debug_name);
    Ok(
      Self {
        logical_device,
        raw: graphics_pipeline,
        layout: pipeline_layout,
        debug_name: debug_name.to_string(),
      }
    )
  }

  /// Create a graphics pipeline.
  /// param logical_device: The logical device.
  /// param swapchain: The swapchain.
  /// param vertex_attribute_descriptions: The vertex attribute descriptions.
  /// param vertex_binding_descriptions: The vertex binding descriptions.
  /// param primitive_topology: The primitive topology.
  /// param color_blend: The color blend(source, destination, operation).
  /// param alpha_blend: The alpha blend(source, destination, operation).
  /// param rasterizer_info: The rasterizer info(line width, front face, cull mode, polygon mode)
  /// param depth_info: The depth info(test enable, write enable, compare operation).
  /// param shaders: The shaders.
  /// param renderpass: The renderpass.
  /// param pipeline_cache: The pipeline cache.
  /// param pipeline_layout: The pipeline layout.
  /// param debug_name: The debug name.
  /// return: The graphics pipeline.
  fn create_pipeline<VIAD, VIBD, S>(
    logical_device: &Rc<RefCell<HalaLogicalDevice>>,
    swapchain: &HalaSwapchain,
    vertex_attribute_descriptions: &[VIAD],
    vertex_binding_descriptions: &[VIBD],
    primitive_topology: HalaPrimitiveTopology,
    color_blend: (HalaBlendFactor, HalaBlendFactor, HalaBlendOp),
    alpha_blend: (HalaBlendFactor, HalaBlendFactor, HalaBlendOp),
    rasterizer_info: (f32, HalaFrontFace, HalaCullModeFlags, HalaPolygonMode),
    depth_info: (bool, bool, HalaCompareOp),
    shaders: &[S],
    pipeline_cache: Option<&HalaPipelineCache>,
    pipeline_layout: vk::PipelineLayout,
    debug_name: &str,
  ) -> Result<vk::Pipeline, HalaGfxError>
    where VIAD: AsRef<HalaVertexInputAttributeDescription>,
          VIBD: AsRef<HalaVertexInputBindingDescription>,
          S: AsRef<HalaShader>
  {
    let vertex_attribute_descriptions: Vec<vk::VertexInputAttributeDescription> = vertex_attribute_descriptions
      .iter()
      .map(|v| v.as_ref().into())
      .collect();
    let vertex_binding_descriptions: Vec<vk::VertexInputBindingDescription> = vertex_binding_descriptions
      .iter()
      .map(|v| v.as_ref().into())
      .collect();
    let vertex_input_info = vk::PipelineVertexInputStateCreateInfo::default()
      .vertex_attribute_descriptions(&vertex_attribute_descriptions)
      .vertex_binding_descriptions(&vertex_binding_descriptions);
    let input_assembly_info = vk::PipelineInputAssemblyStateCreateInfo::default()
      .topology(primitive_topology.into());

    let viewports = [vk::Viewport {
      x: 0.,
      y: 0.,
      width: swapchain.desc.dims.width as f32,
      height: swapchain.desc.dims.height as f32,
      min_depth: 0.,
      max_depth: 1.,
    }];
    let scissors = [vk::Rect2D {
      offset: vk::Offset2D { x: 0, y: 0 },
      extent: swapchain.desc.dims,
    }];
    let viewport_info = vk::PipelineViewportStateCreateInfo::default()
      .viewports(&viewports)
      .scissors(&scissors);

    let rasterizer_info = vk::PipelineRasterizationStateCreateInfo::default()
      .line_width(rasterizer_info.0)
      .front_face(rasterizer_info.1.into())
      .cull_mode(rasterizer_info.2.into())
      .polygon_mode(rasterizer_info.3.into());

    let multisampler_info = vk::PipelineMultisampleStateCreateInfo::default()
      .rasterization_samples(vk::SampleCountFlags::TYPE_1);

    let colourblend_attachments = [vk::PipelineColorBlendAttachmentState::default()
      .blend_enable(true)
      .src_color_blend_factor(color_blend.0.into())
      .dst_color_blend_factor(color_blend.1.into())
      .color_blend_op(color_blend.2.into())
      .src_alpha_blend_factor(alpha_blend.0.into())
      .dst_alpha_blend_factor(alpha_blend.1.into())
      .alpha_blend_op(alpha_blend.2.into())
      .color_write_mask(
        vk::ColorComponentFlags::R | vk::ColorComponentFlags::G | vk::ColorComponentFlags::B | vk::ColorComponentFlags::A,
      )];
    let colourblend_info =
      vk::PipelineColorBlendStateCreateInfo::default().attachments(&colourblend_attachments);

    let main_func_name = std::ffi::CString::new("main")
      .map_err(|err| HalaGfxError::new("Failed to create \"main\" CString.", Some(Box::new(err))))?;
    let shader_stage_infos = shaders
      .iter()
      .map(|shader| {
        let shader = shader.as_ref();
        vk::PipelineShaderStageCreateInfo::default()
          .stage(shader.stage_flags.into())
          .module(shader.module)
          .name(&main_func_name)
      })
      .collect::<Vec<_>>();

    let rendering_info = vk::PipelineRenderingCreateInfo::default()
      .color_attachment_formats(std::slice::from_ref(&swapchain.desc.format));
    let rendering_info = if swapchain.depth_stencil_format != vk::Format::UNDEFINED {
      rendering_info.depth_attachment_format(swapchain.depth_stencil_format)
    } else {
      rendering_info
    };
    let mut rendering_info = if swapchain.has_stencil {
      rendering_info.stencil_attachment_format(swapchain.depth_stencil_format)
    } else {
      rendering_info
    };

    // let mut dynamic_states = vec![vk::DynamicState::VIEWPORT, vk::DynamicState::SCISSOR];
    // if swapchain.depth_stencil_format != vk::Format::UNDEFINED {
    //   dynamic_states.push(vk::DynamicState::DEPTH_BIAS);
    //   dynamic_states.push(vk::DynamicState::DEPTH_BIAS_ENABLE);
    //   dynamic_states.push(vk::DynamicState::DEPTH_TEST_ENABLE_EXT);
    //   dynamic_states.push(vk::DynamicState::DEPTH_WRITE_ENABLE_EXT);
    //   dynamic_states.push(vk::DynamicState::DEPTH_COMPARE_OP_EXT);
    // }
    // if swapchain.has_stencil {
    //   dynamic_states.push(vk::DynamicState::STENCIL_TEST_ENABLE_EXT);
    //   dynamic_states.push(vk::DynamicState::STENCIL_OP_EXT);
    //   dynamic_states.push(vk::DynamicState::STENCIL_WRITE_MASK);
    //   dynamic_states.push(vk::DynamicState::STENCIL_COMPARE_MASK);
    //   dynamic_states.push(vk::DynamicState::STENCIL_REFERENCE);
    // }
    // let dynamic_state_info = vk::PipelineDynamicStateCreateInfo::default()
    //   .dynamic_states(dynamic_states.as_slice());

    let pipeline_info = vk::GraphicsPipelineCreateInfo::default()
      .stages(shader_stage_infos.as_slice())
      .vertex_input_state(&vertex_input_info)
      .input_assembly_state(&input_assembly_info)
      .viewport_state(&viewport_info)
      .rasterization_state(&rasterizer_info)
      .multisample_state(&multisampler_info)
      .color_blend_state(&colourblend_info)
      // .dynamic_state(&dynamic_state_info)
      .layout(pipeline_layout)
      // .render_pass(renderpass.raw)
      .push_next(&mut rendering_info)
      .subpass(0);

    let graphics_pipeline = if swapchain.depth_stencil_format != vk::Format::UNDEFINED {
      let depth_stencil_info = vk::PipelineDepthStencilStateCreateInfo::default()
        .depth_test_enable(depth_info.0)
        .depth_write_enable(depth_info.1)
        .depth_compare_op(depth_info.2.into())
        .depth_bounds_test_enable(false)
        .stencil_test_enable(false)
        .front(Default::default())
        .back(Default::default());
      let pipelines = unsafe {
        logical_device.borrow().raw
          .create_graphics_pipelines(
            pipeline_cache.map_or(vk::PipelineCache::null(), |pc| pc.raw),
            &[pipeline_info.depth_stencil_state(&depth_stencil_info)],
            None,
          )
          .map_err(|err| HalaGfxError::new("Failed to create graphics pipeline", Some(Box::new(err.1))))?
      };
      pipelines.into_iter().next().ok_or(HalaGfxError::new("Failed to create graphics pipeline", None))?
    } else {
      let pipelines = unsafe {
        logical_device.borrow().raw
          .create_graphics_pipelines(
            pipeline_cache.map_or(vk::PipelineCache::null(), |pc| pc.raw),
            &[pipeline_info],
            None,
          )
          .map_err(|err| HalaGfxError::new("Failed to create graphics pipeline", Some(Box::new(err.1))))?
      };
      pipelines.into_iter().next().ok_or(HalaGfxError::new("Failed to create graphics pipeline", None))?
    };

    logical_device.borrow().set_debug_name(
      graphics_pipeline,
      debug_name,
    ).map_err(|err| HalaGfxError::new("Failed to set debug name for graphics pipeline.", Some(Box::new(err))))?;

    Ok(graphics_pipeline)
  }
}


/// The ray tracing pipeline.
pub struct HalaRayTracingPipeline {
  pub(crate) logical_device: Rc<RefCell<HalaLogicalDevice>>,
  pub raw: vk::Pipeline,
  pub layout: vk::PipelineLayout,

  pub(crate) debug_name: String,
}

/// The Drop trait implementation for ray tracing pipeline.
impl Drop for HalaRayTracingPipeline {
  fn drop(&mut self) {
    unsafe {
      let logical_device = self.logical_device.borrow();
      logical_device.raw.destroy_pipeline_layout(self.layout, None);
      logical_device.raw.destroy_pipeline(self.raw, None);
    }
    log::debug!("A HalaRayTracingPipeline \"{}\" is dropped.", self.debug_name);
  }
}

/// The implementation of ray tracing pipeline.
impl HalaRayTracingPipeline {
  /// Create a ray tracing pipeline.
  /// param logical_device: The logical device.
  /// param descriptor_set_layouts: The descriptor set layouts.
  /// param raygen_shaders: The ray generation shaders.
  /// param miss_shaders: The miss shaders.
  /// param hit_shaders: The hit shaders.
  /// param callable_shaders: The callable shaders.
  /// param max_pipeline_ray_recursion_depth: The max pipeline ray recursion depth.
  /// param pipeline_cache: The pipeline cache.
  /// param is_dynamic_stack: The flag to indicate whether the stack is dynamic.
  /// param debug_name: The debug name.
  /// return: The ray tracing pipeline.
  #[allow(clippy::too_many_arguments)]
  pub fn new<DSL, S>(
    logical_device: Rc<RefCell<HalaLogicalDevice>>,
    descriptor_set_layouts: &[DSL],
    raygen_shaders: &[S],
    miss_shaders: &[S],
    hit_shaders: &[(Option<S>, Option<S>, Option<S>)],
    callable_shaders: &[S],
    max_pipeline_ray_recursion_depth: u32,
    pipeline_cache: Option<&HalaPipelineCache>,
    is_dynamic_stack: bool,
    debug_name: &str,
  ) -> Result<HalaRayTracingPipeline, HalaGfxError>
    where DSL: AsRef<HalaDescriptorSetLayout>,
          S: AsRef<HalaShader>
  {
    // Create the pipeline layout.
    let pipeline_layout = HalaPipelineBase::create_pipeline_layout::<HalaPushConstantRange, _>(
      &logical_device,
      &[],
      descriptor_set_layouts,
      debug_name)?;

    // Create the pipeline.
    let pipeline = Self::create_pipeline(
      &logical_device,
      raygen_shaders,
      miss_shaders,
      hit_shaders,
      callable_shaders,
      max_pipeline_ray_recursion_depth,
      pipeline_cache,
      pipeline_layout,
      is_dynamic_stack,
      debug_name)?;

    log::debug!("A HalaRayTracingPipeline \"{}\" is created.", debug_name);
    Ok(
      Self {
        logical_device,
        raw: pipeline,
        layout: pipeline_layout,
        debug_name: debug_name.to_string(),
      }
    )
  }

  // Create a ray tracing pipeline.
  /// param logical_device: The logical device.
  /// param raygen_shaders: The ray generation shaders.
  /// param miss_shaders: The miss shaders.
  /// param hit_shaders: The hit shaders.
  /// param callable_shaders: The callable shaders.
  /// param max_pipeline_ray_recursion_depth: The max pipeline ray recursion depth.
  /// param pipeline_cache: The pipeline cache.
  /// param pipeline_layout: The pipeline layout.
  /// param is_dynamic_stack: The flag to indicate whether the stack is dynamic.
  /// param debug_name: The debug name.
  /// return: The ray tracing pipeline.
  #[allow(clippy::too_many_arguments)]
  fn create_pipeline<S>(
    logical_device: &Rc<RefCell<HalaLogicalDevice>>,
    raygen_shaders: &[S],
    miss_shaders: &[S],
    hit_shaders: &[(Option<S>, Option<S>, Option<S>)],
    callable_shaders: &[S],
    max_pipeline_ray_recursion_depth: u32,
    pipeline_cache: Option<&HalaPipelineCache>,
    pipeline_layout: vk::PipelineLayout,
    is_dynamic_stack: bool,
    debug_name: &str
  ) -> Result<vk::Pipeline, HalaGfxError>
    where S: AsRef<HalaShader>
  {
    let mut stages = Vec::new();
    let mut groups = Vec::new();

    let main_func_name = std::ffi::CString::new("main")
      .map_err(|err| HalaGfxError::new("Failed to create \"main\" CString.", Some(Box::new(err))))?;
    let mut shader_index = 0u32;

    // Create the shader stages and groups for raygen shaders.
    for shader in raygen_shaders.iter() {
      let shader_stage_info = vk::PipelineShaderStageCreateInfo::default()
        .stage(shader.as_ref().stage_flags.into())
        .module(shader.as_ref().module)
        .name(&main_func_name);
      stages.push(shader_stage_info);

      let group = vk::RayTracingShaderGroupCreateInfoKHR::default()
        .ty(shader.as_ref().ray_tracing_group_type.into())
        .general_shader(shader_index)
        .closest_hit_shader(vk::SHADER_UNUSED_KHR)
        .any_hit_shader(vk::SHADER_UNUSED_KHR)
        .intersection_shader(vk::SHADER_UNUSED_KHR);

      shader_index += 1;

      groups.push(group);
    }

    // Create the shader stages and groups for miss shaders.
    for shader in miss_shaders.iter() {
      let shader_stage_info = vk::PipelineShaderStageCreateInfo::default()
        .stage(shader.as_ref().stage_flags.into())
        .module(shader.as_ref().module)
        .name(&main_func_name);
      stages.push(shader_stage_info);

      let group = vk::RayTracingShaderGroupCreateInfoKHR::default()
        .ty(shader.as_ref().ray_tracing_group_type.into())
        .general_shader(shader_index)
        .closest_hit_shader(vk::SHADER_UNUSED_KHR)
        .any_hit_shader(vk::SHADER_UNUSED_KHR)
        .intersection_shader(vk::SHADER_UNUSED_KHR);

      shader_index += 1;

      groups.push(group);
    }

    // Create the shader stages and groups for hit shaders.
    for (closest_hit_shader, any_hit_shader, intersection_shader) in hit_shaders.iter() {
      // closest_hit_shader, any_hit_shader and intersection_shader can not be all None.
      if closest_hit_shader.is_none() && any_hit_shader.is_none() && intersection_shader.is_none() {
        return Err(HalaGfxError::new("The closest_hit_shader, any_hit_shader and intersection_shader can not be all None.", None));
      }

      let mut group = vk::RayTracingShaderGroupCreateInfoKHR::default()
        .ty(if intersection_shader.is_none() { vk::RayTracingShaderGroupTypeKHR::TRIANGLES_HIT_GROUP } else { vk::RayTracingShaderGroupTypeKHR::PROCEDURAL_HIT_GROUP })
        .general_shader(vk::SHADER_UNUSED_KHR)
        .closest_hit_shader(vk::SHADER_UNUSED_KHR)
        .any_hit_shader(vk::SHADER_UNUSED_KHR)
        .intersection_shader(vk::SHADER_UNUSED_KHR);

      group = if let Some(closest_hit_shader) = closest_hit_shader {
        let closest_hit_shader_stage_info = vk::PipelineShaderStageCreateInfo::default()
          .stage(closest_hit_shader.as_ref().stage_flags.into())
          .module(closest_hit_shader.as_ref().module)
          .name(&main_func_name);
        stages.push(closest_hit_shader_stage_info);

        shader_index += 1;
        group.closest_hit_shader(shader_index - 1)
      } else {
        group
      };

      group = if let Some(any_hit_shader) = any_hit_shader {
        let any_hit_shader_stage_info = vk::PipelineShaderStageCreateInfo::default()
          .stage(any_hit_shader.as_ref().stage_flags.into())
          .module(any_hit_shader.as_ref().module)
          .name(&main_func_name);
        stages.push(any_hit_shader_stage_info);

        shader_index += 1;
        group.any_hit_shader(shader_index - 1)
      } else {
        group
      };

      group = if let Some(intersection_shader) = intersection_shader {
        let intersection_shader_stage_info = vk::PipelineShaderStageCreateInfo::default()
          .stage(intersection_shader.as_ref().stage_flags.into())
          .module(intersection_shader.as_ref().module)
          .name(&main_func_name);
        stages.push(intersection_shader_stage_info);

        shader_index += 1;
        group.intersection_shader(shader_index - 1)
      } else {
        group
      };

      groups.push(group);
    }

    // Create the shader stages and groups for callable shaders.
    for shader in callable_shaders.iter() {
      let shader_stage_info = vk::PipelineShaderStageCreateInfo::default()
        .stage(shader.as_ref().stage_flags.into())
        .module(shader.as_ref().module)
        .name(&main_func_name);
      stages.push(shader_stage_info);

      let group = vk::RayTracingShaderGroupCreateInfoKHR::default()
        .ty(shader.as_ref().ray_tracing_group_type.into())
        .general_shader(shader_index)
        .closest_hit_shader(vk::SHADER_UNUSED_KHR)
        .any_hit_shader(vk::SHADER_UNUSED_KHR)
        .intersection_shader(vk::SHADER_UNUSED_KHR);

      shader_index += 1;

      groups.push(group);
    }

    let max_ray_recursion_depth = logical_device.borrow().max_ray_recursion_depth;
    let max_pipeline_ray_recursion_depth = if max_pipeline_ray_recursion_depth > max_ray_recursion_depth {
      log::warn!(
        "The max_pipeline_ray_recursion_depth({}) is greater than max_ray_recursion_depth({}), Use max_ray_recursion_depth instead.",
        max_pipeline_ray_recursion_depth,
        max_ray_recursion_depth
      );
      max_ray_recursion_depth
    } else {
      max_pipeline_ray_recursion_depth
    };
    let dynamic_state_info = vk::PipelineDynamicStateCreateInfo::default()
      .dynamic_states(&[vk::DynamicState::RAY_TRACING_PIPELINE_STACK_SIZE_KHR]);
    let pipeline_info = vk::RayTracingPipelineCreateInfoKHR::default()
      .stages(stages.as_slice())
      .groups(groups.as_slice())
      .max_pipeline_ray_recursion_depth(max_pipeline_ray_recursion_depth)
      .layout(pipeline_layout);
    let pipeline_info = if is_dynamic_stack {
      pipeline_info.dynamic_state(&dynamic_state_info)
    } else {
      pipeline_info
    };

    let pipeline= unsafe {
      let pipelines = logical_device.borrow().ray_tracing_pipeline_loader.create_ray_tracing_pipelines(
        vk::DeferredOperationKHR::null(),
        pipeline_cache.map_or(vk::PipelineCache::null(), |pc| pc.raw),
        std::slice::from_ref(&pipeline_info),
        None,
      ).map_err(|_| HalaGfxError::new("Failed to create ray tracing pipeline", None))?;
      pipelines.into_iter().next().ok_or(HalaGfxError::new("Failed to create ray tracing pipeline", None))?
    };
    logical_device.borrow().set_debug_name(
      pipeline,
      debug_name,
    ).map_err(|err| HalaGfxError::new("Failed to set debug name for pipeline.", Some(Box::new(err))))?;

    Ok(pipeline)
  }
}

/// The compute pipeline.
pub struct HalaComputePipeline {
  pub(crate) logical_device: Rc<RefCell<HalaLogicalDevice>>,
  pub raw: vk::Pipeline,
  pub layout: vk::PipelineLayout,

  pub(crate) debug_name: String,
}

/// The Drop trait implementation for compute pipeline.
impl Drop for HalaComputePipeline {
  fn drop(&mut self) {
    unsafe {
      self.logical_device.borrow().raw.destroy_pipeline_layout(self.layout, None);
      self.logical_device.borrow().raw.destroy_pipeline(self.raw, None);
    }
    log::debug!("A HalaComputePipeline \"{}\" is dropped.", self.debug_name);
  }
}

/// The implementation of compute pipeline.
impl HalaComputePipeline {
  /// Create a compute pipeline.
  /// param logical_device: The logical device.
  /// param descriptor_set_layouts: The descriptor set layouts.
  /// param shader: The shader.
  /// param pipeline_cache: The pipeline cache.
  /// param debug_name: The debug name.
  /// return: The compute pipeline.
  pub fn new<DSL>(
    logical_device: Rc<RefCell<HalaLogicalDevice>>,
    descriptor_set_layouts: &[DSL],
    shader: &HalaShader,
    pipeline_cache: Option<&HalaPipelineCache>,
    debug_name: &str,
  ) -> Result<Self, HalaGfxError>
    where DSL: AsRef<HalaDescriptorSetLayout>
  {
    // Create the pipeline layout.
    let pipeline_layout = HalaPipelineBase::create_pipeline_layout::<HalaPushConstantRange, _>(
      &logical_device,
      &[],
      descriptor_set_layouts,
      debug_name)?;

    // Create the pipeline.
    let pipeline = Self::create_pipeline(
      &logical_device,
      shader,
      pipeline_cache,
      pipeline_layout,
      debug_name)?;

    log::debug!("A HalaComputePipeline \"{}\" is created.", debug_name);
    Ok(
      Self {
        logical_device,
        raw: pipeline,
        layout: pipeline_layout,
        debug_name: debug_name.to_string(),
      }
    )
  }

  /// Create a compute pipeline.
  /// param logical_device: The logical device.
  /// param shader: The shader.
  /// param pipeline_cache: The pipeline cache.
  /// param pipeline_layout: The pipeline layout.
  /// param debug_name: The debug name.
  /// return: The compute pipeline.
  fn create_pipeline(
    logical_device: &Rc<RefCell<HalaLogicalDevice>>,
    shader: &HalaShader,
    pipeline_cache: Option<&HalaPipelineCache>,
    pipeline_layout: vk::PipelineLayout,
    debug_name: &str
  ) -> Result<vk::Pipeline, HalaGfxError> {
    let main_func_name = std::ffi::CString::new("main")
      .map_err(|err| HalaGfxError::new("Failed to create \"main\" CString.", Some(Box::new(err))))?;
    let shader_stage_info = vk::PipelineShaderStageCreateInfo::default()
      .stage(shader.stage_flags.into())
      .module(shader.module)
      .name(&main_func_name);
    let pipeline_info = vk::ComputePipelineCreateInfo::default()
      .stage(shader_stage_info)
      .layout(pipeline_layout);

    let pipeline = unsafe {
      let pipelines = logical_device.borrow().raw
        .create_compute_pipelines(
          pipeline_cache.map_or(vk::PipelineCache::null(), |pc| pc.raw),
          std::slice::from_ref(&pipeline_info),
          None,
        )
        .map_err(|err| HalaGfxError::new("Failed to create compute pipeline", Some(Box::new(err.1))))?;
      pipelines.into_iter().next().ok_or(HalaGfxError::new("Failed to create compute pipeline", None))?
    };
    logical_device.borrow().set_debug_name(
      pipeline,
      debug_name,
    ).map_err(|err| HalaGfxError::new("Failed to set debug name for pipeline.", Some(Box::new(err))))?;

    Ok(pipeline)
  }
}