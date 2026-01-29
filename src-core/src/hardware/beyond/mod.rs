#![cfg_attr(unix, allow(unused_imports))]
#![cfg(windows)]
use std::sync::{atomic::{AtomicBool, Ordering}, LazyLock};

use hidapi::{HidApi, HidDevice};
use log::{error, info, warn};
use tokio::sync::Mutex;

use crate::utils::send_event;

pub mod commands;


static BSB_CONNECTED: LazyLock<AtomicBool> = LazyLock::new(|| AtomicBool::new(false));
static BSB_DEVICE: LazyLock<Mutex<Option<HidDevice>>> = LazyLock::new(|| Mutex::new(None));

pub fn set_led_color(device: &HidDevice, r: u8, g: u8, b: u8) -> Result<(), String> {
    match device.send_feature_report(&[0, 0x4c, r, g, b]) {
        Ok(_) => Ok(()),
        Err(e) => {
            error!(
                "[Core][Beyond] Could not send data to Bigscreen Beyond: {}",
                e
            );
            Ok(())
            //             return Err("DEVICE_WRITE_ERROR".to_string());
        }
    }
}

pub fn set_fan_speed(device: &HidDevice, speed: u8) -> Result<(), String> {
    if speed > 100 {
        error!(
            "[Core][Beyond] Attempted to set fan speed of Bigscreen Beyond to an out of bounds value: {}",
            speed
        );
        return Err("OUT_OF_BOUNDS".to_string());
    }
    match device.send_feature_report(&[0, 0x46, speed]) {
        Ok(_) => Ok(()),
        Err(e) => {
            error!(
                "[Core][Beyond] Could not send data to Bigscreen Beyond: {}",
                e
            );
            Ok(())
            //             return Err("DEVICE_WRITE_ERROR".to_string());
        }
    }
}

pub fn set_brightness(device: &HidDevice, brightness: u16) -> Result<(), String> {
    if brightness >= 0x0400 {
        error!(
            "[Core][Beyond] Attempted to set brightness of Bigscreen Beyond to an out of bounds value: {}",
            brightness
        );
        return Err("OUT_OF_BOUNDS".to_string());
    }
    match device.send_feature_report(&[
        0,
        0x49,
        ((brightness >> 8) & 0xff) as u8,
        (brightness & 0xff) as u8,
    ]) {
        Ok(_) => Ok(()),
        Err(e) => {
            error!(
                "[Core][Beyond] Could not send data to Bigscreen Beyond: {}",
                e
            );
            Ok(())
            //             return Err("DEVICE_WRITE_ERROR".to_string());
        }
    }
}
