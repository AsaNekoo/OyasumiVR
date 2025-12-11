use std::time::Duration;

use glam::{Quat, Vec3, Vec3A};

use crate::{
    utils::{get_time, get_time_u64, send_event},
    vr::model::SleepDetectorStateReport,
};

// use super::models::SleepDetectorStateReport;

const MAX_EVENT_AGE_MS: u128 = 900000; // 15 minutes

#[derive(Clone, Copy)]
struct PoseEvent {
    value: Vec3,
    // quaternion:Quat,
    timestamp: u64, // in milliseconds
}

impl PoseEvent {
    fn distance_to(&self, other: &PoseEvent) -> f32 {
        self.value.distance(other.value)
    }
    // fn angular_distance_degrees(&self, other: &PoseEvent) -> f32 {
    //     let dot_product = self.quaternion.dot(other.quaternion);
    //     let angle = 2.0 * dot_product.abs().clamp(-1.0, 1.0).acos();
    //     angle.to_degrees()
    // }
}

pub struct SleepDetector {
    events: Vec<PoseEvent>,
    distance_in_last_15_minutes: f32,
    distance_in_last_10_minutes: f32,
    distance_in_last_5_minutes: f32,
    distance_in_last_1_minute: f32,
    distance_in_last_10_seconds: f32,
    // rotation_in_last_15_minutes: f32,
    // rotation_in_last_10_minutes: f32,
    // rotation_in_last_5_minutes: f32,
    // rotation_in_last_1_minute: f32,
    // rotation_in_last_10_seconds: f32,
    // reqwest_client: reqwest::Client,
    start_time: u64,
    last_log: u64,
    next_state_report: u64,
}

impl SleepDetector {
    pub fn new() -> Self {
        Self {
            events: Vec::with_capacity(
                (Duration::from_millis(MAX_EVENT_AGE_MS as u64).as_secs_f32() / Duration::from_millis(250).as_secs_f32()) as usize+100,
            ),
            distance_in_last_10_seconds: 0.0,
            distance_in_last_1_minute: 0.0,
            distance_in_last_5_minutes: 0.0,
            distance_in_last_10_minutes: 0.0,
            distance_in_last_15_minutes: 0.0,
            // rotation_in_last_10_seconds: 0.0,
            // rotation_in_last_1_minute: 0.0,
            // rotation_in_last_5_minutes: 0.0,
            // rotation_in_last_10_minutes: 0.0,
            // rotation_in_last_15_minutes: 0.0,
            // reqwest_client: reqwest::Client::new(),
            start_time: 0,
            last_log: 0,
            next_state_report: 0,
        }
    }

    pub async fn log_pose(&mut self, position: Vec3) {
        let now = get_time_u64();
        // Add the event
        let event = PoseEvent {
            value: position,
            timestamp: now,
        };
        self.events.push(event);
        // Remove old events

        // Calculate new distances

        // self.rotation_in_last_15_minutes = self.rotation_in_window(900000);
        // self.rotation_in_last_10_minutes = self.rotation_in_window(600000);
        // self.rotation_in_last_5_minutes = self.rotation_in_window(300000);
        // self.rotation_in_last_1_minute = self.rotation_in_window(60000);
        // self.rotation_in_last_10_seconds = self.rotation_in_window(10000);
        // Set new start time if there hasn't been any data in over a minute

        // Update the last log time

        // Send a state report if it's been over a second since the last one
        if now > self.next_state_report {
            let oldest_time = event.timestamp - MAX_EVENT_AGE_MS as u64;
            let old_event_count = self
                .events
                .iter()
                .take_while(|e| e.timestamp < oldest_time)
                .count();
            self.events.drain(..old_event_count);
            if now.saturating_sub(self.last_log) > 60000 {
                self.start_time = now;
            }
            self.distance_in_last_10_seconds = self.distance_in_window(10000, 0, 0.);
            self.distance_in_last_1_minute =
                self.distance_in_window(60000, 10000, self.distance_in_last_10_seconds);
            self.distance_in_last_5_minutes =
                self.distance_in_window(300000, 60000, self.distance_in_last_1_minute);
            self.distance_in_last_10_minutes =
                self.distance_in_window(600000, 300000, self.distance_in_last_5_minutes);
            self.distance_in_last_15_minutes =
                self.distance_in_window(900000, 600000, self.distance_in_last_10_minutes);

            self.last_log = event.timestamp;
            self.next_state_report = now + 1000;
            self.send_state_report().await;
        }
    }

