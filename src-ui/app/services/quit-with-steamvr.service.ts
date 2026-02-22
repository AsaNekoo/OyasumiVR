import { Injectable } from '@angular/core';
import { QuitWithVRMode } from '../models/settings';
import { AppSettingsService } from './app-settings.service';
import { VRService } from './openvr.service';
import { debounceTime, EMPTY, filter, firstValueFrom, of, pairwise, switchMap } from 'rxjs';
import { exit } from '@tauri-apps/plugin-process';
import { info } from '@tauri-apps/plugin-log';

@Injectable({
  providedIn: 'root',
})
export class QuitWithVRService {
  private mode: QuitWithVRMode = 'DISABLED';

  constructor(
    private appSettings: AppSettingsService,
    private openvr: VRService
  ) {}

  async init() {
    this.appSettings.settings.subscribe((settings) => {
      this.mode = settings.quitWithVR;
    });
    this.openvr.status
      .pipe(
        filter((status) => ['INACTIVE', 'INITIALIZED'].includes(status)),
        pairwise(),
        filter(([oldStatus, newStatus]) => oldStatus === 'INITIALIZED' && newStatus === 'INACTIVE'),
        switchMap(async () => {
          if (this.mode === 'AFTERDELAY') return of(void 0);
          if (this.mode === 'IMMEDIATELY') {
            info('[QuitWithVR] VR has stopped: quitting OyasumiVR immediately.');
            await exit(0);
          }
          return EMPTY;
        }),
        debounceTime(1000 * 60 * 2),
        switchMap(async () => {
          if (
            this.mode === 'AFTERDELAY' &&
            (await firstValueFrom(this.openvr.status)) === 'INACTIVE'
          ) {
            info('[QuitWithVR] VR has stopped for 2 minutes: quitting OyasumiVR.');
            await exit(0);
          }
          return EMPTY;
        })
      )
      .subscribe();
  }
}
