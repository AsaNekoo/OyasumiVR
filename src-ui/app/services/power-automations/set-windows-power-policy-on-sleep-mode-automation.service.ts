import { Injectable } from '@angular/core';

import { AutomationConfigService } from '../automation-config.service';
import { skip } from 'rxjs';
import {
  AUTOMATION_CONFIGS_DEFAULT,
  SystemPowerPolicyOnSleepModeAutomationConfig,
} from '../../models/automations';
import { SleepService } from '../sleep.service';
import { SystemService } from '../windows.service';
import { SleepPreparationService } from '../sleep-preparation.service';

@Injectable({
  providedIn: 'root',
})
export class SetSystemPowerPolicyOnSleepModeAutomationService {
  onSleepModeEnableConfig: SystemPowerPolicyOnSleepModeAutomationConfig = structuredClone(
    AUTOMATION_CONFIGS_DEFAULT.SYSTEM_POWER_POLICY_ON_SLEEP_MODE_ENABLE
  );
  onSleepModePrepareConfig: SystemPowerPolicyOnSleepModeAutomationConfig = structuredClone(
    AUTOMATION_CONFIGS_DEFAULT.SYSTEM_POWER_POLICY_ON_SLEEP_PREPARATION
  );
  onSleepModeDisableConfig: SystemPowerPolicyOnSleepModeAutomationConfig = structuredClone(
    AUTOMATION_CONFIGS_DEFAULT.SYSTEM_POWER_POLICY_ON_SLEEP_MODE_DISABLE
  );

  constructor(
    private automationConfig: AutomationConfigService,
    private windows: SystemService,
    private sleepMode: SleepService,
    private sleepPrepare: SleepPreparationService
  ) {}

  async init() {
    this.automationConfig.configs.subscribe((configs) => {
      this.onSleepModeEnableConfig = configs.SYSTEM_POWER_POLICY_ON_SLEEP_MODE_ENABLE;
      this.onSleepModePrepareConfig = configs.SYSTEM_POWER_POLICY_ON_SLEEP_PREPARATION;
      this.onSleepModeDisableConfig = configs.SYSTEM_POWER_POLICY_ON_SLEEP_MODE_DISABLE;
    });
    this.sleepPrepare.onSleepPreparation.subscribe(async (_) => {
      if (this.onSleepModePrepareConfig && this.onSleepModePrepareConfig.powerPolicy) {
        await this.windows.setPowerPolicy(
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
          await this.windows.setPowerPolicy(
            this.onSleepModeEnableConfig.powerPolicy,
            'SLEEP_MODE_ENABLED'
          );
        } else if (
          !sleepMode &&
          this.onSleepModeDisableConfig.enabled &&
          this.onSleepModeDisableConfig.powerPolicy
        ) {
          await this.windows.setPowerPolicy(
            this.onSleepModeDisableConfig.powerPolicy,
            'SLEEP_MODE_DISABLED'
          );
        }
      });
  }
}
