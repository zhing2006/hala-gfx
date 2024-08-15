use std::rc::Rc;
use std::cell::RefCell;
use std::fmt;

use serde::{Serialize, Deserialize, Serializer, Deserializer};
use serde::de::{self, Unexpected, Visitor};

use ash::vk;

use crate::{
  HalaDescriptorSetLayout,
  HalaFormat,
  HalaGfxError,
  HalaImage,
  HalaLogicalDevice,
  HalaPipelineCache,
  HalaRenderPass,
  HalaShader,
  HalaShaderStageFlags,
  HalaSwapchain,
  HalaSampleCountFlags,
};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HalaPipelineCreateFlags(u32);
crate::hala_bitflags_wrapped!(HalaPipelineCreateFlags, u32);
impl HalaPipelineCreateFlags {
  pub const DISABLE_OPTIMIZATION: Self = Self(vk::PipelineCreateFlags::DISABLE_OPTIMIZATION.as_raw());
  pub const ALLOW_DERIVATIVES: Self = Self(vk::PipelineCreateFlags::ALLOW_DERIVATIVES.as_raw());
  pub const DERIVATIVE: Self = Self(vk::PipelineCreateFlags::DERIVATIVE.as_raw());
  pub const VIEW_INDEX_FROM_DEVICE_INDEX: Self = Self(vk::PipelineCreateFlags::VIEW_INDEX_FROM_DEVICE_INDEX.as_raw());
  pub const DISPATCH_BASE: Self = Self(vk::PipelineCreateFlags::DISPATCH_BASE.as_raw());
  pub const FAIL_ON_PIPELINE_COMPILE_REQUIRED: Self = Self(vk::PipelineCreateFlags::FAIL_ON_PIPELINE_COMPILE_REQUIRED.as_raw());
  pub const EARLY_RETURN_ON_FAILURE: Self = Self(vk::PipelineCreateFlags::EARLY_RETURN_ON_FAILURE.as_raw());
  pub const RENDERING_FRAGMENT_SHADING_RATE_ATTACHMENT: Self = Self(vk::PipelineCreateFlags::RENDERING_FRAGMENT_SHADING_RATE_ATTACHMENT_KHR.as_raw());
  pub const RENDERING_FRAGMENT_DENSITY_MAP_ATTACHMENT: Self = Self(vk::PipelineCreateFlags::RENDERING_FRAGMENT_DENSITY_MAP_ATTACHMENT_EXT.as_raw());
  pub const RAY_TRACING_NO_NULL_ANY_HIT_SHADERS: Self = Self(vk::PipelineCreateFlags::RAY_TRACING_NO_NULL_ANY_HIT_SHADERS_KHR.as_raw());
  pub const RAY_TRACING_NO_NULL_CLOSEST_HIT_SHADERS: Self = Self(vk::PipelineCreateFlags::RAY_TRACING_NO_NULL_CLOSEST_HIT_SHADERS_KHR.as_raw());
  pub const RAY_TRACING_NO_NULL_MISS_SHADERS: Self = Self(vk::PipelineCreateFlags::RAY_TRACING_NO_NULL_MISS_SHADERS_KHR.as_raw());
  pub const RAY_TRACING_NO_NULL_INTERSECTION_SHADERS: Self = Self(vk::PipelineCreateFlags::RAY_TRACING_NO_NULL_INTERSECTION_SHADERS_KHR.as_raw());
  pub const RAY_TRACING_SKIP_TRIANGLES: Self = Self(vk::PipelineCreateFlags::RAY_TRACING_SKIP_TRIANGLES_KHR.as_raw());
  pub const RAY_TRACING_SKIP_AABBS: Self = Self(vk::PipelineCreateFlags::RAY_TRACING_SKIP_AABBS_KHR.as_raw());
  pub const RAY_TRACING_SHADER_GROUP_HANDLE_CAPTURE_REPLAY: Self = Self(vk::PipelineCreateFlags::RAY_TRACING_SHADER_GROUP_HANDLE_CAPTURE_REPLAY_KHR.as_raw());
  pub const DEFER_COMPILE: Self = Self(vk::PipelineCreateFlags::DEFER_COMPILE_NV.as_raw());
  pub const CAPTURE_STATISTICS: Self = Self(vk::PipelineCreateFlags::CAPTURE_STATISTICS_KHR.as_raw());
  pub const CAPTURE_INTERNAL_REPRESENTATIONS: Self = Self(vk::PipelineCreateFlags::CAPTURE_INTERNAL_REPRESENTATIONS_KHR.as_raw());
  pub const INDIRECT_BINDABLE: Self = Self(vk::PipelineCreateFlags::INDIRECT_BINDABLE_NV.as_raw());
  pub const LIBRARY: Self = Self(vk::PipelineCreateFlags::LIBRARY_KHR.as_raw());
  pub const DESCRIPTOR_BUFFER: Self = Self(vk::PipelineCreateFlags::DESCRIPTOR_BUFFER_EXT.as_raw());
  pub const RETAIN_LINK_TIME_OPTIMIZATION_INFO: Self = Self(vk::PipelineCreateFlags::RETAIN_LINK_TIME_OPTIMIZATION_INFO_EXT.as_raw());
  pub const LINK_TIME_OPTIMIZATION: Self = Self(vk::PipelineCreateFlags::LINK_TIME_OPTIMIZATION_EXT.as_raw());
  pub const RAY_TRACING_ALLOW_MOTION: Self = Self(vk::PipelineCreateFlags::RAY_TRACING_ALLOW_MOTION_NV.as_raw());
  pub const COLOR_ATTACHMENT_FEEDBACK_LOOP: Self = Self(vk::PipelineCreateFlags::COLOR_ATTACHMENT_FEEDBACK_LOOP_EXT.as_raw());
  pub const DEPTH_STENCIL_ATTACHMENT_FEEDBACK_LOOP: Self = Self(vk::PipelineCreateFlags::DEPTH_STENCIL_ATTACHMENT_FEEDBACK_LOOP_EXT.as_raw());
  pub const RAY_TRACING_OPACITY_MICROMAP: Self = Self(vk::PipelineCreateFlags::RAY_TRACING_OPACITY_MICROMAP_EXT.as_raw());
  pub const RAY_TRACING_DISPLACEMENT_MICROMAP: Self = Self(vk::PipelineCreateFlags::RAY_TRACING_DISPLACEMENT_MICROMAP_NV.as_raw());
  pub const NO_PROTECTED_ACCESS: Self = Self(vk::PipelineCreateFlags::NO_PROTECTED_ACCESS_EXT.as_raw());
  pub const PROTECTED_ACCESS_ONLY: Self = Self(vk::PipelineCreateFlags::PROTECTED_ACCESS_ONLY_EXT.as_raw());
  pub const RASTERIZATION_STATE_CREATE_FRAGMENT_SHADING_RATE_ATTACHMENT: Self = Self::RENDERING_FRAGMENT_SHADING_RATE_ATTACHMENT;
  pub const RASTERIZATION_STATE_CREATE_FRAGMENT_DENSITY_MAP_ATTACHMENT: Self = Self::RENDERING_FRAGMENT_DENSITY_MAP_ATTACHMENT;
}

impl std::convert::From<vk::PipelineCreateFlags> for HalaPipelineCreateFlags {
  fn from(flags: vk::PipelineCreateFlags) -> Self {
    Self(flags.as_raw())
  }
}

impl std::convert::From<HalaPipelineCreateFlags> for vk::PipelineCreateFlags {
  fn from(flags: HalaPipelineCreateFlags) -> Self {
    vk::PipelineCreateFlags::from_raw(flags.0)
  }
}

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

impl Serialize for HalaPrimitiveTopology {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let s = match *self {
      HalaPrimitiveTopology::POINT_LIST => "point_list",
      HalaPrimitiveTopology::LINE_LIST => "list_list",
      HalaPrimitiveTopology::LINE_STRIP => "line_strip",
      HalaPrimitiveTopology::TRIANGLE_LIST => "triangle_list",
      HalaPrimitiveTopology::TRIANGLE_STRIP => "triangle_strip",
      HalaPrimitiveTopology::TRIANGLE_FAN => "triangle_fan",
      HalaPrimitiveTopology::LINE_LIST_WITH_ADJACENCY => "line_list_with_adjacency",
      HalaPrimitiveTopology::LINE_STRIP_WITH_ADJACENCY => "line_strip_with_adjacency",
      HalaPrimitiveTopology::TRIANGLE_LIST_WITH_ADJACENCY => "triangle_list_with_adjacency",
      HalaPrimitiveTopology::TRIANGLE_STRIP_WITH_ADJACENCY => "triangle_strip_with_adjacency",
      HalaPrimitiveTopology::PATCH_LIST => "patch_list",
      _ => "default",
    };

    serializer.serialize_str(s)
  }
}

