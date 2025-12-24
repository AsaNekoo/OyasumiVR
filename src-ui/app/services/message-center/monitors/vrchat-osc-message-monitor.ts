import { inject } from '@angular/core';
import { VRChatService } from '../../vrchat-api/vrchat.service';
import { MessageMonitor } from './message-monitor';
import { OscService } from '../../osc.service';
import { combineLatest, debounceTime, distinctUntilChanged, map } from 'rxjs';
import { openUrl } from '@tauri-apps/plugin-opener';

export class VRChatOSCMessageMonitor extends MessageMonitor {
  private vrchat = inject(VRChatService);
  private osc = inject(OscService);

  public override async init(): Promise<void> {
    combineLatest([
      this.osc.vrchatOscAddress.pipe(distinctUntilChanged()),
      this.osc.vrchatOscQueryAddress.pipe(distinctUntilChanged()),
    ])
      .pipe(
        map(
          ([oscAddress, oscQueryAddress]) =>
            (!oscAddress || !oscQueryAddress)
        ),
        debounceTime(2000),
        distinctUntilChanged()
      )
      .subscribe((issueFound) => {
        if (issueFound) {
          this.messageCenter.addMessage({
            id: 'vrcOscMissing',
            title: 'message-center.messages.vrcOscMissing.title',
            message: 'message-center.messages.vrcOscMissing.message',
            hideable: true,
            actions: [
              {
                label: 'message-center.actions.moreInfo',
                action: () => {
                  openUrl(
                    'https://raphii.co/oyasumivr/hidden/troubleshooting/vrchat-osc-unavailable/'
                  );
                },
              },
            ],
            type: 'warning',
          });
        } else {
          this.messageCenter.removeMessage('vrcOscMissing');
        }
      });
  }
}
