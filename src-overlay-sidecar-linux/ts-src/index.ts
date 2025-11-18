//address will be replaced at runtime
const ws = new WebSocket('ws://WS_ADDR');
let seq_counter = 0;
enum FuntionCalls {
  setSleepMpde = 0,
  onUiReady = 1,
  syncState = 2,
  sendEventVoid = 3,
  sendEventString = 4,
  sendEventBool = 5,
  sendEventInt = 6,
  sendEventDouble = 7,
  sendEventJson = 8,
  sendEvent = 9,
  addNotification = 10,
  showToolTip = 11,
  dispose = 12,
  getDebugTranslations = 13,
  close = 14,
}

const OyasumiOverlayIPCOut = {
  setSleepMode: async function (enabled: boolean): Promise<void> {
    rpc(FuntionCalls.setSleepMpde, { enabled: enabled });
  },
  onUiReady: async function (): Promise<void> {
    return rpc(FuntionCalls.onUiReady, {});
  },
  syncState: async function (): Promise<void> {
    rpc(FuntionCalls.syncState, {});
  },
  sendEventVoid: async function (eventName: string): Promise<void> {
    rpc(FuntionCalls.sendEventVoid, { eventName: eventName });
  },
  sendEventString: async function (eventName: string, data: string): Promise<void> {
    rpc(FuntionCalls.sendEventString, { eventName: eventName, data: data });
  },
  sendEventBool: async function (eventName: string, data: boolean): Promise<void> {
    rpc(FuntionCalls.sendEventBool, { eventName: eventName, data: data });
  },
  sendEventInt: async function (eventName: string, data: number): Promise<void> {
    rpc(FuntionCalls.sendEventInt, { eventName: eventName, data: data });
  },
  sendEventDouble: async function (eventName: string, data: number): Promise<void> {
    rpc(FuntionCalls.sendEventDouble, { eventName: eventName, data: data });
  },
  sendEventJson: async function (eventName: string, data: string): Promise<void> {
    rpc(FuntionCalls.sendEventJson, { eventName: eventName, data: data });
  },
  sendEvent:async function(eventName:string,data: string | boolean | number):Promise<void>{
        if (typeof data === "boolean"){
            this.sendEventBool(eventName,data);
        }else  if (typeof data === "string"){
            this.sendEventString(eventName,data);
        }else  if (typeof data === "number"){
            this.sendEventInt(eventName,data);
        }
  },

  addNotification: async function (message: string, duration: number): Promise<string | null> {
    return rpc_ret(FuntionCalls.addNotification, { message: message, duration: duration });
  },
  showToolTip: async function (tooltip: string | null): Promise<void> {
    rpc(FuntionCalls.showToolTip, { tooltip: tooltip });
  },
  dispose: async function (): Promise<void> {
    rpc(FuntionCalls.dispose, {});
  },
  getDebugTranslations: async function (): Promise<string> {
    return rpc_ret(FuntionCalls.getDebugTranslations, {});
  },
};
const OyasumiOverlayIPCOut_Dashboard = {
  close: async function (): Promise<void> {
    rpc(FuntionCalls.close, {});
  },
};
const CefSharp = {
  //no op
  BindObjectAsync: async function (_: any): Promise<void> {},
};
//ai generated code below, i read the documentation but i just couldn't process that promise gives u a callback to call when u finish
//and what in a pointer hell u mean it's possible to store callbacks to funtions with different signatures in a single map
const pending = new Map<number, (data: any) => any>();
ws.onmessage = (event) => {
  const data = JSON.parse(event.data);
  const resolver = pending.get(data.seq);
  if (resolver) {
    resolver(data);
    pending.delete(data.seq);
  }
};

function rpc_ret(id: FuntionCalls, payload: any): Promise<any> {
  return new Promise((resolve) => {
    const seq = seq_counter;
    seq_counter += 1;
    pending.set(seq, resolve);
    ws.send(id.toString() + ':' + JSON.stringify({ seq: seq, ...payload }));
  });
}
function rpc(id: number, payload: any):Promise<void> {
  ws.send(id.toString() + ':' + JSON.stringify(payload));
  return new Promise<void>((resolve) => {
    resolve()
  });
}
window.CefSharp = CefSharp;
window.OyasumiIPCOut = OyasumiOverlayIPCOut;
window.OyasumiIPCOut_Dashboard = OyasumiOverlayIPCOut_Dashboard;
