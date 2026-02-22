import { Injectable } from '@angular/core';
import { invoke } from '@tauri-apps/api/core';
import { EventLogService } from './event-log.service';
import { EventLogSystemPowerPolicySet } from '../models/event-log-entry';
import { error } from '@tauri-apps/plugin-log';
import { BehaviorSubject } from 'rxjs';

interface SystemPowerPolicy {
  name: string;
}

@Injectable({
  providedIn: 'root',
})
export class WindowsService {
  private _policies = new BehaviorSubject<SystemPowerPolicy[]>([]);
  public readonly policies = this._policies.asObservable();

  constructor(private eventLog: EventLogService) {}

  public async init() {
    await this.getPowerPolicies();
  }

  public async getPowerPolicies() {
    this._policies.next(await invoke<SystemPowerPolicy[]>('get_system_power_policies'));
    return this._policies.value;
  }

  public async setPowerPolicy(
    name: string,
    reason: 'SLEEP_MODE_ENABLED' | 'SLEEP_MODE_DISABLED'|'SLEEP_PREPARATION'
  ): Promise<void> {
    await invoke<void>('set_system_power_policy', { name: name });
    const currentPolicy = await this.getPowerPolicy();
    if (currentPolicy?.name !== name) {
      error(
        `[Windows] Likely failed to set windows power policy: The newly fetched policy does not match the policy that was just set. (Set Policy = ${name}, Actual Policy = ${currentPolicy?.name})`
      );
    } else if (currentPolicy) {
      this.eventLog.logEvent({
        type: 'SystemPowerPolicySet',
        reason,
        policyName: currentPolicy?.name ?? 'Unknown Policy',
      } as EventLogSystemPowerPolicySet);
    }
  }

  public async getPowerPolicy(): Promise<SystemPowerPolicy | undefined> {
    const policy =
      (await invoke<SystemPowerPolicy | null>('active_system_power_policy')) ?? undefined;
    // Update local policy cache
    if (policy) {
      const knownPolicy = this._policies.value.find((p) => p.name === policy.name);
      if (knownPolicy) Object.assign(knownPolicy, policy);
      else this._policies.value.push(policy);
      this._policies.next(this._policies.value);
    }
    return policy;
  }
}
