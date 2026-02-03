import { Component, HostBinding, OnInit } from '@angular/core';
import { BaseModalComponent } from 'src-ui/app/components/base-modal/base-modal.component';
import { fade, fadeUp, triggerChildren, vshrink } from '../../../../../../utils/animations';
import { TranslateService } from '@ngx-translate/core';
import { getStringForDuration, getStringForDurationAwake } from '../../tabs/sleep-detection-tab.component';

export interface DurationDisableSleepModeModalInputModel {
  duration: string | null;
  awake: string | null;
}

export interface DurationDisableSleepModeModalOutputModel {
  duration: string | null;
  awake: string | null;
}

@Component({
  selector: 'app-duration-disable-sleepmode-modal',
  templateUrl: './duration-disable-sleep-mode-modal.component.html',
  styleUrls: ['./duration-disable-mode-modal.component.scss'],
  animations: [fadeUp(), fade(), triggerChildren(), vshrink()],
  standalone: false,
})
export class DurationDisableSleepModeModalComponent
  extends BaseModalComponent<
    DurationDisableSleepModeModalInputModel,
    DurationDisableSleepModeModalOutputModel
  >
  implements OnInit, DurationDisableSleepModeModalInputModel
{
  duration: string | null = null;
  awake: string | null = null;

  @HostBinding('[@fadeUp]') get fadeUp() {
    return;
  }

  constructor(private translate: TranslateService) {
    super();
  }

  ngOnInit(): void {
    if (this.duration && this.duration.length == 4) {
      this.duration = '0' + this.duration;
    }
    if (this.awake && this.awake.length == 4) {
      this.awake = '0' + this.awake;
    }
    if (!this.duration || !this.duration.match(/[0-2][0-9]:[0-5][0-9]/g)) {
      console.warn('mallformed duration:' + this.duration);
      this.duration = '00:00';
    }
    if (!this.awake || !this.awake.match(/[0-2][0-9]:[0-5][0-9]/g)) {
      console.warn('mallformed awake time:' + this.awake);
      this.awake = '00:00';
    }
  }

  save() {
    this.result = this;
    this.close();
  }

  protected getStringForDuration(duration: string | null) {
    if (!duration) {
      return '';
    }
    return getStringForDuration(this.translate, duration);
  }
  protected getStringForDurationAwake(awake: string | null) {
    if (!awake) {
      return '';
    }
    return getStringForDurationAwake(this.translate, awake);
  }
}
