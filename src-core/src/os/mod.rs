pub mod commands;
pub mod elevation;
pub mod linux;
mod models;
mod notifications;
mod sounds_gen;
use dbus::blocking::Connection;
use log::{debug, error};
use oyasumi_shared::RESOURCES_PATH;
use rodio::DeviceSinkBuilder;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Cursor, Read};
use std::sync::LazyLock;
use tokio::sync::mpsc::Sender;
use tokio::sync::Mutex;
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

pub async fn init_sound_playback() {
    // Create channels
    let (tokio_tx, mut tokio_rx) = tokio::sync::mpsc::channel::<(String, f32)>(32);
    // let (std_tx, std_rx) = std::sync::mpsc::channel::<(String, f32)>();

    // Store the tokio sender
    *PLAY_SOUND_TX.lock().await = Some(tokio_tx);

    // Spawn standard thread to play sounds
    tokio::task::spawn(async move  {
        // Load sound files
        let mut sounds = HashMap::new();
        let stream_handle = match DeviceSinkBuilder::open_default_sink() {
            Ok(sink) => sink,
            Err(e) => {
                error!("[Core] Failed to initialize audio output stream: {}", e);
                return;
            }
        };
        sounds_gen::SOUND_FILES.iter().for_each(|sound| {
            let path = format!(
                "{}/sounds/{}.ogg",
                RESOURCES_PATH.to_string_lossy().to_string(),
                sound
            );
            let mut file = match File::open(path.clone()) {
                Ok(f) => f,
                Err(e) => {
                    error!("[Core] Failed to open sound file: {}", e);
                    return;
                }
            };
            // let reader = BufReader::new(file);
            let mut buff=Vec::new();
            if let Err(err)=file.read_to_end(&mut buff){
                error!("failed to read audio file {:#?} {:?}",path,err);
            }
            let buff=buff.into_boxed_slice();
            sounds.insert(String::from(*sound), buff);
        });

        // Initialize output stream
        let player = rodio::Player::connect_new(stream_handle.mixer());
        // Play sounds when requested
        while let Some((sound, volume)) = tokio_rx.recv().await {
            stream_handle.play();
            if let Some(source) = sounds.get(&sound) {
                // Play sound
                let decoder=rodio::Decoder::new(Cursor::new(source.clone())).unwrap();
                player.append(decoder);

                player.set_volume(volume);
                player.sleep_until_end();
                stream_handle.pause();
            } else {
                error!("[Core] Sound not found: {}", sound);
            }
        }
    });
}
