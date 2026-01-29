import { Injectable } from '@angular/core';
import { AutomationConfigService } from './automation-config.service';
import {
  BehaviorSubject,
  distinctUntilChanged,
  filter,
  firstValueFrom,
  map,
  Observable,
  pairwise,
  skip,
  startWith,
  switchMap,
  take,
} from 'rxjs';
import {
  AUTOMATION_CONFIGS_DEFAULT,
  MSIAfterburnerAutomationConfig,
} from '../models/automations';

import { SleepService } from './sleep.service';
import { error, warn } from '@tauri-apps/plugin-log';
import { invoke } from '@tauri-apps/api/core';
import { ExecutableReferenceStatus } from '../models/settings';
import { EventLogService } from './event-log.service';
import {
  EventLogMsiAfterburnerProfileSet,
} from '../models/event-log-entry';
import { SleepPreparationService } from './sleep-preparation.service';

@Injectable({
  providedIn: 'root',
})
export class GpuAutomationsService {
  // MSI Afterburner
  private currentMSIAfterburnerConfig: MSIAfterburnerAutomationConfig = structuredClone(
    AUTOMATION_CONFIGS_DEFAULT.MSI_AFTERBURNER
  );
  public msiAfterburnerConfig: Observable<MSIAfterburnerAutomationConfig> =
    this.automationConfig.configs.pipe(map((configs) => configs.MSI_AFTERBURNER));
  private _msiAfterburnerStatus: BehaviorSubject<ExecutableReferenceStatus> =
    new BehaviorSubject<ExecutableReferenceStatus>('UNKNOWN');
  public msiAfterburnerStatus: Observable<ExecutableReferenceStatus> =
    this._msiAfterburnerStatus.asObservable();

  constructor(
    private automationConfig: AutomationConfigService,
    private sleep: SleepService,
    private sleep_preparation_service: SleepPreparationService,
    private eventLog: EventLogService
  ) {
    this.msiAfterburnerConfig.subscribe((config) => (this.currentMSIAfterburnerConfig = config));
  }

  async init() {
    // Test MSI Afterburner executable reference
    this.testMSIAfterburnerPathWhenNeeded();
    // Setup sleep based msi afterburner automations
    this.setupMSIAfterburnerProfileSleepAutomations();
  }

  isEnabled(): Observable<boolean> {
    return this.automationConfig.configs.pipe(
      map((configs) => configs.MSI_AFTERBURNER.enabled)
    );
  }

  async enable() {
    await this.automationConfig.updateAutomationConfig<MSIAfterburnerAutomationConfig>(
      'MSI_AFTERBURNER',
      { ...structuredClone(AUTOMATION_CONFIGS_DEFAULT.MSI_AFTERBURNER), enabled: true }
    );
  }

  async disable() {
    await this.automationConfig.updateAutomationConfig<MSIAfterburnerAutomationConfig>(
      'MSI_AFTERBURNER',
      { ...structuredClone(AUTOMATION_CONFIGS_DEFAULT.MSI_AFTERBURNER), enabled: false }
    );
  }

  async setupMSIAfterburnerProfileSleepAutomations() {
    this.sleep.mode
      .pipe(
        // Skip first value from initial load
        skip(1),
        // Only trigger on changes
        distinctUntilChanged(),
        // Check if GPU automations are enabled
        switchMap((sleepModeEnabled) =>
          this.isEnabled().pipe(
            take(1),
            map((gpuAutomationsEnabled) => [gpuAutomationsEnabled, sleepModeEnabled])
          )
        ),
        filter(([gpuAutomationsEnabled]) => gpuAutomationsEnabled),
        // Check profile to be enabled
        map(
          ([, sleepModeEnabled]) =>
            [
              sleepModeEnabled
                ? this.currentMSIAfterburnerConfig.onSleepEnableProfile
                : this.currentMSIAfterburnerConfig.onSleepDisableProfile,
              sleepModeEnabled ? 'SLEEP_MODE_ENABLED' : 'SLEEP_MODE_DISABLED',
            ] as [number, 'SLEEP_MODE_ENABLED' | 'SLEEP_MODE_DISABLED']
        ),
        // Stop if no profile is to be enabled
        filter(([profile]) => profile > 0)
      )
      .subscribe(([profile, reason]) => this.setMSIAfterburnerProfile(profile, reason));
    this.sleep_preparation_service.onSleepPreparation.subscribe(() => {
      this.setMSIAfterburnerProfile(
        this.currentMSIAfterburnerConfig.onSleepPreparation,
        'SLEEP_PREPARATION'
      );
    });
  }

