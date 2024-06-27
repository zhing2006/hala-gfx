use ash::vk;

use crate::{
  HalaGfxError,
  HalaLogicalDevice,
};

/// The filter.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct HalaFilter(i32);
impl HalaFilter {
  pub const NEAREST: Self = Self(vk::Filter::NEAREST.as_raw());
  pub const LINEAR: Self = Self(vk::Filter::LINEAR.as_raw());
}

impl std::convert::From<vk::Filter> for HalaFilter {
  fn from(v: vk::Filter) -> Self {
    Self(v.as_raw())
  }
}

impl std::convert::From<HalaFilter> for vk::Filter {
  fn from(v: HalaFilter) -> Self {
    Self::from_raw(v.0)
  }
}

/// The sampler mipmap mode.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct HalaSamplerMipmapMode(i32);
impl HalaSamplerMipmapMode {
  pub const NEAREST: Self = Self(vk::SamplerMipmapMode::NEAREST.as_raw());
  pub const LINEAR: Self = Self(vk::SamplerMipmapMode::LINEAR.as_raw());
}

impl std::convert::From<vk::SamplerMipmapMode> for HalaSamplerMipmapMode {
  fn from(v: vk::SamplerMipmapMode) -> Self {
    Self(v.as_raw())
  }
}

impl std::convert::From<HalaSamplerMipmapMode> for vk::SamplerMipmapMode {
  fn from(v: HalaSamplerMipmapMode) -> Self {
    Self::from_raw(v.0)
  }
}

/// The sampler address mode.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct HalaSamplerAddressMode(i32);
impl HalaSamplerAddressMode {
  pub const REPEAT: Self = Self(vk::SamplerAddressMode::REPEAT.as_raw());
  pub const MIRRORED_REPEAT: Self = Self(vk::SamplerAddressMode::MIRRORED_REPEAT.as_raw());
  pub const CLAMP_TO_EDGE: Self = Self(vk::SamplerAddressMode::CLAMP_TO_EDGE.as_raw());
  pub const CLAMP_TO_BORDER: Self = Self(vk::SamplerAddressMode::CLAMP_TO_BORDER.as_raw());
  pub const MIRROR_CLAMP_TO_EDGE: Self = Self(vk::SamplerAddressMode::MIRROR_CLAMP_TO_EDGE.as_raw());
}

impl std::convert::From<vk::SamplerAddressMode> for HalaSamplerAddressMode {
  fn from(v: vk::SamplerAddressMode) -> Self {
    Self(v.as_raw())
  }
}

impl std::convert::From<HalaSamplerAddressMode> for vk::SamplerAddressMode {
  fn from(v: HalaSamplerAddressMode) -> Self {
    Self::from_raw(v.0)
  }
}

/// The sampler.
pub struct HalaSampler {
  pub(crate) logical_device: std::rc::Rc<std::cell::RefCell<HalaLogicalDevice>>,
  pub raw: vk::Sampler,
  pub(crate) debug_name: String,
}

/// The AsRef implementation for sampler.
impl AsRef<HalaSampler> for HalaSampler {
  fn as_ref(&self) -> &HalaSampler {
    self
  }
}

/// The Drop implementation for sampler.
impl Drop for HalaSampler {
  fn drop(&mut self) {
    unsafe {
      let logical_device = self.logical_device.borrow();
      logical_device.raw.destroy_sampler(self.raw, None);
    }
    log::debug!("The HalaSampler \"{}\" is dropped.", self.debug_name);
  }
}

/// The implementation for sampler.
impl HalaSampler {
  /// Create a new sampler.
  /// param logical_device: The logical device.
  /// param filters: The filters(mag filter, min filter).
  /// param mipmap_mode: The mipmap mode.
  /// param address_modes: The address modes(u, v, w).
  /// param mip_lod_bias: The mip lod bias.
  /// param anisotropy_enable: The anisotropy enable.
  /// param max_anisotropy: The max anisotropy.
  /// param lod: The lod(min lod, max lod).
  /// param debug_name: The debug name.
  /// return: The sampler.
  #[allow(clippy::too_many_arguments)]
  pub fn new(
    logical_device: std::rc::Rc<std::cell::RefCell<HalaLogicalDevice>>,
    filters: (HalaFilter, HalaFilter),
    mipmap_mode: HalaSamplerMipmapMode,
    address_modes: (HalaSamplerAddressMode, HalaSamplerAddressMode, HalaSamplerAddressMode),
    mip_lod_bias: f32,
    anisotropy_enable: bool,
    max_anisotropy: f32,
    lod: (f32, f32),
    debug_name: &str,
  ) -> Result<Self, HalaGfxError> {
    let create_info = vk::SamplerCreateInfo::default()
      .mag_filter(filters.0.into())
      .min_filter(filters.1.into())
      .mipmap_mode(mipmap_mode.into())
      .address_mode_u(address_modes.0.into())
      .address_mode_v(address_modes.1.into())
      .address_mode_w(address_modes.2.into())
      .mip_lod_bias(mip_lod_bias)
      .anisotropy_enable(anisotropy_enable)
      .max_anisotropy(max_anisotropy)
      .min_lod(lod.0)
      .max_lod(lod.1)
      .unnormalized_coordinates(false);
    let raw = unsafe {
      let sampler = logical_device.borrow().raw.create_sampler(&create_info, None)
        .map_err(|err| HalaGfxError::new("Failed to create sampler.", Some(Box::new(err))))?;
      logical_device.borrow_mut().set_debug_name(
        sampler,
        debug_name,
      ).map_err(|err| HalaGfxError::new("Failed to set debug name for sampler.", Some(Box::new(err))))?;
      sampler
    };

    log::debug!("The HalaSampler \"{}\" is created.", debug_name);
    Ok(Self {
      logical_device,
      raw,
      debug_name: debug_name.to_string(),
    })
  }
}