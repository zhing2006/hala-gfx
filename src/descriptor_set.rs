use std::rc::Rc;
use std::cell::RefCell;

use ash::vk::{self, Handle};

use crate::{
  HalaGfxError,
  HalaLogicalDevice,
  HalaShaderStageFlags,
  HalaDescriptorPool,
};

/// The descriptor type.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct HalaDescriptorType(i32);
impl HalaDescriptorType {
  pub const SAMPLER: Self = Self(vk::DescriptorType::SAMPLER.as_raw());
  pub const UNIFORM_BUFFER: Self = Self(vk::DescriptorType::UNIFORM_BUFFER.as_raw());
  pub const COMBINED_IMAGE_SAMPLER: Self = Self(vk::DescriptorType::COMBINED_IMAGE_SAMPLER.as_raw());
  pub const SAMPLED_IMAGE: Self = Self(vk::DescriptorType::SAMPLED_IMAGE.as_raw());
  pub const STORAGE_IMAGE: Self = Self(vk::DescriptorType::STORAGE_IMAGE.as_raw());
  pub const UNIFORM_TEXEL_BUFFER: Self = Self(vk::DescriptorType::UNIFORM_TEXEL_BUFFER.as_raw());
  pub const STORAGE_TEXEL_BUFFER: Self = Self(vk::DescriptorType::STORAGE_TEXEL_BUFFER.as_raw());
  pub const STORAGE_BUFFER: Self = Self(vk::DescriptorType::STORAGE_BUFFER.as_raw());
  pub const UNIFORM_BUFFER_DYNAMIC: Self = Self(vk::DescriptorType::UNIFORM_BUFFER_DYNAMIC.as_raw());
  pub const STORAGE_BUFFER_DYNAMIC: Self = Self(vk::DescriptorType::STORAGE_BUFFER_DYNAMIC.as_raw());
  pub const INPUT_ATTACHMENT: Self = Self(vk::DescriptorType::INPUT_ATTACHMENT.as_raw());
  pub const ACCELERATION_STRUCTURE: Self = Self(vk::DescriptorType::ACCELERATION_STRUCTURE_KHR.as_raw());
}

impl std::convert::From<vk::DescriptorType> for HalaDescriptorType {
  fn from(descriptor_type: vk::DescriptorType) -> Self {
    Self(descriptor_type.as_raw())
  }
}

impl std::convert::From<HalaDescriptorType> for vk::DescriptorType {
  fn from(descriptor_type: HalaDescriptorType) -> Self {
    Self::from_raw(descriptor_type.0)
  }
}

/// The descriptor binding flags.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HalaDescriptorBindingFlags(u32);
crate::hala_bitflags_wrapped!(HalaDescriptorBindingFlags, u32);
impl HalaDescriptorBindingFlags {
  pub const UPDATE_AFTER_BIND: Self = Self(vk::DescriptorBindingFlags::UPDATE_AFTER_BIND.as_raw());
  pub const UPDATE_UNUSED_WHILE_PENDING: Self = Self(vk::DescriptorBindingFlags::UPDATE_UNUSED_WHILE_PENDING.as_raw());
  pub const PARTIALLY_BOUND: Self = Self(vk::DescriptorBindingFlags::PARTIALLY_BOUND.as_raw());
  pub const VARIABLE_DESCRIPTOR_COUNT: Self = Self(vk::DescriptorBindingFlags::VARIABLE_DESCRIPTOR_COUNT.as_raw());
}

impl std::convert::From<vk::DescriptorBindingFlags> for HalaDescriptorBindingFlags {
  fn from(descriptor_binding_flags: vk::DescriptorBindingFlags) -> Self {
    Self(descriptor_binding_flags.as_raw())
  }
}

impl std::convert::From<HalaDescriptorBindingFlags> for vk::DescriptorBindingFlags {
  fn from(descriptor_binding_flags: HalaDescriptorBindingFlags) -> Self {
    Self::from_raw(descriptor_binding_flags.0)
  }
}

/// The descriptor set layout.
pub struct HalaDescriptorSetLayout {
  pub(crate) logical_device: Rc<RefCell<HalaLogicalDevice>>,
  pub raw: vk::DescriptorSetLayout,

  pub(crate) debug_name: String,
}

/// The AsRef trait implementation of the descriptor set layout.
impl AsRef<HalaDescriptorSetLayout> for HalaDescriptorSetLayout {
  fn as_ref(&self) -> &HalaDescriptorSetLayout {
    self
  }
}

