import { Injectable } from '@angular/core';
import { SleepService } from './sleep.service';
import { distinctUntilChanged, skip } from 'rxjs';
import { invoke } from '@tauri-apps/api/core';
import { SleepPreparationService } from './sleep-preparation.service';
import { AutomationConfigService } from './automation-config.service';
import { AUTOMATION_CONFIGS_DEFAULT, PlaybackAutomationConfig } from '../models/automations';

@Injectable({
  providedIn: 'root',
})
export class MpriService {
    protected config: PlaybackAutomationConfig = structuredClone(
        AUTOMATION_CONFIGS_DEFAULT.PLAYBACK_AUTOMATIONS
      );
  constructor(
    private sleep: SleepService,
    private sleepPreparation: SleepPreparationService,
    private automationConfigService: AutomationConfigService,
  ) {}

  async init() {
    this.automationConfigService.configs.subscribe((configs) => {
      this.config = configs.PLAYBACK_AUTOMATIONS;
    });
    this.sleep.mode.pipe(skip(1),distinctUntilChanged()).subscribe(async (sleepMode)=>{
        if (sleepMode && this.config.pause_on_sleep_enable){
            await invoke("pause_mpris_players");
        }else if (!sleepMode && this.config.pause_on_sleep_disable){
            await invoke("pause_mpris_players");
        }
    });
    this.sleepPreparation.onSleepPreparation.subscribe(async()=>{
        if (this.config.pause_on_sleep_preparation){
            await invoke("pause_mpris_players");
        }
    }); 
  }
}
