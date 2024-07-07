use std::rc::Rc;
use std::cell::RefCell;

use ash::vk;

use crate::{
  HalaGfxError,
  HalaLogicalDevice,
};

/// The shader stage.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HalaShaderStageFlags(u32);
crate::hala_bitflags_wrapped!(HalaShaderStageFlags, u32);
impl HalaShaderStageFlags {
  pub const VERTEX: Self = Self(vk::ShaderStageFlags::VERTEX.as_raw());
  pub const TESSELLATION_CONTROL: Self = Self(vk::ShaderStageFlags::TESSELLATION_CONTROL.as_raw());
  pub const TESSELLATION_EVALUATION: Self = Self(vk::ShaderStageFlags::TESSELLATION_EVALUATION.as_raw());
  pub const GEOMETRY: Self = Self(vk::ShaderStageFlags::GEOMETRY.as_raw());
  pub const FRAGMENT: Self = Self(vk::ShaderStageFlags::FRAGMENT.as_raw());
  pub const COMPUTE: Self = Self(vk::ShaderStageFlags::COMPUTE.as_raw());
  pub const ALL_GRAPHICS: Self = Self(vk::ShaderStageFlags::ALL_GRAPHICS.as_raw());
  pub const ALL: Self = Self(vk::ShaderStageFlags::ALL.as_raw());
  pub const RAYGEN: Self = Self(vk::ShaderStageFlags::RAYGEN_KHR.as_raw());
  pub const ANY_HIT: Self = Self(vk::ShaderStageFlags::ANY_HIT_KHR.as_raw());
  pub const CLOSEST_HIT: Self = Self(vk::ShaderStageFlags::CLOSEST_HIT_KHR.as_raw());
  pub const MISS: Self = Self(vk::ShaderStageFlags::MISS_KHR.as_raw());
  pub const INTERSECTION: Self = Self(vk::ShaderStageFlags::INTERSECTION_KHR.as_raw());
  pub const CALLABLE: Self = Self(vk::ShaderStageFlags::CALLABLE_KHR.as_raw());
  pub const TASK: Self = Self(vk::ShaderStageFlags::TASK_EXT.as_raw());
  pub const MESH: Self = Self(vk::ShaderStageFlags::MESH_EXT.as_raw());
}

impl std::convert::From<vk::ShaderStageFlags> for HalaShaderStageFlags {
  fn from(flags: vk::ShaderStageFlags) -> Self {
    Self(flags.as_raw())
  }
}

impl std::convert::From<HalaShaderStageFlags> for vk::ShaderStageFlags {
  fn from(flags: HalaShaderStageFlags) -> Self {
    vk::ShaderStageFlags::from_raw(flags.0)
  }
}

/// The ray tracing shader group type.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct HalaRayTracingShaderGroupType(i32);
impl HalaRayTracingShaderGroupType {
  pub const GENERAL: Self = Self(vk::RayTracingShaderGroupTypeKHR::GENERAL.as_raw());
  pub const TRIANGLES_HIT_GROUP: Self = Self(vk::RayTracingShaderGroupTypeKHR::TRIANGLES_HIT_GROUP.as_raw());
  pub const PROCEDURAL_HIT_GROUP: Self = Self(vk::RayTracingShaderGroupTypeKHR::PROCEDURAL_HIT_GROUP.as_raw());
}

impl std::convert::From<vk::RayTracingShaderGroupTypeKHR> for HalaRayTracingShaderGroupType {
  fn from(val: vk::RayTracingShaderGroupTypeKHR) -> Self {
    Self(val.as_raw())
  }
}

impl std::convert::From<HalaRayTracingShaderGroupType> for vk::RayTracingShaderGroupTypeKHR {
  fn from(val: HalaRayTracingShaderGroupType) -> Self {
    vk::RayTracingShaderGroupTypeKHR::from_raw(val.0)
  }
}