/// The Drop trait implementation of the descriptor set layout.
impl Drop for HalaDescriptorSetLayout {
  fn drop(&mut self) {
    unsafe {
      self.logical_device.borrow().raw.destroy_descriptor_set_layout(self.raw, None);
    }
    log::debug!("A HalaDescriptorSetLayout \"{}\" is dropped.", self.debug_name);
  }
}

/// The implementation of the descriptor set layout.
impl HalaDescriptorSetLayout {
  /// Create a new descriptor set layout.
  /// param logical_device: The logical device.
  /// param bindings: The bindings(binding, description type, count, stage flags, binding flags).
  /// param debug_name: The debug name.
  /// return: The descriptor set layout.
  pub fn new(
    logical_device: Rc<RefCell<HalaLogicalDevice>>,
    bindings: &[(u32, HalaDescriptorType, u32, HalaShaderStageFlags, HalaDescriptorBindingFlags)],
    debug_name: &str,
  ) -> Result<Self, HalaGfxError> {
    let mut descriptor_set_layout_bindings = Vec::new();
    let mut descriptor_set_layout_bindings_flags = Vec::new();
    for (binding, descriptor_type, count, stage_flags, binding_flags) in bindings {
      descriptor_set_layout_bindings.push(vk::DescriptorSetLayoutBinding::default()
        .binding(*binding)
        .descriptor_type(vk::DescriptorType::from(*descriptor_type))
        .descriptor_count(*count)
        .stage_flags(vk::ShaderStageFlags::from(*stage_flags)));
      descriptor_set_layout_bindings_flags.push(vk::DescriptorBindingFlags::from(*binding_flags));
    }

    let mut binding_flags_create_info = vk::DescriptorSetLayoutBindingFlagsCreateInfo::default()
      .binding_flags(&descriptor_set_layout_bindings_flags);

    let descriptor_set_layout_create_info = vk::DescriptorSetLayoutCreateInfo::default()
      .bindings(&descriptor_set_layout_bindings)
      .flags(vk::DescriptorSetLayoutCreateFlags::UPDATE_AFTER_BIND_POOL)
      .push_next(&mut binding_flags_create_info);

    let raw = unsafe {
      logical_device.borrow().raw.create_descriptor_set_layout(&descriptor_set_layout_create_info, None)
        .map_err(|err| HalaGfxError::new("Failed to create descriptor set layout.", Some(Box::new(err))))?
    };
    logical_device.borrow().set_debug_name(
      raw,
      debug_name,
    ).map_err(|err| HalaGfxError::new("Failed to set debug name for descriptor set layout.", Some(Box::new(err))))?;

    log::debug!("A HalaDescriptorSetLayout \"{}\" is created.", debug_name);
    Ok(Self {
      logical_device,
      raw,
      debug_name: debug_name.to_string(),
    })
  }
}

/// The descriptor set.
pub struct HalaDescriptorSet {
  pub(crate) logical_device: Rc<RefCell<HalaLogicalDevice>>,
  pub(crate) descriptor_pool: Rc<RefCell<HalaDescriptorPool>>,
  pub layout: HalaDescriptorSetLayout,
  pub raw: Vec<vk::DescriptorSet>,
  pub is_static: bool,
  pub(crate) debug_name: String,
}

/// The AsRef trait implementation of the descriptor set.
impl AsRef<HalaDescriptorSet> for HalaDescriptorSet {
  fn as_ref(&self) -> &HalaDescriptorSet {
    self
  }
}

/// The Drop trait implementation of the descriptor set.
impl Drop for HalaDescriptorSet {
  fn drop(&mut self) {
    unsafe {
      self.logical_device.borrow().raw.free_descriptor_sets(
        self.descriptor_pool.borrow().raw,
        &self.raw,
      ).unwrap();
    }
    log::debug!("A HalaDescriptorSet \"{}\" is dropped.", self.debug_name);
  }
}

