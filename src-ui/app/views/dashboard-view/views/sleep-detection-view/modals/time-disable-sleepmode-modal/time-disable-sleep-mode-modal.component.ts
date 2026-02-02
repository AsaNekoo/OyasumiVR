import { Component, HostBinding, OnInit } from '@angular/core';
import { BaseModalComponent } from 'src-ui/app/components/base-modal/base-modal.component';
import { fade, fadeUp, triggerChildren } from '../../../../../../utils/animations';

export interface TimeDisableSleepModeModalInputModel {
  time: string | null;
}

export interface TimeDisableSleepModeModalOutputModel {
  time: string | null;
}

@Component({
  selector: 'app-time-disable-sleepmode-modal',
  templateUrl: './time-disable-sleep-mode-modal.component.html',
  styleUrls: ['./time-disable-mode-modal.component.scss'],
  animations: [fadeUp(), fade(), triggerChildren()],
  standalone: false,
})
export class TimeDisableSleepModeModalComponent
  extends BaseModalComponent<
    TimeDisableSleepModeModalInputModel,
    TimeDisableSleepModeModalOutputModel
  >
  implements OnInit, TimeDisableSleepModeModalInputModel
{
  time: string | null = null;

  @HostBinding('[@fadeUp]') get fadeUp() {
    return;
  }

  constructor() {
    super();
  }

  ngOnInit(): void {
    if (!this.time || !this.time.match(/[0-2][0-9]:[0-5][0-9]/g)) {
      this.time = '00:00';
    }
  }

  save() {
    this.result = this;
    this.close();
  }
}
