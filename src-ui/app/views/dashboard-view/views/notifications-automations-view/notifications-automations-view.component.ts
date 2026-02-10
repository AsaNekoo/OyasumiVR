import { Component, OnInit } from '@angular/core';
import { SelectBoxItem } from '../../../../components/select-box/select-box.component';
import { AutomationConfigService } from '../../../../services/automation-config.service';
import {
  AUTOMATION_CONFIGS_DEFAULT,
  NotificationsAutomationsConfig,
  NotificationSetting,
  SystemMicMuteAutomationsConfig,
} from '../../../../models/automations';
import { fade, vshrink } from '../../../../utils/animations';
import { map } from 'rxjs';

@Component({
  selector: 'app-notifications-automations-view',
  templateUrl: './notifications-automations-view.component.html',
  styleUrls: ['./notifications-automations-view.component.scss'],
  animations: [vshrink(), fade()],
  standalone: false,
})
export class NotificationsAutomationsViewComponent implements OnInit {
  config: NotificationsAutomationsConfig = structuredClone(
    AUTOMATION_CONFIGS_DEFAULT.NOTIFICATIONS_AUTOMATIONS
  );
  inhibitOptions: SelectBoxItem[] = [
    {
      id: NotificationSetting.Keep.toString(),
      label: 'notificationsAutomations.selectBox.keep',
    },
    {
      id: NotificationSetting.Inhibit.toString(),
      label: 'notificationsAutomations.selectBox.enable',
    },
    {
      id: NotificationSetting.UnInhibit.toString(),
      label: 'notificationsAutomations.selectBox.disable',
    },
  ];
  SystemOnSleepEnableInhibitOption: SelectBoxItem = this.inhibitOptions[0];
  SystemOnSleepDisableInhibitOption: SelectBoxItem = this.inhibitOptions[0];
  SystemOnSleepPreparationInhibitOption: SelectBoxItem = this.inhibitOptions[0];

  constructor(private automationConfigService: AutomationConfigService) {}

  async ngOnInit() {
    this.automationConfigService.configs
      .pipe(map((configs) => configs.NOTIFICATIONS_AUTOMATIONS))
      .subscribe((config) => {
        this.config = config;
        this.SystemOnSleepEnableInhibitOption = this.inhibitOptions[config.SystemOnSleepModeEnable];
        this.SystemOnSleepDisableInhibitOption =
          this.inhibitOptions[config.SystemOnSleepModeDisable];
        this.SystemOnSleepPreparationInhibitOption =
          this.inhibitOptions[config.SystemOnSleepPrepare];
      });
  }

  async onChangeSystemInhibitOption(
    automation: 'ON_SLEEP_ENABLE' | 'ON_SLEEP_DISABLE' | 'ON_SLEEP_PREPARATION',
    option: SelectBoxItem | undefined
  ) {
    if (!option) return;
    const keyMap = {
      ON_SLEEP_ENABLE: 'SystemOnSleepModeEnable',
      ON_SLEEP_DISABLE: 'SystemOnSleepModeDisable',
      ON_SLEEP_PREPARATION: 'SystemOnSleepPrepare',
    };
    const key = keyMap[automation];
    await this.automationConfigService.updateAutomationConfig<SystemMicMuteAutomationsConfig>(
      'NOTIFICATIONS_AUTOMATIONS',
      {
        [key]: parseInt(option!.id) as NotificationSetting,
      }
    );
  }
}