/// The implementation of the descriptor set.
impl HalaDescriptorSet {
  /// Create a new descriptor set.
  /// param logical_device: The logical device.
  /// param descriptor_pool: The descriptor pool.
  /// param layout: The descriptor set layout.
  /// param count: The count of the descriptor set.
  /// param variable_descriptor_count: The variable descriptor count.
  /// param debug_name: The debug name.
  /// return: The descriptor set.
  pub fn new(
    logical_device: Rc<RefCell<HalaLogicalDevice>>,
    descriptor_pool: Rc<RefCell<HalaDescriptorPool>>,
    layout: HalaDescriptorSetLayout,
    count: usize,
    variable_descriptor_count: u32,
    debug_name: &str,
  ) -> Result<Self, HalaGfxError> {
    let variable_descriptor_counts = vec![variable_descriptor_count; count];
    let mut variable_descriptor_count_allocate_info: vk::DescriptorSetVariableDescriptorCountAllocateInfo = vk::DescriptorSetVariableDescriptorCountAllocateInfo::default()
      .descriptor_counts(&variable_descriptor_counts);

    let layouts = vec![layout.raw; count];
    let descriptor_set_allocate_info = vk::DescriptorSetAllocateInfo::default()
      .descriptor_pool(descriptor_pool.borrow().raw)
      .set_layouts(&layouts)
      .push_next(&mut variable_descriptor_count_allocate_info);

    let raw = unsafe {
      logical_device.borrow().raw.allocate_descriptor_sets(&descriptor_set_allocate_info)
        .map_err(|err| HalaGfxError::new("Failed to allocate descriptor sets.", Some(Box::new(err))))?
    };
    for (index, &descriptor_set) in raw.iter().enumerate() {
      logical_device.borrow().set_debug_name(
        descriptor_set,
        &format!("{}[{}]", debug_name, index),
      ).map_err(|err| HalaGfxError::new("Failed to set debug name for descriptor set.", Some(Box::new(err))))?;
    }

    log::debug!("A HalaDescriptorSet \"{}\" is created.", debug_name);
    Ok(Self {
      logical_device,
      descriptor_pool,
      layout,
      raw,
      is_static: false,
      debug_name: debug_name.to_string(),
    })
  }

  /// Get the handle of the descriptor set.
  /// param index: The index.
  /// return: The handle.
  pub fn handle(&self, index: usize) -> u64 {
    self.raw[index].as_raw()
  }

  /// Create a new static descriptor set.
  /// param logical_device: The logical device.
  /// param descriptor_pool: The descriptor pool.
  /// param layout: The descriptor set layout.
  /// param count: The count of the descriptor set.
  /// param variable_descriptor_count: The variable descriptor count.
  /// param debug_name: The debug name.
  /// return: The descriptor set.
  pub fn new_static(
    logical_device: Rc<RefCell<HalaLogicalDevice>>,
    descriptor_pool: Rc<RefCell<HalaDescriptorPool>>,
    layout: HalaDescriptorSetLayout,
    count: usize,
    variable_descriptor_count: u32,
    debug_name: &str,
  ) -> Result<Self, HalaGfxError> {
    let mut self_ = Self::new(
      logical_device,
      descriptor_pool,
      layout,
      count,
      variable_descriptor_count,
      debug_name)?;
    self_.is_static = true;

    Ok(self_)
  }

  /// Update the uniform buffer.
  /// param index: The index.
  /// param binding: The binding.
  /// param buffers: The buffers.
  pub fn update_uniform_buffers<B>(&self, index: usize, binding: u32, buffers: &[B])
    where B: AsRef<crate::HalaBuffer>
  {
    let buffer_infos = buffers
      .iter()
      .map(|buffer| vk::DescriptorBufferInfo::default()
        .buffer(buffer.as_ref().raw)
        .range(vk::WHOLE_SIZE))
      .collect::<Vec<_>>();

    let descriptor_write = vk::WriteDescriptorSet::default()
      .dst_set(self.raw[index])
      .dst_binding(binding)
      .descriptor_type(vk::DescriptorType::UNIFORM_BUFFER)
      .buffer_info(buffer_infos.as_slice());

    unsafe {
      self.logical_device.borrow().raw.update_descriptor_sets(&[descriptor_write], &[]);
    }
  }

  /// Update the storage buffer.
  /// param index: The index.
  /// param binding: The binding.
  /// param buffers: The buffers.
  pub fn update_storage_buffers<B>(&self, index: usize, binding: u32, buffers: &[B])
    where B: AsRef<crate::HalaBuffer>
  {
    let buffer_infos = buffers
      .iter()
      .map(|buffer| vk::DescriptorBufferInfo::default()
        .buffer(buffer.as_ref().raw)
        .range(vk::WHOLE_SIZE))
      .collect::<Vec<_>>();

    let descriptor_write = vk::WriteDescriptorSet::default()
      .dst_set(self.raw[index])
      .dst_binding(binding)
      .descriptor_type(vk::DescriptorType::STORAGE_BUFFER)
      .buffer_info(buffer_infos.as_slice());

    unsafe {
      self.logical_device.borrow().raw.update_descriptor_sets(&[descriptor_write], &[]);
    }
  }

