use ash::vk;

#[cfg(target_os = "windows")]
use ash::vk::Win32SurfaceCreateInfoKHR;
#[cfg(target_os = "linux")]
use ash::vk::WaylandSurfaceCreateInfoKHR;
#[cfg(target_os = "linux")]
use ash::vk::XcbSurfaceCreateInfoKHR;
#[cfg(target_os = "linux")]
use ash::vk::XlibSurfaceCreateInfoKHR;
#[cfg(target_os = "macos")]
use ash::vk::MacOSSurfaceCreateInfoMVK;

use winit::raw_window_handle::{
  HasWindowHandle,
  RawWindowHandle,
};
#[cfg(target_os = "linux")]
use winit::raw_window_handle::RawDisplayHandle;


use crate::HalaGfxError;

/// The surface.
pub struct HalaSurface {
  pub raw: vk::SurfaceKHR,
  pub surface_loader: ash::khr::surface::Instance,
}

/// The Drop trait implementation of the surface.
impl Drop for HalaSurface {
  fn drop(&mut self) {
    unsafe {
      self.surface_loader.destroy_surface(self.raw, None);
    }
    log::debug!("A HalaSurface is dropped.");
  }
}

/// The implementation of the surface.
impl HalaSurface {
  pub fn new(instance: &crate::HalaInstance, window: &winit::window::Window) -> Result<Self, HalaGfxError> {
    #[cfg(target_os = "windows")]
    let surface = {
      let platform_surface_loader = ash::khr::win32_surface::Instance::new(&instance.entry, &instance.raw);
      Self::create_win32_surface(window, &platform_surface_loader)
    };
    #[cfg(target_os = "linux")]
    let is_wayland = std::env::var("WAYLAND_DISPLAY")
      .map(|var| !var.is_empty())
      .unwrap_or(false);
    #[cfg(target_os = "linux")]
    let surface = if is_wayland {
      let platform_surface_loader = ash::khr::wayland_surface::Instance::new(&instance.entry, &instance.raw);
      Self::create_wayland_surface(window, &platform_surface_loader)
    // } else {
    //   let platform_surface_loader = ash::khr::xcb_surface::Instance::new(&instance.entry, &instance.raw);
    //   Self::create_xcb_surface(window, &platform_surface_loader)
    // };
    } else {
      let platform_surface_loader = ash::khr::xlib_surface::Instance::new(&instance.entry, &instance.raw);
      Self::create_xlib_surface(window, &platform_surface_loader)
    };
    #[cfg(target_os = "macos")]
    let surface = {
      let platform_surface_loader = ash::mvk::macos_surface::Instance::new(&instance.entry, &instance.raw);
      Self::create_macos_surface(window, &platform_surface_loader)
    };
    let surface_loader = ash::khr::surface::Instance::new(&instance.entry, &instance.raw);

    log::debug!("A HalaSurface is created.");
    Ok(
      Self {
        raw: surface,
        surface_loader,
      }
    )
  }

