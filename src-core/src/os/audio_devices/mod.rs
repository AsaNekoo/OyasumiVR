#[cfg(disabled)]
#[allow(dead_code, unused_variables, non_upper_case_globals)]
pub mod device;
#[allow(dead_code, unused_variables, non_upper_case_globals)]
#[cfg(disabled)]
pub mod manager;
#[allow(dead_code, unused_variables, non_upper_case_globals)]
#[cfg(disabled)]
mod wrappers;
#[cfg(any(windows,linux))]
pub mod linux_hack;