/// The shader.
pub struct HalaShader {
  pub(crate) logical_device: Rc<RefCell<HalaLogicalDevice>>,
  pub module: vk::ShaderModule,
  pub stage_flags: HalaShaderStageFlags,
  pub ray_tracing_group_type: HalaRayTracingShaderGroupType,

  pub(crate) debug_name: String,
}

/// The AsRef trait implementation for shader.
impl AsRef<HalaShader> for HalaShader {
  fn as_ref(&self) -> &HalaShader {
    self
  }
}

/// The Drop trait implementation for shader.
impl Drop for HalaShader {
  fn drop(&mut self) {
    unsafe {
      self.logical_device.borrow().raw.destroy_shader_module(self.module, None);
    }
    log::debug!("A HalaShader \"{}\" is dropped.", self.debug_name);
  }
}

/// The implementation of shader.
impl HalaShader {
  /// Create a new shader.
  /// param logical_device: The logical device.
  /// param code: The compiled shader code.
  /// param stage: The shader stage.
  /// param rt_group_type: The ray tracing shader group type.
  /// param debug_name: The debug name.
  /// return: The shader.
  pub fn new(
    logical_device: Rc<RefCell<HalaLogicalDevice>>,
    code: &[u8],
    stage: HalaShaderStageFlags,
    rt_group_type: HalaRayTracingShaderGroupType,
    debug_name: &str,
  ) -> Result<Self, HalaGfxError> {
    let code = ash::util::read_spv(&mut std::io::Cursor::new(code))
      .map_err(|err| HalaGfxError::new("Failed to read shader code.", Some(Box::new(err))))?;
    let module_create_info = vk::ShaderModuleCreateInfo::default()
      .code(&code);
    let module = unsafe {
      logical_device.borrow().raw.create_shader_module(&module_create_info, None)
        .map_err(|err| HalaGfxError::new("Failed to create shader module.", Some(Box::new(err))))?
    };
    logical_device.borrow().set_debug_name(module, debug_name)
      .map_err(|err| HalaGfxError::new("Failed to set debug name of shader module.", Some(Box::new(err))))?;

    log::debug!("A HalaShader \"{}\" is created.", debug_name);
    Ok(
      Self {
        logical_device,
        module,
        stage_flags: stage,
        ray_tracing_group_type: rt_group_type,
        debug_name: debug_name.to_string(),
      }
    )
  }

  /// Create a new shader with file.
  /// param logical_device: The logical device.
  /// param code: The compiled shader code.
  /// param stage: The shader stage.
  /// param rt_group_type: The ray tracing shader group type.
  /// param debug_name: The debug name.
  /// return: The shader.
  pub fn with_file(
    logical_device: Rc<RefCell<HalaLogicalDevice>>,
    file_path: &str,
    stage: HalaShaderStageFlags,
    rt_group_type: HalaRayTracingShaderGroupType,
    debug_name: &str,
  ) -> Result<Self, HalaGfxError> {
    let code = ash::util::read_spv(&mut std::fs::File::open(file_path)
      .map_err(|err| HalaGfxError::new(&format!("Failed to open shader file {}.", file_path), Some(Box::new(err))))?)
      .map_err(|err| HalaGfxError::new("Failed to read shader code.", Some(Box::new(err))))?;
    let module_create_info = vk::ShaderModuleCreateInfo::default()
      .code(&code);
    let module = unsafe {
      logical_device.borrow().raw.create_shader_module(&module_create_info, None)
        .map_err(|err| HalaGfxError::new("Failed to create shader module.", Some(Box::new(err))))?
    };
    logical_device.borrow().set_debug_name(module, debug_name)
      .map_err(|err| HalaGfxError::new("Failed to set debug name of shader module.", Some(Box::new(err))))?;

    log::debug!("A HalaShader \"{}\" is created.", debug_name);
    Ok(
      Self {
        logical_device,
        module,
        stage_flags: stage,
        ray_tracing_group_type: rt_group_type,
        debug_name: debug_name.to_string(),
      }
    )
  }
}