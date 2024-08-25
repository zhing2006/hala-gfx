use std::rc::Rc;
use std::cell::RefCell;

use ash::vk;

use crate::{
  HalaCommandBufferSet, HalaCommandBufferType, HalaGfxError, HalaLogicalDevice, HalaMemoryLocation
};

/// The buffer usage flags.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HalaBufferUsageFlags(u32);
crate::hala_bitflags_wrapped!(HalaBufferUsageFlags, u32);
impl HalaBufferUsageFlags {
  pub const TRANSFER_SRC: Self = Self(vk::BufferUsageFlags::TRANSFER_SRC.as_raw());
  pub const TRANSFER_DST: Self = Self(vk::BufferUsageFlags::TRANSFER_DST.as_raw());
  pub const UNIFORM_TEXEL_BUFFER: Self = Self(vk::BufferUsageFlags::UNIFORM_TEXEL_BUFFER.as_raw());
  pub const STORAGE_TEXEL_BUFFER: Self = Self(vk::BufferUsageFlags::STORAGE_TEXEL_BUFFER.as_raw());
  pub const UNIFORM_BUFFER: Self = Self(vk::BufferUsageFlags::UNIFORM_BUFFER.as_raw());
  pub const STORAGE_BUFFER: Self = Self(vk::BufferUsageFlags::STORAGE_BUFFER.as_raw());
  pub const INDEX_BUFFER: Self = Self(vk::BufferUsageFlags::INDEX_BUFFER.as_raw());
  pub const VERTEX_BUFFER: Self = Self(vk::BufferUsageFlags::VERTEX_BUFFER.as_raw());
  pub const INDIRECT_BUFFER: Self = Self(vk::BufferUsageFlags::INDIRECT_BUFFER.as_raw());
  pub const SHADER_DEVICE_ADDRESS: Self = Self(vk::BufferUsageFlags::SHADER_DEVICE_ADDRESS.as_raw());
  pub const ACCELERATION_STRUCTURE_STORAGE: Self = Self(vk::BufferUsageFlags::ACCELERATION_STRUCTURE_STORAGE_KHR.as_raw());
  pub const ACCELERATION_STRUCTURE_BUILD_INPUT_READ_ONLY: Self = Self(vk::BufferUsageFlags::ACCELERATION_STRUCTURE_BUILD_INPUT_READ_ONLY_KHR.as_raw());
  pub const SHADER_BINDING_TABLE: Self = Self(vk::BufferUsageFlags::SHADER_BINDING_TABLE_KHR.as_raw());
}

impl std::convert::From<vk::BufferUsageFlags> for HalaBufferUsageFlags {
  fn from(flags: vk::BufferUsageFlags) -> Self {
    Self(flags.as_raw())
  }
}

impl std::convert::From<HalaBufferUsageFlags> for vk::BufferUsageFlags {
  fn from(flags: HalaBufferUsageFlags) -> Self {
    vk::BufferUsageFlags::from_raw(flags.0)
  }
}

/// The buffer.
pub struct HalaBuffer {
  pub(crate) logical_device: Rc<RefCell<HalaLogicalDevice>>,
  pub raw: vk::Buffer,
  pub memory_requirements: vk::MemoryRequirements,
  pub allocation: gpu_allocator::vulkan::Allocation,
  pub memory_location: gpu_allocator::MemoryLocation,
  pub size: u64,
  pub(crate) debug_name: String,
}

/// The AsRef trait implementation of the buffer.
impl AsRef<HalaBuffer> for HalaBuffer {
  fn as_ref(&self) -> &HalaBuffer {
    self
  }
}

/// The Drop trait implementation of the buffer.
impl Drop for HalaBuffer {
  fn drop(&mut self) {
    unsafe {
      let mut logical_device = self.logical_device.borrow_mut();
      let allocation = std::mem::take(&mut self.allocation);
      logical_device.gpu_allocator.free(allocation).unwrap();
      logical_device.raw.destroy_buffer(self.raw, None);
    }
    log::debug!("A HalaBuffer \"{}\" is dropped.", self.debug_name);
  }
}

/// The buffer implementation.
impl HalaBuffer {
  /// Create a buffer with dedicated memory.
  /// param logical_device: The logical device.
  /// param size: The size of the buffer.
  /// param usage_flags: The usage flags of the buffer.
  /// param memory_location: The memory location of the buffer.
  /// param debug_name: The debug name of the buffer.
  /// return: The result.
  pub fn new(
    logical_device: Rc<RefCell<HalaLogicalDevice>>,
    size: u64,
    usage_flags: HalaBufferUsageFlags,
    memory_location: HalaMemoryLocation,
    debug_name: &str,
  ) -> Result<Self, HalaGfxError> {
    Self::new_impl(logical_device, size, usage_flags, memory_location, false, debug_name)
  }

