import { listen } from '@tauri-apps/api/event';
import { invoke } from '@tauri-apps/api/core';
import { Transport } from '@modelcontextprotocol/sdk/shared/transport';
import { JSONRPCMessage, JSONRPCMessageSchema } from '@modelcontextprotocol/sdk/types';


export type ChannleMessage = {
    name: string;
    chunk: string;
};
export type ChannleEvent = {
    payload: ChannleMessage;
    event: string;
    id: number;
}

export class TauriClientTransport implements Transport {    
    private _channelName = "mcpchannel";
    private _startActionName = "mcpstart";
    private _messageActionName = "mcpmessage";
    private _unlisten?: () => void;

    private name: string
    private params: {
        name: string
        command: string
        args: string
    }

    constructor(name: string, command: string, args: string = "") {
        this.name = name
        this.params = {name, command, args}
    }

    onclose?: () => void;
    onerror?: (error: Error) => void;
    onmessage?: (message: JSONRPCMessage) => void;
    start(): Promise<void> {
        return new Promise((resolve, reject) => {
            // 1. 先开启监听
            // 2. 调用 invoke 方法
            listen(this._channelName, (event: ChannleEvent) => {
                console.log(this._channelName, event);
                let message: JSONRPCMessage;
                try {
                    if (event?.payload?.name == this.name) {
                        message = JSONRPCMessageSchema.parse(JSON.parse(event?.payload?.chunk));
                    }
                } catch (error) {
                    this.onerror?.(error as Error);
                    return;
                }
                console.log("onmessage", message);
                this.onmessage?.(message);
            }).then((u) => {
                this._unlisten = u;
            }).then(() => invoke(this._startActionName, this.params).then((response) => {
                console.log(response);
                resolve();
            } ).catch((error) => {
                console.error(error);
                this.close();
            })).catch(reject);
        })
    }
    async close(): Promise<void> {
        invoke("mcpstop", {name: this.name});
        this._unlisten?.();
        this.onclose?.();
    }
    async send(message: JSONRPCMessage): Promise<void> {
        try {
            return invoke(this._messageActionName, {
                name: this.name,
                message: JSON.stringify(message)
            });
        } catch (error) {
          this.onerror?.(error as Error);
          throw error;
        }
      }
}