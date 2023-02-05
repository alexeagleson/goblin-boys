import { Position } from "../utility/types";

export const GAME_CONFIG = {
  SPRITE_SCALE: 0,
  CAMERA_RADIUS: 0,
  TILE_SIZE: 0,
  CAMERA_SIZE: 0,
};

export const setGameConfig = (spriteScale: number, cameraRadius: number) => {
  GAME_CONFIG.SPRITE_SCALE = spriteScale;
  GAME_CONFIG.CAMERA_RADIUS = cameraRadius;
  GAME_CONFIG.TILE_SIZE = 16 * GAME_CONFIG.SPRITE_SCALE;
  GAME_CONFIG.CAMERA_SIZE = GAME_CONFIG.CAMERA_RADIUS * 2 + 1;
};

// Default values
setGameConfig(3, 7);

export interface Camera {
  x: number;
  y: number;
}

export const camera: Camera = { x: 0, y: 0 };

export const setCamera = (pos: Position) => {
  let newX = pos.x - GAME_CONFIG.CAMERA_RADIUS;
  let newY = pos.y - GAME_CONFIG.CAMERA_RADIUS;

  camera.x = newX;
  camera.y = newY;
};

export const mapPosToScreenPos = (mapPos: Position): Position => {
  return {
    x: mapPos.x - camera.x,
    y: mapPos.y - camera.y,
  };
};

export const screenPosToMapPos = (screenPos: Position): Position => {
  return {
    x: screenPos.x + camera.x,
    y: screenPos.y + camera.y,
  };
};
