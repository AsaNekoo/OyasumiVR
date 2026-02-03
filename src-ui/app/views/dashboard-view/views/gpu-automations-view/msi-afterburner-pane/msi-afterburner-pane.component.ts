import { Component, DestroyRef, OnInit } from '@angular/core';
import { Subject } from 'rxjs';
import {
  APP_SETTINGS_DEFAULT,
  AppSettings,
  ExecutableReferenceStatus,
} from '../../../../../models/settings';

import { GpuAutomationsService } from '../../../../../services/gpu-automations.service';
import {
  AUTOMATION_CONFIGS_DEFAULT,
  MSIAfterburnerAutomationConfig,
} from '../../../../../models/automations';
import { vshrink } from '../../../../../utils/animations';
import { SelectBoxItem } from '../../../../../components/select-box/select-box.component';
import { takeUntilDestroyed } from '@angular/core/rxjs-interop';
import { is_windows } from 'src-ui/app/app.module';
import { invoke } from '@tauri-apps/api/core';

@Component({
  selector: 'app-msi-afterburner-pane',
  templateUrl: './msi-afterburner-pane.component.html',
  styleUrls: ['./msi-afterburner-pane.component.scss'],
  animations: [vshrink()],
  standalone: false,
})
export class MsiAfterburnerPaneComponent implements OnInit {
  is_windows: boolean = true;
  msiAfterburnerStatus: ExecutableReferenceStatus = 'UNKNOWN';
  msiAfterburnerPathAlert?: {
    text: string;
    type: 'INFO' | 'SUCCESS' | 'ERROR';
    loadingIndicator?: boolean;
  };
  msiAfterburnerPathInputChange: Subject<string> = new Subject();
  appSettings: AppSettings = structuredClone(APP_SETTINGS_DEFAULT);
  config: MSIAfterburnerAutomationConfig = structuredClone(
    AUTOMATION_CONFIGS_DEFAULT.MSI_AFTERBURNER
  );
  profileOptions: SelectBoxItem[] = [
    {
      id: '0',
      label: 'gpu-automations.msiAfterburner.none',
    },
  ];
  onDisableProfile: SelectBoxItem = this.profileOptions[0];
  onEnableProfile: SelectBoxItem = this.profileOptions[0];
  onPrepareProfile: SelectBoxItem = this.profileOptions[0];

  constructor(
    protected gpuAutomations: GpuAutomationsService,
    private destroyRef: DestroyRef
  ) {}

  async ngOnInit() {
    try {
      let profiles = await invoke<string[]>('gpu_get_profiles');
      profiles.forEach((element) => {
        this.profileOptions.push({
          id: element,
          label: element,
        });
      });
    } catch (e) {
      console.warn('failed to get gpu profile with' + e);
    }
    this.gpuAutomations.msiAfterburnerConfig
      .pipe(takeUntilDestroyed(this.destroyRef))
      .subscribe((config) => {
        this.config = config;
        let onEnableProfile = this.profileOptions.find(
          (element) => element.id == this.config.onSleepEnableProfile
        );
        if (onEnableProfile) {
          this.onEnableProfile = onEnableProfile;
        } else {
          this.onEnableProfile = this.profileOptions[0];
        }
        let onDisableProfile = this.profileOptions.find(
          (element) => element.id == this.config.onSleepDisableProfile
        );
        if (onDisableProfile) {
          this.onDisableProfile = onDisableProfile;
        } else {
          this.onDisableProfile = this.profileOptions[0];
        }
        let onPrepareProfile = this.profileOptions.find(
          (element) => element.id == this.config.onSleepPreparation
        );
        if (onPrepareProfile) {
          this.onPrepareProfile = onPrepareProfile;
        } else {
          this.onPrepareProfile = this.profileOptions[0];
        }
      });
    this.gpuAutomations.msiAfterburnerStatus
      .pipe(takeUntilDestroyed(this.destroyRef))
      .subscribe((status) => this.processMSIAfterburnerStatus(status));
    this.is_windows = is_windows;
  }

  processMSIAfterburnerStatus(status: ExecutableReferenceStatus) {
    this.msiAfterburnerStatus = status;
    const statusToAlertType: {
      [s: string]: 'INFO' | 'SUCCESS' | 'ERROR';
    } = { CHECKING: 'INFO', SUCCESS: 'SUCCESS' };
    this.msiAfterburnerPathAlert = (
      [
        'NOT_FOUND',
        'INVALID_EXECUTABLE',
        'INVALID_SIGNATURE',
        'UNKNOWN_ERROR',
        'CHECKING',
        'SUCCESS',
      ] as ExecutableReferenceStatus[]
    ).includes(status)
      ? {
          type: statusToAlertType[status] || 'ERROR',
          text: 'gpu-automations.msiAfterburner.executable.status.' + status,
          loadingIndicator: status === 'CHECKING',
        }
      : undefined;
  }

  changeProfile(event: 'ON_DISABLE' | 'ON_ENABLE' | 'ON_PREPARE', item: SelectBoxItem) {
    console.warn(item);
    switch (event) {
      case 'ON_DISABLE':
        this.onDisableProfile = item;
        this.gpuAutomations.setMSIAfterburnerProfileOnSleepDisable(item.id);
        break;
      case 'ON_ENABLE':
        this.gpuAutomations.setMSIAfterburnerProfileOnSleepEnable(item.id);
        this.onEnableProfile = item;
        break;
      case 'ON_PREPARE':
        this.gpuAutomations.setMSIAfterburnerProfileOnSleepPreparation(item.id);
        this.onPrepareProfile = item;
        break;
    }
  }
}
