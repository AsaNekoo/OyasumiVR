import { inject } from '@angular/core';
import { MessageMonitor } from './message-monitor';
import { GpuAutomationsService } from '../../gpu-automations.service';
import { ElevatedSidecarService } from '../../elevated-sidecar.service';
import {
  combineLatest,
  map,
} from 'rxjs';
import { AutomationConfigService } from '../../automation-config.service';
import {
  MSIAfterburnerAutomationConfig,
} from 'src-ui/app/models/automations';
import { TString } from 'src-ui/app/models/translatable-string';
import { ExecutableReferenceStatus } from 'src-ui/app/models/settings';
import { Router } from '@angular/router';

export class GpuAutomationMessageMonitor extends MessageMonitor {
  private gpuAutomations = inject(GpuAutomationsService);
  private elevatedSidecar = inject(ElevatedSidecarService);
  private automationConfig = inject(AutomationConfigService);
  private router = inject(Router);

  public override async init(): Promise<void> {
    await Promise.all([
      this.initAfterburnerStatusMonitor(),
    ]);
  }

  private async initAfterburnerStatusMonitor() {
    combineLatest([
      this.gpuAutomations.isEnabled(),
      this.gpuAutomations.msiAfterburnerConfig,
      this.gpuAutomations.msiAfterburnerStatus,
      this.elevatedSidecar.sidecarStarted,
    ])
      .pipe(
        map(
          ([gpuAutomationsEnabled, msiAfterburnerConfig, msiAfterburnerStatus, sidecarRunning]) => {
            if (!gpuAutomationsEnabled || !sidecarRunning) return { error: false };
            if (
              !msiAfterburnerConfig.onSleepEnableProfile &&
              !msiAfterburnerConfig.onSleepDisableProfile &&
              !msiAfterburnerConfig.onSleepPreparation
            )
              return { error: false };
            if (
              (
                [
                  'NOT_FOUND',
                  'INVALID_EXECUTABLE',
                  'INVALID_SIGNATURE',
                  'UNKNOWN_ERROR',
                ] as ExecutableReferenceStatus[]
              ).includes(msiAfterburnerStatus)
            ) {
              return {
                error: true,
                msiAfterburnerStatus,
              };
            }
            return { error: false };
          }
        )
      )
      .subscribe(({ error, msiAfterburnerStatus }) => {
        let message: TString =
          'message-center.messages.gpuAutomationsAfterburnerError.message.UNKNOWN_ERROR';
        if (error) {
          switch (msiAfterburnerStatus) {
            case 'NOT_FOUND':
            case 'INVALID_EXECUTABLE':
            case 'INVALID_SIGNATURE':
            case 'UNKNOWN_ERROR':
              message =
                'message-center.messages.gpuAutomationsAfterburnerError.message.' +
                msiAfterburnerStatus;
              break;
          }
          this.messageCenter.addMessage({
            id: 'gpuAutomationsAfterburnerError',
            title: 'message-center.messages.gpuAutomationsAfterburnerError.title',
            message,
            hideable: false,
            type: 'warning',
            actions: [
              {
                label: 'message-center.messages.gpuAutomationsAfterburnerError.actions.disable',
                action: () => {
                  this.automationConfig.updateAutomationConfig<MSIAfterburnerAutomationConfig>(
                    'MSI_AFTERBURNER',
                    {
                      onSleepEnableProfile: null,
                      onSleepDisableProfile: null,
                    }
                  );
                },
              },
              {
                label: 'message-center.actions.configure',
                action: async () => {
                  this.router.navigate(['/dashboard/gpuAutomations'], {
                    fragment: 'MSI_AFTERBURNER',
                  });
                  this.messageCenter.toggle();
                },
              },
            ],
          });
        } else {
          this.messageCenter.removeMessage('gpuAutomationsAfterburnerError');
          return;
        }
      });
  }

 
}
