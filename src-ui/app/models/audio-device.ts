export type AudioDeviceType = 'Render' | 'Capture';

export interface AudioDevice {
  id: string;
  name: string;
  deviceType: AudioDeviceType;
  volume: number;
  mute: boolean;
  default: boolean;
  parsedName?: AudioDeviceParsedName;
  persistentId?: string;
}

export interface AudioDeviceParsedName {
  display: string;
  driver: string;
}
