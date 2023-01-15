// General config
export const STRICT_MODE: boolean = true;
export const LOG_LEVEL: "trace" | "none" = "trace";

const CURRENT_URL = new URL(document.URL);

const SSL = CURRENT_URL.protocol === "http:" ? "" : "s";

const IS_LOCAL = SSL !== "s";

const HOSTNAME = CURRENT_URL.hostname;

const PORT = 8080;

const OPTIONAL_PORT = IS_LOCAL ? ":" + PORT : "";

// API config
export const GAME_CONFIG_URI: string =
  "http" + SSL + "://" + HOSTNAME + OPTIONAL_PORT + "/api/game-config";

export const WEBSOCKET_URI: string =
  "ws" + SSL + "://" + HOSTNAME + OPTIONAL_PORT + "/api/game";
