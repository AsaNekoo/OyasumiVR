import { EventLogEntryParser } from '../event-log-entry-parser';
import { EventLogType, EventLogSystemPowerPolicySet } from '../../../../models/event-log-entry';

export class EventLogSystemPowerPolicySetEntryParser extends EventLogEntryParser<EventLogSystemPowerPolicySet> {
  entryType(): EventLogType {
    return 'SystemPowerPolicySet';
  }

  override headerInfoTitle(): string {
    return 'comp.event-log-entry.type.SystemPowerPolicySet.title';
  }

  override headerInfoTitleParams(entry: EventLogSystemPowerPolicySet): { [s: string]: string } {
    return {
      policy: entry.policyName,
    };
  }

  override headerInfoSubTitle(entry: EventLogSystemPowerPolicySet): string {
    return 'comp.event-log-entry.type.SystemPowerPolicySet.reason.' + entry.reason;
  }
}
