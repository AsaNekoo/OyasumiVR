export type OSCValueType = OSCValueTypeE.Int |  OSCValueTypeE.Float |  OSCValueTypeE.String |  OSCValueTypeE.Boolean;
export enum OSCValueTypeE{
  Int=1,
  Float=2,
  Boolean=3,
  String=4,
  // unsupported=5
}

export interface OSCValueRaw {
  kind: OSCValueType;
  value: string;
}

export interface OSCMessageRaw {
  address: string;
  values: OSCValueRaw[];
}

export interface OSCValue {
  kind: OSCValueType;
}

export interface OSCIntValue extends OSCValue {
  kind: OSCValueTypeE.Int;
  value: number;
}

export interface OSCFloatValue extends OSCValue {
  kind: OSCValueTypeE.Float;
  value: number;
}

export interface OSCStringValue extends OSCValue {
  kind: OSCValueTypeE.String;
  value: string;
}

export interface OSCBoolValue extends OSCValue {
  kind: OSCValueTypeE.Boolean;
  value: boolean;
}

// export interface OSCUnsupportedValue extends OSCValue {
//   kind: OSCValueTypeE.unsupported;
// }

export interface OSCMessage {
  address: string;
  values: OSCValue[];
}

export function parseOSCMessage(message: OSCMessageRaw): OSCMessage {
  return {
    address: message.address,
    values: message.values.map(parseOSCValue),
  };
}

export function parseOSCValue(value: OSCValueRaw): OSCValue {
  let parsedValue: unknown;
  switch (value.kind) {
    case OSCValueTypeE.Int:
      parsedValue = parseInt(value.value);
      break;
    case OSCValueTypeE.Float:
      parsedValue = parseFloat(value.value);
      break;
    case OSCValueTypeE.String:
      parsedValue = value.value;
      break;
    case OSCValueTypeE.Boolean:
      parsedValue = value.value === 'true';
      break;
    // case OSCValueTypeE.unsupported:
    //   parsedValue = undefined;
    //   break;
  }
  return {
    kind: value.kind,
    value: parsedValue,
  } as OSCValue;
}
