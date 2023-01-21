/** Handles everything related to the actual game canvas rendering
 * and primary consuming of the Pixi.js library */

import {
  Application,
  Assets,
  BaseTexture,
  SCALE_MODES,
  Sprite,
  Texture,
} from "pixi.js";
import { STRICT_MODE } from "../utility/config";
import {
  Dimensions2d,
  EntityIndex,
  Position,
  SpriteTexture,
  SpriteUpdate,
} from "../utility/types";
import { log } from "../utility/functions";
import { CAMERA_SIZE, mapPosToScreenPos, SPRITE_SCALE } from "./camera";

export interface SpritePosition {
  sprite: Sprite;
  pos: Position;
  texture: SpriteTexture;
  remove: () => void;
}

export const spriteMap: Map<EntityIndex["idx"], SpritePosition> = new Map();

export const clearEverything = () => {
  for (const [entityIndex, spritePosition] of spriteMap) {
    spritePosition.remove();
  }

  spriteMap.clear();
};

/** Assert that a sprite exists in the render data map and return it, throw error otherwise. */
const getSpritePositionUnsafe = (entityIndex: EntityIndex): SpritePosition => {
  const spritePosition = spriteMap.get(entityIndex.idx);
  if (STRICT_MODE && spritePosition === undefined) {
    console.error("sprite index", entityIndex.idx);
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

  const bunny = (await Assets.load("sprites/character/zilla_1.png")) as Texture;
  const carrot = (await Assets.load("sprites/character/test_1.png")) as Texture;
  const wall = (await Assets.load("sprites/environment/brick.png")) as Texture;
  const concrete1 = (await Assets.load(
    "sprites/environment/concrete_1.png"
  )) as Texture;
  const concrete2 = (await Assets.load(
    "sprites/environment/concrete_2.png"
  )) as Texture;
  const concrete3 = (await Assets.load(
    "sprites/environment/concrete_3.png"
  )) as Texture;

  bunny.baseTexture.scaleMode = SCALE_MODES.NEAREST;
  carrot.baseTexture.scaleMode = SCALE_MODES.NEAREST;
  wall.baseTexture.scaleMode = SCALE_MODES.NEAREST;
  concrete1.baseTexture.scaleMode = SCALE_MODES.NEAREST;
  concrete2.baseTexture.scaleMode = SCALE_MODES.NEAREST;
  concrete3.baseTexture.scaleMode = SCALE_MODES.NEAREST;

  // bunny.scaleMode = SCALE_MODES.LINEAR;

  const TEXTURE_MAP: Record<SpriteTexture, Texture> = {
    [SpriteTexture.Bunny]: bunny,
    [SpriteTexture.Carrot]: carrot,
    [SpriteTexture.Wall]: wall,
    [SpriteTexture.FloorConcrete]: concrete1,
  };

  const tileToPx = (tilePos: Position): Position => {
    return {
      x: tilePos.x * tileSize + halfTile,
      y: tilePos.y * tileSize + halfTile,
    };
  };

  /** Move a sprite to another position on the canvas */
  const setSpritePosition = (spriteUpdate: SpriteUpdate) => {
    // log.trace("Setting sprite position", newEntityPosition);
    const spritePos = getSpritePositionUnsafe(spriteUpdate.entity);
    const { sprite } = spritePos;

    // Change the actual entity position independent of the camera
    spritePos.pos = spriteUpdate.pos;

    const screenPos = mapPosToScreenPos(spriteUpdate.pos);

    if (
      screenPos.x < 0 ||
      screenPos.x >= CAMERA_SIZE ||
      screenPos.y < 0 ||
      screenPos.y >= CAMERA_SIZE
    ) {
      sprite.visible = false;
    } else {
      sprite.visible = true;
      const pxPos = tileToPx(screenPos);
      sprite.x = pxPos.x;
      sprite.y = pxPos.y;
    }
  };

  /** Add a new sprite to the game canvas if it doesn't exist,
   * if it does exist it will update its position */
  const addSprite = (spriteUpdate: SpriteUpdate) => {
    // Only add the sprite if it doesn't already exist
    if (spriteMap.get(spriteUpdate.entity.idx) === undefined) {
      // log.trace(
      //   "Adding sprite for player",
      //   entityRenderData.entityPosition.entityIndex
      // );

      const randomElement = <T>(arr: T[]): T =>
        arr[Math.floor(Math.random() * arr.length)];

      const getSprite = (): Sprite => {
        if (spriteUpdate.sprite == SpriteTexture.FloorConcrete) {
          return new Sprite(randomElement([concrete1, concrete2, concrete3]));
        } else {
          return new Sprite(TEXTURE_MAP[spriteUpdate.sprite]);
        }
      };

      const newSprite = getSprite();

      newSprite.anchor.x = 0.5;
      newSprite.anchor.y = 0.5;

      // newSprite.scaleMode

      newSprite.scale = { x: SPRITE_SCALE, y: SPRITE_SCALE };

      // Each frame we spin the sprite around in circles just for shits
      // const ticker = app.ticker.add(() => {
      //   newSprite.rotation += 0.05;
      // });

      app.stage.addChild(newSprite);

      spriteMap.set(spriteUpdate.entity.idx, {
        pos: spriteUpdate.pos,
        sprite: newSprite,
        texture: spriteUpdate.sprite,
        remove: () => {
          app.stage.removeChild(newSprite);
          // newSprite.destroy();
          // ticker.destroy();
        },
      });
    } else {
      log.trace("Existing sprite found for player", spriteUpdate.entity.idx);
    }

    setSpritePosition(spriteUpdate);
  };

  /** Remove a sprite from the game canvas */
  const removeSprite = (entityIndex: EntityIndex) => {
    log.trace("Removing sprite for player", entityIndex.idx);
    const spritePosition = getSpritePositionUnsafe(entityIndex);
    app.stage.removeChild(spritePosition.sprite);
  };

  const gameCanvas = app.view as HTMLCanvasElement;

  // app.rend

  // app.stage.scale.set(7);

  return { addSprite, removeSprite, setSpritePosition, gameCanvas };
};
