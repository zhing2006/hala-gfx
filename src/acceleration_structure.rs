use std::rc::Rc;
use std::cell::RefCell;

use ash::vk;

use crate::{
  HalaBuffer,
  HalaBufferUsageFlags,
  HalaCommandBufferSet,
  HalaFormat,
  HalaGfxError,
  HalaLogicalDevice,
  HalaMemoryLocation
};

/// The index type.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct HalaIndexType(i32);
impl HalaIndexType {
  pub const UINT16: Self = Self(vk::IndexType::UINT16.as_raw());
  pub const UINT32: Self = Self(vk::IndexType::UINT32.as_raw());
}

impl std::convert::From<vk::IndexType> for HalaIndexType {
  fn from(val: vk::IndexType) -> Self {
    Self(val.as_raw())
  }
}

impl std::convert::From<HalaIndexType> for vk::IndexType {
  fn from(val: HalaIndexType) -> Self {
    vk::IndexType::from_raw(val.0)
  }
}

// The acceleration structure level.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct HalaAccelerationStructureLevel(i32);
impl HalaAccelerationStructureLevel {
  pub const TOP_LEVEL: Self = Self(vk::AccelerationStructureTypeKHR::TOP_LEVEL.as_raw());
  pub const BOTTOM_LEVEL: Self = Self(vk::AccelerationStructureTypeKHR::BOTTOM_LEVEL.as_raw());
  pub const GENERIC: Self = Self(vk::AccelerationStructureTypeKHR::GENERIC.as_raw());
}

impl std::convert::From<vk::AccelerationStructureTypeKHR> for HalaAccelerationStructureLevel {
  fn from(val: vk::AccelerationStructureTypeKHR) -> Self {
    Self(val.as_raw())
  }
}

impl std::convert::From<HalaAccelerationStructureLevel> for vk::AccelerationStructureTypeKHR {
  fn from(val: HalaAccelerationStructureLevel) -> Self {
    vk::AccelerationStructureTypeKHR::from_raw(val.0)
  }
}

/// The acceleration structure geometry triangles data.
#[derive(Clone, Default)]
pub struct HalaAccelerationStructureGeometryTrianglesData {
  pub vertex_format: HalaFormat,
  pub vertex_data_address: u64,
  pub vertex_stride: u64,
  pub vertex_count: u32,
  pub index_type: HalaIndexType,
  pub index_data_address: u64,
  pub transform_data_address: u64,
}

impl std::convert::From<vk::AccelerationStructureGeometryTrianglesDataKHR<'_>> for HalaAccelerationStructureGeometryTrianglesData {
  fn from(val: vk::AccelerationStructureGeometryTrianglesDataKHR) -> Self {
    Self::from(&val)
  }
}

impl std::convert::From<&vk::AccelerationStructureGeometryTrianglesDataKHR<'_>> for HalaAccelerationStructureGeometryTrianglesData {
  fn from(val: &vk::AccelerationStructureGeometryTrianglesDataKHR) -> Self {
    Self {
      vertex_format: val.vertex_format.into(),
      vertex_data_address: unsafe { val.vertex_data.device_address },
      vertex_stride: val.vertex_stride,
      vertex_count: val.max_vertex,
      index_type: val.index_type.into(),
      index_data_address: unsafe { val.index_data.device_address },
      transform_data_address: unsafe { val.transform_data.device_address },
    }
  }
}

impl std::convert::From<HalaAccelerationStructureGeometryTrianglesData> for vk::AccelerationStructureGeometryTrianglesDataKHR<'_> {
  fn from(val: HalaAccelerationStructureGeometryTrianglesData) -> Self {
    Self::from(&val)
  }
}

impl std::convert::From<&HalaAccelerationStructureGeometryTrianglesData> for vk::AccelerationStructureGeometryTrianglesDataKHR<'_> {
  fn from(val: &HalaAccelerationStructureGeometryTrianglesData) -> Self {
    vk::AccelerationStructureGeometryTrianglesDataKHR::default()
      .vertex_format(val.vertex_format.into())
      .vertex_data(vk::DeviceOrHostAddressConstKHR {
        device_address: val.vertex_data_address,
      })
      .vertex_stride(val.vertex_stride)
      .max_vertex(val.vertex_count)
      .index_type(val.index_type.into())
      .index_data(vk::DeviceOrHostAddressConstKHR {
        device_address: val.index_data_address,
      })
      .transform_data(vk::DeviceOrHostAddressConstKHR {
        device_address: val.transform_data_address,
      })
  }
}

