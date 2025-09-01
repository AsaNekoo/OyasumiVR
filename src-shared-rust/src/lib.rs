#[cfg(disabled)]
mod windows;
#[cfg(any(windows,target_os = "linux"))]
mod linux;
#[cfg(disabled)]
pub use windows::*; 
#[cfg(any(windows,target_os = "linux"))]
pub use linux::*;