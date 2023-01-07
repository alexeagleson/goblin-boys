/** Handles everything related to the actual game canvas rendering
 * and primary consuming of the Pixi.js library */

import { Application, Assets, Sprite, Texture, Ticker } from "pixi.js";
import { STRICT_MODE } from "../utility/config";
import {
  Dimensions2d,
  EntityIndex,
  EntityPositionChange,
  GameEntity,
  SpriteTexture,
} from "../utility/types";
import { log } from "../utility/functions";

const spriteMap = new Map<EntityIndex["index"], Sprite>();

/** Assert that a sprite exists in the sprite map and return it, throw error otherwise. */
const getSpriteUnsafe = (entityIndex: EntityIndex): Sprite => {
  const maybeSprite = spriteMap.get(entityIndex.index);
  if (STRICT_MODE && maybeSprite === undefined) {
    console.error("sprite index", entityIndex.index);
    console.error(spriteMap);
    throw Error("Tried to get a non-existent sprite");
  }
  return maybeSprite as Sprite;
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

  /** Move a sprite to another position on the canvas */
  const setSpritePosition = (entityPositionChange: EntityPositionChange) => {
    log.trace("Setting sprite position", entityPositionChange);
    const sprite = getSpriteUnsafe(entityPositionChange.entityIndex);
    sprite.x = entityPositionChange.pos.x * tileSize + halfTile;
    sprite.y = entityPositionChange.pos.y * tileSize + halfTile;
  };

  /** Add a new sprite to the game canvas if it doesn't exist,
   * if it does exist it will update its position */
  const addSprite = (gameEntity: GameEntity) => {
    // Only add the sprite if it doesn't already exist
    if (
      spriteMap.get(gameEntity.entityPosition.entityIndex.index) === undefined
    ) {
      log.trace(
        "Adding sprite for player",
        gameEntity.entityPosition.entityIndex
      );
      const newSprite = new Sprite(TEXTURE_MAP[gameEntity.sprite]);
      spriteMap.set(gameEntity.entityPosition.entityIndex.index, newSprite);

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
        gameEntity.entityPosition.entityIndex.index
      );
    }

    setSpritePosition(gameEntity.entityPosition);
  };

  /** Remove a sprite from the game canvas */
  const removeSprite = (entityIndex: EntityIndex) => {
    log.trace("Removing sprite for player", entityIndex.index);
    const sprite = getSpriteUnsafe(entityIndex);
    app.stage.removeChild(sprite);
  };

  const gameCanvas = app.view as HTMLCanvasElement;

  return { addSprite, removeSprite, setSpritePosition, gameCanvas };
};