/// The acceleration structure geometry aabbs data.
#[derive(Clone, Default)]
pub struct HalaAccelerationStructureGeometryAabbsData {
  pub data_address: u64,
  pub stride: u64,
}

impl std::convert::From<vk::AccelerationStructureGeometryAabbsDataKHR<'_>> for HalaAccelerationStructureGeometryAabbsData {
  fn from(val: vk::AccelerationStructureGeometryAabbsDataKHR) -> Self {
    Self::from(&val)
  }
}

impl std::convert::From<&vk::AccelerationStructureGeometryAabbsDataKHR<'_>> for HalaAccelerationStructureGeometryAabbsData {
  fn from(val: &vk::AccelerationStructureGeometryAabbsDataKHR) -> Self {
    Self {
      data_address: unsafe { val.data.device_address },
      stride: val.stride,
    }
  }
}

impl std::convert::From<HalaAccelerationStructureGeometryAabbsData> for vk::AccelerationStructureGeometryAabbsDataKHR<'_> {
  fn from(val: HalaAccelerationStructureGeometryAabbsData) -> Self {
    Self::from(&val)
  }
}

impl std::convert::From<&HalaAccelerationStructureGeometryAabbsData> for vk::AccelerationStructureGeometryAabbsDataKHR<'_> {
  fn from(val: &HalaAccelerationStructureGeometryAabbsData) -> Self {
    vk::AccelerationStructureGeometryAabbsDataKHR::default()
      .data(vk::DeviceOrHostAddressConstKHR {
        device_address: val.data_address,
      })
      .stride(val.stride)
  }
}

/// The acceleration structure geometry instances data.
#[derive(Clone, Default)]
pub struct HalaAccelerationStructureGeometryInstancesData {
  pub array_of_pointers: bool,
  pub data_address: u64,
}

impl std::convert::From<vk::AccelerationStructureGeometryInstancesDataKHR<'_>> for HalaAccelerationStructureGeometryInstancesData {
  fn from(val: vk::AccelerationStructureGeometryInstancesDataKHR) -> Self {
    Self::from(&val)
  }
}

impl std::convert::From<&vk::AccelerationStructureGeometryInstancesDataKHR<'_>> for HalaAccelerationStructureGeometryInstancesData {
  fn from(val: &vk::AccelerationStructureGeometryInstancesDataKHR) -> Self {
    Self {
      array_of_pointers: val.array_of_pointers != 0,
      data_address: unsafe { val.data.device_address },
    }
  }
}

impl std::convert::From<HalaAccelerationStructureGeometryInstancesData> for vk::AccelerationStructureGeometryInstancesDataKHR<'_> {
  fn from(val: HalaAccelerationStructureGeometryInstancesData) -> Self {
    Self::from(&val)
  }
}

impl std::convert::From<&HalaAccelerationStructureGeometryInstancesData> for vk::AccelerationStructureGeometryInstancesDataKHR<'_> {
  fn from(val: &HalaAccelerationStructureGeometryInstancesData) -> Self {
    vk::AccelerationStructureGeometryInstancesDataKHR::default()
      .array_of_pointers(val.array_of_pointers)
      .data(vk::DeviceOrHostAddressConstKHR {
        device_address: val.data_address,
      })
  }
}

/// The geometry type.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct HalaGeometryType(i32);
impl HalaGeometryType {
  pub const TRIANGLES: Self = Self(vk::GeometryTypeKHR::TRIANGLES.as_raw());
  pub const AABBS: Self = Self(vk::GeometryTypeKHR::AABBS.as_raw());
  pub const INSTANCES: Self = Self(vk::GeometryTypeKHR::INSTANCES.as_raw());
}

impl std::convert::From<vk::GeometryTypeKHR> for HalaGeometryType {
  fn from(val: vk::GeometryTypeKHR) -> Self {
    Self(val.as_raw())
  }
}

impl std::convert::From<HalaGeometryType> for vk::GeometryTypeKHR {
  fn from(val: HalaGeometryType) -> Self {
    vk::GeometryTypeKHR::from_raw(val.0)
  }
}

