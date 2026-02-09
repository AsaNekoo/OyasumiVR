import { Component, OnDestroy, OnInit } from '@angular/core';
import { SelectBoxItem } from '../../../../components/select-box/select-box.component';
import { AutomationConfigService } from '../../../../services/automation-config.service';
import {
  AUTOMATION_CONFIGS_DEFAULT,
  NotificationsAutomationsConfig,
  SystemMicMuteAutomationsConfig,
  SystemMicMuteControllerBindingBehavior,
  SystemMicMuteStateOption,
  VRChatMicrophoneWorldJoinBehaviour,
} from '../../../../models/automations';
import { OVRInputEventAction } from '../../../../models/ovr-input-event';
import { fade, vshrink } from '../../../../utils/animations';
import { map } from 'rxjs';

@Component({
  selector: 'app-notifications-automations-view',
  templateUrl: './notifications-automations-view.component.html',
  styleUrls: ['./notifications-automations-view.component.scss'],
  animations: [vshrink(), fade()],
  standalone: false,
})
export class NotificationsAutomationsViewComponent implements OnInit, OnDestroy {
  config: NotificationsAutomationsConfig = structuredClone(
    AUTOMATION_CONFIGS_DEFAULT.NOTIFICATIONS_AUTOMATIONS
  );





  constructor(
    private automationConfigService: AutomationConfigService,
  ) {}

  async ngOnInit() {
    this.automationConfigService.configs.pipe(
      map((configs) => configs.NOTIFICATIONS_AUTOMATIONS),
    ).subscribe((config)=>{
      this.config = config;
    });
   
  }

  ngOnDestroy() {
  }

  async onChangeAudioDevice($event: SelectBoxItem | undefined) {
    if (!$event) return;
    await this.automationConfigService.updateAutomationConfig<SystemMicMuteAutomationsConfig>(
      'SYSTEM_MIC_MUTE_AUTOMATIONS',
      {
        audioDevicePersistentId: $event.id,
      }
    );
  }

  async onChangeControlButtonBehaviorOption($event: SelectBoxItem | undefined) {
    // if (!$event) return;
    // this.systemMicMuteAutomationService.setDefaultControlButtonBehavior(
    //   $event.id as SystemMicMuteControllerBindingBehavior
    // );
  }

  async onChangeMuteOption(
    automation: 'ON_SLEEP_ENABLE' | 'ON_SLEEP_DISABLE' | 'ON_SLEEP_PREPARATION',
    option: SelectBoxItem | undefined
  ) {
    if (!option) return;
    const keyMap = {
      ON_SLEEP_ENABLE: 'onSleepModeEnableState',
      ON_SLEEP_DISABLE: 'onSleepModeDisableState',
      ON_SLEEP_PREPARATION: 'onSleepPreparationState',
    };
    const key = keyMap[automation];
    await this.automationConfigService.updateAutomationConfig<SystemMicMuteAutomationsConfig>(
      'SYSTEM_MIC_MUTE_AUTOMATIONS',
      {
        [key]: option!.id as SystemMicMuteStateOption,
      }
    );
  }

  async onChangeControlButtonBehaviorAutomationOption(
    automation: 'ON_SLEEP_ENABLE' | 'ON_SLEEP_DISABLE' | 'ON_SLEEP_PREPARATION',
    option: SelectBoxItem | undefined
  ) {
    if (!option) return;
    const keyMap = {
      ON_SLEEP_ENABLE: 'onSleepModeEnableControllerBindingBehavior',
      ON_SLEEP_DISABLE: 'onSleepModeDisableControllerBindingBehavior',
      ON_SLEEP_PREPARATION: 'onSleepPreparationControllerBindingBehavior',
    };
    const key = keyMap[automation];
    await this.automationConfigService.updateAutomationConfig<SystemMicMuteAutomationsConfig>(
      'NOTIFICATIONS_AUTOMATIONS',
      {
        [key]: option!.id as SystemMicMuteControllerBindingBehavior | 'NONE',
      }
    );
  }

  async onChangeOverlayMuteIndicator() {
    // await this.automationConfigService.updateAutomationConfig<SystemMicMuteAutomationsConfig>(
    //   'NOTIFICATIONS_AUTOMATIONS',
    //   {
    //     overlayMuteIndicator: !this.config.overlayMuteIndicator,
    //   }
    // );
  }

  async onChangeOverlayMuteIndicatorFade() {
    // await this.automationConfigService.updateAutomationConfig<SystemMicMuteAutomationsConfig>(
    //   'SYSTEM_MIC_MUTE_AUTOMATIONS',
    //   {
    //     overlayMuteIndicatorFade: !this.config.overlayMuteIndicatorFade,
    //   }
    // );
  }

  async onChangeControllerBinding() {
    // await this.automationConfigService.updateAutomationConfig<SystemMicMuteAutomationsConfig>(
    //   'SYSTEM_MIC_MUTE_AUTOMATIONS',
    //   {
    //     controllerBinding: !this.config.controllerBinding,
    //   }
    // );
  }

  async onChangeMuteSoundVolume(volume: number) {
    await this.automationConfigService.updateAutomationConfig<SystemMicMuteAutomationsConfig>(
      'SYSTEM_MIC_MUTE_AUTOMATIONS',
      {
        muteSoundVolume: volume,
      }
    );
  }

  async onChangeOverlayMuteIndicatorOpacity(opacity: number) {
    await this.automationConfigService.updateAutomationConfig<SystemMicMuteAutomationsConfig>(
      'SYSTEM_MIC_MUTE_AUTOMATIONS',
      {
        overlayMuteIndicatorOpacity: opacity,
      }
    );
  }

  async onChangeHardwareVoiceActivationThreshold(threshold: number) {
    await this.automationConfigService.updateAutomationConfig<SystemMicMuteAutomationsConfig>(
      'SYSTEM_MIC_MUTE_AUTOMATIONS',
      {
        hardwareVoiceActivationThreshold: threshold,
      }
    );
  }

  async onChangeVoiceActivationMode(option: SelectBoxItem | undefined) {
    if (!option) return;
    await this.automationConfigService.updateAutomationConfig<SystemMicMuteAutomationsConfig>(
      'SYSTEM_MIC_MUTE_AUTOMATIONS',
      {
        voiceActivationMode: option!.id as 'VRCHAT' | 'HARDWARE',
      }
    );
  }

  async onChangeWorldJoinBehaviourOption(option: SelectBoxItem | undefined) {
    if (!option) return;
    await this.automationConfigService.updateAutomationConfig<SystemMicMuteAutomationsConfig>(
      'SYSTEM_MIC_MUTE_AUTOMATIONS',
      {
        vrchatWorldJoinBehaviour: option!.id as VRChatMicrophoneWorldJoinBehaviour,
      }
    );
  }

  protected readonly OVRInputEventAction = OVRInputEventAction;
}
