/** Handlers for the websocket connection */

import { WEBSOCKET_URI } from "../utility/config";
import { log } from "../utility/functions";
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

  let keepAlive = setInterval(() => {
    // Keep from getting timeout kicked
    safeSend({ type: "keepAlive" });
    log.trace("Keep alive signal sent");
  }, 1000 * 30);

  ws.onopen = onOpen;
  ws.onclose = () => {
    clearInterval(keepAlive);
    onClose();
  };
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