/// The geometry flags.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HalaGeometryFlags(u32);
crate::hala_bitflags_wrapped!(HalaGeometryFlags, u32);
impl HalaGeometryFlags {
  pub const OPAQUE: Self = Self(vk::GeometryFlagsKHR::OPAQUE.as_raw());
  pub const NO_DUPLICATE_ANY_HIT_INVOCATION: Self = Self(vk::GeometryFlagsKHR::NO_DUPLICATE_ANY_HIT_INVOCATION.as_raw());
}

impl std::convert::From<vk::GeometryFlagsKHR> for HalaGeometryFlags {
  fn from(flags: vk::GeometryFlagsKHR) -> Self {
    Self(flags.as_raw())
  }
}

impl std::convert::From<HalaGeometryFlags> for vk::GeometryFlagsKHR {
  fn from(flags: HalaGeometryFlags) -> Self {
    vk::GeometryFlagsKHR::from_raw(flags.0)
  }
}


/// The geometry instance flags.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HalaGeometryInstanceFlags(u8);
crate::hala_bitflags_wrapped!(HalaGeometryInstanceFlags, u8);
impl HalaGeometryInstanceFlags {
  pub const TRIANGLE_FACING_CULL_DISABLE: Self = Self(vk::GeometryInstanceFlagsKHR::TRIANGLE_FACING_CULL_DISABLE.as_raw() as u8);
  pub const TRIANGLE_FLIP_FACING: Self = Self(vk::GeometryInstanceFlagsKHR::TRIANGLE_FLIP_FACING.as_raw() as u8);
  pub const FORCE_OPAQUE: Self = Self(vk::GeometryInstanceFlagsKHR::FORCE_OPAQUE.as_raw() as u8);
  pub const FORCE_NO_OPAQUE: Self = Self(vk::GeometryInstanceFlagsKHR::FORCE_NO_OPAQUE.as_raw() as u8);
  pub const TRIANGLE_FRONT_COUNTERCLOCKWISE: Self = Self(vk::GeometryInstanceFlagsKHR::TRIANGLE_FRONT_COUNTERCLOCKWISE.as_raw() as u8);
}

impl std::convert::From<vk::GeometryInstanceFlagsKHR> for HalaGeometryInstanceFlags {
  fn from(flags: vk::GeometryInstanceFlagsKHR) -> Self {
    Self(flags.as_raw() as u8)
  }
}

impl std::convert::From<HalaGeometryInstanceFlags> for vk::GeometryInstanceFlagsKHR {
  fn from(flags: HalaGeometryInstanceFlags) -> Self {
    vk::GeometryInstanceFlagsKHR::from_raw(flags.0 as u32)
  }
}

/// The acceleration structure geometry.
#[derive(Clone, Default)]
pub struct HalaAccelerationStructureGeometry {
  pub ty: HalaGeometryType,
  pub flags: HalaGeometryFlags,
  pub triangles_data: Option<HalaAccelerationStructureGeometryTrianglesData>,
  pub aabbs_data: Option<HalaAccelerationStructureGeometryAabbsData>,
  pub instances_data: Option<HalaAccelerationStructureGeometryInstancesData>,
}

/// The AsRef trait implementation of the acceleration structure geometry.
impl AsRef<HalaAccelerationStructureGeometry> for HalaAccelerationStructureGeometry {
  fn as_ref(&self) -> &HalaAccelerationStructureGeometry {
    self
  }
}

/// The From trait implementation of the acceleration structure geometry.
impl std::convert::From<vk::AccelerationStructureGeometryKHR<'_>> for HalaAccelerationStructureGeometry {
  fn from(val: vk::AccelerationStructureGeometryKHR) -> Self {
    let mut triangles_data = None;
    let mut aabbs_data = None;
    let mut instances_data = None;
    unsafe {
      match val.geometry_type {
        vk::GeometryTypeKHR::TRIANGLES => {
          triangles_data = Some(val.geometry.triangles.into());
        },
        vk::GeometryTypeKHR::AABBS => {
          aabbs_data = Some(val.geometry.aabbs.into());
        },
        vk::GeometryTypeKHR::INSTANCES => {
          instances_data = Some(val.geometry.instances.into());
        },
        _ => {
          panic!("Unsupported geometry type.");
        },
      }
    }
    Self {
      ty: val.geometry_type.into(),
      flags: val.flags.into(),
      triangles_data,
      aabbs_data,
      instances_data,
    }
  }
}

