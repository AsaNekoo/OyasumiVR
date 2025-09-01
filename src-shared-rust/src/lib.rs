#[cfg(disabled)]
mod windows;
#[cfg(any(windows,linux))]
mod linux;
#[cfg(disabled)]
pub use windows::*; 
#[cfg(any(windows,linux))]
pub use linux::*;