pub mod commands;
pub mod elevation;
mod models;
mod sounds_gen;
pub mod linux;
use log::error;
use rodio::{source::Source, Decoder};
use rodio::{OutputStream, Sink};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::sync::LazyLock;
use tokio::sync::mpsc::Sender;
use tokio::sync::Mutex;
type PlaySoundSender = LazyLock<Mutex<Option<Sender<(String, f32)>>>>;

static PLAY_SOUND_TX: PlaySoundSender = LazyLock::new(Mutex::default);

pub async fn init_audio_device_manager() {
  
}


pub async fn init_sound_playback() {
    // Create channels
    let (tokio_tx, mut tokio_rx) = tokio::sync::mpsc::channel::<(String, f32)>(32);
    let (std_tx, std_rx) = std::sync::mpsc::channel::<(String, f32)>();

    // Store the tokio sender
    *PLAY_SOUND_TX.lock().await = Some(tokio_tx);

    // Forward messages from tokio channel to std channel
    std::thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            while let Some(msg) = tokio_rx.recv().await {
                let _ = std_tx.send(msg);
            }
        });
    });

    // Spawn standard thread to play sounds
    std::thread::spawn(move || {
        // Load sound files
        let mut sounds = HashMap::new();
        sounds_gen::SOUND_FILES.iter().for_each(|sound| {
            let path = format!("resources/sounds/{}.ogg", sound);
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