impl std::convert::From<&vk::AccelerationStructureGeometryKHR<'_>> for HalaAccelerationStructureGeometry {
  fn from(val: &vk::AccelerationStructureGeometryKHR) -> Self {
    let mut triangles_data = None;
    let mut aabbs_data = None;
    let mut instances_data = None;
    unsafe {
      match val.geometry_type {
        vk::GeometryTypeKHR::TRIANGLES => {
          triangles_data = Some(val.geometry.triangles.into());
        },
        vk::GeometryTypeKHR::AABBS => {
          aabbs_data = Some(val.geometry.aabbs.into());
        },
        vk::GeometryTypeKHR::INSTANCES => {
          instances_data = Some(val.geometry.instances.into());
        },
        _ => {
          panic!("Unsupported geometry type.");
        },
      }
    }
    Self {
      ty: val.geometry_type.into(),
      flags: val.flags.into(),
      triangles_data,
      aabbs_data,
      instances_data,
    }
  }

}

impl std::convert::From<HalaAccelerationStructureGeometry> for vk::AccelerationStructureGeometryKHR<'_> {
  fn from(val: HalaAccelerationStructureGeometry) -> Self {
    let mut geometry = vk::AccelerationStructureGeometryDataKHR::default();
    match val.ty {
      HalaGeometryType::TRIANGLES => {
        geometry.triangles = val.triangles_data.unwrap().into();
      },
      HalaGeometryType::AABBS => {
        geometry.aabbs = val.aabbs_data.unwrap().into();
      },
      HalaGeometryType::INSTANCES => {
        geometry.instances = val.instances_data.unwrap().into();
      },
      _ => {
        panic!("Unsupported geometry type.");
      },
    }
    vk::AccelerationStructureGeometryKHR::default()
      .geometry_type(val.ty.into())
      .flags(val.flags.into())
      .geometry(geometry)
  }
}

impl std::convert::From<&HalaAccelerationStructureGeometry> for vk::AccelerationStructureGeometryKHR<'_> {
  fn from(val: &HalaAccelerationStructureGeometry) -> Self {
    let mut geometry = vk::AccelerationStructureGeometryDataKHR::default();
    match val.ty {
      HalaGeometryType::TRIANGLES => {
        geometry.triangles = val.triangles_data.as_ref().unwrap().into();
      },
      HalaGeometryType::AABBS => {
        geometry.aabbs = val.aabbs_data.as_ref().unwrap().into();
      },
      HalaGeometryType::INSTANCES => {
        geometry.instances = val.instances_data.as_ref().unwrap().into();
      },
      _ => {
        panic!("Unsupported geometry type.");
      },
    }
    vk::AccelerationStructureGeometryKHR::default()
      .geometry_type(val.ty.into())
      .flags(val.flags.into())
      .geometry(geometry)
  }

}

/// The acceleration structure instance.
#[derive(Clone, Default)]
pub struct HalaAccelerationStructureInstance {
  pub transform: [f32; 12],
  pub custom_index: u32,
  pub mask: u8,
  pub shader_binding_table_record_offset: u32,
  pub shader_binding_table_flags: HalaGeometryInstanceFlags,
  pub acceleration_structure_device_address: u64,
}

impl HalaAccelerationStructureInstance {
  pub fn as_data(&self) -> vk::AccelerationStructureInstanceKHR {
    self.into()
  }
}

impl std::convert::From<vk::AccelerationStructureInstanceKHR> for HalaAccelerationStructureInstance {
  fn from(val: vk::AccelerationStructureInstanceKHR) -> Self {
    Self::from(&val)
  }
}

impl std::convert::From<&vk::AccelerationStructureInstanceKHR> for HalaAccelerationStructureInstance {
  fn from(val: &vk::AccelerationStructureInstanceKHR) -> Self {
    Self {
      transform: val.transform.matrix,
      custom_index: val.instance_custom_index_and_mask.low_24(),
      mask: val.instance_custom_index_and_mask.high_8(),
      shader_binding_table_record_offset: val.instance_shader_binding_table_record_offset_and_flags.low_24(),
      shader_binding_table_flags: HalaGeometryInstanceFlags(val.instance_shader_binding_table_record_offset_and_flags.high_8()),
      acceleration_structure_device_address: unsafe { val.acceleration_structure_reference.device_handle },
    }
  }
}

impl std::convert::From<HalaAccelerationStructureInstance> for vk::AccelerationStructureInstanceKHR {
  fn from(val: HalaAccelerationStructureInstance) -> Self {
    Self::from(&val)
  }
}

