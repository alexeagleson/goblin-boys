/** Handles everything related to the actual game canvas rendering
 * and primary consuming of the Pixi.js library */

import {
  AnimatedSprite,
  Application,
  Assets,
  BaseTexture,
  ISpritesheetData,
  SCALE_MODES,
  Sprite,
  Spritesheet,
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
import { mapPosToScreenPos, GAME_CONFIG } from "./camera";
import { PlayerSpriteName } from "../App";

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
export const getSpritePositionUnsafe = (
  entityIndex: EntityIndex
): SpritePosition => {
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

  app.stage.sortableChildren = true;

  const TEXTURE_MAP: Record<string, Texture | Texture[] | Spritesheet> = {};

  // add it to the stage to render
  // app.stage.addChild(anim);

  function lowerFirst(str: string) {
    return str.charAt(0).toLowerCase() + str.slice(1);
  }

  for (const tex in SpriteTexture) {
    const textureId = lowerFirst(tex);
    if (textureId === "empty") {
      continue;
    }

    try {
      console.log(textureId);
      if (textureId.includes("Frames")) {
        const [_, frames] = textureId.split("Frames");
        const framesNum = Number(frames);
        if (framesNum > 0 === false) {
          console.error("Error with frame count: ", textureId);
          throw Error;
        }

        const animationsArray: any[] = [];
        const framesArray = {} as any;
        for (let i = 0; i < framesNum; i++) {
          framesArray[`animFrame${i}${textureId}`] = {
            frame: { x: i * 16, y: 0, w: 16, h: 16 },
            sourceSize: { w: 16, h: 16 },
            spriteSourceSize: { x: 0, y: 0, w: 16, h: 16 },
          };

          animationsArray.push(`animFrame${i}${textureId}`);
        }

        // Create object to store sprite sheet data
        const atlasData = {
          frames: framesArray,

          meta: {
            image: `sprites/v2/${textureId}.png`,
            // image: "sprites/npcs/rat-sheet.png",
            format: "RGBA8888",
            size: { w: 16 * GAME_CONFIG.SPRITE_SCALE, h: 16 },
            scale: "1",
          },
          animations: {
            anim: animationsArray,
            // rat: ["rat1", "rat2", "rat3", "rat4"], //array of frames by name
          },
        };

        // Create the SpriteSheet from data and image
        const spriteSheet = new Spritesheet(
          BaseTexture.from(atlasData.meta.image),
          atlasData
        );

        // Generate all the Textures asynchronously
        await spriteSheet.parse();

        TEXTURE_MAP[textureId as unknown as SpriteTexture] = spriteSheet;

        continue;
      }

      const texture = (await Assets.load(
        `sprites/v2/${textureId}.png`
      )) as Texture;

      // If that success, it's a regular static texture
      texture.baseTexture.scaleMode = SCALE_MODES.NEAREST;
      TEXTURE_MAP[textureId as unknown as SpriteTexture] = texture;
    } catch (e) {
      const textureArray: Texture[] = [];

      for (let i = 1; i < Infinity; i++) {
        try {
          const texture = (await Assets.load(
            `sprites/v2/${textureId}${i}.png`
          )) as Texture;

          texture.baseTexture.scaleMode = SCALE_MODES.NEAREST;
          textureArray.push(texture);
        } catch (e) {
          // Run out of numbered textures
          TEXTURE_MAP[textureId as unknown as SpriteTexture] = textureArray;
          break;
        }
      }
    }
  }

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
      screenPos.x >= GAME_CONFIG.CAMERA_SIZE ||
      screenPos.y < 0 ||
      screenPos.y >= GAME_CONFIG.CAMERA_SIZE
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
      const randomElement = <T>(arr: T[]): T =>
        arr[Math.floor(Math.random() * arr.length)];

      const getSprite = (): Sprite | AnimatedSprite => {
        const textureOrArray = TEXTURE_MAP[spriteUpdate.sprite];

        let sprite: Sprite | AnimatedSprite | undefined = undefined;
        if (textureOrArray instanceof Spritesheet) {
          sprite = new AnimatedSprite(textureOrArray.animations.anim);

          // Guaranteed
          if (sprite instanceof AnimatedSprite) {
            sprite.texture.baseTexture.scaleMode = SCALE_MODES.NEAREST;

            // set the animation speed
            sprite.animationSpeed = 0.1666;

            // play the animation on a loop
            sprite.play();
          }
        } else if (Array.isArray(textureOrArray)) {
          sprite = new Sprite(randomElement(textureOrArray));

          return sprite;
        } else {
          sprite = new Sprite(textureOrArray);
        }

        const spriteName = spriteUpdate.sprite.toLocaleLowerCase();

        sprite.zIndex = spriteName.startsWith("pc")
          ? 2
          : spriteName.includes("floor")
          ? 0
          : 1;

        return sprite;
      };

      const newSprite = getSprite();

      // var shadowLayer = new DisplayGroup(-1, false);

      newSprite.anchor.x = 0.5;
      newSprite.anchor.y = 0.5;

      // newSprite.scaleMode

      newSprite.scale = {
        x: GAME_CONFIG.SPRITE_SCALE,
        y: GAME_CONFIG.SPRITE_SCALE,
      };

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
    spriteMap.delete(entityIndex.idx);
  };

  const gameCanvas = app.view as HTMLCanvasElement;

  const prepAttackAnimation = async () => {
    const animationsArray: any[] = [];
    const framesArray = {} as any;
    for (let i = 0; i < 4; i++) {
      framesArray[`animFrame${i}${`attackBatFrames4`}`] = {
        frame: { x: i * 16, y: 0, w: 16, h: 16 },
        sourceSize: { w: 16, h: 16 },
        spriteSourceSize: { x: 0, y: 0, w: 16, h: 16 },
      };

      animationsArray.push(`animFrame${i}${`attackBatFrames4`}`);
    }

    // Create object to store sprite sheet data
    const atlasData = {
      frames: framesArray,

      meta: {
        image: `sprites/v2/attackBatFrames4.png`,
        format: "RGBA8888",
        size: { w: 16 * GAME_CONFIG.SPRITE_SCALE, h: 16 },
        scale: "1",
      },
      animations: {
        anim: animationsArray,
      },
    };

    // Create the SpriteSheet from data and image
    const spriteSheet = new Spritesheet(
      BaseTexture.from(atlasData.meta.image),
      atlasData
    );

    // Generate all the Textures asynchronously
    await spriteSheet.parse();

    const animatedSprite = new AnimatedSprite(spriteSheet.animations.anim);

    animatedSprite.texture.baseTexture.scaleMode = SCALE_MODES.NEAREST;
    animatedSprite.animationSpeed = 0.4;

    animatedSprite.scale = {
      x: GAME_CONFIG.SPRITE_SCALE,
      y: GAME_CONFIG.SPRITE_SCALE,
    };
    animatedSprite.zIndex = 2;
    animatedSprite.loop = false;
    animatedSprite.visible = false;
    animatedSprite.anchor.x = 0.5;
    animatedSprite.anchor.y = 0.5;

    app.stage.addChild(animatedSprite);

    animatedSprite.onComplete = () => {
      animatedSprite.visible = false;
      animatedSprite.gotoAndStop(0);
    };

    return animatedSprite;
  };

  const attackSprite = await prepAttackAnimation();

  const showAttackAnimation = (pos: Position, time: number) => {
    const screenPos = mapPosToScreenPos(pos);
    attackSprite.visible = true;
    const pxPos = tileToPx(screenPos);
    attackSprite.x = pxPos.x;
    attackSprite.y = pxPos.y;
    attackSprite.animationSpeed =
      ((1 / attackSprite.totalFrames) * 1) / time / 4;
    attackSprite.play();
  };

  return {
    addSprite,
    removeSprite,
    setSpritePosition,
    gameCanvas,
    showAttackAnimation,
    tileToPx,
  };
};
