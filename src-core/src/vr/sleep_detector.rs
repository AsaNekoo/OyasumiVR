use std::{
    collections::VecDeque,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use glam::Vec3;

use crate::{
    utils::{get_time, send_event},
    vr::model::SleepDetectorStateReport,
};

pub const SLEEP_DETECTOR_PERIOD: Duration = Duration::from_millis(300);
const MAX_EVENT_AGE_MS: Duration = Duration::from_mins(15); // 15 minutes

pub struct SleepDetector {
    disatnces: VecDeque<f32>,
    last_pos: Vec3,
    distance_in_last_15_minutes: f32,
    distance_in_last_10_seconds: f32,
    start_time: u64,
    last_log: u64,
    next_state_report: u64,
}
const EVENT_COUNT: usize =
    (MAX_EVENT_AGE_MS.as_secs_f32() / SLEEP_DETECTOR_PERIOD.as_secs_f32()) as usize;
impl SleepDetector {
    pub fn new() -> Self {
        Self {
            disatnces: VecDeque::with_capacity(EVENT_COUNT + 1),
            distance_in_last_10_seconds: 0.0,
            distance_in_last_15_minutes: 0.0,
            start_time: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64,
            last_log: 0,
            next_state_report: 0,
            last_pos: Vec3::default(),
        }
    }

    pub async fn log_pose(&mut self, position: Vec3) {
        let now = get_time();
        // Add the event
        // if let Some(position) = position {
        self.disatnces.push_back(position.distance(self.last_pos));
        self.last_pos = position;

        if self.disatnces.len() > EVENT_COUNT {
            self.disatnces.pop_front();
        }

        // Send a state report if it's been over a second since the last one
        if now > self.next_state_report {
            const LAST_10_SECS_EVENT_COUNT: usize = (Duration::from_secs(10).as_secs_f32()
                / SLEEP_DETECTOR_PERIOD.as_secs_f32())
                as usize;
            // const LAST_15_MINS_EVENT_COUNT:usize=(Duration::from_mins(15).as_secs_f32()/SLEEP_DETECTOR_PERIOD.as_secs_f32()) as usize;
            self.distance_in_last_10_seconds = self
                .disatnces
                .iter()
                .rev()
                .take(LAST_10_SECS_EVENT_COUNT)
                .sum();

            // self.distance_in_last_15_minutes =self.disatnces.iter().rev().take(LAST_15_MINS_EVENT_COUNT).sum();
            self.distance_in_last_15_minutes = self.disatnces.iter().sum();

            self.last_log = now;
            self.next_state_report = now + Duration::from_secs(10).as_millis() as u64;
            self.send_state_report().await;
        }
    }
    //called when enabling sleep detection after it was disabled
    pub async fn reset_start_time(&mut self) {
        self.start_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;
    }

    async fn send_state_report(&self) {
        send_event(
            "SLEEP_DETECTOR_STATE_REPORT",
            SleepDetectorStateReport {
                distance_in_last_15_minutes: self.distance_in_last_15_minutes,
                distance_in_last_10_seconds: self.distance_in_last_10_seconds,
                start_time: self.start_time,
                last_log: self.last_log,
            },
        )
        .await;
        // self.send_influxdb_report();
    }
}
