import { Injectable } from '@angular/core';
import { AutomationConfigService } from '../automation-config.service';
import {
  AUTOMATION_CONFIGS_DEFAULT,
  SleepModeDisableAfterTimeAutomationConfig,
} from '../../models/automations';

import { distinctUntilChanged, map } from 'rxjs';
import { SleepService } from '../sleep.service';
import { time_to_ms } from 'src-ui/app/utils/time';

@Injectable({
  providedIn: 'root',
})
export class SleepModeDisableAfterTimeAutomationService {
  private config: SleepModeDisableAfterTimeAutomationConfig = structuredClone(
    AUTOMATION_CONFIGS_DEFAULT.SLEEP_MODE_DISABLE_AFTER_TIME
  );

  private timeout: NodeJS.Timeout | null = null;
  private ClearTimeout: NodeJS.Timeout | null = null;
  private SleepEnableTimeout: NodeJS.Timeout | null = null;
  constructor(
    private automationConfig: AutomationConfigService,
    private sleep: SleepService
  ) {}

  async init() {
    this.automationConfig.configs
      .pipe(map((configs) => configs.SLEEP_MODE_DISABLE_AFTER_TIME))
      .subscribe((config) => {
        this.config = config;
      });

    this.sleep.mode.pipe(distinctUntilChanged()).subscribe((mode) => {
      if (!this.config.duration) {
        console.error('SleepModeDisableAfterTimeAutomationService this.config.duration is null!');
        return;
      }
      if (mode) {
        if (this.SleepEnableTimeout) {
          clearTimeout(this.SleepEnableTimeout);
        }
        this.SleepEnableTimeout = setTimeout(() => {
          if (!this.config.duration) {
            console.error(
              'SleepModeDisableAfterTimeAutomationService this.config.duration is null! (2)'
            );
            return;
          }
          if (this.ClearTimeout) {
            clearTimeout(this.ClearTimeout);
          }
          if (!this.timeout) {
            this.timeout = setTimeout(() => this.disable(), time_to_ms(this.config.duration)-time_to_ms(this.config.sleep));
          }
        }, time_to_ms(this.config.sleep));
      } else {
        if (this.SleepEnableTimeout){
          clearTimeout(this.SleepEnableTimeout);
        }
        if (this.ClearTimeout) {
          clearTimeout(this.ClearTimeout);
        }
        if (this.config.awake) {
          this.ClearTimeout = setTimeout(() => {
            if (!this.timeout) {
              console.warn('SleepModeDisableAfterTimeAutomationService upsie');
              return;
            }
            clearTimeout(this.timeout);
          }, time_to_ms(this.config.awake));
        }
      }
    });
  }

  async disable() {
    await this.sleep.disableSleepMode({
      type: 'AUTOMATION',
      automation: 'SLEEP_MODE_DISABLE_AFTER_TIME',
    });
  }
}
