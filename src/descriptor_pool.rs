use std::rc::Rc;
use std::cell::RefCell;

use ash::vk;

use crate::{
  HalaGfxError,
  HalaLogicalDevice,
};

/// The descriptor pool.
pub struct HalaDescriptorPool {
  pub(crate) logical_device: Rc<RefCell<HalaLogicalDevice>>,
  pub raw: vk::DescriptorPool,
  debug_name: String,
}

/// The Drop trait implementation of the descriptor pool.
impl Drop for HalaDescriptorPool {
  fn drop(&mut self) {
    let logical_device = self.logical_device.borrow();
    unsafe {
      logical_device.raw.destroy_descriptor_pool(self.raw, None);
    }
    log::debug!("A HalaDescriptorPool \"{}\" is dropped.", self.debug_name);
  }
}

/// The descriptor pool implementation.
impl HalaDescriptorPool {
  /// Create a new descriptor pool.
  /// param logical_device: The logical device.
  /// param descriptor_sizes: The descriptor sizes(description type, count).
  /// param size: The size of the descriptor pool.
  /// param debug_name: The debug name.
  /// return: The descriptor pool.
  pub fn new(
    logical_device: Rc<RefCell<HalaLogicalDevice>>,
    descriptor_sizes: &[(crate::HalaDescriptorType, usize)],
    size: usize,
    debug_name: &str,
  ) -> Result<Self, HalaGfxError> {
    let raw = {
      let pool_sizes = descriptor_sizes.iter().map(|(descriptor_type, count)| {
        vk::DescriptorPoolSize::default()
          .ty((*descriptor_type).into())
          .descriptor_count(*count as u32)
      }).collect::<Vec<_>>();
      let logical_device = logical_device.borrow();
      let create_info = vk::DescriptorPoolCreateInfo::default()
        .pool_sizes(&pool_sizes)
        .flags(vk::DescriptorPoolCreateFlags::FREE_DESCRIPTOR_SET | vk::DescriptorPoolCreateFlags::UPDATE_AFTER_BIND)
        .max_sets(size as u32);
      unsafe {
        logical_device.raw.create_descriptor_pool(&create_info, None)
          .map_err(|err| HalaGfxError::new("Failed to create descriptor pool.", Some(Box::new(err))))?
      }
    };
    logical_device.borrow().set_debug_name(
      raw,
      debug_name,
    ).map_err(|err| HalaGfxError::new("Failed to set debug name for descriptor pool.", Some(Box::new(err))))?;

    log::debug!("A HalaDescriptorPool \"{}\" is created.", debug_name);
    Ok(
      Self {
        logical_device,
        raw,
        debug_name: debug_name.to_string(),
      }
    )
  }
}