import { Component, DestroyRef, OnInit } from '@angular/core';
import { AutomationConfigService } from '../../../../../../services/automation-config.service';
import {
  AUTOMATION_CONFIGS_DEFAULT,
  SystemPowerPolicyOnSleepModeAutomationConfig,
} from '../../../../../../models/automations';
import { takeUntilDestroyed } from '@angular/core/rxjs-interop';
import { SelectBoxItem } from '../../../../../../components/select-box/select-box.component';
import { SystemService } from '../../../../../../services/windows.service';
import { combineLatest, tap } from 'rxjs';

@Component({
  selector: 'app-power-policy-tab',
  templateUrl: './power-policy-tab.component.html',
  styleUrls: ['./power-policy-tab.component.scss'],
  standalone: false,
})
export class PowerPolicyTabComponent implements OnInit {
  protected is_windows: boolean = false;
  protected policyOptions: SelectBoxItem[] = [
    {
      id: 'NONE',
      label: 'shared.common.none',
    },
  ];
  protected policyProviders: SelectBoxItem[] = [
    {
      id: 'NONE',
      label: 'shared.common.none',
    },
  ];

  protected onSleepModeEnablePolicy: SelectBoxItem =
    this.policyOptions.find(
      (p) =>
        p.id === AUTOMATION_CONFIGS_DEFAULT.SYSTEM_POWER_POLICY_ON_SLEEP_MODE_ENABLE.powerPolicy
    ) ?? this.policyOptions[0];
  protected onSleepPreparePolicy: SelectBoxItem =
    this.policyOptions.find(
      (p) =>
        p.id === AUTOMATION_CONFIGS_DEFAULT.SYSTEM_POWER_POLICY_ON_SLEEP_PREPARATION.powerPolicy
    ) ?? this.policyOptions[0];

  protected onSleepModeDisablePolicy: SelectBoxItem =
    this.policyOptions.find(
      (p) =>
        p.id === AUTOMATION_CONFIGS_DEFAULT.SYSTEM_POWER_POLICY_ON_SLEEP_MODE_DISABLE.powerPolicy
    ) ?? this.policyOptions[0];

  constructor(
    private automationConfigService: AutomationConfigService,
    private destroyRef: DestroyRef,
    private windowsService: SystemService
  ) {}

  async ngOnInit() {
    combineLatest([
      this.automationConfigService.configs,
      // Update options when the windows power policies are updated
      this.windowsService.policies.pipe(
        tap((policies) => {
          this.policyOptions = [
            {
              id: 'NONE',
              label: 'shared.common.none',
            },
          ];
          policies.forEach((policy) => {
            this.policyOptions.push({
              id: policy.name,
              label: policy.name,
            });
          });
        })
      ),
    ])
      .pipe(takeUntilDestroyed(this.destroyRef))
      .subscribe(([configs]) => {
        // Update options when the windows power policies are updated
        this.onSleepModeEnablePolicy =
          this.policyOptions.find(
            (p) => p.id === configs.SYSTEM_POWER_POLICY_ON_SLEEP_MODE_ENABLE.powerPolicy
          ) ?? this.policyOptions[0];
        this.onSleepPreparePolicy =
          this.policyOptions.find(
            (p) => p.id === configs.SYSTEM_POWER_POLICY_ON_SLEEP_PREPARATION.powerPolicy
          ) ?? this.policyOptions[0];
        this.onSleepModeDisablePolicy =
          this.policyOptions.find(
            (p) => p.id === configs.SYSTEM_POWER_POLICY_ON_SLEEP_MODE_DISABLE.powerPolicy
          ) ?? this.policyOptions[0];
      });
    await this.windowsService.getPowerPolicies();
  }

  async setPolicy(
    automation: 'ON_ENABLE' | 'ON_DISABLE' | 'ON_PREPARE',
    selectBoxItem: SelectBoxItem
  ) {
    switch (automation) {
      case 'ON_ENABLE':
        await this.automationConfigService.updateAutomationConfig<SystemPowerPolicyOnSleepModeAutomationConfig>(
          'SYSTEM_POWER_POLICY_ON_SLEEP_MODE_ENABLE',
          {
            enabled: selectBoxItem.id !== 'NONE',
            powerPolicy: selectBoxItem.id === 'NONE' ? undefined : selectBoxItem.id,
          }
        );
        break;
      case 'ON_PREPARE':
        await this.automationConfigService.updateAutomationConfig<SystemPowerPolicyOnSleepModeAutomationConfig>(
          'SYSTEM_POWER_POLICY_ON_SLEEP_PREPARATION',
          {
            enabled: selectBoxItem.id !== 'NONE',
            powerPolicy: selectBoxItem.id === 'NONE' ? undefined : selectBoxItem.id,
          }
        );
        break;
      case 'ON_DISABLE':
        await this.automationConfigService.updateAutomationConfig<SystemPowerPolicyOnSleepModeAutomationConfig>(
          'SYSTEM_POWER_POLICY_ON_SLEEP_MODE_DISABLE',
          {
            enabled: selectBoxItem.id !== 'NONE',
            powerPolicy: selectBoxItem.id === 'NONE' ? undefined : selectBoxItem.id,
          }
        );
        break;
    }
  }
}