  /// Create a buffer with managed memory.
  /// param logical_device: The logical device.
  /// param size: The size of the buffer.
  /// param usage_flags: The usage flags of the buffer.
  /// param memory_location: The memory location of the buffer.
  /// param debug_name: The debug name of the buffer.
  /// return: The result.
  pub fn new_managed(
    logical_device: Rc<RefCell<HalaLogicalDevice>>,
    size: u64,
    usage_flags: HalaBufferUsageFlags,
    memory_location: HalaMemoryLocation,
    debug_name: &str,
  ) -> Result<Self, HalaGfxError> {
    Self::new_impl(logical_device, size, usage_flags, memory_location, true, debug_name)
  }

  /// Create a buffer.
  /// param logical_device: The logical device.
  /// param size: The size of the buffer.
  /// param usage_flags: The usage flags of the buffer.
  /// param memory_location: The memory location of the buffer.
  /// param use_managed_memory: Whether to use managed memory.
  /// param debug_name: The debug name of the buffer.
  /// return: The result.
  fn new_impl(
    logical_device: Rc<RefCell<HalaLogicalDevice>>,
    size: u64,
    usage_flags: HalaBufferUsageFlags,
    memory_location: HalaMemoryLocation,
    use_managed_memory: bool,
    debug_name: &str,
  ) -> Result<Self, HalaGfxError> {
    let buffer_info = vk::BufferCreateInfo::default()
      .size(size)
      .usage(usage_flags.into())
      .sharing_mode(vk::SharingMode::EXCLUSIVE);
    let (raw, memory_requirements) = unsafe {
      let logical_device = logical_device.borrow();
      let buffer = logical_device.raw.create_buffer(&buffer_info, None)
        .map_err(|err| HalaGfxError::new("Failed to create buffer.", Some(Box::new(err))))?;
      logical_device.set_debug_name(buffer, debug_name)
        .map_err(|err| HalaGfxError::new("Failed to set debug name of buffer.", Some(Box::new(err))))?;
      (buffer, logical_device.raw.get_buffer_memory_requirements(buffer))
    };

    let allocation = logical_device.borrow_mut().gpu_allocator
      .allocate(
        &gpu_allocator::vulkan::AllocationCreateDesc {
          name: debug_name,
          requirements: memory_requirements,
          location: memory_location.into(),
          linear: true,
          allocation_scheme: if use_managed_memory { gpu_allocator::vulkan::AllocationScheme::GpuAllocatorManaged } else { gpu_allocator::vulkan::AllocationScheme::DedicatedBuffer(raw) },
        }
      ).map_err(|err| HalaGfxError::new("Failed to allocate buffer.", Some(Box::new(err))))?;
    unsafe {
      let logical_device = logical_device.borrow();
      logical_device.set_debug_name(allocation.memory(), debug_name)
        .map_err(|err| HalaGfxError::new("Failed to set debug name of buffer memory.", Some(Box::new(err))))?;
      let bind_infos = [vk::BindBufferMemoryInfo::default()
        .buffer(raw)
        .memory(allocation.memory())
        .memory_offset(allocation.offset())];
      logical_device.raw.bind_buffer_memory2(&bind_infos)
        .map_err(|err| HalaGfxError::new("Failed to bind buffer memory.", Some(Box::new(err))))?;
    }

    log::debug!("A HalaBuffer \"{}\" is created.", debug_name);
    Ok(
      Self {
        logical_device,
        raw,
        memory_requirements,
        allocation,
        memory_location: memory_location.into(),
        size,
        debug_name: debug_name.to_string(),
      }
    )
  }

  /// Upload data to the buffer.
  /// This is expensive and should not be done in a hot loop.
  /// param offset: The offset in the buffer.
  /// param data: The data to be uploaded.
  /// return: The result.
  pub fn update_memory<T: Copy>(&self, offset: usize, data: &[T]) -> Result<(), HalaGfxError> {
    let src = data.as_ptr() as *const u8;
    let src_size = std::mem::size_of_val(data);
    self.update_memory_raw(offset, src, src_size)?;

    Ok(())
  }

  /// Upload raw data to the buffer.
  /// This is expensive and should not be done in a hot loop.
  /// param offset: The offset in the buffer.
  /// param data: The data to be uploaded.
  /// param size: The size of the data.
  /// return: The result.
  pub fn update_memory_raw(&self, offset: usize, data: *const u8, size: usize) -> Result<(), HalaGfxError> {
    if self.memory_location != gpu_allocator::MemoryLocation::GpuOnly {
      let src = data;
      let src_bytes = size;
      let dst = self.allocation.mapped_ptr().unwrap().as_ptr() as *mut u8;
      let dst_bytes = self.size as usize;
      unsafe { std::ptr::copy_nonoverlapping(src, dst.add(offset), std::cmp::min(src_bytes, dst_bytes)) };
    } else {
      return Err(HalaGfxError::new("Cannot update memory of a GPU only buffer.", None));
    }

    Ok(())
  }

