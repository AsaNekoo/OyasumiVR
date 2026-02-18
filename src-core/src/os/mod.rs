pub mod commands;
pub mod elevation;
pub mod linux;
mod models;
mod notifications;
mod sounds_gen;
use dbus::blocking::Connection;
use log::{debug, error};
use oyasumi_shared::RESOURCES_PATH;
use rodio::{Decoder, source::Source};
use rodio::{OutputStream, Sink};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::sync::LazyLock;
use tokio::sync::Mutex;
use std::sync::mpsc::Sender;
type PlaySoundSender = LazyLock<Mutex<Option<Sender<(String, f32)>>>>;
pub static DBUS_CONNECTION: Mutex<Option<Connection>> = Mutex::const_new(None);
pub async fn connect_dbus() -> bool {
    let mut lock = DBUS_CONNECTION.lock().await;
    if lock.is_some() {
        return true;
    }
    match Connection::new_session() {
        Ok(v) => {
            lock.replace(v);
            debug!("[core] connected to dbus");
            true
        }
        Err(err) => {
            error!("[core] failed to coonect to dbus: {:?}", err);
            false
        }
    }
}
static PLAY_SOUND_TX: PlaySoundSender = LazyLock::new(Mutex::default);

pub async fn init_audio_device_manager() {}

pub async fn init_sound_playback() {
    // Create channels
    // let (tokio_tx, mut tokio_rx) = tokio::sync::mpsc::channel::<(String, f32)>(32);
    let (std_tx, std_rx) = std::sync::mpsc::channel::<(String, f32)>();

    // Store the tokio sender
    *PLAY_SOUND_TX.lock().await = Some(std_tx);


    // Spawn standard thread to play sounds
    std::thread::spawn(move || {
        // Load sound files
        let mut sounds = HashMap::new();
        sounds_gen::SOUND_FILES.iter().for_each(|sound| {
            let path = format!(
                "{}/sounds/{}.ogg",
                RESOURCES_PATH.to_string_lossy().to_string(),
                sound
            );
            let file = match File::open(path.clone()) {
                Ok(f) => f,
                Err(e) => {
                    error!("[Core] Failed to open sound file: {}", e);
                    return;
                }
            };
            let reader = BufReader::new(file);
            let source = match Decoder::new(reader) {
                Ok(s) => s.buffered(),
                Err(e) => {
                    error!(
                        "[Core] Failed to decode sound file at path ({}): {}",
                        path.clone(),
                        e
                    );
                    return;
                }
            };
            sounds.insert(String::from(*sound), source);
        });

        // Initialize output stream
        let (_stream, stream_handle) = match OutputStream::try_default() {
            Ok((stream, handle)) => (stream, handle),
            Err(e) => {
                error!("[Core] Failed to initialize audio output stream: {}", e);
                return;
            }
        };

        // Play sounds when requested
        while let Ok((sound, volume)) = std_rx.recv() {
            if let Some(source) = sounds.get(&sound) {
                // Play sound
                let source = source.clone();
                let sink = match Sink::try_new(&stream_handle) {
                    Ok(s) => s,
                    Err(e) => {
                        error!("[Core] Failed to create audio sink: {}", e);
                        continue;
                    }
                };
                sink.set_volume(volume);
                sink.append(source.clone());
                sink.detach();
            } else {
                error!("[Core] Sound not found: {}", sound);
            }
        }
    });
}
