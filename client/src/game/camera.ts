import { Log } from "../components";
import { log } from "../utility/functions";
import { Dimensions2d, EntityPosition, Position } from "../utility/types";

// Game config
export const TILE_SIZE = 24;

export const CAMERA_RADIUS = 3;
export const CAMERA_SIZE = CAMERA_RADIUS * 2 + 1;
// const CAMERA_MIN = 0;
// const CAMERA_MAX = MAP_SIZE - CAMERA_DIAMETER;

// export const SCREEN_DIMENSIONS: Dimensions2d = { width: CAMERA_RADIUS * 2 + 1 , height: 6 };

// if (SCREEN_DIMENSIONS.width % 2 !== 0 || SCREEN_DIMENSIONS.height % 2 !== 0) {
//   throw Error("SCREEN_DIMENSIONS must be even values.");
// }

export interface Camera {
  x: number;
  y: number;
}

export const camera: Camera = { x: 0, y: 0 };

// export const mapToCamera = (mapPos: Position): Position => {
//   return { x: mapPos.x + camera.x, y: mapPos.y + camera.y };
// };

export const setCamera = (pos: Position) => {
  let newX = pos.x - CAMERA_RADIUS;
  let newY = pos.y - CAMERA_RADIUS;

  //   newX = newX < 0 ? 0 : newX;
  //   newY = newY < 0 ? 0 : newY;

  camera.x = newX;
  camera.y = newY;

  log.trace("Camera", camera);
};