  /// Upload data to the gpu buffer with a staging buffer.
  /// This is expensive and should not be done in a hot loop.
  /// param data: The data to be uploaded.
  /// param staging_buffer: The staging buffer.
  /// param command_buffers: The transfer command buffer set.
  /// return: The result.
  pub fn update_gpu_memory_with_buffer<T: Copy>(
    &self,
    data: &[T],
    staging_buffer: &HalaBuffer,
    command_buffers: &HalaCommandBufferSet
  ) -> Result<(), HalaGfxError> {
    let src = data.as_ptr() as *const u8;
    let src_size = std::mem::size_of_val(data);
    self.update_gpu_memory_with_buffer_raw(src, src_size, staging_buffer, command_buffers)?;

    Ok(())
  }

  /// Upload raw data to the gpu buffer with a staging buffer.
  /// This is expensive and should not be done in a hot loop.
  /// param data: The data to be uploaded.
  /// param size: The size of the data.
  /// param staging_buffer: The staging buffer.
  /// param command_buffers: The transfer command buffer set.
  /// return: The result.
  pub fn update_gpu_memory_with_buffer_raw(
    &self,
    data: *const u8,
    size: usize,
    staging_buffer: &HalaBuffer,
    command_buffers: &HalaCommandBufferSet
  ) -> Result<(), HalaGfxError> {
    if self.memory_location == gpu_allocator::MemoryLocation::GpuOnly {
      let src = data;
      let src_bytes = size;

      let dst = staging_buffer.allocation.mapped_ptr().unwrap().as_ptr() as *mut u8;
      let dst_bytes = staging_buffer.size as usize;
      unsafe { std::ptr::copy_nonoverlapping(src, dst, std::cmp::min(src_bytes, dst_bytes)) };

      unsafe {
        let logical_device = self.logical_device.borrow();
        let queue = match command_buffers.command_buffer_type {
          HalaCommandBufferType::GRAPHICS => logical_device.get_graphics_queue(0),
          HalaCommandBufferType::TRANSFER => logical_device.get_transfer_queue(0),
          HalaCommandBufferType::COMPUTE => logical_device.get_compute_queue(0),
          _ => return Err(HalaGfxError::new("Invalid command buffer type.", None)),
        };
        logical_device.execute_and_submit(command_buffers, 0, |logical_device, command_buffers, index| {
          let copy_regions = [vk::BufferCopy::default()
            .src_offset(0)
            .dst_offset(0)
            .size(self.size)];
          logical_device.raw.cmd_copy_buffer(command_buffers.raw[index], staging_buffer.raw, self.raw, &copy_regions);
        },
        queue)?;
      }
    } else {
      return Err(HalaGfxError::new("Cannot update GPU memory of a non GPU only buffer.", None));
    }

    Ok(())
  }

  /// Upload data to the gpu buffer.
  /// This is expensive and should not be done in a hot loop.
  /// param data: The data to be uploaded.
  /// param command_buffers: The transfer command buffer set.
  /// return: The result.
  pub fn update_gpu_memory<T: Copy>(&self, data: &[T], command_buffers: &HalaCommandBufferSet) -> Result<(), HalaGfxError> {
    if self.memory_location == gpu_allocator::MemoryLocation::GpuOnly {
      let staging_buffer = HalaBuffer::new(
        Rc::clone(&self.logical_device),
        std::mem::size_of_val(data) as u64,
        HalaBufferUsageFlags::TRANSFER_SRC,
        HalaMemoryLocation::CpuToGpu,
        "staging_buffer",
      )?;

      self.update_gpu_memory_with_buffer(data, &staging_buffer, command_buffers)?;
    } else {
      return Err(HalaGfxError::new("Cannot update GPU memory of a non GPU only buffer.", None));
    }

    Ok(())
  }

  /// Download data from the buffer.
  /// This is expensive and should not be done in a hot loop.
  /// param offset: The offset in the buffer.
  /// param data: The data to be downloaded.
  /// return: The result.
  pub fn download_memory<T: Copy>(&self, offset: usize, data: &mut [T]) -> Result<(), HalaGfxError> {
    let dst = data.as_mut_ptr() as *mut u8;
    let dst_size = std::mem::size_of_val(data);
    self.download_memory_raw(offset, dst, dst_size)?;

    Ok(())
  }

