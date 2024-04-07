use ash::vk;

use crate::HalaPipelineStageFlags2;

/// The image layout.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct HalaImageLayout(i32);
impl HalaImageLayout {
  pub const UNDEFINED: Self = Self(vk::ImageLayout::UNDEFINED.as_raw());
  pub const GENERAL: Self = Self(vk::ImageLayout::GENERAL.as_raw());
  pub const COLOR_ATTACHMENT_OPTIMAL: Self = Self(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL.as_raw());
  pub const DEPTH_STENCIL_ATTACHMENT_OPTIMAL: Self = Self(vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL.as_raw());
  pub const DEPTH_STENCIL_READ_ONLY_OPTIMAL: Self = Self(vk::ImageLayout::DEPTH_STENCIL_READ_ONLY_OPTIMAL.as_raw());
  pub const SHADER_READ_ONLY_OPTIMAL: Self = Self(vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL.as_raw());
  pub const TRANSFER_SRC_OPTIMAL: Self = Self(vk::ImageLayout::TRANSFER_SRC_OPTIMAL.as_raw());
  pub const TRANSFER_DST_OPTIMAL: Self = Self(vk::ImageLayout::TRANSFER_DST_OPTIMAL.as_raw());
  pub const PREINITIALIZED: Self = Self(vk::ImageLayout::PREINITIALIZED.as_raw());
  pub const PRESENT_SRC: Self = Self(vk::ImageLayout::PRESENT_SRC_KHR.as_raw());
}

impl std::convert::From<vk::ImageLayout> for HalaImageLayout {
  fn from(layout: vk::ImageLayout) -> Self {
    Self(layout.as_raw())
  }
}

impl std::convert::From<HalaImageLayout> for vk::ImageLayout {
  fn from(layout: HalaImageLayout) -> Self {
    Self::from_raw(layout.0)
  }
}

/// The access flags.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HalaAccessFlags2(u64);
crate::hala_bitflags_wrapped!(HalaAccessFlags2, u64);
impl HalaAccessFlags2 {
  pub const NONE: Self = Self(vk::AccessFlags2::NONE.as_raw());
  pub const INDIRECT_COMMAND_READ: Self = Self(vk::AccessFlags2::INDIRECT_COMMAND_READ.as_raw());
  pub const INDEX_READ: Self = Self(vk::AccessFlags2::INDEX_READ.as_raw());
  pub const VERTEX_ATTRIBUTE_READ: Self = Self(vk::AccessFlags2::VERTEX_ATTRIBUTE_READ.as_raw());
  pub const UNIFORM_READ: Self = Self(vk::AccessFlags2::UNIFORM_READ.as_raw());
  pub const INPUT_ATTACHMENT_READ: Self = Self(vk::AccessFlags2::INPUT_ATTACHMENT_READ.as_raw());
  pub const SHADER_READ: Self = Self(vk::AccessFlags2::SHADER_READ.as_raw());
  pub const SHADER_WRITE: Self = Self(vk::AccessFlags2::SHADER_WRITE.as_raw());
  pub const COLOR_ATTACHMENT_READ: Self = Self(vk::AccessFlags2::COLOR_ATTACHMENT_READ.as_raw());
  pub const COLOR_ATTACHMENT_WRITE: Self = Self(vk::AccessFlags2::COLOR_ATTACHMENT_WRITE.as_raw());
  pub const DEPTH_STENCIL_ATTACHMENT_READ: Self = Self(vk::AccessFlags2::DEPTH_STENCIL_ATTACHMENT_READ.as_raw());
  pub const DEPTH_STENCIL_ATTACHMENT_WRITE: Self = Self(vk::AccessFlags2::DEPTH_STENCIL_ATTACHMENT_WRITE.as_raw());
  pub const TRANSFER_READ: Self = Self(vk::AccessFlags2::TRANSFER_READ.as_raw());
  pub const TRANSFER_WRITE: Self = Self(vk::AccessFlags2::TRANSFER_WRITE.as_raw());
  pub const HOST_READ: Self = Self(vk::AccessFlags2::HOST_READ.as_raw());
  pub const HOST_WRITE: Self = Self(vk::AccessFlags2::HOST_WRITE.as_raw());
  pub const MEMORY_READ: Self = Self(vk::AccessFlags2::MEMORY_READ.as_raw());
  pub const MEMORY_WRITE: Self = Self(vk::AccessFlags2::MEMORY_WRITE.as_raw());
  pub const SHADER_SAMPLED_READ: Self = Self(vk::AccessFlags2::SHADER_SAMPLED_READ.as_raw());
  pub const SHADER_STORAGE_READ: Self = Self(vk::AccessFlags2::SHADER_STORAGE_READ.as_raw());
  pub const SHADER_STORAGE_WRITE: Self = Self(vk::AccessFlags2::SHADER_STORAGE_WRITE.as_raw());
}

