use ash::vk;

use crate::{
  HalaPhysicalDevice,
  HalaLogicalDevice,
  HalaGfxError,
};

/// The query pipeline statistic flags.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HalaQueryPipelineStatisticFlags(u32);
crate::hala_bitflags_wrapped!(HalaQueryPipelineStatisticFlags, u32);
impl HalaQueryPipelineStatisticFlags {
  pub const INPUT_ASSEMBLY_VERTICES: Self = Self(vk::QueryPipelineStatisticFlags::INPUT_ASSEMBLY_VERTICES.as_raw());
  pub const INPUT_ASSEMBLY_PRIMITIVES: Self = Self(vk::QueryPipelineStatisticFlags::INPUT_ASSEMBLY_PRIMITIVES.as_raw());
  pub const VERTEX_SHADER_INVOCATIONS: Self = Self(vk::QueryPipelineStatisticFlags::VERTEX_SHADER_INVOCATIONS.as_raw());
  pub const GEOMETRY_SHADER_INVOCATIONS: Self = Self(vk::QueryPipelineStatisticFlags::GEOMETRY_SHADER_INVOCATIONS.as_raw());
  pub const GEOMETRY_SHADER_PRIMITIVES: Self = Self(vk::QueryPipelineStatisticFlags::GEOMETRY_SHADER_PRIMITIVES.as_raw());
  pub const CLIPPING_INVOCATIONS: Self = Self(vk::QueryPipelineStatisticFlags::CLIPPING_INVOCATIONS.as_raw());
  pub const CLIPPING_PRIMITIVES: Self = Self(vk::QueryPipelineStatisticFlags::CLIPPING_PRIMITIVES.as_raw());
  pub const FRAGMENT_SHADER_INVOCATIONS: Self = Self(vk::QueryPipelineStatisticFlags::FRAGMENT_SHADER_INVOCATIONS.as_raw());
  pub const TESSELLATION_CONTROL_SHADER_PATCHES: Self = Self(vk::QueryPipelineStatisticFlags::TESSELLATION_CONTROL_SHADER_PATCHES.as_raw());
  pub const TESSELLATION_EVALUATION_SHADER_INVOCATIONS: Self = Self(vk::QueryPipelineStatisticFlags::TESSELLATION_EVALUATION_SHADER_INVOCATIONS.as_raw());
  pub const COMPUTE_SHADER_INVOCATIONS: Self = Self(vk::QueryPipelineStatisticFlags::COMPUTE_SHADER_INVOCATIONS.as_raw());
}

impl std::convert::From<vk::QueryPipelineStatisticFlags> for HalaQueryPipelineStatisticFlags {
  fn from(flags: vk::QueryPipelineStatisticFlags) -> Self {
    Self(flags.as_raw())
  }
}

impl std::convert::From<HalaQueryPipelineStatisticFlags> for vk::QueryPipelineStatisticFlags {
  fn from(flags: HalaQueryPipelineStatisticFlags) -> Self {
    Self::from_raw(flags.0)
  }
}

/// The query pool.
pub struct HalaQueryPool {
  pub(crate) logical_device: std::rc::Rc<std::cell::RefCell<HalaLogicalDevice>>,
  pub raw: vk::QueryPool,
  pub size: u32,
  pub(crate) timestamp_period: f64,
  pub(crate) debug_name: String,
}

/// The Drop implementation for query pool.
impl Drop for HalaQueryPool {
  fn drop(&mut self) {
    unsafe {
      self.logical_device.borrow().raw.destroy_query_pool(self.raw, None);
    }
    log::debug!("A HalaQueryPool \"{}\" is dropped.", self.debug_name);
  }
}

/// The query pool implementation.
impl HalaQueryPool {
  /// Create a new query pool.
  /// param physical_device: The physical device.
  /// param logical_device: The logical device.
  /// param count: The query count.
  /// param debug_name: The debug name.
  /// return: The query pool.
  pub fn new_timestamp(
    physical_device: &HalaPhysicalDevice,
    logical_device: std::rc::Rc<std::cell::RefCell<HalaLogicalDevice>>,
    count: u32,
    debug_name: &str,
  ) -> Result<Self, HalaGfxError> {
    let query_pool_info = vk::QueryPoolCreateInfo::default()
      .query_type(vk::QueryType::TIMESTAMP)
      .query_count(count);
    let (raw, timestamp_period) = unsafe {
      let logical_device = logical_device.borrow();
      let pool = logical_device.raw.create_query_pool(&query_pool_info, None)
        .map_err(|err| HalaGfxError::new("Failed to create query pool.", Some(Box::new(err))))?;
      logical_device.set_debug_name(
        pool,
        debug_name,
      ).map_err(|err| HalaGfxError::new("Failed to set debug name.", Some(Box::new(err))))?;
      (pool, physical_device.properties.limits.timestamp_period as f64)
    };


    log::debug!("A HalaQueryPool \"{}\" is created.", debug_name);
    Ok(Self {
      logical_device,
      raw,
      size: count,
      timestamp_period,
      debug_name: debug_name.to_string(),
    })
  }

  /// Create a new query pool.
  /// param logical_device: The logical device.
  /// param count: The query count.
  /// param pipeline_statistics: The pipeline statistics.
  /// param debug_name: The debug name.
  /// return: The query pool.
  pub fn new_pipeline_statistics(
    logical_device: std::rc::Rc<std::cell::RefCell<HalaLogicalDevice>>,
    count: u32,
    pipeline_statistics: HalaQueryPipelineStatisticFlags,
    debug_name: &str,
  ) -> Result<Self, HalaGfxError> {
    let query_pool_info = vk::QueryPoolCreateInfo::default()
      .query_type(vk::QueryType::PIPELINE_STATISTICS)
      .query_count(count)
      .pipeline_statistics(pipeline_statistics.into());
    let raw = unsafe {
      logical_device.borrow().raw.create_query_pool(&query_pool_info, None)
        .map_err(|err| HalaGfxError::new("Failed to create query pool.", Some(Box::new(err))))?
    };
    logical_device.borrow().set_debug_name(
      raw,
      debug_name,
    ).map_err(|err| HalaGfxError::new("Failed to set debug name.", Some(Box::new(err))))?;

    log::debug!("A HalaQueryPool \"{}\" is created.", debug_name);
    Ok(Self {
      logical_device,
      raw,
      size: count,
      timestamp_period: 0.0,
      debug_name: debug_name.to_string(),
    })
  }

  /// Reset the query.
  /// param first: The first query.
  /// param count: The query count.
  pub fn reset(&self, first: u32, count: u32) {
    unsafe {
      self.logical_device.borrow().raw.reset_query_pool(self.raw, first, count);
    }
  }

  /// Reset all the query.
  pub fn reset_all(&self) {
    self.reset(0, self.size);
  }

  /// Get the data.
  pub fn wait(&self, first: u32) -> Result<Vec<u64>, HalaGfxError> {
    let mut data: Vec<u64> = vec![0; self.size as usize];

    unsafe {
      self.logical_device.borrow().raw.get_query_pool_results(
        self.raw,
        first,
        &mut data,
        vk::QueryResultFlags::TYPE_64 | vk::QueryResultFlags::WAIT,
      ).map_err(|err| HalaGfxError::new("Failed to get query pool results.", Some(Box::new(err))))?;
    }

    Ok(data)
  }
}