  /// Download raw data from the buffer.
  /// This is expensive and should not be done in a hot loop.
  /// param offset: The offset in the buffer.
  /// param data: The data to be downloaded.
  /// param size: The size of the data.
  /// return: The result.
  pub fn download_memory_raw(&self, offset: usize, data: *mut u8, size: usize) -> Result<(), HalaGfxError> {
    if self.memory_location != gpu_allocator::MemoryLocation::GpuOnly {
      let src = self.allocation.mapped_ptr().unwrap().as_ptr() as *const u8;
      let src_bytes = self.size as usize;
      let dst = data;
      let dst_bytes = size;
      unsafe { std::ptr::copy_nonoverlapping(src.add(offset), dst, std::cmp::min(src_bytes, dst_bytes)) };
    } else {
      return Err(HalaGfxError::new("Cannot download memory of a GPU only buffer.", None));
    }

    Ok(())
  }

  /// Download data from the gpu buffer with a staging buffer.
  /// This is expensive and should not be done in a hot loop.
  /// param data: The data to be downloaded to.
  /// param staging_buffer: The staging buffer.
  /// param command_buffers: The transfer command buffer set.
  /// return: The result.
  pub fn download_gpu_memory_with_buffer<T: Copy>(
    &self,
    data: &mut [T],
    staging_buffer: &HalaBuffer,
    command_buffers: &HalaCommandBufferSet
  ) -> Result<(), HalaGfxError> {
    let dst = data.as_mut_ptr() as *mut u8;
    let dst_size = std::mem::size_of_val(data);
    self.download_gpu_memory_with_buffer_raw(dst, dst_size, staging_buffer, command_buffers)?;

    Ok(())
  }

  /// Download raw data from the gpu buffer with a staging buffer.
  /// This is expensive and should not be done in a hot loop.
  /// param data: The data to be downloaded to.
  /// param size: The size of the data.
  /// param staging_buffer: The staging buffer.
  /// param command_buffers: The transfer command buffer set.
  /// return: The result.
  pub fn download_gpu_memory_with_buffer_raw(
    &self,
    data: *mut u8,
    size: usize,
    staging_buffer: &HalaBuffer,
    command_buffers: &HalaCommandBufferSet,
  ) -> Result<(), HalaGfxError> {
    if self.memory_location == gpu_allocator::MemoryLocation::GpuOnly {
      unsafe {
        let logical_device = self.logical_device.borrow();
        let queue = match command_buffers.command_buffer_type {
          HalaCommandBufferType::GRAPHICS => logical_device.get_graphics_queue(0),
          HalaCommandBufferType::TRANSFER => logical_device.get_transfer_queue(0),
          HalaCommandBufferType::COMPUTE => logical_device.get_compute_queue(0),
          _ => return Err(HalaGfxError::new("Invalid command buffer type.", None)),
        };
        logical_device.execute_and_submit(command_buffers, 0, |logical_device, command_buffers, index| {
          let copy_regions = [vk::BufferCopy::default()
            .src_offset(0)
            .dst_offset(0)
            .size(self.size)];
          logical_device.raw.cmd_copy_buffer(command_buffers.raw[index], self.raw, staging_buffer.raw, &copy_regions);
        },
        queue)?;
      }

      let src = staging_buffer.allocation.mapped_ptr().unwrap().as_ptr() as *mut u8;
      let src_bytes = staging_buffer.size as usize;

      let dst = data;
      let dst_bytes = size;
      unsafe { std::ptr::copy_nonoverlapping(src, dst, std::cmp::min(src_bytes, dst_bytes)) };
    } else {
      return Err(HalaGfxError::new("Cannot update GPU memory of a non GPU only buffer.", None));
    }

    Ok(())
  }

  /// Download data from the gpu buffer.
  /// This is expensive and should not be done in a hot loop.
  /// param data: The data to be uploaded.
  /// param command_buffers: The transfer command buffer set.
  /// return: The result.
  pub fn download_gpu_memory<T: Copy>(&self, data: &mut [T], command_buffers: &HalaCommandBufferSet) -> Result<(), HalaGfxError> {
    if self.memory_location == gpu_allocator::MemoryLocation::GpuOnly {
      let staging_buffer = HalaBuffer::new(
        Rc::clone(&self.logical_device),
        std::mem::size_of_val(data) as u64,
        HalaBufferUsageFlags::TRANSFER_DST,
        HalaMemoryLocation::CpuToGpu,
        "staging_buffer",
      )?;

      self.download_gpu_memory_with_buffer(data, &staging_buffer, command_buffers)?;
    } else {
      return Err(HalaGfxError::new("Cannot update GPU memory of a non GPU only buffer.", None));
    }

    Ok(())
  }

  /// Get the device address of the buffer.
  /// return: The device address.
  pub fn get_device_address(&self) -> u64 {
    let buffer_device_address_info = vk::BufferDeviceAddressInfo::default()
      .buffer(self.raw);
    unsafe {
      let logical_device = self.logical_device.borrow();
      logical_device.raw.get_buffer_device_address(&buffer_device_address_info)
    }
  }
}