impl std::convert::From<&HalaAccelerationStructureInstance> for vk::AccelerationStructureInstanceKHR {
  fn from(val: &HalaAccelerationStructureInstance) -> Self {
    vk::AccelerationStructureInstanceKHR {
      transform: vk::TransformMatrixKHR {
        matrix: val.transform,
      },
      instance_custom_index_and_mask: vk::Packed24_8::new(val.custom_index, val.mask),
      instance_shader_binding_table_record_offset_and_flags: vk::Packed24_8::new(val.shader_binding_table_record_offset, val.shader_binding_table_flags.as_raw()),
      acceleration_structure_reference: vk::AccelerationStructureReferenceKHR {
        device_handle: val.acceleration_structure_device_address,
      },
    }
  }
}

/// The acceleration structure build range info.
#[derive(Clone, Default)]
pub struct HalaAccelerationStructureBuildRangeInfo {
  pub primitive_count: u32,
  pub primitive_offset: u32,
  pub first_vertex: u32,
  pub transform_offset: u32,
}

/// The AsRef trait implementation of the acceleration structure build range info.
impl AsRef<HalaAccelerationStructureBuildRangeInfo> for HalaAccelerationStructureBuildRangeInfo {
  fn as_ref(&self) -> &HalaAccelerationStructureBuildRangeInfo {
    self
  }
}

/// The From trait implementation of the acceleration structure build range info.
impl std::convert::From<vk::AccelerationStructureBuildRangeInfoKHR> for HalaAccelerationStructureBuildRangeInfo {
  fn from(val: vk::AccelerationStructureBuildRangeInfoKHR) -> Self {
    Self::from(&val)
  }
}

impl std::convert::From<&vk::AccelerationStructureBuildRangeInfoKHR> for HalaAccelerationStructureBuildRangeInfo {
  fn from(val: &vk::AccelerationStructureBuildRangeInfoKHR) -> Self {
    Self {
      primitive_count: val.primitive_count,
      primitive_offset: val.primitive_offset,
      first_vertex: val.first_vertex,
      transform_offset: val.transform_offset,
    }
  }
}

impl std::convert::From<HalaAccelerationStructureBuildRangeInfo> for vk::AccelerationStructureBuildRangeInfoKHR {
  fn from(val: HalaAccelerationStructureBuildRangeInfo) -> Self {
    Self::from(&val)
  }
}

impl std::convert::From<&HalaAccelerationStructureBuildRangeInfo> for vk::AccelerationStructureBuildRangeInfoKHR {
  fn from(val: &HalaAccelerationStructureBuildRangeInfo) -> Self {
    vk::AccelerationStructureBuildRangeInfoKHR::default()
      .primitive_count(val.primitive_count)
      .primitive_offset(val.primitive_offset)
      .first_vertex(val.first_vertex)
      .transform_offset(val.transform_offset)
  }
}

/// The acceleration structure.
pub struct HalaAccelerationStructure {
  pub(crate) logical_device: Rc<RefCell<HalaLogicalDevice>>,
  pub raw: vk::AccelerationStructureKHR,
  pub buffer: HalaBuffer,
  pub address: u64,
  pub(crate) debug_name: String,
}

/// The AsRef trait implementation of the acceleration structure.
impl AsRef<HalaAccelerationStructure> for HalaAccelerationStructure {
  fn as_ref(&self) -> &HalaAccelerationStructure {
    self
  }
}

/// The Drop trait implementation of the acceleration structure.
impl Drop for HalaAccelerationStructure {
  fn drop(&mut self) {
    unsafe {
      self.logical_device.borrow()
        .acceleration_structure_loader
        .destroy_acceleration_structure(self.raw, None);
    }
    log::debug!("A HalaAccelerationStructure \"{}\" is dropped.", self.debug_name);
  }
}

