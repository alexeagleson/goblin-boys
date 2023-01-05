/** Handles everything related to the actual game canvas rendering
 * and primary consuming of the Pixi.js library */

import { Application, Assets, Sprite, Texture, Ticker } from "pixi.js";
import { STRICT_MODE } from "../utility/config";
import { MapDimensions, PlayerPosition, UserId } from "../utility/types";
import { log } from "../utility/functions";

const spriteMap = new Map<UserId, Sprite>();
const tickerMap = new Map<UserId, Ticker>();

export type TextureId = "bunny";

/** Assert that a sprite exists in the sprite map and return it, throw error otherwise. */
const getSpriteUnsafe = (id: UserId): Sprite => {
  const maybeSprite = spriteMap.get(id);
  if (STRICT_MODE && maybeSprite === undefined) {
    console.error("id", id);
    console.error(spriteMap);
    throw Error("Tried to get a non-existent sprite");
  }
  return maybeSprite as Sprite;
};

/** Assert that a ticker exists in the ticker map and return it, throw error otherwise. */
const getTickerUnsafe = (id: UserId): Ticker => {
  const maybeTicker = tickerMap.get(id);
  if (STRICT_MODE && maybeTicker === undefined) {
    console.error("id", id);
    console.error(tickerMap);
    throw Error("Tried to get a non-existent ticker");
  }
  return maybeTicker as Ticker;
};

export const createGameApp = async (
  dimensions: MapDimensions,
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

  const TEXTURE_MAP: Record<TextureId, Texture> = {
    bunny,
  };

  /** Move a sprite to another position on the canvas */
  const setSpritePosition = (playerPosition: PlayerPosition) => {
    log.trace("Setting sprite position", playerPosition);
    const sprite = getSpriteUnsafe(playerPosition.id);
    sprite.x = playerPosition.pos.x * tileSize + halfTile;
    sprite.y = playerPosition.pos.y * tileSize + halfTile;
  };

  /** Add a new sprite to the game canvas if it doesn't exist,
   * if it does exist it will update its position */
  const addSprite = (playerPosition: PlayerPosition, textureId: TextureId) => {
    // Only add the sprite if it doesn't already exist
    if (spriteMap.get(playerPosition.id) === undefined) {
      log.trace("Adding sprite for player", playerPosition.id);
      const newSprite = new Sprite(TEXTURE_MAP[textureId]);
      spriteMap.set(playerPosition.id, newSprite);

      newSprite.anchor.x = 0.5;
      newSprite.anchor.y = 0.5;

      // Each frame we spin the sprite around in circles just for shits
      const ticker = app.ticker.add(() => {
        newSprite.rotation += 0.05;
      });

      tickerMap.set(playerPosition.id, ticker);

      app.stage.addChild(newSprite);
    } else {
      log.trace("Existing sprite found for player", playerPosition.id);
    }

    setSpritePosition(playerPosition);
  };

  /** Remove a sprite from the game canvas */
  const removeSprite = (id: UserId) => {
    log.trace("Removing sprite for player", id);
    const sprite = getSpriteUnsafe(id);
    const ticker = getTickerUnsafe(id);
    tickerMap.delete(id);
    app.stage.removeChild(sprite);
    ticker.destroy();
  };

  const gameCanvas = app.view as HTMLCanvasElement;

  return { addSprite, removeSprite, setSpritePosition, gameCanvas };
};
