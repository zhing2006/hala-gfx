use ash::vk;

/// The format.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct HalaFormat(i32);
impl HalaFormat {
  pub const UNDEFINED: Self = Self(vk::Format::UNDEFINED.as_raw());
  pub const R4G4_UNORM_PACK8: Self = Self(vk::Format::R4G4_UNORM_PACK8.as_raw());
  pub const R4G4B4A4_UNORM_PACK16: Self = Self(vk::Format::R4G4B4A4_UNORM_PACK16.as_raw());
  pub const B4G4R4A4_UNORM_PACK16: Self = Self(vk::Format::B4G4R4A4_UNORM_PACK16.as_raw());
  pub const R5G6B5_UNORM_PACK16: Self = Self(vk::Format::R5G6B5_UNORM_PACK16.as_raw());
  pub const B5G6R5_UNORM_PACK16: Self = Self(vk::Format::B5G6R5_UNORM_PACK16.as_raw());
  pub const R5G5B5A1_UNORM_PACK16: Self = Self(vk::Format::R5G5B5A1_UNORM_PACK16.as_raw());
  pub const B5G5R5A1_UNORM_PACK16: Self = Self(vk::Format::B5G5R5A1_UNORM_PACK16.as_raw());
  pub const A1R5G5B5_UNORM_PACK16: Self = Self(vk::Format::A1R5G5B5_UNORM_PACK16.as_raw());
  pub const R8_UNORM: Self = Self(vk::Format::R8_UNORM.as_raw());
  pub const R8_SNORM: Self = Self(vk::Format::R8_SNORM.as_raw());
  pub const R8_USCALED: Self = Self(vk::Format::R8_USCALED.as_raw());
  pub const R8_SSCALED: Self = Self(vk::Format::R8_SSCALED.as_raw());
  pub const R8_UINT: Self = Self(vk::Format::R8_UINT.as_raw());
  pub const R8_SINT: Self = Self(vk::Format::R8_SINT.as_raw());
  pub const R8_SRGB: Self = Self(vk::Format::R8_SRGB.as_raw());
  pub const R8G8_UNORM: Self = Self(vk::Format::R8G8_UNORM.as_raw());
  pub const R8G8_SNORM: Self = Self(vk::Format::R8G8_SNORM.as_raw());
  pub const R8G8_USCALED: Self = Self(vk::Format::R8G8_USCALED.as_raw());
  pub const R8G8_SSCALED: Self = Self(vk::Format::R8G8_SSCALED.as_raw());
  pub const R8G8_UINT: Self = Self(vk::Format::R8G8_UINT.as_raw());
  pub const R8G8_SINT: Self = Self(vk::Format::R8G8_SINT.as_raw());
  pub const R8G8_SRGB: Self = Self(vk::Format::R8G8_SRGB.as_raw());
  pub const R8G8B8_UNORM: Self = Self(vk::Format::R8G8B8_UNORM.as_raw());
  pub const R8G8B8_SNORM: Self = Self(vk::Format::R8G8B8_SNORM.as_raw());
  pub const R8G8B8_USCALED: Self = Self(vk::Format::R8G8B8_USCALED.as_raw());
  pub const R8G8B8_SSCALED: Self = Self(vk::Format::R8G8B8_SSCALED.as_raw());
  pub const R8G8B8_UINT: Self = Self(vk::Format::R8G8B8_UINT.as_raw());
  pub const R8G8B8_SINT: Self = Self(vk::Format::R8G8B8_SINT.as_raw());
  pub const R8G8B8_SRGB: Self = Self(vk::Format::R8G8B8_SRGB.as_raw());
  pub const B8G8R8_UNORM: Self = Self(vk::Format::B8G8R8_UNORM.as_raw());
  pub const B8G8R8_SNORM: Self = Self(vk::Format::B8G8R8_SNORM.as_raw());
  pub const B8G8R8_USCALED: Self = Self(vk::Format::B8G8R8_USCALED.as_raw());
  pub const B8G8R8_SSCALED: Self = Self(vk::Format::B8G8R8_SSCALED.as_raw());
  pub const B8G8R8_UINT: Self = Self(vk::Format::B8G8R8_UINT.as_raw());
  pub const B8G8R8_SINT: Self = Self(vk::Format::B8G8R8_SINT.as_raw());
  pub const B8G8R8_SRGB: Self = Self(vk::Format::B8G8R8_SRGB.as_raw());
  pub const R8G8B8A8_UNORM: Self = Self(vk::Format::R8G8B8A8_UNORM.as_raw());
  pub const R8G8B8A8_SNORM: Self = Self(vk::Format::R8G8B8A8_SNORM.as_raw());
  pub const R8G8B8A8_USCALED: Self = Self(vk::Format::R8G8B8A8_USCALED.as_raw());
  pub const R8G8B8A8_SSCALED: Self = Self(vk::Format::R8G8B8A8_SSCALED.as_raw());
  pub const R8G8B8A8_UINT: Self = Self(vk::Format::R8G8B8A8_UINT.as_raw());
  pub const R8G8B8A8_SINT: Self = Self(vk::Format::R8G8B8A8_SINT.as_raw());
  pub const R8G8B8A8_SRGB: Self = Self(vk::Format::R8G8B8A8_SRGB.as_raw());
  pub const B8G8R8A8_UNORM: Self = Self(vk::Format::B8G8R8A8_UNORM.as_raw());
  pub const B8G8R8A8_SNORM: Self = Self(vk::Format::B8G8R8A8_SNORM.as_raw());
  pub const B8G8R8A8_USCALED: Self = Self(vk::Format::B8G8R8A8_USCALED.as_raw());
  pub const B8G8R8A8_SSCALED: Self = Self(vk::Format::B8G8R8A8_SSCALED.as_raw());
  pub const B8G8R8A8_UINT: Self = Self(vk::Format::B8G8R8A8_UINT.as_raw());
  pub const B8G8R8A8_SINT: Self = Self(vk::Format::B8G8R8A8_SINT.as_raw());
  pub const B8G8R8A8_SRGB: Self = Self(vk::Format::B8G8R8A8_SRGB.as_raw());
  pub const A8B8G8R8_UNORM_PACK32: Self = Self(vk::Format::A8B8G8R8_UNORM_PACK32.as_raw());
  pub const A8B8G8R8_SNORM_PACK32: Self = Self(vk::Format::A8B8G8R8_SNORM_PACK32.as_raw());
  pub const A8B8G8R8_USCALED_PACK32: Self = Self(vk::Format::A8B8G8R8_USCALED_PACK32.as_raw());
  pub const A8B8G8R8_SSCALED_PACK32: Self = Self(vk::Format::A8B8G8R8_SSCALED_PACK32.as_raw());
  pub const A8B8G8R8_UINT_PACK32: Self = Self(vk::Format::A8B8G8R8_UINT_PACK32.as_raw());
  pub const A8B8G8R8_SINT_PACK32: Self = Self(vk::Format::A8B8G8R8_SINT_PACK32.as_raw());
  pub const A8B8G8R8_SRGB_PACK32: Self = Self(vk::Format::A8B8G8R8_SRGB_PACK32.as_raw());
  pub const A2R10G10B10_UNORM_PACK32: Self = Self(vk::Format::A2R10G10B10_UNORM_PACK32.as_raw());
  pub const A2R10G10B10_SNORM_PACK32: Self = Self(vk::Format::A2R10G10B10_SNORM_PACK32.as_raw());
  pub const A2R10G10B10_USCALED_PACK32: Self = Self(vk::Format::A2R10G10B10_USCALED_PACK32.as_raw());
  pub const A2R10G10B10_SSCALED_PACK32: Self = Self(vk::Format::A2R10G10B10_SSCALED_PACK32.as_raw());
  pub const A2R10G10B10_UINT_PACK32: Self = Self(vk::Format::A2R10G10B10_UINT_PACK32.as_raw());
  pub const A2R10G10B10_SINT_PACK32: Self = Self(vk::Format::A2R10G10B10_SINT_PACK32.as_raw());
  pub const A2B10G10R10_UNORM_PACK32: Self = Self(vk::Format::A2B10G10R10_UNORM_PACK32.as_raw());
  pub const A2B10G10R10_SNORM_PACK32: Self = Self(vk::Format::A2B10G10R10_SNORM_PACK32.as_raw());
  pub const A2B10G10R10_USCALED_PACK32: Self = Self(vk::Format::A2B10G10R10_USCALED_PACK32.as_raw());
  pub const A2B10G10R10_SSCALED_PACK32: Self = Self(vk::Format::A2B10G10R10_SSCALED_PACK32.as_raw());
  pub const A2B10G10R10_UINT_PACK32: Self = Self(vk::Format::A2B10G10R10_UINT_PACK32.as_raw());
  pub const A2B10G10R10_SINT_PACK32: Self = Self(vk::Format::A2B10G10R10_SINT_PACK32.as_raw());
  pub const R16_UNORM: Self = Self(vk::Format::R16_UNORM.as_raw());
  pub const R16_SNORM: Self = Self(vk::Format::R16_SNORM.as_raw());
  pub const R16_USCALED: Self = Self(vk::Format::R16_USCALED.as_raw());
  pub const R16_SSCALED: Self = Self(vk::Format::R16_SSCALED.as_raw());
  pub const R16_UINT: Self = Self(vk::Format::R16_UINT.as_raw());
  pub const R16_SINT: Self = Self(vk::Format::R16_SINT.as_raw());
  pub const R16_SFLOAT: Self = Self(vk::Format::R16_SFLOAT.as_raw());
  pub const R16G16_UNORM: Self = Self(vk::Format::R16G16_UNORM.as_raw());
  pub const R16G16_SNORM: Self = Self(vk::Format::R16G16_SNORM.as_raw());
  pub const R16G16_USCALED: Self = Self(vk::Format::R16G16_USCALED.as_raw());
  pub const R16G16_SSCALED: Self = Self(vk::Format::R16G16_SSCALED.as_raw());
  pub const R16G16_UINT: Self = Self(vk::Format::R16G16_UINT.as_raw());
  pub const R16G16_SINT: Self = Self(vk::Format::R16G16_SINT.as_raw());
  pub const R16G16_SFLOAT: Self = Self(vk::Format::R16G16_SFLOAT.as_raw());
  pub const R16G16B16_UNORM: Self = Self(vk::Format::R16G16B16_UNORM.as_raw());
  pub const R16G16B16_SNORM: Self = Self(vk::Format::R16G16B16_SNORM.as_raw());
  pub const R16G16B16_USCALED: Self = Self(vk::Format::R16G16B16_USCALED.as_raw());
  pub const R16G16B16_SSCALED: Self = Self(vk::Format::R16G16B16_SSCALED.as_raw());
  pub const R16G16B16_UINT: Self = Self(vk::Format::R16G16B16_UINT.as_raw());
  pub const R16G16B16_SINT: Self = Self(vk::Format::R16G16B16_SINT.as_raw());
  pub const R16G16B16_SFLOAT: Self = Self(vk::Format::R16G16B16_SFLOAT.as_raw());
  pub const R16G16B16A16_UNORM: Self = Self(vk::Format::R16G16B16A16_UNORM.as_raw());
  pub const R16G16B16A16_SNORM: Self = Self(vk::Format::R16G16B16A16_SNORM.as_raw());
  pub const R16G16B16A16_USCALED: Self = Self(vk::Format::R16G16B16A16_USCALED.as_raw());
  pub const R16G16B16A16_SSCALED: Self = Self(vk::Format::R16G16B16A16_SSCALED.as_raw());
  pub const R16G16B16A16_UINT: Self = Self(vk::Format::R16G16B16A16_UINT.as_raw());
  pub const R16G16B16A16_SINT: Self = Self(vk::Format::R16G16B16A16_SINT.as_raw());
  pub const R16G16B16A16_SFLOAT: Self = Self(vk::Format::R16G16B16A16_SFLOAT.as_raw());
  pub const R32_UINT: Self = Self(vk::Format::R32_UINT.as_raw());
  pub const R32_SINT: Self = Self(vk::Format::R32_SINT.as_raw());
  pub const R32_SFLOAT: Self = Self(vk::Format::R32_SFLOAT.as_raw());
  pub const R32G32_UINT: Self = Self(vk::Format::R32G32_UINT.as_raw());
  pub const R32G32_SINT: Self = Self(vk::Format::R32G32_SINT.as_raw());
  pub const R32G32_SFLOAT: Self = Self(vk::Format::R32G32_SFLOAT.as_raw());
  pub const R32G32B32_UINT: Self = Self(vk::Format::R32G32B32_UINT.as_raw());
  pub const R32G32B32_SINT: Self = Self(vk::Format::R32G32B32_SINT.as_raw());
  pub const R32G32B32_SFLOAT: Self = Self(vk::Format::R32G32B32_SFLOAT.as_raw());
  pub const R32G32B32A32_UINT: Self = Self(vk::Format::R32G32B32A32_UINT.as_raw());
  pub const R32G32B32A32_SINT: Self = Self(vk::Format::R32G32B32A32_SINT.as_raw());
  pub const R32G32B32A32_SFLOAT: Self = Self(vk::Format::R32G32B32A32_SFLOAT.as_raw());
  pub const R64_UINT: Self = Self(vk::Format::R64_UINT.as_raw());
  pub const R64_SINT: Self = Self(vk::Format::R64_SINT.as_raw());
  pub const R64_SFLOAT: Self = Self(vk::Format::R64_SFLOAT.as_raw());
  pub const R64G64_UINT: Self = Self(vk::Format::R64G64_UINT.as_raw());
  pub const R64G64_SINT: Self = Self(vk::Format::R64G64_SINT.as_raw());
  pub const R64G64_SFLOAT: Self = Self(vk::Format::R64G64_SFLOAT.as_raw());
  pub const R64G64B64_UINT: Self = Self(vk::Format::R64G64B64_UINT.as_raw());
  pub const R64G64B64_SINT: Self = Self(vk::Format::R64G64B64_SINT.as_raw());
  pub const R64G64B64_SFLOAT: Self = Self(vk::Format::R64G64B64_SFLOAT.as_raw());
  pub const R64G64B64A64_UINT: Self = Self(vk::Format::R64G64B64A64_UINT.as_raw());
  pub const R64G64B64A64_SINT: Self = Self(vk::Format::R64G64B64A64_SINT.as_raw());
  pub const R64G64B64A64_SFLOAT: Self = Self(vk::Format::R64G64B64A64_SFLOAT.as_raw());
  pub const B10G11R11_UFLOAT_PACK32: Self = Self(vk::Format::B10G11R11_UFLOAT_PACK32.as_raw());
  pub const E5B9G9R9_UFLOAT_PACK32: Self = Self(vk::Format::E5B9G9R9_UFLOAT_PACK32.as_raw());
  pub const D16_UNORM: Self = Self(vk::Format::D16_UNORM.as_raw());
  pub const X8_D24_UNORM_PACK32: Self = Self(vk::Format::X8_D24_UNORM_PACK32.as_raw());
  pub const D32_SFLOAT: Self = Self(vk::Format::D32_SFLOAT.as_raw());
  pub const S8_UINT: Self = Self(vk::Format::S8_UINT.as_raw());
  pub const D16_UNORM_S8_UINT: Self = Self(vk::Format::D16_UNORM_S8_UINT.as_raw());
  pub const D24_UNORM_S8_UINT: Self = Self(vk::Format::D24_UNORM_S8_UINT.as_raw());
  pub const D32_SFLOAT_S8_UINT: Self = Self(vk::Format::D32_SFLOAT_S8_UINT.as_raw());
  pub const BC1_RGB_UNORM_BLOCK: Self = Self(vk::Format::BC1_RGB_UNORM_BLOCK.as_raw());
  pub const BC1_RGB_SRGB_BLOCK: Self = Self(vk::Format::BC1_RGB_SRGB_BLOCK.as_raw());
  pub const BC1_RGBA_UNORM_BLOCK: Self = Self(vk::Format::BC1_RGBA_UNORM_BLOCK.as_raw());
  pub const BC1_RGBA_SRGB_BLOCK: Self = Self(vk::Format::BC1_RGBA_SRGB_BLOCK.as_raw());
  pub const BC2_UNORM_BLOCK: Self = Self(vk::Format::BC2_UNORM_BLOCK.as_raw());
  pub const BC2_SRGB_BLOCK: Self = Self(vk::Format::BC2_SRGB_BLOCK.as_raw());
  pub const BC3_UNORM_BLOCK: Self = Self(vk::Format::BC3_UNORM_BLOCK.as_raw());
  pub const BC3_SRGB_BLOCK: Self = Self(vk::Format::BC3_SRGB_BLOCK.as_raw());
  pub const BC4_UNORM_BLOCK: Self = Self(vk::Format::BC4_UNORM_BLOCK.as_raw());
  pub const BC4_SNORM_BLOCK: Self = Self(vk::Format::BC4_SNORM_BLOCK.as_raw());
  pub const BC5_UNORM_BLOCK: Self = Self(vk::Format::BC5_UNORM_BLOCK.as_raw());
  pub const BC5_SNORM_BLOCK: Self = Self(vk::Format::BC5_SNORM_BLOCK.as_raw());
  pub const BC6H_UFLOAT_BLOCK: Self = Self(vk::Format::BC6H_UFLOAT_BLOCK.as_raw());
  pub const BC6H_SFLOAT_BLOCK: Self = Self(vk::Format::BC6H_SFLOAT_BLOCK.as_raw());
  pub const BC7_UNORM_BLOCK: Self = Self(vk::Format::BC7_UNORM_BLOCK.as_raw());
  pub const BC7_SRGB_BLOCK: Self = Self(vk::Format::BC7_SRGB_BLOCK.as_raw());
  pub const ETC2_R8G8B8_UNORM_BLOCK: Self = Self(vk::Format::ETC2_R8G8B8_UNORM_BLOCK.as_raw());
  pub const ETC2_R8G8B8_SRGB_BLOCK: Self = Self(vk::Format::ETC2_R8G8B8_SRGB_BLOCK.as_raw());
  pub const ETC2_R8G8B8A1_UNORM_BLOCK: Self = Self(vk::Format::ETC2_R8G8B8A1_UNORM_BLOCK.as_raw());
  pub const ETC2_R8G8B8A1_SRGB_BLOCK: Self = Self(vk::Format::ETC2_R8G8B8A1_SRGB_BLOCK.as_raw());
  pub const ETC2_R8G8B8A8_UNORM_BLOCK: Self = Self(vk::Format::ETC2_R8G8B8A8_UNORM_BLOCK.as_raw());
  pub const ETC2_R8G8B8A8_SRGB_BLOCK: Self = Self(vk::Format::ETC2_R8G8B8A8_SRGB_BLOCK.as_raw());
  pub const EAC_R11_UNORM_BLOCK: Self = Self(vk::Format::EAC_R11_UNORM_BLOCK.as_raw());
  pub const EAC_R11_SNORM_BLOCK: Self = Self(vk::Format::EAC_R11_SNORM_BLOCK.as_raw());
  pub const EAC_R11G11_UNORM_BLOCK: Self = Self(vk::Format::EAC_R11G11_UNORM_BLOCK.as_raw());
  pub const EAC_R11G11_SNORM_BLOCK: Self = Self(vk::Format::EAC_R11G11_SNORM_BLOCK.as_raw());
  pub const ASTC_4X4_UNORM_BLOCK: Self = Self(vk::Format::ASTC_4X4_UNORM_BLOCK.as_raw());
  pub const ASTC_4X4_SRGB_BLOCK: Self = Self(vk::Format::ASTC_4X4_SRGB_BLOCK.as_raw());
  pub const ASTC_5X4_UNORM_BLOCK: Self = Self(vk::Format::ASTC_5X4_UNORM_BLOCK.as_raw());
  pub const ASTC_5X4_SRGB_BLOCK: Self = Self(vk::Format::ASTC_5X4_SRGB_BLOCK.as_raw());
  pub const ASTC_5X5_UNORM_BLOCK: Self = Self(vk::Format::ASTC_5X5_UNORM_BLOCK.as_raw());
  pub const ASTC_5X5_SRGB_BLOCK: Self = Self(vk::Format::ASTC_5X5_SRGB_BLOCK.as_raw());
  pub const ASTC_6X5_UNORM_BLOCK: Self = Self(vk::Format::ASTC_6X5_UNORM_BLOCK.as_raw());
  pub const ASTC_6X5_SRGB_BLOCK: Self = Self(vk::Format::ASTC_6X5_SRGB_BLOCK.as_raw());
  pub const ASTC_6X6_UNORM_BLOCK: Self = Self(vk::Format::ASTC_6X6_UNORM_BLOCK.as_raw());
  pub const ASTC_6X6_SRGB_BLOCK: Self = Self(vk::Format::ASTC_6X6_SRGB_BLOCK.as_raw());
  pub const ASTC_8X5_UNORM_BLOCK: Self = Self(vk::Format::ASTC_8X5_UNORM_BLOCK.as_raw());
  pub const ASTC_8X5_SRGB_BLOCK: Self = Self(vk::Format::ASTC_8X5_SRGB_BLOCK.as_raw());
  pub const ASTC_8X6_UNORM_BLOCK: Self = Self(vk::Format::ASTC_8X6_UNORM_BLOCK.as_raw());
  pub const ASTC_8X6_SRGB_BLOCK: Self = Self(vk::Format::ASTC_8X6_SRGB_BLOCK.as_raw());
  pub const ASTC_8X8_UNORM_BLOCK: Self = Self(vk::Format::ASTC_8X8_UNORM_BLOCK.as_raw());
  pub const ASTC_8X8_SRGB_BLOCK: Self = Self(vk::Format::ASTC_8X8_SRGB_BLOCK.as_raw());
  pub const ASTC_10X5_UNORM_BLOCK: Self = Self(vk::Format::ASTC_10X5_UNORM_BLOCK.as_raw());
  pub const ASTC_10X5_SRGB_BLOCK: Self = Self(vk::Format::ASTC_10X5_SRGB_BLOCK.as_raw());
  pub const ASTC_10X6_UNORM_BLOCK: Self = Self(vk::Format::ASTC_10X6_UNORM_BLOCK.as_raw());
  pub const ASTC_10X6_SRGB_BLOCK: Self = Self(vk::Format::ASTC_10X6_SRGB_BLOCK.as_raw());
  pub const ASTC_10X8_UNORM_BLOCK: Self = Self(vk::Format::ASTC_10X8_UNORM_BLOCK.as_raw());
  pub const ASTC_10X8_SRGB_BLOCK: Self = Self(vk::Format::ASTC_10X8_SRGB_BLOCK.as_raw());
  pub const ASTC_10X10_UNORM_BLOCK: Self = Self(vk::Format::ASTC_10X10_UNORM_BLOCK.as_raw());
  pub const ASTC_10X10_SRGB_BLOCK: Self = Self(vk::Format::ASTC_10X10_SRGB_BLOCK.as_raw());
  pub const ASTC_12X10_UNORM_BLOCK: Self = Self(vk::Format::ASTC_12X10_UNORM_BLOCK.as_raw());
  pub const ASTC_12X10_SRGB_BLOCK: Self = Self(vk::Format::ASTC_12X10_SRGB_BLOCK.as_raw());
  pub const ASTC_12X12_UNORM_BLOCK: Self = Self(vk::Format::ASTC_12X12_UNORM_BLOCK.as_raw());
  pub const ASTC_12X12_SRGB_BLOCK: Self = Self(vk::Format::ASTC_12X12_SRGB_BLOCK.as_raw());
}

impl std::fmt::Display for HalaFormat {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:?}", vk::Format::from_raw(self.0))
  }
}

impl std::convert::From<vk::Format> for HalaFormat {
  fn from(val: vk::Format) -> Self {
    Self(val.as_raw())
  }
}

impl std::convert::From<HalaFormat> for vk::Format {
  fn from(val: HalaFormat) -> Self {
    vk::Format::from_raw(val.0)
  }
}
