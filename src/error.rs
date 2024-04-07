use thiserror::Error;

/// The error type of the hala-gfx crate.
#[derive(Error, Debug)]
pub struct HalaGfxError {
  msg: String,
  #[source]
  source: Option<Box<dyn std::error::Error + Send + Sync>>,
}

/// The implementation of the error type of the hala-gfx crate.
impl HalaGfxError {
  pub fn new(msg: &str, source: Option<Box<dyn std::error::Error + Send + Sync>>) -> Self {
    Self {
      msg: msg.to_string(),
      source,
    }
  }
  pub fn message(&self) -> &str {
    &self.msg
  }
  pub fn is_device_lost(&self) -> bool {
    if let Some(ref source) = self.source {
      if let Some(err) = source.downcast_ref::<ash::vk::Result>() {
        return matches!(err, &ash::vk::Result::ERROR_DEVICE_LOST) || matches!(err, &ash::vk::Result::ERROR_OUT_OF_DATE_KHR);
      }
    }
    false
  }
}

/// The implementation Display trait for the error type of the hala-gfx crate.
impl std::fmt::Display for HalaGfxError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.msg)
  }
}