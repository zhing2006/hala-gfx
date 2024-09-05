use ash::vk;

use crate::error::HalaGfxError;

/// The reserved descriptor count for the fixed descriptor sets.
pub const RESERVED_DESCRIPTOR_COUNT: usize = 1024;

/// The GPU queue family.
pub struct HalaQueueFamily {
  pub index: u32,
  pub properties: vk::QueueFamilyProperties,
}

/// The physical device.
pub struct HalaPhysicalDevice {
  pub raw: vk::PhysicalDevice,
  pub properties: vk::PhysicalDeviceProperties,
  pub memory_properties: vk::PhysicalDeviceMemoryProperties,
  #[allow(dead_code)]
  pub(crate) enable_buffer_device_address: bool,
  pub device_name: String,
}

/// The Debug trait implementation of the physical device.
impl std::fmt::Debug for HalaPhysicalDevice {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "HalaPhysicalDevice {{ {:#?} }}", self.properties)
  }
}

/// The implementation of the physical device.
impl HalaPhysicalDevice {
  /// Create a new physical device.
  /// param gpu_req: The GPU requirements.
  /// param instance: The instance.
  /// return: The physical device.
  pub fn new(gpu_req: &crate::HalaGPURequirements, instance: &crate::HalaInstance) -> Result<Self, HalaGfxError> {
    let phys_devs = unsafe {
      instance.raw.enumerate_physical_devices()
        .map_err(|err| HalaGfxError::new("Failed to enumerate physical devices.", Some(Box::new(err))))?
    };
    let mut chosen = None;
    for p in phys_devs.into_iter() {
      let properties = unsafe { instance.raw.get_physical_device_properties(p) };
      if gpu_req.is_gpu && match properties.device_type {
        vk::PhysicalDeviceType::DISCRETE_GPU => !gpu_req.gpu_types.contains(&crate::HalaGPUType::Discrete),
        vk::PhysicalDeviceType::INTEGRATED_GPU => !gpu_req.gpu_types.contains(&crate::HalaGPUType::Integrated),
        vk::PhysicalDeviceType::VIRTUAL_GPU => !gpu_req.gpu_types.contains(&crate::HalaGPUType::Virtual),
        _ => false,
      } {
        continue;
      }
      if gpu_req.is_gpu && properties.device_type == vk::PhysicalDeviceType::CPU {
        continue;
      }
      if properties.api_version < vk::make_api_version(0, gpu_req.version.0, gpu_req.version.1, gpu_req.version.2) {
        continue;
      }
      let device_name = unsafe { std::ffi::CStr::from_ptr(properties.device_name.as_ptr()).to_str().unwrap().to_lowercase() };
      let is_gpu_name_match = gpu_req.gpu_names.is_empty() || gpu_req.gpu_names.iter().any(|n| device_name.contains(n.to_lowercase().as_str()));
      if !is_gpu_name_match {
        continue;
      }
      chosen = Some((p, properties));
      // If we find a discrete GPU, we use it directly.
      if properties.device_type == vk::PhysicalDeviceType::DISCRETE_GPU {
        break;
      }
    }

    let (physical_device, properties) = chosen
      .ok_or_else(|| HalaGfxError::new("Failed to find a suitable physical device.", None))?;

    let (
      memory_properties,
      device_name
    ) = Self::get_device_info(instance, physical_device, &properties);

    let mut features11 = vk::PhysicalDeviceVulkan11Features::default();
    let mut features12 = vk::PhysicalDeviceVulkan12Features::default();
    let mut features13 = vk::PhysicalDeviceVulkan13Features::default();
    let mut features2 = vk::PhysicalDeviceFeatures2::default()
      .push_next(&mut features11)
      .push_next(&mut features12)
      .push_next(&mut features13);
    Self::get_device_info2(instance, physical_device, &mut features2);

    Ok(
      Self {
        raw: physical_device,
        properties,
        memory_properties,
        enable_buffer_device_address: features12.buffer_device_address == vk::TRUE,
        device_name,
      }
    )
  }

  pub(crate) fn find_memory_type_index(
    &self,
    memory_requset: &vk::MemoryRequirements,
    memory_property_flags: vk::MemoryPropertyFlags
  ) -> Option<u32> {
    (0..self.memory_properties.memory_type_count).find(
      |&i| memory_requset.memory_type_bits & (1 << i) != 0 &&
        self.memory_properties.memory_types[i as usize].property_flags.contains(memory_property_flags)
    )
  }

  /// Get the device informaton.
  /// param instance: The instance.
  /// param physical_device: The vk physical device.
  /// param properties: The vk physical device properties.
  /// return: The device information.
  fn get_device_info(
    instance: &crate::HalaInstance,
    physical_device: vk::PhysicalDevice,
    properties: &vk::PhysicalDeviceProperties
  ) -> (
    vk::PhysicalDeviceMemoryProperties,
    String
  ) {
    let memory_properties = unsafe { instance.raw.get_physical_device_memory_properties(physical_device) };
    let device_name = unsafe { std::ffi::CStr::from_ptr(properties.device_name.as_ptr()).to_str().unwrap().to_owned() };
    (memory_properties, device_name)
  }

  /// Get the device informaton2.
  /// param instance: The instance.
  /// return: The device information2.
  fn get_device_info2(
    instance: &crate::HalaInstance,
    physical_device: vk::PhysicalDevice,
    features2: &mut vk::PhysicalDeviceFeatures2,
  ) {
    unsafe {
      instance.raw.get_physical_device_features2(physical_device, features2);
    };
  }

}
