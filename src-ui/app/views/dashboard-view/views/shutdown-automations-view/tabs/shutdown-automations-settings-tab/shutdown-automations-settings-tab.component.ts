import { Component, DestroyRef, OnInit } from '@angular/core';
import {
  ConfirmModalComponent,
  ConfirmModalInputModel,
  ConfirmModalOutputModel,
} from '../../../../../../components/confirm-modal/confirm-modal.component';
import { filter } from 'rxjs';
import { ShutdownAutomationsService } from '../../../../../../services/shutdown-automations.service';
import { ModalService } from '../../../../../../services/modal.service';
import {
  AUTOMATION_CONFIGS_DEFAULT,
  PowerDownSystemMode,
  ShutdownAutomationsConfig,
} from '../../../../../../models/automations';

import { takeUntilDestroyed } from '@angular/core/rxjs-interop';
import { AutomationConfigService } from '../../../../../../services/automation-config.service';
import { AppSettingsService } from '../../../../../../services/app-settings.service';
import { QuitWithVRMode } from '../../../../../../models/settings';
import { SelectBoxItem } from '../../../../../../components/select-box/select-box.component';
import { Router } from '@angular/router';
import { fade, vshrink } from '../../../../../../utils/animations';
import { DeviceSelection } from 'src-ui/app/models/device-manager';

@Component({
  selector: 'app-shutdown-automations-settings-tab',
  templateUrl: './shutdown-automations-settings-tab.component.html',
  styleUrls: ['./shutdown-automations-settings-tab.component.scss'],
  animations: [fade(), vshrink()],
  standalone: false,
})
export class ShutdownAutomationsSettingsTabComponent implements OnInit {
  protected config: ShutdownAutomationsConfig = structuredClone(
    AUTOMATION_CONFIGS_DEFAULT.SHUTDOWN_AUTOMATIONS
  );
  protected quitWithVRMode: QuitWithVRMode = 'DISABLED';
  protected lighthouseControlDisabled = false;
  protected powerDownOptions: SelectBoxItem[] = [
    {
      id: 'SHUTDOWN',
      label: 'shutdown-automations.sequence.powerDownSystem.options.SHUTDOWN',
    },
    {
      id: 'SLEEP',
      label: 'shutdown-automations.sequence.powerDownSystem.options.SLEEP',
    },
    {
      id: 'HIBERNATE',
      label: 'shutdown-automations.sequence.powerDownSystem.options.HIBERNATE',
    },
    {
      id: 'REBOOT',
      label: 'shutdown-automations.sequence.powerDownSystem.options.REBOOT',
    },
    {
      id: 'LOGOUT',
      label: 'shutdown-automations.sequence.powerDownSystem.options.LOGOUT',
    },
  ];
  protected powerDownOption: SelectBoxItem | undefined;

  constructor(
    private shutdownAutomations: ShutdownAutomationsService,
    private modalService: ModalService,
    private destroyRef: DestroyRef,
    private automationConfigs: AutomationConfigService,
    private settingsService: AppSettingsService,
    private router: Router
  ) {}

  ngOnInit() {
    this.automationConfigs.configs
      .pipe(takeUntilDestroyed(this.destroyRef))
      .subscribe((configs) => {
        this.config = configs.SHUTDOWN_AUTOMATIONS;
        this.powerDownOption = this.powerDownOptions.find(
          (o) => o.id === configs.SHUTDOWN_AUTOMATIONS.powerDownSystemMode
        );
      });
    this.settingsService.settings
      .pipe(takeUntilDestroyed(this.destroyRef))
      .subscribe((settings) => {
        this.lighthouseControlDisabled = !settings.lighthousePowerControl;
        this.quitWithVRMode = settings.quitWithVR;
      });
  }

  protected runSequence() {
    this.modalService
      .addModal<ConfirmModalInputModel, ConfirmModalOutputModel>(ConfirmModalComponent, {
        title: 'shutdown-automations.confirm-modal.title',
        message: 'shutdown-automations.confirm-modal.message',
      })
      .pipe(filter((result) => !!result?.confirmed))
      .subscribe(() => this.shutdownAutomations.runSequence('MANUAL'));
  }

  get noOptionsSelected() {
    return (
      !this.config.quitVR &&
      this.config.turnOffDevices.devices.length === 0 &&
      this.config.turnOffDevices.types.length === 0 &&
      this.config.turnOffDevices.tagIds.length === 0 &&
      !this.config.powerDownSystem
    );
  }

  async toggleQuitVR() {
    await this.automationConfigs.updateAutomationConfig<ShutdownAutomationsConfig>(
      'SHUTDOWN_AUTOMATIONS',
      {
        quitVR: !this.config.quitVR,
      }
    );
  }

  async onChangeDeviceSelection(selection: DeviceSelection) {
    await this.automationConfigs.updateAutomationConfig<ShutdownAutomationsConfig>(
      'SHUTDOWN_AUTOMATIONS',
      {
        turnOffDevices: selection,
      }
    );
  }

  async togglePowerDownSystem() {
    await this.automationConfigs.updateAutomationConfig<ShutdownAutomationsConfig>(
      'SHUTDOWN_AUTOMATIONS',
      {
        powerDownSystem: !this.config.powerDownSystem,
      }
    );
  }

  onChangePowerDownOption(option: SelectBoxItem | undefined) {
    if (!option) return;
    this.automationConfigs.updateAutomationConfig<ShutdownAutomationsConfig>(
      'SHUTDOWN_AUTOMATIONS',
      {
        powerDownSystemMode: option!.id as PowerDownSystemMode,
      }
    );
  }

  goToGeneralSettings() {
    this.router.navigate(['dashboard', 'settings', 'general']);
  }
}
