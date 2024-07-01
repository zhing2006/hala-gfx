use std::ffi::CString;

use ash::vk;

use crate::error::HalaGfxError;

unsafe extern "system" fn vulkan_debug_utils_callback(
  message_severity: vk::DebugUtilsMessageSeverityFlagsEXT,
  message_type: vk::DebugUtilsMessageTypeFlagsEXT,
  p_callback_data: *const vk::DebugUtilsMessengerCallbackDataEXT,
  _p_user_data: *mut std::ffi::c_void,
) -> vk::Bool32 {
  let message = std::ffi::CStr::from_ptr((*p_callback_data).p_message);
  let severity = format!("{:?}", message_severity).to_lowercase();
  let ty = format!("{:?}", message_type).to_lowercase();
  match severity {
    s if s.contains("error") => log::error!("[{}] {:?}", ty, message),
    s if s.contains("warning") => log::warn!("[{}] {:?}", ty, message),
    s if s.contains("info") => log::info!("[{}] {:?}", ty, message),
    s if s.contains("verbose") => log::debug!("[{}] {:?}", ty, message),
    _ => (),
  }
  vk::FALSE
}

/// The instance.
pub struct HalaInstance {
  #[allow(dead_code)]
  pub(crate) entry: ash::Entry,
  pub raw: ash::Instance,

  pub(crate) debug_utils_loader: ash::ext::debug_utils::Instance,
  pub(crate) debug_call_back: vk::DebugUtilsMessengerEXT,
}

/// The Drop trait implementation of the instance.
impl Drop for HalaInstance {
  fn drop(&mut self) {
    unsafe {
      self.debug_utils_loader.destroy_debug_utils_messenger(self.debug_call_back, None);
      self.raw.destroy_instance(None);
    }
    log::debug!("A HalaInstance is dropped.");
  }
}

/// The implementation of the instance.
impl HalaInstance {
  /// Create a new instance.
  /// param name: The name of the instance.
  /// param gpu_req: The GPU requirements.
  /// return: The instance.
  pub fn new(name: &str, gpu_req: &crate::HalaGPURequirements) -> Result<Self, HalaGfxError> {
    // Load Vulkan entry.
    let entry = unsafe {
      ash::Entry::load()
        .map_err(|err| HalaGfxError::new("Failed to load Vulkan entry.", Some(Box::new(err))))?
    };

    // Create Vulkan instance.
    let (debug_utils_loader, debug_call_back, instance) = Self::create_instance(name, gpu_req, &entry)?;

    log::debug!("A HalaInstance is created.");
    Ok(
      Self {
        entry,
        raw: instance,
        debug_utils_loader,
        debug_call_back,
      }
    )
  }

  /// Create a Vulkan instance.
  /// param name: The name of the instance.
  /// param gpu_req: The GPU requirements.
  /// param entry: The Vulkan entry.
  /// return: The debug utils loader, the debug call back, and the instance.
  fn create_instance(name: &str, gpu_req: &crate::HalaGPURequirements, entry: &ash::Entry) -> Result<(ash::ext::debug_utils::Instance, vk::DebugUtilsMessengerEXT, ash::Instance), HalaGfxError> {
    let (debug_utils_loader, debug_call_back, instance) = unsafe {
      let app_name = CString::new(name)
        .map_err(|err| HalaGfxError::new("Failed to create CString app_name.", Some(Box::new(err))))?;
      let engine_name = CString::new("Hala")
        .map_err(|err| HalaGfxError::new("Failed to create CString engine_name.", Some(Box::new(err))))?;
      let app_info = vk::ApplicationInfo::default()
        .application_name(&app_name)
        .application_version(vk::make_api_version(0, 0, 1, 0))
        .engine_name(&engine_name)
        .engine_version(vk::make_api_version(0, 0, 1, 0))
        .api_version(vk::make_api_version(0, gpu_req.version.0, gpu_req.version.1, gpu_req.version.2));

      let debug_create_info = if cfg!(debug_assertions) {
        vk::DebugUtilsMessengerCreateInfoEXT::default()
          .message_severity(
            vk::DebugUtilsMessageSeverityFlagsEXT::WARNING
              | vk::DebugUtilsMessageSeverityFlagsEXT::ERROR
              | vk::DebugUtilsMessageSeverityFlagsEXT::INFO
              // | vk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE
          )
      } else {
        vk::DebugUtilsMessengerCreateInfoEXT::default()
          .message_severity(
            vk::DebugUtilsMessageSeverityFlagsEXT::WARNING
              | vk::DebugUtilsMessageSeverityFlagsEXT::ERROR
          )
      }
      .message_type(
          vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION
          | vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE
          // | vk::DebugUtilsMessageTypeFlagsEXT::GENERAL
      )
      .pfn_user_callback(Some(vulkan_debug_utils_callback));

      let layer_names = if cfg!(debug_assertions) {
        vec![
          CString::new("VK_LAYER_KHRONOS_validation")
            .map_err(|err| HalaGfxError::new("Failed to create CString VK_LAYER_KHRONOS_validation.", Some(Box::new(err))))?,
        ]
      } else {
        vec![]
      };
      let layer_name_ptrs = layer_names.iter().map(|layer_name| layer_name.as_ptr()).collect::<Vec<_>>();
      let extension_name_ptrs = vec![
        ash::ext::debug_utils::NAME.as_ptr(),
        ash::khr::surface::NAME.as_ptr(),
        // If this windows.
        #[cfg(target_os = "windows")]
        ash::khr::win32_surface::NAME.as_ptr(),
        // If this linux.
        #[cfg(target_os = "linux")]
        ash::khr::wayland_surface::NAME.as_ptr(),
        #[cfg(target_os = "linux")]
        ash::khr::xlib_surface::NAME.as_ptr(),
        #[cfg(target_os = "linux")]
        ash::khr::xcb_surface::NAME.as_ptr(),
        // If this macos.
        #[cfg(target_os = "macos")]
        ash::mvk::macos_surface::NAME.as_ptr(),
      ];

      let validation_feature_enables = vec![vk::ValidationFeatureEnableEXT::DEBUG_PRINTF];
      let mut validation_features = vk::ValidationFeaturesEXT::default()
        .enabled_validation_features(&validation_feature_enables);

      let instance_create_info = if cfg!(debug_assertions) && gpu_req.require_printf_in_shader {
        vk::InstanceCreateInfo::default()
          .push_next(&mut validation_features)
          .application_info(&app_info)
          .enabled_layer_names(layer_name_ptrs.as_slice())
          .enabled_extension_names(extension_name_ptrs.as_slice())
      } else {
        vk::InstanceCreateInfo::default()
          .application_info(&app_info)
          .enabled_layer_names(layer_name_ptrs.as_slice())
          .enabled_extension_names(extension_name_ptrs.as_slice())
      };
      let instance = entry.create_instance(&instance_create_info, None)
        .map_err(|err| HalaGfxError::new("Failed to create Vulkan instance.", Some(Box::new(err))))?;

      let debug_utils_loader = ash::ext::debug_utils::Instance::new(&entry, &instance);
      let debug_call_back = debug_utils_loader
        .create_debug_utils_messenger(&debug_create_info, None)
        .map_err(|err| HalaGfxError::new("Failed to create Vulkan debug utils messenger.", Some(Box::new(err))))?;

      (debug_utils_loader, debug_call_back, instance)
    };
    Ok((debug_utils_loader, debug_call_back, instance))
  }
}