  /// Update the storage images.
  /// param index: The index.
  /// param binding: The binding.
  /// param images: The images.
  pub fn update_storage_images<T>(&self, index: usize, binding: u32, images: &[T])
    where T: AsRef<crate::HalaImage>
  {
    let image_infos = images
      .iter()
      .map(|image| vk::DescriptorImageInfo::default()
        .image_view(image.as_ref().view)
        .image_layout(vk::ImageLayout::GENERAL))
      .collect::<Vec<_>>();

    let descriptor_write = vk::WriteDescriptorSet::default()
      .dst_set(self.raw[index])
      .dst_binding(binding)
      .descriptor_type(vk::DescriptorType::STORAGE_IMAGE)
      .image_info(image_infos.as_slice());

    unsafe {
      self.logical_device.borrow().raw.update_descriptor_sets(&[descriptor_write], &[]);
    }
  }

  /// Update the sampled images.
  /// param index: The index.
  /// param binding: The binding.
  /// param images: The images.
  pub fn update_sampled_images<T>(&self, index: usize, binding: u32, images: &[T])
    where T: AsRef<crate::HalaImage>
  {
    let image_infos = images
      .iter()
      .map(|image| vk::DescriptorImageInfo::default()
        .image_view(image.as_ref().view)
        .image_layout(vk::ImageLayout::GENERAL))
      .collect::<Vec<_>>();

    let descriptor_write = vk::WriteDescriptorSet::default()
      .dst_set(self.raw[index])
      .dst_binding(binding)
      .descriptor_type(vk::DescriptorType::SAMPLED_IMAGE)
      .image_info(image_infos.as_slice());

    unsafe {
      self.logical_device.borrow().raw.update_descriptor_sets(&[descriptor_write], &[]);
    }
  }

  /// Update the samplers.
  /// param index: The index.
  /// param binding: The binding.
  /// param samplers: The samplers.
  pub fn update_sampler<T>(&self, index: usize, binding: u32, samplers: &[T])
    where T: AsRef<crate::HalaSampler>
  {
    let sampler_infos = samplers
      .iter()
      .map(|sampler| vk::DescriptorImageInfo::default()
        .sampler(sampler.as_ref().raw))
      .collect::<Vec<_>>();

    let descriptor_write = vk::WriteDescriptorSet::default()
      .dst_set(self.raw[index])
      .dst_binding(binding)
      .descriptor_type(vk::DescriptorType::SAMPLER)
      .image_info(sampler_infos.as_slice());

    unsafe {
      self.logical_device.borrow().raw.update_descriptor_sets(&[descriptor_write], &[]);
    }
  }

  /// Update the combined image samplers.
  /// param index: The index.
  /// param binding: The binding.
  /// param images: The images.
  pub fn update_combined_image_samplers<I, S>(
    &self,
    index: usize,
    binding: u32,
    images_and_samplers: &[(I, S)],
  )
    where I: AsRef<crate::HalaImage>,
          S: AsRef<crate::HalaSampler>
  {
    let image_infos = images_and_samplers
      .iter()
      .map(|(image, sampler)| vk::DescriptorImageInfo::default()
        .image_layout(vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL)
        .image_view(image.as_ref().view)
        .sampler(sampler.as_ref().raw))
      .collect::<Vec<_>>();

    let descriptor_write = vk::WriteDescriptorSet::default()
      .dst_set(self.raw[index])
      .dst_binding(binding)
      .descriptor_type(vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
      .image_info(image_infos.as_slice());

    unsafe {
      self.logical_device.borrow().raw.update_descriptor_sets(&[descriptor_write], &[]);
    }
  }

  /// Update the acceleration structures.
  /// param index: The index.
  /// param binding: The binding.
  /// param acceleration_structures: The acceleration structures.
  pub fn update_acceleration_structures<A>(&self, index: usize, binding: u32, acceleration_structures: &[A])
    where A: AsRef<crate::HalaAccelerationStructure>
  {
    let acceleration_structures = acceleration_structures
      .iter()
      .map(|acceleration_structure| acceleration_structure.as_ref().raw)
      .collect::<Vec<_>>();
    let mut acceleration_structure_info = vk::WriteDescriptorSetAccelerationStructureKHR::default()
      .acceleration_structures(acceleration_structures.as_slice());

    let mut descriptor_write = vk::WriteDescriptorSet::default()
      .dst_set(self.raw[index])
      .dst_binding(binding)
      .descriptor_type(vk::DescriptorType::ACCELERATION_STRUCTURE_KHR)
      .push_next(&mut acceleration_structure_info);
    descriptor_write.descriptor_count = acceleration_structures.len() as u32;

    unsafe {
      self.logical_device.borrow().raw.update_descriptor_sets(&[descriptor_write], &[]);
    }
  }
}