/// The implementation of the acceleration structure.
impl HalaAccelerationStructure {
  pub fn new<ASG, ASBRI>(
    logical_device: Rc<RefCell<HalaLogicalDevice>>,
    graphics_command_buffers: &HalaCommandBufferSet,
    level: HalaAccelerationStructureLevel,
    geometries: &[ASG],
    range_infos: &[&[ASBRI]],
    max_primitive_counts: &[u32],
    debug_name: &str,
  ) -> Result<Self, HalaGfxError>
    where ASG: AsRef<HalaAccelerationStructureGeometry>,
          ASBRI: AsRef<HalaAccelerationStructureBuildRangeInfo>
  {
    let geometries = geometries.iter()
      .map(|geometry| geometry.as_ref().into())
      .collect::<Vec<_>>();
    let range_infos = range_infos.iter()
      .map(|&ris| ris.iter().map(|ri| ri.as_ref().into()).collect::<Vec<_>>())
      .collect::<Vec<_>>();
    let range_infos = range_infos.iter()
      .map(|ris| ris.as_slice())
      .collect::<Vec<_>>();

    let build_geometry_info = vk::AccelerationStructureBuildGeometryInfoKHR::default()
      .ty(level.into())
      .flags(vk::BuildAccelerationStructureFlagsKHR::PREFER_FAST_TRACE)
      .geometries(
        geometries.as_slice(),
      );
    let build_size = unsafe {
      let mut size_info = vk::AccelerationStructureBuildSizesInfoKHR::default();
      logical_device.borrow()
        .acceleration_structure_loader
        .get_acceleration_structure_build_sizes(
          vk::AccelerationStructureBuildTypeKHR::DEVICE,
          &build_geometry_info,
          max_primitive_counts,
          &mut size_info,
        );
      size_info
    };

    let buffer = HalaBuffer::new(
      Rc::clone(&logical_device),
      build_size.acceleration_structure_size,
      HalaBufferUsageFlags::ACCELERATION_STRUCTURE_STORAGE | HalaBufferUsageFlags::SHADER_DEVICE_ADDRESS,
      HalaMemoryLocation::GpuOnly,
      &format!("{}.buffer", debug_name),
    )?;

    let create_info = vk::AccelerationStructureCreateInfoKHR::default()
      .buffer(buffer.raw)
      .size(build_size.acceleration_structure_size)
      .ty(level.into());

    let acceleration_structure = unsafe {
      let logical_device = logical_device.borrow();
      let acceleration_structure = logical_device
        .acceleration_structure_loader
        .create_acceleration_structure(&create_info, None)
        .map_err(|err| HalaGfxError::new("Failed to create the acceleration structure.", Some(Box::new(err))))?;
      logical_device.set_debug_name(
        acceleration_structure,
        debug_name,
      ).map_err(|err| HalaGfxError::new("Failed to set debug name for the acceleration structure.", Some(Box::new(err))))?;
      acceleration_structure
    };

    let scratch_buffer_alignment = logical_device.borrow().min_acceleration_structure_scratch_offset_alignment as u64;
    let scratch_buffer = HalaBuffer::new(
      Rc::clone(&logical_device),
      build_size.build_scratch_size + scratch_buffer_alignment,
      HalaBufferUsageFlags::STORAGE_BUFFER | HalaBufferUsageFlags::SHADER_DEVICE_ADDRESS,
      HalaMemoryLocation::GpuOnly,
      &format!("{}.scratch.buffer", debug_name),
    )?;
    let scratch_buffer_address = scratch_buffer.get_device_address();
    let scratch_buffer_address = (scratch_buffer_address + scratch_buffer_alignment - 1) & !(scratch_buffer_alignment - 1);

    let build_geometry_info = vk::AccelerationStructureBuildGeometryInfoKHR::default()
      .ty(level.into())
      .flags(vk::BuildAccelerationStructureFlagsKHR::PREFER_FAST_TRACE)
      .mode(vk::BuildAccelerationStructureModeKHR::BUILD)
      .dst_acceleration_structure(acceleration_structure)
      .geometries(geometries.as_slice())
      .scratch_data(vk::DeviceOrHostAddressKHR {
        device_address: scratch_buffer_address,
      });

    unsafe {
      logical_device.borrow().graphics_execute_and_submit(graphics_command_buffers, 0, |logical_device, command_buffers, index| {
        logical_device.acceleration_structure_loader.cmd_build_acceleration_structures(
          command_buffers.raw[index],
          std::slice::from_ref(&build_geometry_info),
          range_infos.as_slice(),
        );
      },
      0)?;
    }

    let address_info = vk::AccelerationStructureDeviceAddressInfoKHR::default()
      .acceleration_structure(acceleration_structure);
    let address = unsafe {
      logical_device.borrow()
        .acceleration_structure_loader
        .get_acceleration_structure_device_address(&address_info)
    };

    log::debug!("A HalaAccelerationStructure \"{}\" is created.", debug_name);
    Ok(Self {
      logical_device: logical_device.clone(),
      raw: acceleration_structure,
      buffer,
      address,
      debug_name: debug_name.to_string(),
    })
  }
}