/** Handlers for user input (keyboard and mouse) */

import { BodyRelative, Position } from "../utility/types";
import { screenPosToMapPos, GAME_CONFIG } from "./camera";
import { SafeSend } from "./connection";

export interface DirectionHandlers {
  up: () => void;
  left: () => void;
  right: () => void;
  down: () => void;
}

export interface GameInputState {
  enabled: boolean;
}

const isTouchStart = (e: MouseEvent | TouchEvent): e is TouchEvent => {
  return e.type === "touchstart";
};

export const addInputListeners = (
  gameCanvas: HTMLCanvasElement,
  updateHoverMenuPosition: (x: number, y: number) => void,
  safeSend: SafeSend,
  gameInputState: GameInputState
) => {
  const processTileSelectEvent = (e: MouseEvent | TouchEvent) => {
    const rect = gameCanvas.getBoundingClientRect();

    let xPixel: number;
    let yPixel: number;

    if (isTouchStart(e)) {
      xPixel = e.touches[0].clientX - rect.left;
      yPixel = e.touches[0].clientY - rect.top;
    } else {
      xPixel = e.clientX - rect.left;
      yPixel = e.clientY - rect.top;
    }

    const pixelPos: Position = { x: xPixel, y: yPixel };

    const screenPos: Position = {
      x: Math.trunc(pixelPos.x / GAME_CONFIG.TILE_SIZE),
      y: Math.trunc(pixelPos.y / GAME_CONFIG.TILE_SIZE),
    };

    const mapPos = screenPosToMapPos(screenPos);

    return { mapPos, pixelPos };
  };

  gameCanvas.onmousemove = (e) => {
    const { mapPos, pixelPos } = processTileSelectEvent(e);
    updateHoverMenuPosition(pixelPos.x, pixelPos.y);
    safeSend({ type: "tileHover", content: mapPos });
  };

  const onTileSelect = (e: MouseEvent | TouchEvent) => {
    const { mapPos } = processTileSelectEvent(e);
    safeSend({ type: "tileClick", content: mapPos });
  };

  gameCanvas.onmousedown = onTileSelect;
  gameCanvas.ontouchstart = onTileSelect;

  const sendKey = (key: BodyRelative) => {
    safeSend({ type: "keypress", content: key });
  };

  const directionHandlers: DirectionHandlers = {
    up: () => sendKey(BodyRelative.Up),
    left: () => sendKey(BodyRelative.Left),
    right: () => sendKey(BodyRelative.Right),
    down: () => sendKey(BodyRelative.Down),
  };

  // Registers a key handler on the main window for
  // any keys supported by the game
  window.addEventListener("keydown", (e) => {
    e.preventDefault();
    if (gameInputState.enabled === false) {
      return;
    }

    switch (e.key) {
      case "ArrowUp":
        directionHandlers.up();
        break;
      case "ArrowRight":
        directionHandlers.right();
        break;
      case "ArrowDown":
        directionHandlers.down();
        break;
      case "ArrowLeft":
        directionHandlers.left();
        break;
    }
  });

  return directionHandlers;
};
