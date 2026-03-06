import { Component, OnInit } from '@angular/core';
import { VRService } from './services/openvr.service';
import { routeAnimations } from './app-routing.module';
import { TranslateService } from '@ngx-translate/core';
import { AppSettingsService } from './services/app-settings.service';
import { takeUntilDestroyed } from '@angular/core/rxjs-interop';
import { debounceTime, distinctUntilChanged, map, skip, tap } from 'rxjs';
import { fade } from './utils/animations';
import { isHolidaysEventActive } from './utils/event-utils';

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.scss'],
  animations: [routeAnimations, fade()],
  standalone: false,
})
export class AppComponent implements OnInit {
  showSnowverlay = false;

  constructor(
    public openvr: VRService,
    translate: TranslateService,
    private settings: AppSettingsService
  ) {
    this.settings.settings
      .pipe(
        takeUntilDestroyed(),
        map((settings) => settings.userLanguage),
        distinctUntilChanged(),
        tap((userLanguage) => translate.use(userLanguage)),
        debounceTime(10000)
      )
      .subscribe();
    // Snowverlay
    this.settings.settings
      .pipe(
        takeUntilDestroyed(),
        skip(1),
        map((settings) => settings.hideSnowverlay),
        distinctUntilChanged()
      )
      .subscribe((hideSnowverlay) => {
        this.showSnowverlay = !hideSnowverlay && isHolidaysEventActive();
      });
  }

  async ngOnInit(): Promise<void> {}
}
