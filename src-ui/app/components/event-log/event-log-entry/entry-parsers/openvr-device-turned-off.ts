import { EventLogEntryParser } from '../event-log-entry-parser';
import { EventLogTurnedOffVRDevices, EventLogType } from '../../../../models/event-log-entry';

export class EventLogTurnedOffVRDevicesEntryParser extends EventLogEntryParser<EventLogTurnedOffVRDevices> {
  entryType(): EventLogType {
    return 'turnedOffVRDevices';
  }

  override headerInfoTitle(entry: EventLogTurnedOffVRDevices): string {
    return 'comp.event-log-entry.type.turnedOffVRDevices.title.' + entry.devices;
  }

  override headerInfoSubTitle(entry: EventLogTurnedOffVRDevices): string {
    return 'comp.event-log-entry.type.turnedOffVRDevices.reason.' + entry.reason;
  }

  headerInfoSubTitleParams(entry: EventLogTurnedOffVRDevices): { [p: string]: string } {
    return {
      threshold: entry.batteryThreshold?.toString() ?? '',
    };
  }
}
