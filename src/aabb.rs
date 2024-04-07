use ash::vk;

/// The axis-aligned bounding box.
#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Copy, Clone, Default)]
pub struct HalaAABB {
  pub min: [f32; 3],
  pub max: [f32; 3],
}

impl std::convert::From<vk::AabbPositionsKHR> for HalaAABB {
  fn from(aabb_pos: vk::AabbPositionsKHR) -> Self {
    Self {
      min: [aabb_pos.min_x, aabb_pos.min_y, aabb_pos.min_z],
      max: [aabb_pos.max_x, aabb_pos.max_y, aabb_pos.max_z],
    }
  }
}

impl std::convert::From<HalaAABB> for vk::AabbPositionsKHR {
  fn from(aabb: HalaAABB) -> Self {
    Self {
      min_x: aabb.min[0],
      min_y: aabb.min[1],
      min_z: aabb.min[2],
      max_x: aabb.max[0],
      max_y: aabb.max[1],
      max_z: aabb.max[2],
    }
  }
}