impl<'de> Deserialize<'de> for HalaPrimitiveTopology {
  fn deserialize<D>(deserializer: D) -> Result<HalaPrimitiveTopology, D::Error>
  where
    D: Deserializer<'de>,
  {
    struct HalaPrimitiveTopologyVisitor;

    impl<'de> Visitor<'de> for HalaPrimitiveTopologyVisitor {
      type Value = HalaPrimitiveTopology;

      fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string of primitive topology")
      }

      fn visit_str<E>(self, value: &str) -> Result<HalaPrimitiveTopology, E>
      where
        E: de::Error,
      {
        match value {
          "POINT_LIST" => Ok(HalaPrimitiveTopology::POINT_LIST),
          "point_list" => Ok(HalaPrimitiveTopology::POINT_LIST),
          "LINE_LIST" => Ok(HalaPrimitiveTopology::LINE_LIST),
          "line_list" => Ok(HalaPrimitiveTopology::LINE_LIST),
          "LINE_STRIP" => Ok(HalaPrimitiveTopology::LINE_STRIP),
          "line_strip" => Ok(HalaPrimitiveTopology::LINE_STRIP),
          "TRIANGLE_LIST" => Ok(HalaPrimitiveTopology::TRIANGLE_LIST),
          "triangle_list" => Ok(HalaPrimitiveTopology::TRIANGLE_LIST),
          "TRIANGLE_STRIP" => Ok(HalaPrimitiveTopology::TRIANGLE_STRIP),
          "triangle_strip" => Ok(HalaPrimitiveTopology::TRIANGLE_STRIP),
          "TRIANGLE_FAN" => Ok(HalaPrimitiveTopology::TRIANGLE_FAN),
          "triangle_fan" => Ok(HalaPrimitiveTopology::TRIANGLE_FAN),
          "LINE_LIST_WITH_ADJACENCY" => Ok(HalaPrimitiveTopology::LINE_LIST_WITH_ADJACENCY),
          "line_list_with_adjacency" => Ok(HalaPrimitiveTopology::LINE_LIST_WITH_ADJACENCY),
          "LINE_STRIP_WITH_ADJACENCY" => Ok(HalaPrimitiveTopology::LINE_STRIP_WITH_ADJACENCY),
          "line_strip_with_adjacency" => Ok(HalaPrimitiveTopology::LINE_STRIP_WITH_ADJACENCY),
          "TRIANGLE_LIST_WITH_ADJACENCY" => Ok(HalaPrimitiveTopology::TRIANGLE_LIST_WITH_ADJACENCY),
          "triangle_list_with_adjacency" => Ok(HalaPrimitiveTopology::TRIANGLE_LIST_WITH_ADJACENCY),
          "TRIANGLE_STRIP_WITH_ADJACENCY" => Ok(HalaPrimitiveTopology::TRIANGLE_STRIP_WITH_ADJACENCY),
          "triangle_strip_with_adjacency" => Ok(HalaPrimitiveTopology::TRIANGLE_STRIP_WITH_ADJACENCY),
          "PATCH_LIST" => Ok(HalaPrimitiveTopology::PATCH_LIST),
          "patch_list" => Ok(HalaPrimitiveTopology::PATCH_LIST),
          "default" => Ok(HalaPrimitiveTopology::default()),
                  _ => return Err(de::Error::invalid_value(Unexpected::Str(value), &"a primitive topology")),
        }
      }
    }

    deserializer.deserialize_str(HalaPrimitiveTopologyVisitor)
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

impl Serialize for HalaBlendFactor {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let s = match *self {
      HalaBlendFactor::ZERO => "zero",
      HalaBlendFactor::ONE => "one",
      HalaBlendFactor::SRC_COLOR => "src_color",
      HalaBlendFactor::ONE_MINUS_SRC_COLOR => "one_minus_src_color",
      HalaBlendFactor::DST_COLOR => "dst_color",
      HalaBlendFactor::ONE_MINUS_DST_COLOR => "one_minus_dst_color",
      HalaBlendFactor::SRC_ALPHA => "src_alpha",
      HalaBlendFactor::ONE_MINUS_SRC_ALPHA => "one_minus_src_alpha",
      HalaBlendFactor::DST_ALPHA => "dst_alpha",
      HalaBlendFactor::ONE_MINUS_DST_ALPHA => "one_minus_dst_alpha",
      HalaBlendFactor::CONSTANT_COLOR => "constant_color",
      HalaBlendFactor::ONE_MINUS_CONSTANT_COLOR => "one_minus_constant_color",
      HalaBlendFactor::CONSTANT_ALPHA => "constant_alpha",
      HalaBlendFactor::ONE_MINUS_CONSTANT_ALPHA => "one_minus_constant_alpha",
      HalaBlendFactor::SRC_ALPHA_SATURATE => "src_alpha_saturate",
      HalaBlendFactor::SRC1_COLOR => "src1_color",
      HalaBlendFactor::ONE_MINUS_SRC1_COLOR => "one_minus_src1_color",
      HalaBlendFactor::SRC1_ALPHA => "src1_alpha",
      HalaBlendFactor::ONE_MINUS_SRC1_ALPHA => "one_minus_src1_alpha",
      _ => "default",
    };

    serializer.serialize_str(s)
  }
}

impl<'de> Deserialize<'de> for HalaBlendFactor {
  fn deserialize<D>(deserializer: D) -> Result<HalaBlendFactor, D::Error>
  where
    D: Deserializer<'de>,
  {
    struct HalaBlendFactorVisitor;

    impl<'de> Visitor<'de> for HalaBlendFactorVisitor {
      type Value = HalaBlendFactor;

      fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a string of blend factor")
      }

      fn visit_str<E>(self, value: &str) -> Result<HalaBlendFactor, E>
      where
        E: de::Error,
      {
        let val = match value {
          "ZERO" => HalaBlendFactor::ZERO,
          "zero" => HalaBlendFactor::ZERO,
          "ONE" => HalaBlendFactor::ONE,
          "one" => HalaBlendFactor::ONE,
          "SRC_COLOR" => HalaBlendFactor::SRC_COLOR,
          "src_color" => HalaBlendFactor::SRC_COLOR,
          "ONE_MINUS_SRC_COLOR" => HalaBlendFactor::ONE_MINUS_SRC_COLOR,
          "one_minus_src_color" => HalaBlendFactor::ONE_MINUS_SRC_COLOR,
          "DST_COLOR" => HalaBlendFactor::DST_COLOR,
          "dst_color" => HalaBlendFactor::DST_COLOR,
          "ONE_MINUS_DST_COLOR" => HalaBlendFactor::ONE_MINUS_DST_COLOR,
          "one_minus_dst_color" => HalaBlendFactor::ONE_MINUS_DST_COLOR,
          "SRC_ALPHA" => HalaBlendFactor::SRC_ALPHA,
          "src_alpha" => HalaBlendFactor::SRC_ALPHA,
          "ONE_MINUS_SRC_ALPHA" => HalaBlendFactor::ONE_MINUS_SRC_ALPHA,
          "one_minus_src_alpha" => HalaBlendFactor::ONE_MINUS_SRC_ALPHA,
          "DST_ALPHA" => HalaBlendFactor::DST_ALPHA,
          "dst_alpha" => HalaBlendFactor::DST_ALPHA,
          "ONE_MINUS_DST_ALPHA" => HalaBlendFactor::ONE_MINUS_DST_ALPHA,
          "one_minus_dst_alpha" => HalaBlendFactor::ONE_MINUS_DST_ALPHA,
          "CONSTANT_COLOR" => HalaBlendFactor::CONSTANT_COLOR,
          "constant_color" => HalaBlendFactor::CONSTANT_COLOR,
          "ONE_MINUS_CONSTANT_COLOR" => HalaBlendFactor::ONE_MINUS_CONSTANT_COLOR,
          "one_minus_constant_color" => HalaBlendFactor::ONE_MINUS_CONSTANT_COLOR,
          "CONSTANT_ALPHA" => HalaBlendFactor::CONSTANT_ALPHA,
          "constant_alpha" => HalaBlendFactor::CONSTANT_ALPHA,
          "ONE_MINUS_CONSTANT_ALPHA" => HalaBlendFactor::ONE_MINUS_CONSTANT_ALPHA,
          "one_minus_constant_alpha" => HalaBlendFactor::ONE_MINUS_CONSTANT_ALPHA,
          "SRC_ALPHA_SATURATE" => HalaBlendFactor::SRC_ALPHA_SATURATE,
          "src_alpha_saturate" => HalaBlendFactor::SRC_ALPHA_SATURATE,
          "SRC1_COLOR" => HalaBlendFactor::SRC1_COLOR,
          "src1_color" => HalaBlendFactor::SRC1_COLOR,
          "ONE_MINUS_SRC1_COLOR" => HalaBlendFactor::ONE_MINUS_SRC1_COLOR,
          "one_minus_src1_color" => HalaBlendFactor::ONE_MINUS_SRC1_COLOR,
          "SRC1_ALPHA" => HalaBlendFactor::SRC1_ALPHA,
          "src1_alpha" => HalaBlendFactor::SRC1_ALPHA,
          "ONE_MINUS_SRC1_ALPHA" => HalaBlendFactor::ONE_MINUS_SRC1_ALPHA,
          "one_minus_src1_alpha" => HalaBlendFactor::ONE_MINUS_SRC1_ALPHA,
          "default" => HalaBlendFactor::default(),
          _ => return Err(de::Error::invalid_value(Unexpected::Str(value), &"a blend factor")),
        };

        Ok(val)
      }
    }