impl std::convert::From<vk::AccessFlags2> for HalaAccessFlags2 {
  fn from(flags: vk::AccessFlags2) -> Self {
    Self(flags.as_raw())
  }
}

impl std::convert::From<HalaAccessFlags2> for vk::AccessFlags2 {
  fn from(flags: HalaAccessFlags2) -> Self {
    Self::from_raw(flags.0)
  }
}

/// The image aspect flags.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HalaImageAspectFlags(u32);
crate::hala_bitflags_wrapped!(HalaImageAspectFlags, u32);
impl HalaImageAspectFlags {
  pub const NONE: Self = Self(vk::ImageAspectFlags::NONE.as_raw());
  pub const COLOR: Self = Self(vk::ImageAspectFlags::COLOR.as_raw());
  pub const DEPTH: Self = Self(vk::ImageAspectFlags::DEPTH.as_raw());
  pub const STENCIL: Self = Self(vk::ImageAspectFlags::STENCIL.as_raw());
  pub const METADATA: Self = Self(vk::ImageAspectFlags::METADATA.as_raw());
}

impl std::convert::From<vk::ImageAspectFlags> for HalaImageAspectFlags {
  fn from(flags: vk::ImageAspectFlags) -> Self {
    Self(flags.as_raw())
  }
}

impl std::convert::From<HalaImageAspectFlags> for vk::ImageAspectFlags {
  fn from(flags: HalaImageAspectFlags) -> Self {
    Self::from_raw(flags.0)
  }
}

/// The image barrier.
#[derive(Clone, Copy)]
pub struct HalaImageBarrierInfo {
  pub src_stage_mask: HalaPipelineStageFlags2,
  pub src_access_mask: HalaAccessFlags2,
  pub dst_stage_mask: HalaPipelineStageFlags2,
  pub dst_access_mask: HalaAccessFlags2,
  pub old_layout: HalaImageLayout,
  pub new_layout: HalaImageLayout,
  pub src_queue_family_index: u32,
  pub dst_queue_family_index: u32,
  pub aspect_mask: HalaImageAspectFlags,
  pub base_mip_level: u32,
  pub level_count: u32,
  pub base_array_layer: u32,
  pub layer_count: u32,
  pub image: vk::Image,
}

/// The AsRef trait implementation for HalaImageBarrier.
impl AsRef<HalaImageBarrierInfo> for HalaImageBarrierInfo {
  fn as_ref(&self) -> &Self {
    self
  }
}

/// The Default trait implementation for HalaImageBarrier.
impl Default for HalaImageBarrierInfo {
  fn default() -> Self {
    Self {
      src_stage_mask: HalaPipelineStageFlags2::NONE,
      src_access_mask: HalaAccessFlags2::NONE,
      dst_stage_mask: HalaPipelineStageFlags2::NONE,
      dst_access_mask: HalaAccessFlags2::NONE,
      old_layout: HalaImageLayout::UNDEFINED,
      new_layout: HalaImageLayout::UNDEFINED,
      src_queue_family_index: 0,
      dst_queue_family_index: 0,
      aspect_mask: HalaImageAspectFlags::NONE,
      base_mip_level: 0,
      level_count: 1,
      base_array_layer: 0,
      layer_count: 1,
      image: vk::Image::null(),
    }
  }
}

/// The buffer barrier.
#[derive(Clone, Copy, Default)]
pub struct HalaBufferBarrierInfo {
  pub src_stage_mask: HalaPipelineStageFlags2,
  pub src_access_mask: HalaAccessFlags2,
  pub dst_stage_mask: HalaPipelineStageFlags2,
  pub dst_access_mask: HalaAccessFlags2,
  pub src_queue_family_index: u32,
  pub dst_queue_family_index: u32,
  pub offset: u64,
  pub size: u64,
  pub buffer: vk::Buffer,
}

/// The AsRef trait implementation for HalaBufferBarrier.
impl AsRef<HalaBufferBarrierInfo> for HalaBufferBarrierInfo {
  fn as_ref(&self) -> &Self {
    self
  }
}

/// The barrier.
#[derive(Clone, Copy, Default)]
pub struct HalaMemoryBarrierInfo {
  pub src_stage_mask: HalaPipelineStageFlags2,
  pub src_access_mask: HalaAccessFlags2,
  pub dst_stage_mask: HalaPipelineStageFlags2,
  pub dst_access_mask: HalaAccessFlags2,
}

/// The AsRef trait implementation for HalaMemoryBarrier.
impl AsRef<HalaMemoryBarrierInfo> for HalaMemoryBarrierInfo {
  fn as_ref(&self) -> &Self {
    self
  }
}