import { Injectable } from '@angular/core';
import { IPCService } from '../ipc.service';
import { map, pairwise, switchMap, take, tap } from 'rxjs';
import {
  Empty,
} from '../../../../src-grpc-web-client/overlay-sidecar_pb';
import { AppSettingsService } from '../app-settings.service';
import { APP_SETTINGS_DEFAULT, AppSettings } from '../../models/settings';

import { invoke } from '@tauri-apps/api/core';
import { VRChatService } from '../vrchat-api/vrchat.service';

@Injectable({
  providedIn: 'root',
})
export class OverlayService {
  public readonly sidecarStarted = this.ipcService.overlaySidecarClient.pipe(map(Boolean));
  private appSettings: AppSettings = structuredClone(APP_SETTINGS_DEFAULT);

  constructor(
    private ipcService: IPCService,
    private appSettingsService: AppSettingsService,
    private vrchat: VRChatService
  ) {}

  async init() {
    // Start the sidecar on launch
    this.appSettingsService.settings
      .pipe(
        take(1),
        map((config) => config.overlayGpuAcceleration),
        switchMap((gpuAcceleration) => this.startOrRestartSidecar(gpuAcceleration))
      )
      .subscribe();
    // Respond to settings changes
    this.appSettingsService.settings
      .pipe(
        // Store the settings on the service
        tap((settings) => (this.appSettings = settings)),
        pairwise(),
        tap(([previous, current]) => {
          // When disabling the overlay menu, close it if it's currently open
          if (!current.overlayMenuEnabled && previous.overlayMenuEnabled) {
            this.ipcService.getOverlaySidecarClient()?.closeOverlayMenu({} as Empty);
          }
          // When changing the GPU fix setting, restart the sidecar
          if (current.overlayGpuAcceleration !== previous.overlayGpuAcceleration) {
            this.startOrRestartSidecar(current.overlayGpuAcceleration);
          }
          // When enabling the overlay menu only open when VRChat is running setting, close the overlay menu if it's open
          if (
            current.overlayMenuOnlyOpenWhenVRChatIsRunning &&
            current.overlayMenuOnlyOpenWhenVRChatIsRunning !==
              previous.overlayMenuOnlyOpenWhenVRChatIsRunning
          ) {
            this.ipcService.getOverlaySidecarClient()?.closeOverlayMenu({} as Empty);
          }
        })
      )
      .subscribe();
    // Respond to VRChat process state changes
    this.vrchat.vrchatProcessActive.subscribe((active) => {
      // Close the overlay menu if it's open and VRChat is no longer active
      if (!active && this.appSettings.overlayMenuOnlyOpenWhenVRChatIsRunning) {
        this.ipcService.getOverlaySidecarClient()?.closeOverlayMenu({} as Empty);
      }
    });
  }

  private async startOrRestartSidecar(gpuAcceleration: boolean) {
    await invoke('start_overlay_sidecar', { gpuAcceleration });
  }
}
