use std::rc::Rc;
use std::cell::RefCell;

use ash::vk;

use crate::{
  HalaGfxError,
  HalaLogicalDevice,
};

/// The pipeline cache.
pub struct HalaPipelineCache {
  pub(crate) logical_device: Rc<RefCell<HalaLogicalDevice>>,
  pub raw: vk::PipelineCache,
}

/// The Drop trait implementation for pipeline cache.
impl Drop for HalaPipelineCache {
  fn drop(&mut self) {
    unsafe {
      self.logical_device.borrow().raw.destroy_pipeline_cache(self.raw, None);
    }
    log::debug!("A HalaPipelineCache is dropped.");
  }
}

/// The implementation of pipeline cache.
impl HalaPipelineCache {
  /// Create a new pipeline cache.
  /// param logical_device: The logical device.
  /// return: The pipeline cache.
  pub fn new(logical_device: Rc<RefCell<HalaLogicalDevice>>) -> Result<Self, HalaGfxError> {
    let create_info = vk::PipelineCacheCreateInfo::default();
    let raw = unsafe {
      logical_device.borrow().raw.create_pipeline_cache(&create_info, None)
        .map_err(|err| HalaGfxError::new("Failed to create pipeline cache.", Some(Box::new(err))))?
    };

    logical_device.borrow().set_debug_name(
      raw,
      "pipeline_cache"
    ).map_err(|err| HalaGfxError::new("Failed to set debug name for pipeline cache.", Some(Box::new(err))))?;

    log::debug!("A HalaPipelineCache is created.");
    Ok(
      Self {
        logical_device,
        raw,
      }
    )
  }

  pub fn with_cache_file(logical_device: Rc<RefCell<HalaLogicalDevice>>, path: &str) -> Result<Self, HalaGfxError> {
    let data = std::fs::read(path)
      .map_err(|err| HalaGfxError::new("Failed to read pipeline cache file.", Some(Box::new(err))))?;

    let create_info = vk::PipelineCacheCreateInfo::default()
      .initial_data(&data);
    let raw = unsafe {
      logical_device.borrow().raw.create_pipeline_cache(&create_info, None)
        .map_err(|err| HalaGfxError::new("Failed to create pipeline cache.", Some(Box::new(err))))?
    };
    Ok(
      Self {
        logical_device,
        raw,
      }
    )
  }

  /// Load a pipeline cache from a file.
  /// param path: The file path.
  /// return: The result.
  pub fn save(&self, path: &str) -> Result<(), HalaGfxError> {
    let data = unsafe {
      self.logical_device.borrow().raw.get_pipeline_cache_data(self.raw)
        .map_err(|err| HalaGfxError::new("Failed to get pipeline cache data.", Some(Box::new(err))))?
    };
    std::fs::write(path, data)
      .map_err(|err| HalaGfxError::new("Failed to save pipeline cache.", Some(Box::new(err))))?;
    Ok(())
  }
}