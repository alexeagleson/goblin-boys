/** Handlers for the websocket connection */

import { WEBSOCKET_URI } from "../utility/config";
import { ClientMessage } from "../utility/types";

type SendResult = "success" | "failure";

export type SafeSend = (request: ClientMessage) => SendResult;

export interface ConnectToGameServerConfig {
  onOpen: () => void;
  onClose: () => void;
  onMessage: (msg: MessageEvent<unknown>) => void;
}

export const connectToGameServer = ({
  onClose,
  onOpen,
  onMessage,
}: ConnectToGameServerConfig): { safeSend: SafeSend } => {
  const ws = new WebSocket(WEBSOCKET_URI);

  ws.onopen = onOpen;
  ws.onclose = onClose;
  ws.onmessage = onMessage;

  const safeSend = (request: ClientMessage): SendResult => {
    if (ws.readyState === ws.OPEN) {
      ws.send(JSON.stringify(request));
      return "success";
    }
    return "failure";
  };

  return { safeSend };
};
