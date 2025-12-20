import { Injectable } from '@angular/core';

import { AutomationConfigService } from '../automation-config.service';
import { skip } from 'rxjs';
import {
  AUTOMATION_CONFIGS_DEFAULT,
  WindowsPowerPolicyOnSleepModeAutomationConfig,
} from '../../models/automations';
import { SleepService } from '../sleep.service';
import { WindowsService } from '../windows.service';
import { SleepPreparationService } from '../sleep-preparation.service';

@Injectable({
  providedIn: 'root',
})
export class SetWindowsPowerPolicyOnSleepModeAutomationService {
  onSleepModeEnableConfig: WindowsPowerPolicyOnSleepModeAutomationConfig = structuredClone(
    AUTOMATION_CONFIGS_DEFAULT.WINDOWS_POWER_POLICY_ON_SLEEP_MODE_ENABLE
  );
  onSleepModePrepareConfig: WindowsPowerPolicyOnSleepModeAutomationConfig = structuredClone(
    AUTOMATION_CONFIGS_DEFAULT.WINDOWS_POWER_POLICY_ON_SLEEP_PREPARATION
  );
  onSleepModeDisableConfig: WindowsPowerPolicyOnSleepModeAutomationConfig = structuredClone(
    AUTOMATION_CONFIGS_DEFAULT.WINDOWS_POWER_POLICY_ON_SLEEP_MODE_DISABLE
  );

  constructor(
    private automationConfig: AutomationConfigService,
    private windows: WindowsService,
    private sleepMode: SleepService,
    private sleepPrepare: SleepPreparationService
  ) {}

  async init() {
    this.automationConfig.configs.subscribe((configs) => {
      this.onSleepModeEnableConfig = configs.WINDOWS_POWER_POLICY_ON_SLEEP_MODE_ENABLE;
      this.onSleepModePrepareConfig = configs.WINDOWS_POWER_POLICY_ON_SLEEP_PREPARATION;
      this.onSleepModeDisableConfig = configs.WINDOWS_POWER_POLICY_ON_SLEEP_MODE_DISABLE;
    });
    this.sleepPrepare.onSleepPreparation.subscribe(async (_) => {
      if (this.onSleepModePrepareConfig && this.onSleepModePrepareConfig.powerPolicy) {
        await this.windows.setWindowsPowerPolicy(
          this.onSleepModePrepareConfig.powerPolicy,
          'SLEEP_PREPARATION'
        );
      }
    });
    this.sleepMode.mode
      .pipe(
        skip(1) // Skip first value from initial load
      )
      .subscribe(async (sleepMode) => {
        if (
          sleepMode &&
          this.onSleepModeEnableConfig.enabled &&
          this.onSleepModeEnableConfig.powerPolicy
        ) {
          await this.windows.setWindowsPowerPolicy(
            this.onSleepModeEnableConfig.powerPolicy,
            'SLEEP_MODE_ENABLED'
          );
        } else if (
          !sleepMode &&
          this.onSleepModeDisableConfig.enabled &&
          this.onSleepModeDisableConfig.powerPolicy
        ) {
          await this.windows.setWindowsPowerPolicy(
            this.onSleepModeDisableConfig.powerPolicy,
            'SLEEP_MODE_DISABLED'
          );
        }
      });
  }
}