  async testMSIAfterburnerPathWhenNeeded() {
    this.msiAfterburnerConfig
      .pipe(
        startWith(await firstValueFrom(this.msiAfterburnerConfig)),
        pairwise(),
        // Only if the status is still unknown, or if the path was changed (by the user)
        filter(
          () =>
            this._msiAfterburnerStatus.value === 'UNKNOWN'
        ),
        map(([, curr]) => curr.msiAfterburnerPath),
        // Only while one of the profile automations is active (so we don't launch afterburner for nothing)
        switchMap(() =>
          this.msiAfterburnerConfig.pipe(
            filter((config) => !!(config.onSleepEnableProfile || config.onSleepDisableProfile)),
            take(1),
          )
        )
      )
      .subscribe(() => {
        this.testmsi();
      });
  }

  async setMSIAfterburnerProfile(
    index: number,
    reason: 'SLEEP_MODE_ENABLED' | 'SLEEP_MODE_DISABLED' | 'SLEEP_PREPARATION'
  ) {
    if (index < 1 || index > 5) {
      await error(`[GpuAutomations] Attempted to set invalid MSI Afterburner profile (${index})`);
      return;
    }
    if (this._msiAfterburnerStatus.value !== 'SUCCESS') {
      await warn(
        `[GpuAutomations] Could not set MSI Afterburner profile as no valid installation is currently configured`
      );
      return;
    }
    try {
      await invoke<boolean>('msi_afterburner_set_profile', {
        executablePath: this.currentMSIAfterburnerConfig.msiAfterburnerPath,
        profile: index,
      });
      this.eventLog.logEvent({
        type: 'msiAfterburnerProfileSet',
        profile: index,
        reason,
      } as EventLogMsiAfterburnerProfileSet);
    } catch (e) {
      if (typeof e === 'string') {
        this.handleMSIAfterburnerError(e);
      } else {
        error('[GpuAutomations] Failed to set MSI Afterburner profile: ' + e);
        this._msiAfterburnerStatus.next('UNKNOWN_ERROR');
      }
      return;
    }
  }

  async testmsi() {
    this._msiAfterburnerStatus.next('CHECKING');
    // Try running it
    try {
      await invoke<boolean>('msi_afterburner_set_profile', {
        profile: 0, // Profile 0 for testing without actually setting a profile
      });
    } catch (e) {
      if (typeof e === 'string') {
        this.handleMSIAfterburnerError(e);
      } else {
        error('[GpuAutomations] Failed to set MSI Afterburner path: ' + e);
        this._msiAfterburnerStatus.next('UNKNOWN_ERROR');
      }
      return;
    }
    this._msiAfterburnerStatus.next('SUCCESS');
  }

  async handleMSIAfterburnerError(e: string) {
    switch (e) {
      case 'ExeNotFound':
        this._msiAfterburnerStatus.next('NOT_FOUND');
        break;
      case 'ExeCannotExecute':
      case 'ExeUnverifiable':
        this._msiAfterburnerStatus.next('INVALID_EXECUTABLE');
        break;
      case 'ExeNotSigned':
      case 'ExeSignatureDisallowedNonEmbedded':
      case 'ExeSignatureDisallowedNoIssuer':
      case 'ExeSignatureDisallowedNoSubject':
      case 'ExeSignatureDisallowedNoMatch':
        this._msiAfterburnerStatus.next('INVALID_SIGNATURE');
        break;
      // Should never happen
      case 'InvalidProfileIndex':
      case 'UnknownError':
      default:
        this._msiAfterburnerStatus.next('UNKNOWN_ERROR');
        break;
    }
  }

  async setMSIAfterburnerProfileOnSleepEnable(number: number) {
    await this.automationConfig.updateAutomationConfig<MSIAfterburnerAutomationConfig>(
      'MSI_AFTERBURNER',
      { onSleepEnableProfile: number }
    );
  }

  async setMSIAfterburnerProfileOnSleepDisable(number: number) {
    await this.automationConfig.updateAutomationConfig<MSIAfterburnerAutomationConfig>(
      'MSI_AFTERBURNER',
      { onSleepDisableProfile: number }
    );
  }
  async setMSIAfterburnerProfileOnSleepPreparation(number: number) {
    await this.automationConfig.updateAutomationConfig<MSIAfterburnerAutomationConfig>(
      'MSI_AFTERBURNER',
      { onSleepPreparation: number }
    );
  }
}