    deserializer.deserialize_str(HalaBlendFactorVisitor)
  }
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

impl Serialize for HalaBlendOp {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let s = match *self {
      HalaBlendOp::ADD => "add",
      HalaBlendOp::SUBTRACT => "subtract",
      HalaBlendOp::REVERSE_SUBTRACT => "reverse_subtract",
      HalaBlendOp::MIN => "min",
      HalaBlendOp::MAX => "max",
      _ => "default",
    };

    serializer.serialize_str(s)
  }
}

impl<'de> Deserialize<'de> for HalaBlendOp {
  fn deserialize<D>(deserializer: D) -> Result<HalaBlendOp, D::Error>
  where
    D: Deserializer<'de>,
  {
    struct HalaBlendOpVisitor;

    impl<'de> Visitor<'de> for HalaBlendOpVisitor {
      type Value = HalaBlendOp;

      fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a string of blend operation")
      }

      fn visit_str<E>(self, value: &str) -> Result<HalaBlendOp, E>
      where
        E: de::Error,
      {
        let val = match value {
          "ADD" => HalaBlendOp::ADD,
          "add" => HalaBlendOp::ADD,
          "SUBTRACT" => HalaBlendOp::SUBTRACT,
          "subtract" => HalaBlendOp::SUBTRACT,
          "REVERSE_SUBTRACT" => HalaBlendOp::REVERSE_SUBTRACT,
          "reverse_subtract" => HalaBlendOp::REVERSE_SUBTRACT,
          "MIN" => HalaBlendOp::MIN,
          "min" => HalaBlendOp::MIN,
          "MAX" => HalaBlendOp::MAX,
          "max" => HalaBlendOp::MAX,
          "default" => HalaBlendOp::default(),
                  _ => return Err(de::Error::invalid_value(Unexpected::Str(value), &"a blend operation")),
        };

        Ok(val)
      }
    }

    deserializer.deserialize_str(HalaBlendOpVisitor)
  }
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

impl Serialize for HalaFrontFace {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let s = match *self {
      HalaFrontFace::COUNTER_CLOCKWISE => "counter_clockwise",
      HalaFrontFace::CLOCKWISE => "clockwise",
      _ => "default",
    };

    serializer.serialize_str(s)
  }
}

impl<'de> Deserialize<'de> for HalaFrontFace {
  fn deserialize<D>(deserializer: D) -> Result<HalaFrontFace, D::Error>
  where
    D: Deserializer<'de>,
  {
    struct HalaFrontFaceVisitor;

    impl<'de> Visitor<'de> for HalaFrontFaceVisitor {
      type Value = HalaFrontFace;

      fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a string of front face")
      }

      fn visit_str<E>(self, value: &str) -> Result<HalaFrontFace, E>
      where
        E: de::Error,
      {
        let val = match value {
          "COUNTER_CLOCKWISE" => HalaFrontFace::COUNTER_CLOCKWISE,
          "counter_clockwise" => HalaFrontFace::COUNTER_CLOCKWISE,
          "CLOCKWISE" => HalaFrontFace::CLOCKWISE,
          "clockwise" => HalaFrontFace::CLOCKWISE,
          "default" => HalaFrontFace::default(),
          _ => return Err(de::Error::invalid_value(Unexpected::Str(value), &"a front face")),
        };

        Ok(val)
      }
    }

    deserializer.deserialize_str(HalaFrontFaceVisitor)
  }
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

impl Serialize for HalaCullModeFlags {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let s = match *self {
      HalaCullModeFlags::NONE => "none",
      HalaCullModeFlags::FRONT => "front",
      HalaCullModeFlags::BACK => "back",
      HalaCullModeFlags::FRONT_AND_BACK => "front_and_back",
      _ => return Err(serde::ser::Error::custom("unexpected cull mode flags value")),
    };

    serializer.serialize_str(s)
  }
}

impl<'de> Deserialize<'de> for HalaCullModeFlags {
  fn deserialize<D>(deserializer: D) -> Result<HalaCullModeFlags, D::Error>
  where
    D: Deserializer<'de>,
  {
    struct HalaCullModeFlagsVisitor;

    impl<'de> Visitor<'de> for HalaCullModeFlagsVisitor {
      type Value = HalaCullModeFlags;

      fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a string of cull mode flags")
      }

      fn visit_str<E>(self, value: &str) -> Result<HalaCullModeFlags, E>
      where
        E: de::Error,
      {
        let val = match value {
          "NONE" => HalaCullModeFlags::NONE,
          "none" => HalaCullModeFlags::NONE,
          "FRONT" => HalaCullModeFlags::FRONT,
          "front" => HalaCullModeFlags::FRONT,
          "BACK" => HalaCullModeFlags::BACK,
          "back" => HalaCullModeFlags::BACK,
          "FRONT_AND_BACK" => HalaCullModeFlags::FRONT_AND_BACK,
          "front_and_back" => HalaCullModeFlags::FRONT_AND_BACK,
          _ => return Err(de::Error::invalid_value(Unexpected::Str(value), &"a cull mode flags")),
        };

        Ok(val)
      }
    }

    deserializer.deserialize_str(HalaCullModeFlagsVisitor)
  }
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

impl Serialize for HalaPolygonMode {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let s = match *self {
      HalaPolygonMode::FILL => "fill",
      HalaPolygonMode::LINE => "line",
      HalaPolygonMode::POINT => "point",
      _ => "default",
    };

    serializer.serialize_str(s)
  }
}

impl<'de> Deserialize<'de> for HalaPolygonMode {
  fn deserialize<D>(deserializer: D) -> Result<HalaPolygonMode, D::Error>
  where
    D: Deserializer<'de>,
  {
    struct HalaPolygonModeVisitor;

    impl<'de> Visitor<'de> for HalaPolygonModeVisitor {
      type Value = HalaPolygonMode;

      fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a string of polygon mode")
      }

      fn visit_str<E>(self, value: &str) -> Result<HalaPolygonMode, E>
      where
        E: de::Error,
      {
        let val = match value {
          "FILL" => HalaPolygonMode::FILL,
          "fill" => HalaPolygonMode::FILL,
          "LINE" => HalaPolygonMode::LINE,
          "line" => HalaPolygonMode::LINE,
          "POINT" => HalaPolygonMode::POINT,
          "point" => HalaPolygonMode::POINT,
          "default" => HalaPolygonMode::default(),
          _ => return Err(de::Error::invalid_value(Unexpected::Str(value), &"a polygon mode")),
        };

        Ok(val)
      }
    }

    deserializer.deserialize_str(HalaPolygonModeVisitor)
  }
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

impl Serialize for HalaCompareOp {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let s = match *self {
      HalaCompareOp::NEVER => "never",
      HalaCompareOp::LESS => "less",
      HalaCompareOp::EQUAL => "equal",
      HalaCompareOp::LESS_OR_EQUAL => "less_or_equal",
      HalaCompareOp::GREATER => "greater",
      HalaCompareOp::NOT_EQUAL => "not_equal",
      HalaCompareOp::GREATER_OR_EQUAL => "greater_or_equal",
      HalaCompareOp::ALWAYS => "always",
      _ => "default",
    };

    serializer.serialize_str(s)
  }
}

impl<'de> Deserialize<'de> for HalaCompareOp {
  fn deserialize<D>(deserializer: D) -> Result<HalaCompareOp, D::Error>
  where
    D: Deserializer<'de>,
  {
    struct HalaCompareOpVisitor;

    impl<'de> Visitor<'de> for HalaCompareOpVisitor {
      type Value = HalaCompareOp;

      fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a string of compare operation")
      }

      fn visit_str<E>(self, value: &str) -> Result<HalaCompareOp, E>
      where
        E: de::Error,
      {
        let val = match value {
          "NEVER" => HalaCompareOp::NEVER,
          "never" => HalaCompareOp::NEVER,
          "LESS" => HalaCompareOp::LESS,
          "less" => HalaCompareOp::LESS,
          "EQUAL" => HalaCompareOp::EQUAL,
          "equal" => HalaCompareOp::EQUAL,
          "LESS_OR_EQUAL" => HalaCompareOp::LESS_OR_EQUAL,
          "less_or_equal" => HalaCompareOp::LESS_OR_EQUAL,
          "GREATER" => HalaCompareOp::GREATER,
          "greater" => HalaCompareOp::GREATER,
          "NOT_EQUAL" => HalaCompareOp::NOT_EQUAL,
          "not_equal" => HalaCompareOp::NOT_EQUAL,
          "GREATER_OR_EQUAL" => HalaCompareOp::GREATER_OR_EQUAL,
          "greater_or_equal" => HalaCompareOp::GREATER_OR_EQUAL,
          "ALWAYS" => HalaCompareOp::ALWAYS,
          "always" => HalaCompareOp::ALWAYS,
          "default" => HalaCompareOp::default(),
          _ => return Err(de::Error::invalid_value(Unexpected::Str(value), &"a compare operation")),
        };

        Ok(val)
      }
    }

