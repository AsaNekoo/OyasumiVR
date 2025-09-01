#[cfg(target_os="windows")]
#[allow(dead_code, unused_variables, non_upper_case_globals)]
pub mod device;
#[allow(dead_code, unused_variables, non_upper_case_globals)]
#[cfg(target_os="windows")]
pub mod manager;
#[allow(dead_code, unused_variables, non_upper_case_globals)]
#[cfg(target_os="windows")]
mod wrappers;
#[cfg(target_os="windows")]
pub mod linux_hack;
