pub mod constants;
pub mod prelude;
pub mod macros;
pub mod error;
pub mod instance;
pub mod physical_device;
pub mod surface;
pub mod logical_device;
pub mod swapchain;
pub mod context;
pub mod renderpass;
pub mod frame_buffer;
pub mod shader;
pub mod pipeline;
pub mod pipeline_cache;
pub mod pools;
pub mod command_buffer;
pub mod buffer;
pub mod descriptor_set;
pub mod format;
pub mod acceleration_structure;
pub mod image;
pub mod sampler;
pub mod shader_binding_table;
pub mod query;
pub mod barrier;
pub mod aabb;

pub use prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HalaGPUType {
  Discrete,
  Integrated,
  Virtual,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HalaGPURequirements {
  pub width: u32,
  pub height: u32,
  pub version: (u32, u32, u32),
  pub is_gpu: bool,
  pub gpu_types: Vec<HalaGPUType>,
  pub require_srgb_surface: bool,
  pub require_mesh_shader: bool,
  pub require_ray_tracing: bool,
  pub require_10bits_output: bool,
  pub is_immediate: bool,
  pub is_low_latency: bool,
  pub require_depth: bool,
  pub require_stencil: bool,
  pub require_printf_in_shader: bool,
  pub require_depth_stencil_resolve: bool,
}

impl Default for HalaGPURequirements {
  fn default() -> Self {
    Self {
      width: 1280,
      height: 720,
      version: (1, 2, 0),
      is_gpu: true,
      gpu_types: vec![HalaGPUType::Discrete, HalaGPUType::Integrated],
      require_srgb_surface: false,
      require_mesh_shader: false,
      require_ray_tracing: false,
      require_10bits_output: false,
      is_immediate: false,
      is_low_latency: false,
      require_depth: true,
      require_stencil: false,
      require_printf_in_shader: false,
      require_depth_stencil_resolve: false,
    }
  }
}