use std::{ffi::CString, io::BufReader, num::Wrapping, os::unix::net::UnixStream};

use pulseaudio::protocol::{
    self, ChannelVolume, ProtocolError, SetDeviceMuteParams, SetDeviceVolumeParams, Volume,
};
use tokio::sync::Mutex;

use crate::os::models::{AudioDeviceDto, AudioDeviceType};

#[allow(dead_code)]
pub static LINUX_AUDIO_DEVICE_MANAGER: Mutex<Option<LinuxAudioDeviceManager>> =
    Mutex::const_new(None);
pub async fn get_linux_audio_manager<'a>(
) -> tokio::sync::MutexGuard<'a, Option<LinuxAudioDeviceManager>> {
    let mut audio: tokio::sync::MutexGuard<'a, Option<LinuxAudioDeviceManager>> =
        LINUX_AUDIO_DEVICE_MANAGER.lock().await;
    match *audio {
        Some(_) => audio,
        None => {
            let au = {
                let mut manager = LinuxAudioDeviceManager::default();
                let mut error = false;
                if let Err(err) = manager.connect() {
                    error = true;
                    log::error!("[core] failed to connect to pulseaudio server: {:?}", err);
                }
                if let Err(err) = manager.refresh_devices() {
                    error = true;
                    log::error!("[core] failed to refresh audio devices: {:?}", err);
                }
                if error {
                    None
                } else {
                    Some(manager)
                }
            };
            if let Some(au) = au {
                audio.replace(au);
            };
            audio
        }
    }
}
#[allow(dead_code)]
#[derive(Debug)]
pub enum LinuxAudioError {
    PulseAudioUnavalible,
    IoError(std::io::Error),
    PulseProtocolError(ProtocolError),
    NotConnected,
    AudioDeviceNotFound,
    OutOfOrder,
}
impl From<ProtocolError> for LinuxAudioError {
    fn from(value: ProtocolError) -> Self {
        Self::PulseProtocolError(value)
    }
}
impl From<std::io::Error> for LinuxAudioError {
    fn from(value: std::io::Error) -> Self {
        Self::IoError(value)
    }
}
#[allow(dead_code)]
#[derive(Clone)]
pub struct LinuxAudioDevice {
    pub id: String,
    pub name: String,
    pub device_type: AudioDeviceType,
    pub volume: f32,
    pub mute: bool,
    pub default: bool,
    pub index: u32,
}
impl From<LinuxAudioDevice> for AudioDeviceDto {
    fn from(value: LinuxAudioDevice) -> Self {
        Self {
            id: value.id,
            name: value.name,
            device_type: value.device_type,
            volume: value.volume,
            mute: value.mute,
            default: value.default,
            default_communications: true,
        }
    }
}
///could probaly be always zero
#[derive(Default, Clone, Copy)]
struct SeqCounter {
    seq: Wrapping<u32>,
}
impl SeqCounter {
    fn next(&mut self) -> u32 {
        self.seq += 1;
        self.seq.0
    }
}
pub struct AudioManagerConnectionInfo {
    protocol_version: u16,
    sock: BufReader<UnixStream>,
    seq: SeqCounter,
}
#[derive(Default)]
pub struct LinuxAudioDeviceManager {
    devices: Vec<LinuxAudioDevice>,
    connection: Option<AudioManagerConnectionInfo>,
}
impl LinuxAudioDeviceManager {
    ///connects to pulseaudio
    pub fn connect(&mut self) -> Result<(), LinuxAudioError> {
        let mut seq = SeqCounter::default();
        // Find and connect to PulseAudio. The socket is usually in a well-known
        // location under XDG_RUNTIME_DIR.
        let socket_path =
            pulseaudio::socket_path_from_env().ok_or(LinuxAudioError::PulseAudioUnavalible)?;
        let mut sock = std::io::BufReader::new(UnixStream::connect(socket_path)?);

        // PulseAudio usually puts an authentication "cookie" in ~/.config/pulse/cookie.
        let cookie = pulseaudio::cookie_path_from_env()
            .and_then(|path| std::fs::read(path).ok())
            .unwrap_or_default();
        let auth = protocol::AuthParams {
            version: protocol::MAX_VERSION,
            supports_shm: false,
            supports_memfd: false,
            cookie,
        };

        // Write the auth "command" to the socket, and read the reply. The reply
        // contains the negotiated protocol version.
        let seq_c = seq.next();
        protocol::write_command_message(
            sock.get_mut(),
            seq_c,
            &protocol::Command::Auth(auth),
            protocol::MAX_VERSION,
        )?;
        let (seq_r, auth_info) =
            protocol::read_reply_message::<protocol::AuthReply>(&mut sock, protocol::MAX_VERSION)?;
        self.connection.replace(AudioManagerConnectionInfo {
            protocol_version: auth_info.version,
            sock,
            seq,
        });
        if seq_c != seq_r {
            return Err(LinuxAudioError::OutOfOrder);
        }
        Ok(())
    }
    pub fn refresh_devices(&mut self) -> Result<(), LinuxAudioError> {
        let connection = self
            .connection
            .as_mut()
            .ok_or(LinuxAudioError::NotConnected)?;
        //no idea what it does but it's needed
        let mut props = protocol::Props::new();
        props.set(
            protocol::Prop::ApplicationName,
            CString::new("list-sinks").unwrap(),
        );
        let seq_c = connection.seq.next();
        protocol::write_command_message(
            connection.sock.get_mut(),
            seq_c,
            &protocol::Command::SetClientName(props),
            connection.protocol_version,
        )?;

        let (seq_r, _) = protocol::read_reply_message::<protocol::SetClientNameReply>(
            &mut connection.sock,
            connection.protocol_version,
        )?;
        if seq_c != seq_r {
            return Err(LinuxAudioError::OutOfOrder);
        }
        let seq_c = connection.seq.next();
        //get microphones
        protocol::write_command_message(
            connection.sock.get_mut(),
            seq_c,
            &protocol::Command::GetSourceInfoList,
            connection.protocol_version,
        )?;

        let (seq_r, source_list) = protocol::read_reply_message::<protocol::SourceInfoList>(
            &mut connection.sock,
            connection.protocol_version,
        )?;
        if seq_c != seq_r {
            return Err(LinuxAudioError::OutOfOrder);
        }

        // get speakers
        let seq_c = connection.seq.next();
        protocol::write_command_message(
            connection.sock.get_mut(),
            seq_c,
            &protocol::Command::GetSinkInfoList,
            connection.protocol_version,
        )?;
        let (seq_r, sink_list) = protocol::read_reply_message::<protocol::SinkInfoList>(
            &mut connection.sock,
            connection.protocol_version,
        )?;
        if seq_c != seq_r {
            return Err(LinuxAudioError::OutOfOrder);
        }
        let seq_c = connection.seq.next();
        //get server info to check default devices
        protocol::write_command_message(
            connection.sock.get_mut(),
            seq_c,
            &protocol::Command::GetServerInfo,
            connection.protocol_version,
        )?;
        let (seq_r, server_info) = protocol::read_reply_message::<protocol::ServerInfo>(
            &mut connection.sock,
            connection.protocol_version,
        )?;
        if seq_c != seq_r {
            return Err(LinuxAudioError::OutOfOrder);
        }

        self.devices.clear();
        self.devices.extend(
            source_list
                .into_iter()
                .filter(|microphone| !microphone.name.to_string_lossy().ends_with(".monitor"))
                .map(|microphone| LinuxAudioDevice {
                    id: microphone.name.clone().to_string_lossy().to_string(),
                    name: microphone
                        .description
                        .unwrap_or(microphone.name.clone())
                        .to_string_lossy()
                        .to_string(),
                    device_type: AudioDeviceType::Capture,
                    volume: microphone.cvolume.channels()[0].to_linear(),
                    mute: microphone.muted,
                    default: Some(microphone.name) == server_info.default_source_name,
                    index: microphone.index,
                }),
        );
        self.devices.extend(sink_list.into_iter().map(|speakear| {
            LinuxAudioDevice {
                id: speakear.name.clone().to_string_lossy().to_string(),
                name: speakear
                    .description
                    .unwrap_or(speakear.name.clone())
                    .to_string_lossy()
                    .to_string(),
                device_type: AudioDeviceType::Render,
                volume: speakear.cvolume.channels()[0].to_linear(),
                mute: speakear.muted,
                default: Some(speakear.name) == server_info.default_sink_name,
                index: speakear.index,
            }
        }));

        Ok(())
    }
    pub fn devices(&self) -> &[LinuxAudioDevice] {
        self.devices.as_slice()
    }
    pub fn set_audio_device_volume(
        &mut self,
        device_id: String,
        volume: f32,
    ) -> Result<(), LinuxAudioError> {
        let connection = self
            .connection
            .as_mut()
            .ok_or(LinuxAudioError::NotConnected)?;
        let device = self
            .devices
            .iter_mut()
            .find(|device| device.id == device_id)
            .ok_or(LinuxAudioError::AudioDeviceNotFound)?;
        let mut channel_volume = ChannelVolume::empty();
        //idk but this formula works for me
        channel_volume.push(Volume::from_u32_clamped(
            (volume * (u16::MAX) as f32) as u32,
        ));
        let seq_c = connection.seq.next();
        match device.device_type {
            AudioDeviceType::Capture => protocol::write_command_message(
                connection.sock.get_mut(),
                seq_c,
                &protocol::Command::SetSourceVolume(SetDeviceVolumeParams {
                    device_index: Some(device.index),
                    device_name: None,
                    volume: channel_volume,
                }),
                connection.protocol_version,
            )?,
            AudioDeviceType::Render => protocol::write_command_message(
                connection.sock.get_mut(),
                seq_c,
                &protocol::Command::SetSinkVolume(SetDeviceVolumeParams {
                    device_index: Some(device.index),
                    device_name: None,
                    volume: channel_volume,
                }),
                connection.protocol_version,
            )?,
        }
        //wrong type but it seems to only reply with device not found error
        let (seq_r, _) = protocol::read_reply_message::<protocol::CardInfoList>(
            &mut connection.sock,
            connection.protocol_version,
        )?;
        if seq_c != seq_r {
            return Err(LinuxAudioError::OutOfOrder);
        }
        device.volume = volume;

        Ok(())
    }
    pub fn set_audio_device_mute(&mut self, id: String, mute: bool) -> Result<(), LinuxAudioError> {
        let connection = self
            .connection
            .as_mut()
            .ok_or(LinuxAudioError::NotConnected)?;
        let device = self
            .devices
            .iter_mut()
            .find(|device| device.id == id)
            .ok_or(LinuxAudioError::AudioDeviceNotFound)?;
        let seq_c = connection.seq.next();
        match device.device_type {
            AudioDeviceType::Capture => protocol::write_command_message(
                connection.sock.get_mut(),
                seq_c,
                &protocol::Command::SetSourceMute(SetDeviceMuteParams {
                    device_index: Some(device.index),
                    device_name: None,
                    mute,
                }),
                connection.protocol_version,
            )?,
            AudioDeviceType::Render => protocol::write_command_message(
                connection.sock.get_mut(),
                seq_c,
                &protocol::Command::SetSinkMute(SetDeviceMuteParams {
                    device_index: Some(device.index),
                    device_name: None,
                    mute,
                }),
                connection.protocol_version,
            )?,
        }
        device.mute = mute;
        //wrong type but it seems to only reply with device not found error
        let (seq_r, _) = protocol::read_reply_message::<protocol::CardInfoList>(
            &mut connection.sock,
            connection.protocol_version,
        )?;
        if seq_c != seq_r {
            return Err(LinuxAudioError::OutOfOrder);
        }
        Ok(())
    }
}