    deserializer.deserialize_str(HalaCompareOpVisitor)
  }
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

impl Serialize for HalaStencilOp {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let s = match *self {
      HalaStencilOp::KEEP => "keep",
      HalaStencilOp::ZERO => "zero",
      HalaStencilOp::REPLACE => "replace",
      HalaStencilOp::INCREMENT_AND_CLAMP => "increment_and_clamp",
      HalaStencilOp::DECREMENT_AND_CLAMP => "decrement_and_clamp",
      HalaStencilOp::INVERT => "invert",
      HalaStencilOp::INCREMENT_AND_WRAP => "increment_and_wrap",
      HalaStencilOp::DECREMENT_AND_WRAP => "decrement_and_wrap",
      _ => "default",
    };

    serializer.serialize_str(s)
  }
}

impl<'de> Deserialize<'de> for HalaStencilOp {
  fn deserialize<D>(deserializer: D) -> Result<HalaStencilOp, D::Error>
  where
    D: Deserializer<'de>,
  {
    struct HalaStencilOpVisitor;

    impl<'de> Visitor<'de> for HalaStencilOpVisitor {
      type Value = HalaStencilOp;

      fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a string of stencil operation")
      }

      fn visit_str<E>(self, value: &str) -> Result<HalaStencilOp, E>
      where
        E: de::Error,
      {
        let val = match value {
          "KEEP" => HalaStencilOp::KEEP,
          "keep" => HalaStencilOp::KEEP,
          "ZERO" => HalaStencilOp::ZERO,
          "zero" => HalaStencilOp::ZERO,
          "REPLACE" => HalaStencilOp::REPLACE,
          "replace" => HalaStencilOp::REPLACE,
          "INCREMENT_AND_CLAMP" => HalaStencilOp::INCREMENT_AND_CLAMP,
          "increment_and_clamp" => HalaStencilOp::INCREMENT_AND_CLAMP,
          "DECREMENT_AND_CLAMP" => HalaStencilOp::DECREMENT_AND_CLAMP,
          "decrement_and_clamp" => HalaStencilOp::DECREMENT_AND_CLAMP,
          "INVERT" => HalaStencilOp::INVERT,
          "invert" => HalaStencilOp::INVERT,
          "INCREMENT_AND_WRAP" => HalaStencilOp::INCREMENT_AND_WRAP,
          "increment_and_wrap" => HalaStencilOp::INCREMENT_AND_WRAP,
          "DECREMENT_AND_WRAP" => HalaStencilOp::DECREMENT_AND_WRAP,
          "decrement_and_wrap" => HalaStencilOp::DECREMENT_AND_WRAP,
          "default" => HalaStencilOp::default(),
          _ => return Err(de::Error::invalid_value(Unexpected::Str(value), &"a stencil operation")),
        };

        Ok(val)
      }
    }

    deserializer.deserialize_str(HalaStencilOpVisitor)
  }
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

/// The dynamic state.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct HalaDynamicState(i32);
impl HalaDynamicState {
  pub const VIEWPORT: Self = Self(vk::DynamicState::VIEWPORT.as_raw());
  pub const SCISSOR: Self = Self(vk::DynamicState::SCISSOR.as_raw());
  pub const LINE_WIDTH: Self = Self(vk::DynamicState::LINE_WIDTH.as_raw());
  pub const DEPTH_BIAS: Self = Self(vk::DynamicState::DEPTH_BIAS.as_raw());
  pub const BLEND_CONSTANTS: Self = Self(vk::DynamicState::BLEND_CONSTANTS.as_raw());
  pub const DEPTH_BOUNDS: Self = Self(vk::DynamicState::DEPTH_BOUNDS.as_raw());
  pub const STENCIL_COMPARE_MASK: Self = Self(vk::DynamicState::STENCIL_COMPARE_MASK.as_raw());
  pub const STENCIL_WRITE_MASK: Self = Self(vk::DynamicState::STENCIL_WRITE_MASK.as_raw());
  pub const STENCIL_REFERENCE: Self = Self(vk::DynamicState::STENCIL_REFERENCE.as_raw());
  pub const VIEWPORT_W_SCALING_NV: Self = Self(vk::DynamicState::VIEWPORT_W_SCALING_NV.as_raw());
  pub const DISCARD_RECTANGLE_EXT: Self = Self(vk::DynamicState::DISCARD_RECTANGLE_EXT.as_raw());
  pub const SAMPLE_LOCATIONS_EXT: Self = Self(vk::DynamicState::SAMPLE_LOCATIONS_EXT.as_raw());
  pub const RAY_TRACING_PIPELINE_STACK_SIZE_KHR: Self = Self(vk::DynamicState::RAY_TRACING_PIPELINE_STACK_SIZE_KHR.as_raw());
  pub const VIEWPORT_SHADING_RATE_PALETTE_NV: Self = Self(vk::DynamicState::VIEWPORT_SHADING_RATE_PALETTE_NV.as_raw());
  pub const VIEWPORT_COARSE_SAMPLE_ORDER_NV: Self = Self(vk::DynamicState::VIEWPORT_COARSE_SAMPLE_ORDER_NV.as_raw());
  pub const EXCLUSIVE_SCISSOR_NV: Self = Self(vk::DynamicState::EXCLUSIVE_SCISSOR_NV.as_raw());
  pub const FRAGMENT_SHADING_RATE_KHR: Self = Self(vk::DynamicState::FRAGMENT_SHADING_RATE_KHR.as_raw());
  pub const LINE_STIPPLE_EXT: Self = Self(vk::DynamicState::LINE_STIPPLE_EXT.as_raw());
  pub const CULL_MODE_EXT: Self = Self(vk::DynamicState::CULL_MODE_EXT.as_raw());
  pub const FRONT_FACE_EXT: Self = Self(vk::DynamicState::FRONT_FACE_EXT.as_raw());
  pub const PRIMITIVE_TOPOLOGY_EXT: Self = Self(vk::DynamicState::PRIMITIVE_TOPOLOGY_EXT.as_raw());
  pub const VIEWPORT_WITH_COUNT_EXT: Self = Self(vk::DynamicState::VIEWPORT_WITH_COUNT_EXT.as_raw());
  pub const SCISSOR_WITH_COUNT_EXT: Self = Self(vk::DynamicState::SCISSOR_WITH_COUNT_EXT.as_raw());
  pub const VERTEX_INPUT_BINDING_STRIDE_EXT: Self = Self(vk::DynamicState::VERTEX_INPUT_BINDING_STRIDE_EXT.as_raw());
  pub const DEPTH_TEST_ENABLE_EXT: Self = Self(vk::DynamicState::DEPTH_TEST_ENABLE_EXT.as_raw());
  pub const DEPTH_WRITE_ENABLE_EXT: Self = Self(vk::DynamicState::DEPTH_WRITE_ENABLE_EXT.as_raw());
  pub const DEPTH_COMPARE_OP_EXT: Self = Self(vk::DynamicState::DEPTH_COMPARE_OP_EXT.as_raw());
  pub const DEPTH_BOUNDS_TEST_ENABLE_EXT: Self = Self(vk::DynamicState::DEPTH_BOUNDS_TEST_ENABLE_EXT.as_raw());
  pub const STENCIL_TEST_ENABLE_EXT: Self = Self(vk::DynamicState::STENCIL_TEST_ENABLE_EXT.as_raw());
  pub const STENCIL_OP_EXT: Self = Self(vk::DynamicState::STENCIL_OP_EXT.as_raw());
}

impl std::convert::From<vk::DynamicState> for HalaDynamicState {
  fn from(val: vk::DynamicState) -> Self {
    Self(val.as_raw())
  }
}

impl std::convert::From<HalaDynamicState> for vk::DynamicState {
  fn from(val: HalaDynamicState) -> Self {
    vk::DynamicState::from_raw(val.0)
  }
}

/// The blend state.
#[derive(Serialize, Deserialize)]
pub struct HalaBlendState {
  pub src_factor: HalaBlendFactor,
  pub dst_factor: HalaBlendFactor,
  pub op: HalaBlendOp,
}

