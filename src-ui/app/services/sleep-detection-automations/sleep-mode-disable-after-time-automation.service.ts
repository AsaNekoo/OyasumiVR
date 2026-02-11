import { Injectable } from '@angular/core';
import { AutomationConfigService } from '../automation-config.service';
import {
  AUTOMATION_CONFIGS_DEFAULT,
  SleepModeDisableAfterTimeAutomationConfig,
} from '../../models/automations';

import { distinctUntilChanged, interval, map } from 'rxjs';
import { SleepService } from '../sleep.service';

@Injectable({
  providedIn: 'root',
})
export class SleepModeDisableAfterTimeAutomationService {
  private config: SleepModeDisableAfterTimeAutomationConfig = structuredClone(
    AUTOMATION_CONFIGS_DEFAULT.SLEEP_MODE_DISABLE_AFTER_TIME
  );
  sleepLastEnabled = -1;
  sleepLastDisabled = -1;
  sleepEnabled = false;
  sleep_duration = -1;
  awake_duration: number | null = null;

  constructor(
    private automationConfig: AutomationConfigService,
    private sleep: SleepService
  ) {}

  async init() {
    this.automationConfig.configs
      .pipe(map((configs) => configs.SLEEP_MODE_DISABLE_AFTER_TIME))
      .subscribe((config) => {
        this.config = config;
        if (config.duration) {
          const [hours, minutes] = config.duration.split(':').map((v) => parseInt(v));
          this.sleep_duration = hours * 60 * 60 * 1000 + minutes * 60 * 1000;
        }
        if (config.awake) {
          const [hours, minutes] = config.awake.split(':').map((v) => parseInt(v));
          this.awake_duration = hours * 60 * 60 * 1000 + minutes * 60 * 1000;
        }
      });
    this.sleep.mode.pipe(distinctUntilChanged()).subscribe((mode) => {
      this.sleepEnabled = mode;
      if (mode) {
        if (this.awake_duration) {
          if (Date.now() - this.sleepLastDisabled > this.awake_duration) {
            this.sleepLastEnabled = Date.now();
          }
        } else {
          this.sleepLastEnabled = Date.now();
        }
      } else {
        this.sleepLastDisabled = Date.now();
      }
    });
    interval(30000).subscribe(() => this.onTick());
  }

  async onTick() {
    if (!this.config.enabled || !this.config.duration) return;
    if (this.sleep_duration <= 0 || this.sleepLastEnabled <= 0) return;
    if (this.sleepEnabled && Date.now() - this.sleepLastEnabled >= this.sleep_duration) {
      this.sleepEnabled = false;
      this.sleepLastEnabled = -1;
      await this.sleep.disableSleepMode({
        type: 'AUTOMATION',
        automation: 'SLEEP_MODE_DISABLE_AFTER_TIME',
      });
    }
  }
}
