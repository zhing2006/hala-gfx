use std::rc::Rc;
use std::cell::RefCell;

use ash::vk;

use crate::{
  HalaGfxError,
  HalaLogicalDevice,
};

/// The command pools.
pub struct HalaCommandPools {
  pub(crate) logical_device: Rc<RefCell<HalaLogicalDevice>>,
  pub graphics: vk::CommandPool,
  pub compute: vk::CommandPool,
  pub transfer: vk::CommandPool,
  debug_name: String,
}

/// The Drop implementation of the command pools.
impl Drop for HalaCommandPools {
  fn drop(&mut self) {
    let logical_device = self.logical_device.borrow();
    unsafe {
      logical_device.raw.destroy_command_pool(self.graphics, None);
      logical_device.raw.destroy_command_pool(self.compute, None);
      logical_device.raw.destroy_command_pool(self.transfer, None);
    }
    log::debug!("A HalaCommandPools \"{}\" is dropped.", self.debug_name);
  }
}

/// The command pools implementation.
impl HalaCommandPools {
  /// Create a new command pools.
  /// param logical_device: The logical device.
  /// param is_short_time: Whether the command pools is used for short time commands.
  /// param debug_name: The debug name.
  /// return: The command pools.
  pub fn new(
    logical_device: Rc<RefCell<HalaLogicalDevice>>,
    is_short_time: bool,
    debug_name: &str,
  ) -> Result<Self, HalaGfxError> {
    let (
      graphics,
      compute,
      transfer,
    ) = {
      Self::create_pools(&logical_device, is_short_time, debug_name)?
    };

    log::debug!("A HalaCommandPools \"{}\" is created.", debug_name);
    Ok(
      Self {
        logical_device,
        graphics,
        compute,
        transfer,
        debug_name: debug_name.to_string(),
      }
    )
  }

  /// Create a command pools.
  /// param logical_device: The logical device.
  /// param is_short_time: Whether the command pools is used for short time commands.
  /// param debug_name: The debug name.
  /// return: The command pools.
  fn create_pools(
    logical_device: &Rc<RefCell<HalaLogicalDevice>>,
    is_short_time: bool,
    debug_name: &str,
  ) -> Result<(vk::CommandPool, vk::CommandPool, vk::CommandPool), HalaGfxError> {
    let logical_device = logical_device.borrow();
    let graphics_family = logical_device.graphics_queue_family_index;
    let compute_family = logical_device.compute_queue_family_index;
    let transfer_family = logical_device.transfer_queue_family_index;

    let create_info = vk::CommandPoolCreateInfo::default()
      .queue_family_index(graphics_family)
      .flags(if is_short_time { vk::CommandPoolCreateFlags::TRANSIENT } else { vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER });
    let graphics = unsafe {
      logical_device.raw.create_command_pool(&create_info, None)
        .map_err(|err| HalaGfxError::new("Failed to create graphics command pool.", Some(Box::new(err))))?
    };
    logical_device.set_debug_name(
      graphics,
      &format!("{}.graphics", debug_name))
      .map_err(|err| HalaGfxError::new("Failed to set debug name for graphics command pool.", Some(Box::new(err))))?;

    let create_info = vk::CommandPoolCreateInfo::default()
    .queue_family_index(compute_family)
    .flags(if is_short_time { vk::CommandPoolCreateFlags::TRANSIENT } else { vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER });
    let compute = unsafe {
      logical_device.raw.create_command_pool(&create_info, None)
        .map_err(|err| HalaGfxError::new("Failed to create compute command pool.", Some(Box::new(err))))?
    };
    logical_device.set_debug_name(
      compute,
      &format!("{}.compute", debug_name))
      .map_err(|err| HalaGfxError::new("Failed to set debug name for compute command pool.", Some(Box::new(err))))?;

    let create_info = vk::CommandPoolCreateInfo::default()
      .queue_family_index(transfer_family)
      .flags(if is_short_time { vk::CommandPoolCreateFlags::TRANSIENT } else { vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER });
    let transfer = unsafe {
      logical_device.raw.create_command_pool(&create_info, None)
        .map_err(|err| HalaGfxError::new("Failed to create transfer command pool.", Some(Box::new(err))))?
    };
    logical_device.set_debug_name(
      transfer,
      &format!("{}.transfer", debug_name))
      .map_err(|err| HalaGfxError::new("Failed to set debug name for transfer command pool.", Some(Box::new(err))))?;

    Ok((graphics, compute, transfer))
  }
}
