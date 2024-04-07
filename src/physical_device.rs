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
  pub features: vk::PhysicalDeviceFeatures,
  pub memory_properties: vk::PhysicalDeviceMemoryProperties,
  #[allow(dead_code)]
  pub(crate) queue_families: Vec<HalaQueueFamily>,
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
      chosen = Some((p, properties));
    }

    let (physical_device, properties) = chosen
      .ok_or_else(|| HalaGfxError::new("Failed to find a suitable physical device.", None))?;

    let (
      features,
      memory_properties,
      queue_families,
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
        features,
        memory_properties,
        queue_families,
        enable_buffer_device_address: features12.buffer_device_address == vk::TRUE,
        device_name,
      }
    )
  }

  /// Get the graphics family index.
  pub fn graphics_family(&self) -> u32 {
    self.queue_families.iter().position(|f| f.properties.queue_flags.contains(vk::QueueFlags::GRAPHICS)).unwrap() as u32
  }

  /// Get the compute family index.
  pub fn compute_family(&self) -> u32 {
    self.queue_families.iter().position(|f| f.properties.queue_flags.contains(vk::QueueFlags::COMPUTE)).unwrap() as u32
  }

  /// Get the transfer family index.
  pub fn transfer_family(&self) -> u32 {
    self.queue_families.iter().position(|f| f.properties.queue_flags.contains(vk::QueueFlags::TRANSFER)).unwrap() as u32
  }

  /// Get the max bind descriptor count.
  pub fn max_bind_descriptor_count(&self) -> usize {
    RESERVED_DESCRIPTOR_COUNT / 4
  }

  /// Get the max bindless descriptor count.
  pub fn max_bindless_descriptor_count(&self) -> usize {
    (512 * 510).min(
      (self.properties.limits.max_per_stage_descriptor_sampled_images as usize) - RESERVED_DESCRIPTOR_COUNT)
  }

  /// Get the dynamic constants alignment.
  pub fn dynamic_constants_alignment(&self) -> usize {
    (256).max(
      self.properties.limits.min_uniform_buffer_offset_alignment as usize)
  }

  /// Get the max dynamic constants bytes per dispatch.
  pub fn max_dynamic_constants_bytes_per_dispatch(&self) -> usize {
    (16384).min(
      self.properties.limits.max_uniform_buffer_range as usize)
  }

  /// Get the max dynamic constants storage buffer bytes.
  pub fn max_dynamic_constants_storage_buffer_bytes(&self) -> usize {
    1024 * 1024
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
    vk::PhysicalDeviceFeatures,
    vk::PhysicalDeviceMemoryProperties,
    Vec<HalaQueueFamily>,
    String
  ) {
    let features = unsafe { instance.raw.get_physical_device_features(physical_device) };
    let memory_properties = unsafe { instance.raw.get_physical_device_memory_properties(physical_device) };
    let queue_families = unsafe {
      instance.raw.get_physical_device_queue_family_properties(physical_device)
        .into_iter()
        .enumerate()
        .map(|(index, properties)| HalaQueueFamily { index: index as u32, properties })
        .collect::<Vec<_>>()
    };
    let device_name = unsafe { std::ffi::CStr::from_ptr(properties.device_name.as_ptr()).to_str().unwrap().to_owned() };
    (features, memory_properties, queue_families, device_name)
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
