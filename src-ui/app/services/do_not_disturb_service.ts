import { Injectable } from '@angular/core';
import { SleepService } from './sleep.service';
import { distinctUntilChanged, skip } from 'rxjs';
import { invoke } from '@tauri-apps/api/core';
import { SleepPreparationService } from './sleep-preparation.service';
import { AutomationConfigService } from './automation-config.service';
import {
  AUTOMATION_CONFIGS_DEFAULT,
  NotificationsAutomationsConfig,
  NotificationSetting,
} from '../models/automations';

@Injectable({
  providedIn: 'root',
})
export class DNDService {
  protected config: NotificationsAutomationsConfig = structuredClone(
    AUTOMATION_CONFIGS_DEFAULT.NOTIFICATIONS_AUTOMATIONS
  );
  constructor(
    private sleep: SleepService,
    private sleepPreparation: SleepPreparationService,
    private automationConfigService: AutomationConfigService,
  ) {}

  async init() {
    this.automationConfigService.configs.subscribe((configs) => {
      this.config = configs.NOTIFICATIONS_AUTOMATIONS;
    });
    this.sleep.mode.pipe(skip(1), distinctUntilChanged()).subscribe(async (sleepMode) => {
      if (sleepMode) {
        switch (this.config.SystemOnSleepModeEnable) {
          case NotificationSetting.Inhibit:
            await invoke('n_os_inhibit');
            break;
          case NotificationSetting.UnInhibit:
            await invoke('n_os_un_inhibit');
            break;
        }
      } else {
        switch (this.config.SystemOnSleepModeDisable) {
          case NotificationSetting.Inhibit:
            await invoke('n_os_inhibit');
            break;
          case NotificationSetting.UnInhibit:
            await invoke('n_os_un_inhibit');
            break;
        }
      }
    });
    this.sleepPreparation.onSleepPreparation.subscribe(async () => {
      switch (this.config.SystemOnSleepPrepare) {
        case NotificationSetting.Inhibit:
          await invoke('n_os_inhibit');
          break;
        case NotificationSetting.UnInhibit:
          await invoke('n_os_un_inhibit');
          break;
      }
    });
  }
}