  /// Create a surface.
  /// param window: The window.
  /// param platform_surface_loader: The Vulkan platform surface loader.
  /// return: The Vulkan surface.
  #[cfg(target_os = "windows")]
  fn create_win32_surface(window: &winit::window::Window, platform_surface_loader: &ash::khr::win32_surface::Instance) -> vk::SurfaceKHR {
    let h = window.window_handle().unwrap();
    let rh = h.as_raw();
    match rh {
      RawWindowHandle::Win32(win32_rh) => {
        let create_info = Win32SurfaceCreateInfoKHR::default()
          .hinstance(win32_rh.hinstance.unwrap().get())
          .hwnd(win32_rh.hwnd.get());
        unsafe {
          platform_surface_loader.create_win32_surface(&create_info, None)
            .map_err(|err| HalaGfxError::new("Failed to create Win32 surface.", Some(Box::new(err)))).unwrap()
        }
      }
      _ => {
        panic!("Unsupported window handle.");
      }
    }
  }
  #[cfg(target_os = "linux")]
  fn create_wayland_surface(window: &winit::window::Window, platform_surface_loader: &ash::khr::wayland_surface::Instance) -> vk::SurfaceKHR {
    use winit::raw_window_handle::HasDisplayHandle;

    let wh = window.window_handle().unwrap();
    let dh = window.display_handle().unwrap();
    let rwh = wh.as_raw();
    let rdh = dh.as_raw();
    match (rwh, rdh) {
      (
        RawWindowHandle::Wayland(wayland_rwh),
        RawDisplayHandle::Wayland(wayland_rdh)
       ) => {
        let create_info = WaylandSurfaceCreateInfoKHR::default()
          .display(wayland_rdh.display.as_ptr())
          .surface(wayland_rwh.surface.as_ptr());
        unsafe {
          platform_surface_loader.create_wayland_surface(&create_info, None)
            .map_err(|err| HalaGfxError::new("Failed to create Xlib surface.", Some(Box::new(err)))).unwrap()
        }
      }
      _ => {
        panic!("Unsupported window handle.");
      }
    }
  }
  #[cfg(target_os = "linux")]
  #[allow(dead_code)]
  fn create_xcb_surface(window: &winit::window::Window, platform_surface_loader: &ash::khr::xcb_surface::Instance) -> vk::SurfaceKHR {
    use winit::raw_window_handle::HasDisplayHandle;

    let wh = window.window_handle().unwrap();
    let dh = window.display_handle().unwrap();
    let rwh = wh.as_raw();
    let rdh = dh.as_raw();
    match (rwh, rdh) {
      (
        RawWindowHandle::Xcb(xcb_rwh),
        RawDisplayHandle::Xcb(xcb_rdh)
      ) => {
        let create_info = XcbSurfaceCreateInfoKHR::default()
          .window(xcb_rwh.window.get())
          .connection(xcb_rdh.connection.unwrap().as_ptr());
        unsafe {
          platform_surface_loader.create_xcb_surface(&create_info, None)
            .map_err(|err| HalaGfxError::new("Failed to create Xlib surface.", Some(Box::new(err)))).unwrap()
        }
      }
      _ => {
        panic!("Unsupported window handle.");
      }
    }
  }
  #[cfg(target_os = "linux")]
  #[allow(dead_code)]
  fn create_xlib_surface(window: &winit::window::Window, platform_surface_loader: &ash::khr::xlib_surface::Instance) -> vk::SurfaceKHR {
    use winit::raw_window_handle::HasDisplayHandle;

    let wh = window.window_handle().unwrap();
    let dh = window.display_handle().unwrap();
    let rwh = wh.as_raw();
    let rdh = dh.as_raw();
    match (rwh, rdh) {
      (
        RawWindowHandle::Xlib(xlib_rwh),
        RawDisplayHandle::Xlib(xlib_rdh)
      ) => {
        let create_info = XlibSurfaceCreateInfoKHR::default()
          .dpy(xlib_rdh.display.unwrap().as_ptr() as *mut *const std::ffi::c_void)
          .window(xlib_rwh.window);
        unsafe {
          platform_surface_loader.create_xlib_surface(&create_info, None)
            .map_err(|err| HalaGfxError::new("Failed to create Xlib surface.", Some(Box::new(err)))).unwrap()
        }
      }
      _ => {
        panic!("Unsupported window handle.");
      }
    }
  }
  #[cfg(target_os = "macos")]
  fn create_macos_surface(window: &winit::window::Window, platform_surface_loader: &ash::mvk::macos_surface::Instance) -> vk::SurfaceKHR {
    let h = window.window_handle().unwrap();
    let rh = h.as_raw();
    match rh {
      RawWindowHandle::AppKit(mac_rh) => {
        let create_info = MacOSSurfaceCreateInfoMVK::default()
          .view(mac_rh.ns_view.as_ptr());
        unsafe {
          platform_surface_loader.create_mac_os_surface(&create_info, None)
            .map_err(|err| HalaGfxError::new("Failed to create macOS surface.", Some(Box::new(err)))).unwrap()
        }
      }
      _ => {
        panic!("Unsupported window handle.");
      }
    }
  }
}