/// The blend state implementation.
impl AsRef<HalaBlendState> for HalaBlendState {
  fn as_ref(&self) -> &Self {
    self
  }
}

/// The default implementation for the blend state.
impl Default for HalaBlendState {
  fn default() -> Self {
    Self {
      src_factor: HalaBlendFactor::ONE,
      dst_factor: HalaBlendFactor::ZERO,
      op: HalaBlendOp::ADD,
    }
  }
}

/// The blend state implementation.
impl HalaBlendState {

    pub fn new(
      src_factor: HalaBlendFactor,
      dst_factor: HalaBlendFactor,
      op: HalaBlendOp,
    ) -> Self {
      Self {
        src_factor,
        dst_factor,
        op,
      }
    }

}

/// The rasterizer state.
#[derive(Serialize, Deserialize)]
pub struct HalaRasterizerState {
  pub front_face: HalaFrontFace,
  pub cull_mode: HalaCullModeFlags,
  pub polygon_mode: HalaPolygonMode,
  pub line_width: f32,
}

/// The rasterizer state implementation.
impl AsRef<HalaRasterizerState> for HalaRasterizerState {
  fn as_ref(&self) -> &Self {
    self
  }
}

/// The default implementation for the rasterizer state.
impl Default for HalaRasterizerState {
  fn default() -> Self {
    Self {
      front_face: HalaFrontFace::COUNTER_CLOCKWISE,
      cull_mode: HalaCullModeFlags::NONE,
      polygon_mode: HalaPolygonMode::FILL,
      line_width: 1.0,
    }
  }
}

/// The rasterizer state implementation.
impl HalaRasterizerState {

  pub fn new(
    front_face: HalaFrontFace,
    cull_mode: HalaCullModeFlags,
    polygon_mode: HalaPolygonMode,
    line_width: f32,
  ) -> Self {
    Self {
      front_face,
      cull_mode,
      polygon_mode,
      line_width,
    }
  }

}

/// The multisample state.
#[derive(Serialize, Deserialize)]
pub struct HalaMultisampleState {
  pub rasterization_samples: HalaSampleCountFlags,
  pub sample_shading_enable: bool,
  pub min_sample_shading: f32,
  pub sample_masks: Vec<u32>,
  pub alpha_to_coverage_enable: bool,
  pub alpha_to_one_enable: bool,
}

impl AsRef<HalaMultisampleState> for HalaMultisampleState {
  fn as_ref(&self) -> &Self {
    self
  }
}

/// The default implementation for the multisample state.
impl Default for HalaMultisampleState {
  fn default() -> Self {
    Self {
      rasterization_samples: HalaSampleCountFlags::TYPE_1,
      sample_shading_enable: false,
      min_sample_shading: 1.0,
      sample_masks: vec![],
      alpha_to_coverage_enable: false,
      alpha_to_one_enable: false,
    }
  }
}

/// The multisample state implementation.
impl HalaMultisampleState {

  pub fn new(
    rasterization_samples: HalaSampleCountFlags,
    sample_shading_enable: bool,
    min_sample_shading: f32,
    sample_masks: &[u32],
    alpha_to_coverage_enable: bool,
    alpha_to_one_enable: bool,
  ) -> Self {
    Self {
      rasterization_samples,
      sample_shading_enable,
      min_sample_shading,
      sample_masks: sample_masks.to_vec(),
      alpha_to_coverage_enable,
      alpha_to_one_enable,
    }
  }

}

/// The depth state.
#[derive(Serialize, Deserialize)]
pub struct HalaDepthState {
  pub test_enable: bool,
  pub write_enable: bool,
  pub compare_op: HalaCompareOp,
}

/// The depth state implementation.
impl AsRef<HalaDepthState> for HalaDepthState {
  fn as_ref(&self) -> &Self {
    self
  }
}

/// The default implementation for the depth state.
impl Default for HalaDepthState {
  fn default() -> Self {
    Self {
      test_enable: true,
      write_enable: true,
      compare_op: HalaCompareOp::LESS,
    }
  }
}

/// The depth state implementation.
impl HalaDepthState {

  pub fn new(
    test_enable: bool,
    write_enable: bool,
    compare_op: HalaCompareOp,
  ) -> Self {
    Self {
      test_enable,
      write_enable,
      compare_op,
    }
  }

}

/// The stencil operation state.
#[derive(Copy, Clone, Default, Serialize, Deserialize)]
pub struct HalaStencilOpState {
  pub fail_op: HalaStencilOp,
  pub pass_op: HalaStencilOp,
  pub depth_fail_op: HalaStencilOp,
  pub compare_op: HalaCompareOp,
  pub compare_mask: u32,
  pub write_mask: u32,
  pub reference: u32,
}

/// The stencil operation state implementation.
impl AsRef<HalaStencilOpState> for HalaStencilOpState {
  fn as_ref(&self) -> &Self {
    self
  }
}

/// The from implementation for HalaStencilOpState.
impl std::convert::From<vk::StencilOpState> for HalaStencilOpState {
  fn from(val: vk::StencilOpState) -> Self {
    Self::from(&val)
  }
}

/// The from implementation for HalaStencilOpState.
impl std::convert::From<&vk::StencilOpState> for HalaStencilOpState {
  fn from(val: &vk::StencilOpState) -> Self {
    Self {
      fail_op: HalaStencilOp::from(val.fail_op),
      pass_op: HalaStencilOp::from(val.pass_op),
      depth_fail_op: HalaStencilOp::from(val.depth_fail_op),
      compare_op: HalaCompareOp::from(val.compare_op),
      compare_mask: val.compare_mask,
      write_mask: val.write_mask,
      reference: val.reference,
    }
  }
}

/// The from implementation for vk::StencilOpState.
impl std::convert::From<HalaStencilOpState> for vk::StencilOpState {
  fn from(val: HalaStencilOpState) -> Self {
    Self::from(&val)
  }
}

/// The from implementation for vk::StencilOpState.
impl std::convert::From<&HalaStencilOpState> for vk::StencilOpState {
  fn from(val: &HalaStencilOpState) -> Self {
    Self {
      fail_op: vk::StencilOp::from(val.fail_op),
      pass_op: vk::StencilOp::from(val.pass_op),
      depth_fail_op: vk::StencilOp::from(val.depth_fail_op),
      compare_op: vk::CompareOp::from(val.compare_op),
      compare_mask: val.compare_mask,
      write_mask: val.write_mask,
      reference: val.reference,
    }
  }
}

/// The stencil state.
#[derive(Serialize, Deserialize)]
pub struct HalaStencilState {
  pub test_enable: bool,
  pub front: HalaStencilOpState,
  pub back: HalaStencilOpState,
}

/// The stencil state implementation.
impl AsRef<HalaStencilState> for HalaStencilState {
  fn as_ref(&self) -> &Self {
    self
  }
}

/// The default implementation for the stencil state.
impl Default for HalaStencilState {
  fn default() -> Self {
    Self {
      test_enable: false,
      front: HalaStencilOpState::default(),
      back: HalaStencilOpState::default(),
    }
  }
}

/// The stencil state implementation.
impl HalaStencilState {

