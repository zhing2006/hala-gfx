use ash::vk;

use crate::{
  HalaCommandBufferSet,
  HalaCommandBufferUsageFlags,
  HalaGfxError,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum HalaMemoryLocation {
  Unknown,
  GpuOnly,
  CpuToGpu,
  GpuToCpu,
}

impl std::convert::From<gpu_allocator::MemoryLocation> for HalaMemoryLocation {
  fn from(location: gpu_allocator::MemoryLocation) -> Self {
    match location {
      gpu_allocator::MemoryLocation::Unknown => Self::Unknown,
      gpu_allocator::MemoryLocation::GpuOnly => Self::GpuOnly,
      gpu_allocator::MemoryLocation::CpuToGpu => Self::CpuToGpu,
      gpu_allocator::MemoryLocation::GpuToCpu => Self::GpuToCpu,
    }
  }
}

impl std::convert::From<HalaMemoryLocation> for gpu_allocator::MemoryLocation {
  fn from(location: HalaMemoryLocation) -> Self {
    match location {
      HalaMemoryLocation::Unknown => Self::Unknown,
      HalaMemoryLocation::GpuOnly => Self::GpuOnly,
      HalaMemoryLocation::CpuToGpu => Self::CpuToGpu,
      HalaMemoryLocation::GpuToCpu => Self::GpuToCpu,
    }
  }
}

/// The logical device.
pub struct HalaLogicalDevice {
  pub raw: ash::Device,
  pub debug_utils_loader: Option<ash::ext::debug_utils::Device>,
  pub mesh_shader_loader: ash::ext::mesh_shader::Device,
  pub graphics_queue_family_index: u32,
  pub transfer_queue_family_index: u32,
  pub compute_queue_family_index: u32,

  pub gpu_allocator: std::mem::ManuallyDrop<gpu_allocator::vulkan::Allocator>,

  pub acceleration_structure_loader: ash::khr::acceleration_structure::Device,
  pub deferred_host_operations_loader: ash::khr::deferred_host_operations::Device,
  pub ray_tracing_pipeline_loader: ash::khr::ray_tracing_pipeline::Device,
  pub min_acceleration_structure_scratch_offset_alignment: u32,
  pub max_ray_recursion_depth: u32,
  pub shader_group_handle_size: u32,
  pub shader_group_handle_alignment: u32,
  pub shader_group_base_alignment: u32,
}

/// The Drop trait implementation of the logical device.
impl Drop for HalaLogicalDevice {
  fn drop(&mut self) {
    unsafe {
      std::mem::ManuallyDrop::drop(&mut self.gpu_allocator);
      self.raw.destroy_device(None);
    }
    log::debug!("A HalaLogicalDevice is dropped.");
  }
}

/// The implementation of the logical device.
impl HalaLogicalDevice {
  /// Create a logical device.
  /// param gpu_req: The GPU requirements.
  /// param instance: The instance.
  /// param physical_device: The physical device.
  /// param graphics_queue_family_index: The graphics queue family index.
  /// param transfer_queue_family_index: The transfer queue family index.
  /// param compute_queue_family_index: The compute queue family index.
  /// return: The logical device.
  pub fn new(
    gpu_req: &crate::HalaGPURequirements,
    instance: &crate::HalaInstance,
    physical_device: &crate::HalaPhysicalDevice,
    surface: &crate::HalaSurface,
  ) -> Result<Self, HalaGfxError> {
    // Find queue family indices.
    let (
      (graphics_queue_family_index, graphics_queue_count),
      (transfer_queue_family_index, transfer_queue_count),
      (compute_queue_family_index, compute_queue_count),
    ) = Self::find_queue_family_indices(
      instance,
      physical_device,
      surface
    )?;
    log::debug!("Queue family indices: graphics: {}, transfer: {}, compute: {}",
      graphics_queue_family_index, transfer_queue_family_index, compute_queue_family_index);

    // Create logical device.
    let logical_device = Self::create_logical_device(
      gpu_req,
      instance,
      physical_device,
      (
        (
          graphics_queue_family_index,
          graphics_queue_count,
        ),
        (
          transfer_queue_family_index,
          transfer_queue_count,
        ),
        (
          compute_queue_family_index,
          compute_queue_count,
        ),
      )
    )?;

    // Create ray tracing objects.
    let (
      acceleration_structure,
      deferred_host_operations,
      ray_tracing_pipeline,
    ) = Self::get_ray_tracing_info(instance, &logical_device);

    let (
      acceleration_structure_properties,
      _acceleration_structure_features,
      ray_tracing_pipeline_properties,
      _ray_tracing_pipeline_features,
    ) = Self::get_ray_tracing_features(instance, physical_device);

    let gpu_allocator = Self::create_gpu_allocator(
      instance,
      &logical_device,
      physical_device,
      gpu_allocator::AllocationSizes::default(),
    )?;

    log::debug!("A HalaLogicalDevice is created.");
    Ok(
      Self {
        raw: logical_device.clone(),
        debug_utils_loader: if cfg!(debug_assertions) {
          Some(ash::ext::debug_utils::Device::new(&instance.raw, &logical_device))
        } else {
          None
        },
        mesh_shader_loader: ash::ext::mesh_shader::Device::new(&instance.raw, &logical_device),
        graphics_queue_family_index,
        transfer_queue_family_index,
        compute_queue_family_index,
        gpu_allocator: std::mem::ManuallyDrop::new(gpu_allocator),
        acceleration_structure_loader: acceleration_structure,
        deferred_host_operations_loader: deferred_host_operations,
        ray_tracing_pipeline_loader: ray_tracing_pipeline,

        min_acceleration_structure_scratch_offset_alignment: acceleration_structure_properties.min_acceleration_structure_scratch_offset_alignment,
        max_ray_recursion_depth: ray_tracing_pipeline_properties.max_ray_recursion_depth,
        shader_group_handle_size: ray_tracing_pipeline_properties.shader_group_handle_size,
        shader_group_handle_alignment: ray_tracing_pipeline_properties.shader_group_handle_alignment,
        shader_group_base_alignment: ray_tracing_pipeline_properties.shader_group_base_alignment,
      }
    )
  }

  /// Get a queue.
  /// param queue_family_index: The queue family index.
  /// param queue_index: The queue index.
  /// return: The queue.
  pub fn get_queue(&self, queue_family_index: u32, queue_index: u32) -> vk::Queue {
    unsafe {
      self.raw.get_device_queue(queue_family_index, queue_index)
    }
  }

  /// Get a graphics queue.
  /// param queue_index: The queue index.
  /// return: The queue.
  pub fn get_graphics_queue(&self, queue_index: u32) -> vk::Queue {
    self.get_queue(self.graphics_queue_family_index, queue_index)
  }

  /// Get a transfer queue.
  /// param queue_index: The queue index.
  /// return: The queue.
  pub fn get_transfer_queue(&self, queue_index: u32) -> vk::Queue {
    self.get_queue(self.transfer_queue_family_index, queue_index)
  }

  /// Get a compute queue.
  /// param queue_index: The queue index.
  /// return: The queue.
  pub fn get_compute_queue(&self, queue_index: u32) -> vk::Queue {
    self.get_queue(self.compute_queue_family_index, queue_index)
  }

  /// Wait the logical device idle.
  pub fn wait_idle(&self) -> Result<(), HalaGfxError> {
    unsafe {
      self.raw.device_wait_idle()
        .map_err(|err| HalaGfxError::new("Failed to wait idle.", Some(Box::new(err))))?;
    }
    Ok(())
  }

  /// Set debug name.
  /// param handle: The vk object handle.
  /// param name: The name.
  /// return: The result.
  pub fn set_debug_name<T: vk::Handle>(&self, handle: T, name: &str) -> Result<(), HalaGfxError> {
    let name = std::ffi::CString::new(name).unwrap();
    let info = vk::DebugUtilsObjectNameInfoEXT::default()
      .object_handle(handle)
      .object_name(&name);
    unsafe {
      if let Some(debug_utils_loader) = &self.debug_utils_loader {
        debug_utils_loader.set_debug_utils_object_name(&info)
          .map_err(|err| HalaGfxError::new("Failed to set debug name.", Some(Box::new(err))))?;
      }
    }
    Ok(())
  }

  /// Execute and submit a transfer command buffer.
  /// param command_buffers: The transfer command buffer set.
  /// param buffer_index: The buffer index.
  /// param recording_fn: The recording function.
  /// param queue_index: The queue index.
  /// return: The result.
  pub fn transfer_execute_and_submit<F: FnOnce(&HalaLogicalDevice, &HalaCommandBufferSet, usize)>(
    &self,
    command_buffers: &HalaCommandBufferSet,
    buffer_index: usize,
    recording_fn: F,
    queue_index: u32,
  ) -> Result<(), HalaGfxError> {
    self.execute_and_submit(command_buffers, buffer_index, recording_fn, self.get_transfer_queue(queue_index))
  }

  /// Execute and submit a graphics command buffer.
  /// param command_buffers: The graphics command buffer set.
  /// param buffer_index: The buffer index.
  /// param recording_fn: The recording function.
  /// param queue_index: The queue index.
  /// return: The result.
  pub fn graphics_execute_and_submit<F: FnOnce(&HalaLogicalDevice, &HalaCommandBufferSet, usize)>(
    &self,
    command_buffers: &HalaCommandBufferSet,
    buffer_index: usize,
    recording_fn: F,
    queue_index: u32,
  ) -> Result<(), HalaGfxError> {
    self.execute_and_submit(command_buffers, buffer_index, recording_fn, self.get_graphics_queue(queue_index))
  }

  /// Execute and submit a compute command buffer.
  /// param command_buffers: The compute command buffer set.
  /// param buffer_index: The buffer index.
  /// param recording_fn: The recording function.
  /// param queue_index: The queue index.
  /// return: The result.
  pub fn compute_execute_and_submit<F: FnOnce(&HalaLogicalDevice, &HalaCommandBufferSet, usize)>(
    &self,
    command_buffers: &HalaCommandBufferSet,
    buffer_index: usize,
    recording_fn: F,
    queue_index: u32,
  ) -> Result<(), HalaGfxError> {
    self.execute_and_submit(command_buffers, buffer_index, recording_fn, self.get_compute_queue(queue_index))
  }

  /// Execute and submit a command buffer.
  /// param command_buffers: The command buffer set.
  /// param index: The buffer index.
  /// param recording_fn: The recording function.
  /// return: The result.
  pub fn execute_and_submit<F: FnOnce(&HalaLogicalDevice, &HalaCommandBufferSet, usize)>(
    &self,
    command_buffers: &HalaCommandBufferSet,
    index: usize,
    recording_fn: F,
    queue: vk::Queue,
  ) -> Result<(), HalaGfxError> {
    command_buffers.begin(index, HalaCommandBufferUsageFlags::ONE_TIME_SUBMIT)?;
    recording_fn(self, command_buffers, index);
    command_buffers.end(index)?;

    self.submit(command_buffers, index, queue)?;
    self.wait(queue)?;

    command_buffers.reset(index, false)?;

    Ok(())
  }

  /// Submit a graphics command buffer.
  /// param command_buffers: The graphics command buffer set.
  /// param index: The buffer index.
  /// param queue_index: The queue index.
  /// return: The result.
  pub fn graphics_submit(
    &self,
    command_buffers: &HalaCommandBufferSet,
    index: usize,
    queue_index: u32,
  ) -> Result<(), HalaGfxError> {
    self.submit(command_buffers, index, self.get_graphics_queue(queue_index))
  }

  /// Submit a transfer command buffer.
  /// param command_buffers: The transfer command buffer set.
  /// param index: The buffer index.
  /// param queue_index: The queue index.
  /// return: The result.
  pub fn transfer_submit(
    &self,
    command_buffers: &HalaCommandBufferSet,
    index: usize,
    queue_index: u32,
  ) -> Result<(), HalaGfxError> {
    self.submit(command_buffers, index, self.get_transfer_queue(queue_index))
  }

  /// Submit a compute command buffer.
  /// param command_buffers: The compute command buffer set.
  /// param index: The buffer index.
  /// param queue_index: The queue index.
  pub fn compute_submit(
    &self,
    command_buffers: &HalaCommandBufferSet,
    index: usize,
    queue_index: u32,
  ) -> Result<(), HalaGfxError> {
    self.submit(command_buffers, index, self.get_compute_queue(queue_index))
  }

  /// Submit a command buffer.
  /// param command_buffers: The command buffer set.
  /// param index: The buffer index.
  /// param queue: The queue.
  /// return: The result.
  fn submit(
    &self,
    command_buffers: &HalaCommandBufferSet,
    index: usize,
    queue: vk::Queue,
  ) -> Result<(), HalaGfxError> {
    let submit_info = vk::SubmitInfo::default()
      .command_buffers(std::slice::from_ref(&command_buffers.raw[index]));

    unsafe {
      self.raw.queue_submit(queue, &[submit_info], vk::Fence::null())
        .map_err(|err| HalaGfxError::new("Failed to submit queue.", Some(Box::new(err))))?;
    }

    Ok(())
  }

  /// Wait a graphics queue.
  /// param queue_index: The queue index.
  /// return: The result.
  pub fn graphics_wait(
    &self,
    queue_index: u32,
  ) -> Result<(), HalaGfxError> {
    self.wait(self.get_graphics_queue(queue_index))
  }

  /// Wait a transfer queue.
  /// param queue_index: The queue index.
  /// return: The result.
  pub fn transfer_wait(
    &self,
    queue_index: u32,
  ) -> Result<(), HalaGfxError> {
    self.wait(self.get_transfer_queue(queue_index))
  }

  /// Wait a compute queue.
  /// param queue_index: The queue index.
  /// return: The result.
  pub fn compute_wait(
    &self,
    queue_index: u32,
  ) -> Result<(), HalaGfxError> {
    self.wait(self.get_compute_queue(queue_index))
  }

  /// Wait a queue.
  /// param queue: The queue.
  /// return: The result.
  fn wait(
    &self,
    queue: vk::Queue,
  ) -> Result<(), HalaGfxError> {
    unsafe {
      self.raw.queue_wait_idle(queue)
        .map_err(|err| HalaGfxError::new("Failed to wait queue idle.", Some(Box::new(err))))?;
    }

    Ok(())
  }

  /// Find queue family indices.
  /// param instance: The instance.
  /// param physical_device: The physical device.
  /// param surface: The surface.
  /// return: The queue family index and queue count pairs.
  fn find_queue_family_indices(
    instance: &crate::HalaInstance,
    physical_device: &crate::HalaPhysicalDevice,
    surface: &crate::HalaSurface
  ) -> Result<((u32, u32), (u32, u32), (u32, u32)), HalaGfxError> {
    let queue_family_properties = unsafe { instance.raw.get_physical_device_queue_family_properties(physical_device.raw) };
    let queue_family_pairs = {
      let mut found_graphics_q_index = None;
      let mut found_transfer_q_index = None;
      let mut found_compute_q_index = None;
      let mut found_graphics_q_count = 0;
      let mut found_transfer_q_count = 0;
      let mut found_compute_q_count = 0;
      for (index, queue_family) in queue_family_properties.iter().enumerate() {
        if queue_family.queue_count > 0 &&
            queue_family.queue_flags.contains(vk::QueueFlags::GRAPHICS) &&
            unsafe {
              surface.surface_loader.get_physical_device_surface_support(
              physical_device.raw,
              index as u32,
              surface.raw).unwrap_or(false)
            } && (
              found_graphics_q_index.is_none() ||
              queue_family.queue_count > found_graphics_q_count
            )
        {
          found_graphics_q_index = Some(index as u32);
          found_graphics_q_count = queue_family.queue_count;
        }
        if queue_family.queue_count > 0 &&
            queue_family.queue_flags.contains(vk::QueueFlags::TRANSFER) && (
              found_transfer_q_index.is_none() ||
              (found_transfer_q_index.unwrap() == 0 || queue_family.queue_count > found_transfer_q_count) && (
                !queue_family.queue_flags.contains(vk::QueueFlags::GRAPHICS) &&
                !queue_family.queue_flags.contains(vk::QueueFlags::OPTICAL_FLOW_NV) &&
                !queue_family.queue_flags.contains(vk::QueueFlags::VIDEO_DECODE_KHR) &&
                !queue_family.queue_flags.contains(vk::QueueFlags::VIDEO_ENCODE_KHR)
              )
            )
        {
          found_transfer_q_index = Some(index as u32);
          found_transfer_q_count = queue_family.queue_count;
        }
        if queue_family.queue_count > 0 &&
            queue_family.queue_flags.contains(vk::QueueFlags::COMPUTE) && (
              found_compute_q_index.is_none() ||
              (found_compute_q_index.unwrap() == 0 || queue_family.queue_count > found_compute_q_count) && (
                !queue_family.queue_flags.contains(vk::QueueFlags::GRAPHICS) &&
                !queue_family.queue_flags.contains(vk::QueueFlags::OPTICAL_FLOW_NV) &&
                !queue_family.queue_flags.contains(vk::QueueFlags::VIDEO_DECODE_KHR) &&
                !queue_family.queue_flags.contains(vk::QueueFlags::VIDEO_ENCODE_KHR)
              )
            )
        {
          found_compute_q_index = Some(index as u32);
          found_compute_q_count = queue_family.queue_count;
        }
      }
      (
        (
          found_graphics_q_index
            .ok_or_else(|| HalaGfxError::new("Failed to find a graphics queue.", None))?,
          found_graphics_q_count
        ),
        (
          found_transfer_q_index
            .ok_or_else(|| HalaGfxError::new("Failed to find a transfer queue.", None))?,
          found_transfer_q_count
        ),
        (
          found_compute_q_index
            .ok_or_else(|| HalaGfxError::new("Failed to find a compute queue.", None))?,
          found_compute_q_count
        ),
      )
    };
    Ok(queue_family_pairs)
  }

  /// Create a logical device.
  /// param gpu_req: The GPU requirements.
  /// param instance: The instance.
  /// param physical_device: The physical device.
  /// param queue_family_pairs: The queue family pairs.
  /// return: The logical device.
  fn create_logical_device(
    gpu_req: &crate::HalaGPURequirements,
    instance: &crate::HalaInstance,
    physical_device: &crate::HalaPhysicalDevice,
    queue_family_pairs: ((u32, u32), (u32, u32), (u32, u32))) -> Result<ash::Device, HalaGfxError>
  {
    let (
      (graphics_queue_family_index, graphics_queue_count),
      (transfer_queue_family_index, transfer_queue_count),
      (compute_queue_family_index, compute_queue_count),
    ) = queue_family_pairs;
    let graphics_priorities = (0..graphics_queue_count)
      .map(|i| (graphics_queue_count as f32 - i as f32) / graphics_queue_count as f32)
      .collect::<Vec<_>>();
    let transfer_priorities = (0..transfer_queue_count)
      .map(|i| (transfer_queue_count as f32 - i as f32) / transfer_queue_count as f32)
      .collect::<Vec<_>>();
    let compute_priorities = (0..compute_queue_count)
      .map(|i| (compute_queue_count as f32 - i as f32) / compute_queue_count as f32)
      .collect::<Vec<_>>();
    let mut queue_infos = vec![
      vk::DeviceQueueCreateInfo::default()
        .queue_family_index(graphics_queue_family_index)
        .queue_priorities(graphics_priorities.as_slice()),
    ];
    if graphics_queue_family_index != transfer_queue_family_index {
      queue_infos.push(
        vk::DeviceQueueCreateInfo::default()
          .queue_family_index(transfer_queue_family_index)
          .queue_priorities(transfer_priorities.as_slice())
      );
    }
    if graphics_queue_family_index != compute_queue_family_index && transfer_queue_family_index != compute_queue_family_index {
      queue_infos.push(
        vk::DeviceQueueCreateInfo::default()
          .queue_family_index(compute_queue_family_index)
          .queue_priorities(compute_priorities.as_slice())
      );
    }
    let mut extension_name_ptrs =  vec![
      ash::khr::spirv_1_4::NAME.as_ptr(),
      ash::khr::swapchain::NAME.as_ptr(),
      ash::khr::maintenance1::NAME.as_ptr(),
      ash::khr::maintenance2::NAME.as_ptr(),
      ash::khr::maintenance3::NAME.as_ptr(),
      ash::khr::maintenance4::NAME.as_ptr(),
      ash::ext::descriptor_indexing::NAME.as_ptr(),
      ash::khr::synchronization2::NAME.as_ptr(),
      ash::khr::shader_float_controls::NAME.as_ptr(),
      ash::khr::shader_atomic_int64::NAME.as_ptr(),
      ash::ext::shader_atomic_float::NAME.as_ptr(),
      ash::ext::shader_image_atomic_int64::NAME.as_ptr(),
      ash::khr::buffer_device_address::NAME.as_ptr(),
    ];
    if !cfg!(debug_assertions) {
      // These extensions are cause nSight stop working.
      // So only enable them in release mode.
      extension_name_ptrs.push(ash::khr::maintenance5::NAME.as_ptr());
      extension_name_ptrs.push(ash::khr::maintenance6::NAME.as_ptr());
      extension_name_ptrs.push(ash::khr::shader_float_controls2::NAME.as_ptr());
    };
    if gpu_req.require_mesh_shader {
      extension_name_ptrs.push(ash::ext::mesh_shader::NAME.as_ptr());
      extension_name_ptrs.push(ash::khr::fragment_shading_rate::NAME.as_ptr());
    }
    if gpu_req.require_ray_tracing {
      extension_name_ptrs.push(ash::khr::acceleration_structure::NAME.as_ptr());
      extension_name_ptrs.push(ash::khr::deferred_host_operations::NAME.as_ptr());
      extension_name_ptrs.push(ash::khr::ray_tracing_pipeline::NAME.as_ptr());
      // extension_name_ptrs.push(ash::khr::ray_tracing_maintenance1::NAME.as_ptr());
      extension_name_ptrs.push(ash::ext::scalar_block_layout::NAME.as_ptr());
    }
    log::debug!("Extension names: {:?}", extension_name_ptrs.iter().map(|&ptr| unsafe { std::ffi::CStr::from_ptr(ptr) }).collect::<Vec<_>>() );

    let mut descriptor_indexing_features =
      vk::PhysicalDeviceDescriptorIndexingFeaturesEXT::default();
    let mut buffer_device_address_features =
      vk::PhysicalDeviceBufferDeviceAddressFeaturesKHR::default();
    let mut scalar_block_layout_features =
      vk::PhysicalDeviceScalarBlockLayoutFeatures::default();
    let mut dynamic_rendering_features =
      vk::PhysicalDeviceDynamicRenderingFeatures::default();
    let mut synchronization2_features =
      vk::PhysicalDeviceSynchronization2FeaturesKHR::default();
    let mut shader_demote_to_helper_invocation_features =
      vk::PhysicalDeviceShaderDemoteToHelperInvocationFeatures::default()
        .shader_demote_to_helper_invocation(true);
    let mut timeline_semaphore_features =
      vk::PhysicalDeviceTimelineSemaphoreFeatures::default()
        .timeline_semaphore(true);
    let mut mesh_shader_features = vk::PhysicalDeviceMeshShaderFeaturesEXT::default()
      .mesh_shader(true)
      .task_shader(true)
      .multiview_mesh_shader(false)
      .primitive_fragment_shading_rate_mesh_shader(true);
    if cfg!(debug_assertions) && physical_device.features.pipeline_statistics_query == vk::TRUE {
      mesh_shader_features = mesh_shader_features.mesh_shader_queries(true);
    }
    let mut multiview_features = vk::PhysicalDeviceMultiviewFeatures::default()
      .multiview(false);
    let mut primitive_fragment_shading_rate_features = vk::PhysicalDeviceFragmentShadingRateFeaturesKHR::default()
      .pipeline_fragment_shading_rate(false)
      .primitive_fragment_shading_rate(false)
      .attachment_fragment_shading_rate(false);
    let mut ray_tracing_pipeline_features =
      vk::PhysicalDeviceRayTracingPipelineFeaturesKHR::default();
    let mut acceleration_structure_features =
      vk::PhysicalDeviceAccelerationStructureFeaturesKHR::default();
    let mut features2 = vk::PhysicalDeviceFeatures2::default()
      .push_next(&mut descriptor_indexing_features)
      .push_next(&mut buffer_device_address_features)
      .push_next(&mut scalar_block_layout_features)
      .push_next(&mut dynamic_rendering_features)
      .push_next(&mut synchronization2_features)
      .push_next(&mut shader_demote_to_helper_invocation_features)
      .push_next(&mut timeline_semaphore_features);
    if gpu_req.require_mesh_shader {
      features2 = features2
        .push_next(&mut mesh_shader_features)
        .push_next(&mut multiview_features)
        .push_next(&mut primitive_fragment_shading_rate_features);
    }
    if gpu_req.require_ray_tracing {
      features2 = features2
        .push_next(&mut ray_tracing_pipeline_features)
        .push_next(&mut acceleration_structure_features);
    }
    unsafe {
      instance.raw.get_physical_device_features2(physical_device.raw, &mut features2);
    }
    log::debug!("Features2: {:?}", features2);

    let device_create_infos = vk::DeviceCreateInfo::default()
      .queue_create_infos(&queue_infos)
      .enabled_extension_names(&extension_name_ptrs)
      .push_next(&mut features2);
    let logical_device = unsafe {
      instance.raw.create_device(physical_device.raw, &device_create_infos, None)
        .map_err(|err| HalaGfxError::new("Failed to create logical device.", Some(Box::new(err))))?
    };
    Ok(logical_device)
  }

  /// Get ray tracing information.
  /// param instance: The instance.
  /// param logical_device: The ash logical device.
  /// return: The ray tracing information.
  fn get_ray_tracing_info(
    instance: &crate::HalaInstance,
    logical_device: &ash::Device,
  ) -> (
    ash::khr::acceleration_structure::Device,
    ash::khr::deferred_host_operations::Device,
    ash::khr::ray_tracing_pipeline::Device,
  ) {
    let acceleration_structure = ash::khr::acceleration_structure::Device::new(
      &instance.raw,
      logical_device,
    );
    let deferred_host_operations = ash::khr::deferred_host_operations::Device::new(
      &instance.raw,
      logical_device,
    );
    let ray_tracing_pipeline = ash::khr::ray_tracing_pipeline::Device::new(
      &instance.raw,
      logical_device,
    );

    (
      acceleration_structure,
      deferred_host_operations,
      ray_tracing_pipeline,
    )
  }

  /// Get ray tracing features.
  /// param instance: The instance.
  /// param physical_device: The physical device.
  /// return: The ray tracing features.
  fn get_ray_tracing_features<'a>(
    instance: &crate::HalaInstance,
    physical_device: &crate::HalaPhysicalDevice,
  ) -> (
    vk::PhysicalDeviceAccelerationStructurePropertiesKHR<'a>,
    vk::PhysicalDeviceAccelerationStructureFeaturesKHR<'a>,
    vk::PhysicalDeviceRayTracingPipelinePropertiesKHR<'a>,
    vk::PhysicalDeviceRayTracingPipelineFeaturesKHR<'a>,
  ) {
    let acceleration_structure_properties = unsafe {
      let mut acceleration_structure_properties =
        vk::PhysicalDeviceAccelerationStructurePropertiesKHR::default();
      let mut properties2 = vk::PhysicalDeviceProperties2::default()
        .push_next(&mut acceleration_structure_properties);
      instance.raw.get_physical_device_properties2(physical_device.raw, &mut properties2);
      acceleration_structure_properties
    };
    let acceleration_structure_features = unsafe {
      let mut acceleration_structure_features =
        vk::PhysicalDeviceAccelerationStructureFeaturesKHR::default();
      let mut features2 = vk::PhysicalDeviceFeatures2::default()
        .push_next(&mut acceleration_structure_features);
      instance.raw.get_physical_device_features2(physical_device.raw, &mut features2);
      acceleration_structure_features
    };
    let ray_tracing_pipeline_properties = unsafe {
      let mut ray_tracing_pipeline_properties =
        vk::PhysicalDeviceRayTracingPipelinePropertiesKHR::default();
      let mut properties2 = vk::PhysicalDeviceProperties2::default()
        .push_next(&mut ray_tracing_pipeline_properties);
      instance.raw.get_physical_device_properties2(physical_device.raw, &mut properties2);
      ray_tracing_pipeline_properties
    };
    let ray_tracing_pipeline_features = unsafe {
      let mut ray_tracing_pipeline_features =
        vk::PhysicalDeviceRayTracingPipelineFeaturesKHR::default();
      let mut features2 = vk::PhysicalDeviceFeatures2::default()
        .push_next(&mut ray_tracing_pipeline_features);
      instance.raw.get_physical_device_features2(physical_device.raw, &mut features2);
      ray_tracing_pipeline_features
    };

    (
      acceleration_structure_properties,
      acceleration_structure_features,
      ray_tracing_pipeline_properties,
      ray_tracing_pipeline_features,
    )
  }

  /// Create a GPU allocator.
  /// param instance: The instance.
  /// param logical_device: The ash logical device.
  /// param physical_device: The physical device.
  /// param allocation_sizes: The allocation sizes.
  /// return: The GPU allocator.
  fn create_gpu_allocator(
    instance: &crate::HalaInstance,
    logical_device: &ash::Device,
    physical_device: &crate::HalaPhysicalDevice,
    allocation_sizes: gpu_allocator::AllocationSizes,
  ) -> Result<gpu_allocator::vulkan::Allocator, HalaGfxError> {
    let gpu_allocator = gpu_allocator::vulkan::Allocator::new(
      &gpu_allocator::vulkan::AllocatorCreateDesc {
        instance: instance.raw.clone(),
        device: logical_device.clone(),
        physical_device: physical_device.raw,
        debug_settings: if cfg!(debug_assertions) {
          gpu_allocator::AllocatorDebugSettings {
            log_leaks_on_shutdown: true,
            log_memory_information: true,
            log_allocations: true,
            log_frees: true,
            log_stack_traces: false,
            ..Default::default()
          }
        } else {
          gpu_allocator::AllocatorDebugSettings::default()
        },
        buffer_device_address: physical_device.enable_buffer_device_address,
        allocation_sizes,
      }
    ).map_err(|err| HalaGfxError::new("Failed to create GPU allocator.", Some(Box::new(err))))?;

    Ok(gpu_allocator)
  }
}
