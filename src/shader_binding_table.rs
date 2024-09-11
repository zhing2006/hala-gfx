use ash::vk;

use crate::{
  HalaGfxError,
  HalaLogicalDevice,
  HalaBufferUsageFlags,
  HalaMemoryLocation,
  HalaBuffer,
  HalaCommandBufferSet,
  HalaShader,
  HalaRayTracingPipeline,
};

/// The shader binding table.
pub struct HalaShaderBindingTable {
  pub raygen_region: vk::StridedDeviceAddressRegionKHR,
  pub miss_region: vk::StridedDeviceAddressRegionKHR,
  pub hit_region: vk::StridedDeviceAddressRegionKHR,
  pub callable_region: vk::StridedDeviceAddressRegionKHR,
  pub buffer: HalaBuffer,
}

/// The AsRef implementation for shader binding table.
impl AsRef<HalaShaderBindingTable> for HalaShaderBindingTable {
  fn as_ref(&self) -> &Self {
    self
  }
}

/// The Drop implementation for shader binding table.
impl Drop for HalaShaderBindingTable {
  fn drop(&mut self) {
    log::debug!("The HalaShaderBindingTable is dropped.");
  }
}

/// The shader binding table implementation.
impl HalaShaderBindingTable {
  /// Create a new shader binding table.
  /// param logical_device: The logical device.
  /// param raygen_shaders: The ray generation shaders.
  /// param miss_shaders: The miss shaders.
  /// param hit_shaders: The hit shaders.
  /// param callable_shaders: The callable shaders.
  /// param pipeline: The ray tracing pipeline.
  /// param staging_buffer: The staging buffer.
  /// param transfer_command_buffers: The transfer command buffers.
  /// param debug_name: The debug name.
  /// return: The shader binding table.
  #[allow(clippy::too_many_arguments)]
  pub fn new<S>(
    logical_device: std::rc::Rc<std::cell::RefCell<HalaLogicalDevice>>,
    raygen_shaders: &[S],
    miss_shaders: &[S],
    hit_shaders: &[(Option<S>, Option<S>, Option<S>)],
    callable_shaders: &[S],
    pipeline: &HalaRayTracingPipeline,
    staging_buffer: &HalaBuffer,
    transfer_command_buffers: &HalaCommandBufferSet,
    debug_name: &str,
  ) -> Result<Self, HalaGfxError>
    where S: AsRef<HalaShader>
  {
    let (
      raygen_shader_count,
      miss_shader_count,
      hit_shader_count,
      callable_shader_count
    ) = (
      raygen_shaders.len() as u32,
      miss_shaders.len() as u32,
      hit_shaders.len() as u32,
      callable_shaders.len() as u32,
    );
    let group_count =
      raygen_shader_count +
      miss_shader_count +
      hit_shader_count +
      callable_shader_count;

    // Get the shader group handles.
    let (
      handle_size,
      handle_alignment,
      group_alignment,
    ) = {
      let logical_device = logical_device.borrow();
      (
        logical_device.shader_group_handle_size,
        logical_device.shader_group_handle_alignment,
        logical_device.shader_group_base_alignment,
      )
    };
    let aligned_handle_size = (handle_size + handle_alignment - 1) & !(handle_alignment - 1);
    let handle_pad = aligned_handle_size - handle_size;

    let data_size = handle_size * group_count;
    let handles = unsafe {
      let logical_device = logical_device.borrow();
      logical_device.ray_tracing_pipeline_loader.get_ray_tracing_shader_group_handles(
        pipeline.raw,
        0,
        group_count,
        data_size as usize,
      ).map_err(|err| HalaGfxError::new("Failed to get ray tracing shader group handles.", Some(Box::new(err))))?
    };

    // Calculate the region sizes.
    let raygen_region_size = raygen_shader_count * aligned_handle_size;
    let raygen_region_aligned_size = (raygen_region_size + group_alignment - 1) & !(group_alignment - 1);
    let miss_region_size = miss_shader_count * aligned_handle_size;
    let miss_region_aligned_size = (miss_region_size + group_alignment - 1) & !(group_alignment - 1);
    let hit_region_size = hit_shader_count * aligned_handle_size;
    let hit_region_aligned_size = (hit_region_size + group_alignment - 1) & !(group_alignment - 1);
    let callable_region_size = if callable_shader_count > 0 { callable_shader_count * aligned_handle_size } else { 0 };
    let callable_region_aligned_size = if callable_shader_count > 0 { (callable_region_size + group_alignment - 1) & !(group_alignment - 1) } else { 0 };

    // Create buffer.
    let buffer_size = raygen_region_size + miss_region_size + hit_region_size + callable_region_size;
    let mut stb_data = Vec::with_capacity(buffer_size as _);
    let mut offset = 0;
    for &(group_shader_count, group_size, group_aligned_size) in [
      (raygen_shader_count, raygen_region_size, raygen_region_aligned_size),
      (miss_shader_count, miss_region_size, miss_region_aligned_size),
      (hit_shader_count, hit_region_size, hit_region_aligned_size),
      (callable_shader_count, callable_region_size, callable_region_aligned_size),
    ].iter() {
      let group_pad = group_aligned_size - group_size;

      for _ in 0..group_shader_count {
        stb_data.extend_from_slice(&handles[offset..offset + handle_size as usize]);
        offset += handle_size as usize;
        stb_data.extend(std::iter::repeat(0u8).take(handle_pad as usize));
      }
      stb_data.extend(std::iter::repeat(0u8).take(group_pad as usize));
    }

    let buffer = HalaBuffer::new(
      std::rc::Rc::clone(&logical_device),
      stb_data.len() as _,
        HalaBufferUsageFlags::SHADER_DEVICE_ADDRESS | HalaBufferUsageFlags::SHADER_BINDING_TABLE | HalaBufferUsageFlags::TRANSFER_DST,
        HalaMemoryLocation::GpuOnly,
        &format!("{}_buffer", debug_name)
    )?;
    buffer.update_gpu_memory_with_buffer(&stb_data, staging_buffer, transfer_command_buffers)?;
    let address = buffer.get_device_address();
    let raygen_region = vk::StridedDeviceAddressRegionKHR::default()
      .device_address(address)
      .size(raygen_region_aligned_size as _)
      .stride(raygen_region_aligned_size as _);
    let miss_region = vk::StridedDeviceAddressRegionKHR::default()
      .device_address(address + raygen_region.size)
      .size(miss_region_aligned_size as _)
      .stride(aligned_handle_size as _);
    let hit_region = vk::StridedDeviceAddressRegionKHR::default()
      .device_address(address + raygen_region.size + miss_region.size)
      .size(hit_region_aligned_size as _)
      .stride(aligned_handle_size as _);
    let callable_region = if callable_shader_count > 0 {
      vk::StridedDeviceAddressRegionKHR::default()
        .device_address(address + raygen_region.size + miss_region.size + hit_region.size)
        .size(callable_region_aligned_size as _)
        .stride(aligned_handle_size as _)
    } else {
      vk::StridedDeviceAddressRegionKHR::default()
    };

    log::debug!("The HalaShaderBindingTable is created.");
    Ok(Self {
      buffer,
      raygen_region,
      miss_region,
      hit_region,
      callable_region,
    })
  }
}