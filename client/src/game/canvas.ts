/** Handles everything related to the actual game canvas rendering
 * and primary consuming of the Pixi.js library */

import { Application, Assets, Sprite, Texture, Ticker } from "pixi.js";
import { STRICT_MODE } from "../utility/config";
import {
  Dimensions2d,
  EntityIndex,
  EntityPosition,
  EntityRenderData,
  Position,
  SpriteTexture,
} from "../utility/types";
import { log } from "../utility/functions";
import { camera, CAMERA_SIZE } from "./camera";

export interface SpritePosition {
  sprite: Sprite;
  pos: Position;
  texture: SpriteTexture;
}

export const spriteMap = new Map<EntityIndex["index"], SpritePosition>();

/** Assert that a sprite exists in the render data map and return it, throw error otherwise. */
const getSpritePositionUnsafe = (entityIndex: EntityIndex): SpritePosition => {
  const spritePosition = spriteMap.get(entityIndex.index);
  if (STRICT_MODE && spritePosition === undefined) {
    console.error("sprite index", entityIndex.index);
    console.error(spriteMap);
    throw Error("Tried to get a non-existent sprite");
  }
  return spritePosition as SpritePosition;
};

export const createGameApp = async (
  dimensions: Dimensions2d,
  tileSize: number
) => {
  if (tileSize % 2 !== 0) {
    console.error("Tile size must be an even number", tileSize);
    throw Error;
  }
  const halfTile = tileSize / 2;
  const app = new Application({
    width: dimensions.width * tileSize,
    height: dimensions.height * tileSize,
  });

  const bunny = await Assets.load("bunny.png");
  const carrot = await Assets.load("carrot.png");
  const wall = await Assets.load("wall.jpg");

  const TEXTURE_MAP: Record<SpriteTexture, Texture> = {
    [SpriteTexture.Bunny]: bunny,
    [SpriteTexture.Carrot]: carrot,
    [SpriteTexture.Wall]: wall,
  };

  // const pxToTile = (pxPos: Position): Position => {
  //   return {
  //     x: (pxPos.x - halfTile) / tileSize,
  //     y: (pxPos.y - halfTile) / tileSize,
  //   };
  // };

  const tileToPx = (tilePos: Position): Position => {
    return {
      x: tilePos.x * tileSize + halfTile,
      y: tilePos.y * tileSize + halfTile,
    };
  };

  /** Move a sprite to another position on the canvas */
  const setSpritePosition = (newEntityPosition: EntityPosition) => {
    // log.trace("Setting sprite position", newEntityPosition);
    const spritePos = getSpritePositionUnsafe(newEntityPosition.entityIndex);
    const { sprite } = spritePos;

    // Change the actual entity position independent of the camera
    spritePos.pos = newEntityPosition.pos;

    const cameraPos: Position = {
      x: newEntityPosition.pos.x - camera.x,
      y: newEntityPosition.pos.y - camera.y,
    };

    if (
      cameraPos.x < 0 ||
      cameraPos.x >= CAMERA_SIZE ||
      cameraPos.y < 0 ||
      cameraPos.y >= CAMERA_SIZE
    ) {
      sprite.visible = false;
    } else {
      sprite.visible = true;
      const pxPos = tileToPx(cameraPos);
      sprite.x = pxPos.x;
      sprite.y = pxPos.y;
    }
  };

  /** Add a new sprite to the game canvas if it doesn't exist,
   * if it does exist it will update its position */
  const addSprite = (entityRenderData: EntityRenderData) => {
    // Only add the sprite if it doesn't already exist
    if (
      spriteMap.get(entityRenderData.entityPosition.entityIndex.index) ===
      undefined
    ) {
      // log.trace(
      //   "Adding sprite for player",
      //   entityRenderData.entityPosition.entityIndex
      // );
      const newSprite = new Sprite(TEXTURE_MAP[entityRenderData.sprite]);
      spriteMap.set(entityRenderData.entityPosition.entityIndex.index, {
        pos: entityRenderData.entityPosition.pos,
        sprite: newSprite,
        texture: entityRenderData.sprite,
      });

      newSprite.anchor.x = 0.5;
      newSprite.anchor.y = 0.5;

      // Each frame we spin the sprite around in circles just for shits
      const ticker = app.ticker.add(() => {
        newSprite.rotation += 0.05;
      });

      app.stage.addChild(newSprite);
    } else {
      log.trace(
        "Existing sprite found for player",
        entityRenderData.entityPosition.entityIndex.index
      );
    }

    setSpritePosition(entityRenderData.entityPosition);
  };

  /** Remove a sprite from the game canvas */
  const removeSprite = (entityIndex: EntityIndex) => {
    log.trace("Removing sprite for player", entityIndex.index);
    const spritePosition = getSpritePositionUnsafe(entityIndex);
    app.stage.removeChild(spritePosition.sprite);
  };

  const gameCanvas = app.view as HTMLCanvasElement;

  return { addSprite, removeSprite, setSpritePosition, gameCanvas };
};