  pub fn new(
    test_enable: bool,
    front: HalaStencilOpState,
    back: HalaStencilOpState,
  ) -> Self {
    Self {
      test_enable,
      front,
      back,
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

/// The graphics pipeline implementation.
#[allow(clippy::too_many_arguments)]
impl HalaGraphicsPipeline {

  /// Create a graphics pipeline.
  /// param logical_device: The logical device.
  /// param swapchain: The swapchain.
  /// param descriptor_set_layouts: The descriptor set layouts.
  /// param flags: The pipeline create flags.
  /// param vertex_attribute_descriptions: The vertex attribute descriptions.
  /// param vertex_binding_descriptions: The vertex binding descriptions.
  /// param push_constant_ranges: The push constant ranges.
  /// param primitive_topology: The primitive topology.
  /// param color_blend: The color blend(source, destination, operation).
  /// param alpha_blend: The alpha blend(source, destination, operation).
  /// param rasterizer_info: The rasterizer info(line width, front face, cull mode, polygon mode)
  /// param multisample_info: The multisample info(rasterization samples, sample shading enable, min sample shading, sample masks, alpha to coverage enable, alpha to one enable).
  /// param depth_info: The depth info(test enable, write enable, compare operation).
  /// param stencil_info: The stencil info(test enable, front, back).
  /// param shaders: The shaders.
  /// param dynamic_states: The dynamic states.
  /// param pipeline_cache: The pipeline cache.
  /// param debug_name: The debug name.
  /// return: The graphics pipeline.
  pub fn new<DSL, VIAD, VIBD, PCR, S>(
    logical_device: Rc<RefCell<HalaLogicalDevice>>,
    swapchain: &HalaSwapchain,
    descriptor_set_layouts: &[DSL],
    flags: HalaPipelineCreateFlags,
    vertex_attribute_descriptions: &[VIAD],
    vertex_binding_descriptions: &[VIBD],
    push_constant_ranges: &[PCR],
    primitive_topology: HalaPrimitiveTopology,
    color_blend: &HalaBlendState,
    alpha_blend: &HalaBlendState,
    rasterizer_info: &HalaRasterizerState,
    multisample_info: &HalaMultisampleState,
    depth_info: &HalaDepthState,
    stencil_info: Option<&HalaStencilState>,
    shaders: &[S],
    dynamic_states: &[HalaDynamicState],
    pipeline_cache: Option<&HalaPipelineCache>,
    debug_name: &str,
  ) -> Result<Self, HalaGfxError>
    where DSL: AsRef<HalaDescriptorSetLayout>,
          VIAD: AsRef<HalaVertexInputAttributeDescription>,
          VIBD: AsRef<HalaVertexInputBindingDescription>,
          PCR: AsRef<HalaPushConstantRange>,
          S: AsRef<HalaShader>,
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
      flags,
      vertex_attribute_descriptions,
      vertex_binding_descriptions,
      primitive_topology,
      color_blend,
      alpha_blend,
      rasterizer_info,
      multisample_info,
      depth_info,
      stencil_info,
      shaders,
      dynamic_states,
      pipeline_cache,
      pipeline_layout,
      None,
      0,
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

  /// Create a graphics pipeline with specified render targets.
  /// param logical_device: The logical device.
  /// color_images: The color render targets.
  /// depth_image: The depth render target.
  /// descriptor_set_layouts: The descriptor set layouts.
  /// flags: The pipeline create flags.
  /// vertex_attribute_descriptions: The vertex attribute descriptions.
  /// vertex_binding_descriptions: The vertex binding descriptions.
  /// push_constant_ranges: The push constant ranges.
  /// primitive_topology: The primitive topology.
  /// color_blends: The color blend(source, destination, operation).
  /// alpha_blends: The alpha blend(source, destination, operation).
  /// rasterizer_info: The rasterizer info(line width, front face, cull mode, polygon mode)
  /// multisample_info: The multisample info(rasterization samples, sample shading enable, min sample shading, sample masks, alpha to coverage enable, alpha to one enable).
  /// depth_info: The depth info(test enable, write enable, compare operation).
  /// stencil_info: The stencil info(test enable, front, back).
  /// shaders: The shaders.
  /// dynamic_states: The dynamic states.
  /// pipeline_cache: The pipeline cache.
  /// debug_name: The debug name.
  /// return: The graphics pipeline.
  pub fn with_rt<T, DSL, VIAD, VIBD, PCR, BS, S>(
    logical_device: Rc<RefCell<HalaLogicalDevice>>,
    color_images: &[T],
    depth_image: Option<&T>,
    descriptor_set_layouts: &[DSL],
    flags: HalaPipelineCreateFlags,
    vertex_attribute_descriptions: &[VIAD],
    vertex_binding_descriptions: &[VIBD],
    push_constant_ranges: &[PCR],
    primitive_topology: HalaPrimitiveTopology,
    color_blends: &[BS],
    alpha_blends: &[BS],
    rasterizer_info: &HalaRasterizerState,
    multisample_info: &HalaMultisampleState,
    depth_info: &HalaDepthState,
    stencil_info: Option<&HalaStencilState>,
    shaders: &[S],
    dynamic_states: &[HalaDynamicState],
    pipeline_cache: Option<&HalaPipelineCache>,
    debug_name: &str,
  ) -> Result<Self, HalaGfxError>
    where T: AsRef<HalaImage>,
          DSL: AsRef<HalaDescriptorSetLayout>,
          VIAD: AsRef<HalaVertexInputAttributeDescription>,
          VIBD: AsRef<HalaVertexInputBindingDescription>,
          PCR: AsRef<HalaPushConstantRange>,
          BS: AsRef<HalaBlendState>,
          S: AsRef<HalaShader>,
  {
    let pipeline_layout = HalaPipelineBase::create_pipeline_layout(
      &logical_device,
      push_constant_ranges,
      descriptor_set_layouts,
      debug_name
    )?;

    let graphics_pipeline = Self::create_pipeline_with_rt(
      &logical_device,
      color_images,
      depth_image,
      flags,
      vertex_attribute_descriptions,
      vertex_binding_descriptions,
      primitive_topology,
      color_blends,
      alpha_blends,
      rasterizer_info,
      multisample_info,
      depth_info,
      stencil_info,
      shaders,
      dynamic_states,
      pipeline_cache,
      pipeline_layout,
      None,
      0,
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

  /// Create a graphics pipeline with specified formats and size.
  /// param logical_device: The logical device.
  /// color_formats: The color formats.
  /// depth_format: The depth format.
  /// width: The width.
  /// height: The height.
  /// descriptor_set_layouts: The descriptor set layouts.
  /// flags: The pipeline create flags.
  /// vertex_attribute_descriptions: The vertex attribute descriptions.
  /// vertex_binding_descriptions: The vertex binding descriptions.
  /// push_constant_ranges: The push constant ranges.
  /// primitive_topology: The primitive topology.
  /// color_blends: The color blend(source, destination, operation).
  /// alpha_blends: The alpha blend(source, destination, operation).
  /// rasterizer_info: The rasterizer info(line width, front face, cull mode, polygon mode)
  /// multisample_info: The multisample info(rasterization samples, sample shading enable, min sample shading, sample masks, alpha to coverage enable, alpha to one enable).
  /// depth_info: The depth info(test enable, write enable, compare operation).
  /// stencil_info: The stencil info(test enable, front, back).
  /// shaders: The shaders.
  /// dynamic_states: The dynamic states.
  /// pipeline_cache: The pipeline cache.
  /// debug_name: The debug name.
  /// return: The graphics pipeline.
  pub fn with_format_and_size<DSL, VIAD, VIBD, PCR, BS, S>(
    logical_device: Rc<RefCell<HalaLogicalDevice>>,
    color_formats: &[HalaFormat],
    depth_format: Option<HalaFormat>,
    width: u32,
    height: u32,
    descriptor_set_layouts: &[DSL],
    flags: HalaPipelineCreateFlags,
    vertex_attribute_descriptions: &[VIAD],
    vertex_binding_descriptions: &[VIBD],
    push_constant_ranges: &[PCR],
    primitive_topology: HalaPrimitiveTopology,
    color_blends: &[BS],
    alpha_blends: &[BS],
    rasterizer_info: &HalaRasterizerState,
    multisample_info: &HalaMultisampleState,
    depth_info: &HalaDepthState,
    stencil_info: Option<&HalaStencilState>,
    shaders: &[S],
    dynamic_states: &[HalaDynamicState],
    pipeline_cache: Option<&HalaPipelineCache>,
    debug_name: &str,
  ) -> Result<Self, HalaGfxError>
    where DSL: AsRef<HalaDescriptorSetLayout>,
          VIAD: AsRef<HalaVertexInputAttributeDescription>,
          VIBD: AsRef<HalaVertexInputBindingDescription>,
          PCR: AsRef<HalaPushConstantRange>,
          BS: AsRef<HalaBlendState>,
          S: AsRef<HalaShader>,
  {
    Self::with_renderpass_format_and_size(
      logical_device,
      color_formats,
      depth_format,
      width,
      height,
      descriptor_set_layouts,
      flags,
      vertex_attribute_descriptions,
      vertex_binding_descriptions,
      push_constant_ranges,
      primitive_topology,
      color_blends,
      alpha_blends,
      rasterizer_info,
      multisample_info,
      depth_info,
      stencil_info,
      shaders,
      dynamic_states,
      None,
      0,
      pipeline_cache,
      debug_name
    )
  }

  /// Create a graphics pipeline with specified render pass, formats and size.
  /// param logical_device: The logical device.
  /// color_formats: The color formats.
  /// depth_format: The depth format.
  /// width: The width.
  /// height: The height.
  /// descriptor_set_layouts: The descriptor set layouts.
  /// flags: The pipeline create flags.
  /// vertex_attribute_descriptions: The vertex attribute descriptions.
  /// vertex_binding_descriptions: The vertex binding descriptions.
  /// push_constant_ranges: The push constant ranges.
  /// primitive_topology: The primitive topology.
  /// color_blends: The color blend(source, destination, operation).
  /// alpha_blends: The alpha blend(source, destination, operation).
  /// rasterizer_info: The rasterizer info(line width, front face, cull mode, polygon mode)
  /// multisample_info: The multisample info(rasterization samples, sample shading enable, min sample shading, sample masks, alpha to coverage enable, alpha to one enable).
  /// depth_info: The depth info(test enable, write enable, compare operation).
  /// stencil_info: The stencil info(test enable, front, back).
  /// shaders: The shaders.
  /// dynamic_states: The dynamic states.
  /// render_pass: The render pass.
  /// subpass_index: The subpass index.
  /// pipeline_cache: The pipeline cache.
  /// debug_name: The debug name.
  /// return: The graphics pipeline.
  pub fn with_renderpass_format_and_size<DSL, VIAD, VIBD, PCR, BS, S>(
    logical_device: Rc<RefCell<HalaLogicalDevice>>,
    color_formats: &[HalaFormat],
    depth_format: Option<HalaFormat>,
    width: u32,
    height: u32,
    descriptor_set_layouts: &[DSL],
    flags: HalaPipelineCreateFlags,
    vertex_attribute_descriptions: &[VIAD],
    vertex_binding_descriptions: &[VIBD],
    push_constant_ranges: &[PCR],
    primitive_topology: HalaPrimitiveTopology,
    color_blends: &[BS],
    alpha_blends: &[BS],
    rasterizer_info: &HalaRasterizerState,
    multisample_info: &HalaMultisampleState,
    depth_info: &HalaDepthState,
    stencil_info: Option<&HalaStencilState>,
    shaders: &[S],
    dynamic_states: &[HalaDynamicState],
    render_pass: Option<&HalaRenderPass>,
    subpass_index: u32,
    pipeline_cache: Option<&HalaPipelineCache>,
    debug_name: &str,
  ) -> Result<Self, HalaGfxError>
    where DSL: AsRef<HalaDescriptorSetLayout>,
          VIAD: AsRef<HalaVertexInputAttributeDescription>,
          VIBD: AsRef<HalaVertexInputBindingDescription>,
          PCR: AsRef<HalaPushConstantRange>,
          BS: AsRef<HalaBlendState>,
          S: AsRef<HalaShader>,
  {
    let pipeline_layout = HalaPipelineBase::create_pipeline_layout(
      &logical_device,
      push_constant_ranges,
      descriptor_set_layouts,
      debug_name
    )?;

    let graphics_pipeline = Self::create_pipeline_with_format_and_size(
      &logical_device,
      color_formats,
      depth_format,
      width,
      height,
      flags,
      vertex_attribute_descriptions,
      vertex_binding_descriptions,
      primitive_topology,
      color_blends,
      alpha_blends,
      rasterizer_info,
      multisample_info,
      depth_info,
      stencil_info,
      shaders,
      dynamic_states,
      pipeline_cache,
      pipeline_layout,
      render_pass,
      subpass_index,
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
  /// param flags: The pipeline create flags.
  /// param vertex_attribute_descriptions: The vertex attribute descriptions.
  /// param vertex_binding_descriptions: The vertex binding descriptions.
  /// param primitive_topology: The primitive topology.
  /// param color_blend: The color blend(source, destination, operation).
  /// param alpha_blend: The alpha blend(source, destination, operation).
  /// param rasterizer_info: The rasterizer info(line width, front face, cull mode, polygon mode)
  /// param multisample_info: The multisample info(rasterization samples, sample shading enable, min sample shading, sample masks, alpha to coverage enable, alpha to one enable).
  /// param depth_info: The depth info(test enable, write enable, compare operation).
  /// param stencil_info: The stencil info(test enable, front, back).
  /// param shaders: The shaders.
  /// param dynamic_states: The dynamic states.
  /// param pipeline_cache: The pipeline cache.
  /// param pipeline_layout: The pipeline layout.
  /// param render_pass: The render pass.
  /// param subpass_index: The subpass index.
  /// param debug_name: The debug name.
  /// return: The graphics pipeline.
  fn create_pipeline<VIAD, VIBD, S>(
    logical_device: &Rc<RefCell<HalaLogicalDevice>>,
    swapchain: &HalaSwapchain,
    flags: HalaPipelineCreateFlags,
    vertex_attribute_descriptions: &[VIAD],
    vertex_binding_descriptions: &[VIBD],
    primitive_topology: HalaPrimitiveTopology,
    color_blend: &HalaBlendState,
    alpha_blend: &HalaBlendState,
    rasterizer_info: &HalaRasterizerState,
    multisample_info: &HalaMultisampleState,
    depth_info: &HalaDepthState,
    stencil_info: Option<&HalaStencilState>,
    shaders: &[S],
    dynamic_states: &[HalaDynamicState],
    pipeline_cache: Option<&HalaPipelineCache>,
    pipeline_layout: vk::PipelineLayout,
    render_pass: Option<&HalaRenderPass>,
    subpass_index: u32,
    debug_name: &str,
  ) -> Result<vk::Pipeline, HalaGfxError>
    where VIAD: AsRef<HalaVertexInputAttributeDescription>,
          VIBD: AsRef<HalaVertexInputBindingDescription>,
          S: AsRef<HalaShader>
  {
    Self::create_pipeline_with_format_and_size(
      logical_device,
      &[swapchain.desc.format],
      Some(swapchain.depth_stencil_format),
      swapchain.desc.dims.width,
      swapchain.desc.dims.height,
      flags,
      vertex_attribute_descriptions,
      vertex_binding_descriptions,
      primitive_topology,
      &[color_blend],
      &[alpha_blend],
      rasterizer_info,
      multisample_info,
      depth_info,
      stencil_info,
      shaders,
      dynamic_states,
      pipeline_cache,
      pipeline_layout,
      render_pass,
      subpass_index,
      debug_name
    )
  }

  /// Create a graphics pipeline with specified render targets.
  /// param logical_device: The logical device.
  /// param color_images: The color render targets.
  /// param depth_image: The depth render target.
  /// param flags: The pipeline create flags.
  /// param vertex_attribute_descriptions: The vertex attribute descriptions.
  /// param vertex_binding_descriptions: The vertex binding descriptions.
  /// param primitive_topology: The primitive topology.
  /// param color_blends: The color blend(source, destination, operation).
  /// param alpha_blends: The alpha blend(source, destination, operation).
  /// param rasterizer_info: The rasterizer info(line width, front face, cull mode, polygon mode)
  /// param multisample_info: The multisample info(rasterization samples, sample shading enable, min sample shading, sample masks, alpha to coverage enable, alpha to one enable).
  /// param depth_info: The depth info(test enable, write enable, compare operation).
  /// param stencil_info: The stencil info(test enable, front, back).
  /// param shaders: The shaders.
  /// param dynamic_states: The dynamic states.
  /// param pipeline_cache: The pipeline cache.
  /// param pipeline_layout: The pipeline layout.
  /// param render_pass: The render pass.
  /// param subpass_index: The subpass index.
  /// param debug_name: The debug name.
  /// return: The graphics pipeline.
  fn create_pipeline_with_rt<T, VIAD, VIBD, BS, S>(
    logical_device: &Rc<RefCell<HalaLogicalDevice>>,
    color_images: &[T],
    depth_image: Option<&T>,
    flags: HalaPipelineCreateFlags,
    vertex_attribute_descriptions: &[VIAD],
    vertex_binding_descriptions: &[VIBD],
    primitive_topology: HalaPrimitiveTopology,
    color_blends: &[BS],
    alpha_blends: &[BS],
    rasterizer_info: &HalaRasterizerState,
    multisample_info: &HalaMultisampleState,
    depth_info: &HalaDepthState,
    stencil_info: Option<&HalaStencilState>,
    shaders: &[S],
    dynamic_states: &[HalaDynamicState],
    pipeline_cache: Option<&HalaPipelineCache>,
    pipeline_layout: vk::PipelineLayout,
    render_pass: Option<&HalaRenderPass>,
    subpass_index: u32,
    debug_name: &str,
  ) -> Result<vk::Pipeline, HalaGfxError>
    where T: AsRef<HalaImage>,
          VIAD: AsRef<HalaVertexInputAttributeDescription>,
          VIBD: AsRef<HalaVertexInputBindingDescription>,
          BS: AsRef<HalaBlendState>,
          S: AsRef<HalaShader>
  {
    Self::create_pipeline_with_format_and_size(
      logical_device,
      color_images.iter().map(|i| i.as_ref().format).collect::<Vec<_>>().as_slice(),
      depth_image.map(|i| i.as_ref().format),
      color_images[0].as_ref().extent.width,
      color_images[0].as_ref().extent.height,
      flags,
      vertex_attribute_descriptions,
      vertex_binding_descriptions,
      primitive_topology,
      color_blends,
      alpha_blends,
      rasterizer_info,
      multisample_info,
      depth_info,
      stencil_info,
      shaders,
      dynamic_states,
      pipeline_cache,
      pipeline_layout,
      render_pass,
      subpass_index,
      debug_name
    )
  }

  /// Create a graphics pipeline with specified format and size.
  /// param logical_device: The logical device.
  /// param color_formats: The color formats.
  /// param depth_format: The depth format.
  /// param width: The width.
  /// param height: The height.
  /// param flags: The pipeline create flags.
  /// param vertex_attribute_descriptions: The vertex attribute descriptions.
  /// param vertex_binding_descriptions: The vertex binding descriptions.
  /// param primitive_topology: The primitive topology.
  /// param color_blends: The color blend(source, destination, operation).
  /// param alpha_blends: The alpha blend(source, destination, operation).
  /// param rasterizer_info: The rasterizer info(line width, front face, cull mode, polygon mode)
  /// param multisample_info: The multisample info(rasterization samples, sample shading enable, min sample shading, sample masks, alpha to coverage enable, alpha to one enable).
  /// param depth_info: The depth info(test enable, write enable, compare operation).
  /// param stencil_info: The stencil info(test enable, front, back).
  /// param shaders: The shaders.
  /// param dynamic_states: The dynamic states.
  /// param pipeline_cache: The pipeline cache.
  /// param pipeline_layout: The pipeline layout.
  /// param render_pass: The render pass.
  /// param subpass_index: The subpass index.
  /// param debug_name: The debug name.
  /// return: The graphics pipeline.
  fn create_pipeline_with_format_and_size<VIAD, VIBD, BS, S>(
    logical_device: &Rc<RefCell<HalaLogicalDevice>>,
    color_formats: &[HalaFormat],
    depth_format: Option<HalaFormat>,
    width: u32,
    height: u32,
    flags: HalaPipelineCreateFlags,
    vertex_attribute_descriptions: &[VIAD],
    vertex_binding_descriptions: &[VIBD],
    primitive_topology: HalaPrimitiveTopology,
    color_blends: &[BS],
    alpha_blends: &[BS],
    rasterizer_info: &HalaRasterizerState,
    multisample_info: &HalaMultisampleState,
    depth_info: &HalaDepthState,
    stencil_info: Option<&HalaStencilState>,
    shaders: &[S],
    dynamic_states: &[HalaDynamicState],
    pipeline_cache: Option<&HalaPipelineCache>,
    pipeline_layout: vk::PipelineLayout,
    render_pass: Option<&HalaRenderPass>,
    subpass_index: u32,
    debug_name: &str,
  ) -> Result<vk::Pipeline, HalaGfxError>
    where VIAD: AsRef<HalaVertexInputAttributeDescription>,
          VIBD: AsRef<HalaVertexInputBindingDescription>,
          BS: AsRef<HalaBlendState>,
          S: AsRef<HalaShader>
  {
    let has_depth = depth_format.is_some();
    let has_stencil = depth_format.map_or(false, |fmt| fmt == HalaFormat::D16_UNORM_S8_UINT || fmt == HalaFormat::D24_UNORM_S8_UINT || fmt == HalaFormat::D32_SFLOAT_S8_UINT);

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
      y: height as f32,
      width: width as f32,
      height: -(height as f32),
      min_depth: 0.,
      max_depth: 1.,
    }];
    let scissors = [vk::Rect2D {
      offset: vk::Offset2D { x: 0, y: 0 },
      extent: vk::Extent2D { width, height },
    }];
    let viewport_info = vk::PipelineViewportStateCreateInfo::default()
      .viewports(&viewports)
      .scissors(&scissors);

    let rasterizer_info = vk::PipelineRasterizationStateCreateInfo::default()
      .line_width(rasterizer_info.line_width)
      .front_face(rasterizer_info.front_face.into())
      .cull_mode(rasterizer_info.cull_mode.into())
      .polygon_mode(rasterizer_info.polygon_mode.into());

    let multisampler_info = vk::PipelineMultisampleStateCreateInfo::default()
      .rasterization_samples(multisample_info.rasterization_samples.into())
      .sample_shading_enable(multisample_info.sample_shading_enable)
      .min_sample_shading(multisample_info.min_sample_shading)
      .sample_mask(multisample_info.sample_masks.as_ref())
      .alpha_to_coverage_enable(multisample_info.alpha_to_coverage_enable)
      .alpha_to_one_enable(multisample_info.alpha_to_one_enable);

    let color_blend_attachments = color_blends.iter().zip(alpha_blends).map(|(color_blend, alpha_blend)| {
      vk::PipelineColorBlendAttachmentState::default()
        .blend_enable(true)
        .src_color_blend_factor(color_blend.as_ref().src_factor.into())
        .dst_color_blend_factor(color_blend.as_ref().dst_factor.into())
        .color_blend_op(color_blend.as_ref().op.into())
        .src_alpha_blend_factor(alpha_blend.as_ref().src_factor.into())
        .dst_alpha_blend_factor(alpha_blend.as_ref().dst_factor.into())
        .alpha_blend_op(alpha_blend.as_ref().op.into())
        .color_write_mask(
          vk::ColorComponentFlags::R | vk::ColorComponentFlags::G | vk::ColorComponentFlags::B | vk::ColorComponentFlags::A,
        )
    }).collect::<Vec<_>>();
    let color_blend_info =
      vk::PipelineColorBlendStateCreateInfo::default().attachments(color_blend_attachments.as_slice());

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

    let formats = color_formats
      .iter()
      .map(|fmt| fmt.into())
      .collect::<Vec<vk::Format>>();
    let rendering_info = vk::PipelineRenderingCreateInfo::default()
      .color_attachment_formats(formats.as_slice());
    let rendering_info = if has_depth {
      rendering_info.depth_attachment_format(depth_format.unwrap().into())
    } else {
      rendering_info
    };
    let mut rendering_info = if has_stencil {
      rendering_info.stencil_attachment_format(depth_format.unwrap().into())
    } else {
      rendering_info
    };

    let dynamic_states = dynamic_states
      .iter()
      .map(|ds| vk::DynamicState::from(*ds))
      .collect::<Vec<_>>();
    let dynamic_state_info = vk::PipelineDynamicStateCreateInfo::default()
      .dynamic_states(dynamic_states.as_slice());

    let pipeline_info = vk::GraphicsPipelineCreateInfo::default()
      .flags(flags.into())
      .stages(shader_stage_infos.as_slice())
      .vertex_input_state(&vertex_input_info)
      .input_assembly_state(&input_assembly_info)
      .viewport_state(&viewport_info)
      .rasterization_state(&rasterizer_info)
      .multisample_state(&multisampler_info)
      .color_blend_state(&color_blend_info)
      .dynamic_state(&dynamic_state_info)
      .layout(pipeline_layout)
      .push_next(&mut rendering_info);
    let pipeline_info = if let Some(rp) = render_pass {
      pipeline_info
        .render_pass(rp.raw)
        .subpass(subpass_index)
    } else {
      pipeline_info
        .subpass(0)
    };

    let graphics_pipeline = if has_depth {
      let depth_stencil_info = if !has_stencil {
        vk::PipelineDepthStencilStateCreateInfo::default()
          .depth_test_enable(depth_info.test_enable)
          .depth_write_enable(depth_info.write_enable)
          .depth_compare_op(depth_info.compare_op.into())
          .depth_bounds_test_enable(false)
          .stencil_test_enable(false)
          .front(Default::default())
          .back(Default::default())
      } else {
        let stencil_info = stencil_info.ok_or(HalaGfxError::new("Stencil info is required.", None))?;
        vk::PipelineDepthStencilStateCreateInfo::default()
          .depth_test_enable(depth_info.test_enable)
          .depth_write_enable(depth_info.write_enable)
          .depth_compare_op(depth_info.compare_op.into())
          .depth_bounds_test_enable(false)
          .stencil_test_enable(stencil_info.test_enable)
          .front(stencil_info.front.into())
          .back(stencil_info.back.into())
      };
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
  /// param push_constant_ranges: The push constant ranges.
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
  pub fn new<DSL, PCR, S>(
    logical_device: Rc<RefCell<HalaLogicalDevice>>,
    descriptor_set_layouts: &[DSL],
    push_constant_ranges: &[PCR],
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
          PCR: AsRef<HalaPushConstantRange>,
          S: AsRef<HalaShader>
  {
    // Create the pipeline layout.
    let pipeline_layout = HalaPipelineBase::create_pipeline_layout(
      &logical_device,
      push_constant_ranges,
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
  /// param push_constant_ranges: The push constant ranges.
  /// param shader: The shader.
  /// param pipeline_cache: The pipeline cache.
  /// param debug_name: The debug name.
  /// return: The compute pipeline.
  pub fn new<DSL, PCR>(
    logical_device: Rc<RefCell<HalaLogicalDevice>>,
    descriptor_set_layouts: &[DSL],
    push_constant_ranges: &[PCR],
    shader: &HalaShader,
    pipeline_cache: Option<&HalaPipelineCache>,
    debug_name: &str,
  ) -> Result<Self, HalaGfxError>
    where DSL: AsRef<HalaDescriptorSetLayout>,
          PCR: AsRef<HalaPushConstantRange>
  {
    // Create the pipeline layout.
    let pipeline_layout = HalaPipelineBase::create_pipeline_layout(
      &logical_device,
      push_constant_ranges,
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