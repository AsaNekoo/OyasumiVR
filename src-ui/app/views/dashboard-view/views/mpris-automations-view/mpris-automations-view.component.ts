import { Component, OnInit } from '@angular/core';
import { AutomationConfigService } from '../../../../services/automation-config.service';
import {
  AUTOMATION_CONFIGS_DEFAULT,
  PlaybackAutomationConfig,
} from '../../../../models/automations';

@Component({
  selector: 'app-mpris-automations-view',
  templateUrl: './mpris-automations-view.component.html',
  styleUrls: ['./mpris-automations-view.component.scss'],
  animations: [],
  standalone: false,
})
export class PlaybackAutomationsViewComponent implements OnInit {
  protected config: PlaybackAutomationConfig = structuredClone(
    AUTOMATION_CONFIGS_DEFAULT.PLAYBACK_AUTOMATIONS
  );

  constructor(private automationConfigService: AutomationConfigService) {}

  ngOnInit(): void {
    this.automationConfigService.configs.subscribe((configs) => {
      this.config = configs.PLAYBACK_AUTOMATIONS;
    });
  }
  async set_enabled(when: 'SLEEP_ENABLE' | 'SLEEP_DISABLE' | 'SLEEP_PREPARATION', value: boolean) {
    switch (when) {
      case 'SLEEP_ENABLE':
        await this.automationConfigService.updateAutomationConfig<PlaybackAutomationConfig>(
          'PLAYBACK_AUTOMATIONS',
          {
            pause_on_sleep_enable: value,
          }
        );
        break;
      case 'SLEEP_PREPARATION':
        await this.automationConfigService.updateAutomationConfig<PlaybackAutomationConfig>(
          'PLAYBACK_AUTOMATIONS',
          {
            pause_on_sleep_preparation: value,
          }
        );
        break;
      case 'SLEEP_DISABLE':
        await this.automationConfigService.updateAutomationConfig<PlaybackAutomationConfig>(
          'PLAYBACK_AUTOMATIONS',
          {
            pause_on_sleep_disable: value,
          }
        );
        break;
    }
  }
}