    fn distance_in_window(&mut self, window_ms: u64, prev_ms: u64, mut prev_v: f32) -> f32 {
        let start_time = get_time_u64() - window_ms;
        let start_index = match self
            .events
            .iter()
            .enumerate()
            .skip_while(|(_, e)| e.timestamp < start_time)
            .map(|e| e.0)
            .next()
        {
            Some(v) => v,
            None => return prev_v,
        };

        let previous_count = match prev_ms == 0 {
            true => 0,
            false => {
                let start_time_previous = get_time_u64() - prev_ms;
                self.events
                    .iter()
                    .skip_while(|e| e.timestamp < start_time_previous)
                    .count()
            }
        };
        let events = &self.events[start_index..];
        let events = &events[..(events.len() - previous_count)];
        for events in events.windows(2) {
            prev_v += events[0].distance_to(&events[1]);
        }
        prev_v
    }

    // fn rotation_in_window(&mut self, window_ms: u128) -> f32 {
    //     let start_time = get_time() - window_ms;
    //     let start_index = self
    //         .events
    //         .iter()
    //         .position(|e| e.timestamp >= start_time)
    //         .unwrap_or(0);
    //     let events = &self.events[start_index..];
    //     let mut total_rotation = 0.0;
    //     let mut i = 0;
    //     while i < events.len() - 1 {
    //         let event_a = &events[i];
    //         let event_b = &events[i + 1];
    //         let rotation = event_a.angular_distance_degrees(event_b);
    //         total_rotation += rotation;
    //         i += 1;
    //     }
    //     total_rotation
    // }

    async fn send_state_report(&self) {
        send_event(
            "SLEEP_DETECTOR_STATE_REPORT",
            SleepDetectorStateReport {
                distance_in_last_15_minutes: self.distance_in_last_15_minutes,
                distance_in_last_10_minutes: self.distance_in_last_10_minutes,
                distance_in_last_5_minutes: self.distance_in_last_5_minutes,
                distance_in_last_1_minute: self.distance_in_last_1_minute,
                distance_in_last_10_seconds: self.distance_in_last_10_seconds,
                // rotation_in_last_15_minutes: self.rotation_in_last_15_minutes,
                // rotation_in_last_10_minutes: self.rotation_in_last_10_minutes,
                // rotation_in_last_5_minutes: self.rotation_in_last_5_minutes,
                // rotation_in_last_1_minute: self.rotation_in_last_1_minute,
                // rotation_in_last_10_seconds: self.rotation_in_last_10_seconds,
                start_time: self.start_time,
                last_log: self.last_log,
            },
        )
        .await;
        // self.send_influxdb_report();
    }

    // #[tokio::main]
    // async fn send_influxdb_report(&self) {
    //     let f = self.reqwest_client
    //     .post("http://localhost:8086/api/v2/write?org=org&bucket=bucket&precision=ms")
    //     .header("Authorization", "Token yXuwflYgacQn8GQp7VmXV23jdC5mG3k5XVBHiA7_Ojv7xCLZyB-FttolJcCRop4knUvN-vi_uMxbZjaBa5SfbQ==") // Yes this is a token I checked in. It's for a local test database, for debugging. Don't worry about it.
    //     .header("Content-Type", "text/plain; charset=utf-8")
    //     .header("Accept", "application/json")
    //     .body(format!(
    //         "sleep_detector distance_in_last_15_minutes={},distance_in_last_10_minutes={},distance_in_last_5_minutes={},distance_in_last_1_minute={},distance_in_last_10_seconds={},rotation_in_last_15_minutes={},rotation_in_last_10_minutes={},rotation_in_last_5_minutes={},rotation_in_last_1_minute={},rotation_in_last_10_seconds={} {}",
    //         self.distance_in_last_15_minutes,
    //         self.distance_in_last_10_minutes,
    //         self.distance_in_last_5_minutes,
    //         self.distance_in_last_1_minute,
    //         self.distance_in_last_10_seconds,
    //         self.rotation_in_last_15_minutes,
    //         self.rotation_in_last_10_minutes,
    //         self.rotation_in_last_5_minutes,
    //         self.rotation_in_last_1_minute,
    //         self.rotation_in_last_10_seconds,
    //         self.last_log
    //     ))
    //     .send();
    //     // Block until the request is sent
    //     let _ = futures::executor::block_on(f);
    // }
}
