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
      console.log(textureId)
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
          framesArray[`animFrame${i}`] = {
            frame: { x: i * 16, y: 0, w: 16, h: 16 },
            sourceSize: { w: 16, h: 16 },
            spriteSourceSize: { x: 0, y: 0, w: 16, h: 16 },
          };

          animationsArray.push(`animFrame${i}`);
        }

        // Create object to store sprite sheet data
        const atlasData = {
          frames: framesArray,
          // {
          //   rat1: {
          //     frame: { x: 0, y: 0, w: 16, h: 16 },
          //     sourceSize: { w: 16, h: 16 },
          //     spriteSourceSize: { x: 0, y: 0, w: 16, h: 16 },
          //   },
          //   rat2: {
          //     frame: { x: 16, y: 0, w: 16, h: 16 },
          //     sourceSize: { w: 16, h: 16 },
          //     spriteSourceSize: { x: 0, y: 0, w: 16, h: 16 },
          //   },
          //   rat3: {
          //     frame: { x: 32, y: 0, w: 16, h: 16 },
          //     sourceSize: { w: 16, h: 16 },
          //     spriteSourceSize: { x: 0, y: 0, w: 16, h: 16 },
          //   },
          //   rat4: {
          //     frame: { x: 48, y: 0, w: 16, h: 16 },
          //     sourceSize: { w: 16, h: 16 },
          //     spriteSourceSize: { x: 0, y: 0, w: 16, h: 16 },
          //   },
          // },
          meta: {
            image: `sprites/v2/${textureId}.png`,
            // image: "sprites/npcs/rat-sheet.png",
            format: "RGBA8888",
            size: { w: 16 * SPRITE_SCALE, h: 16 },
            scale: "1",
          },
          animations: {
            anim: animationsArray,
            // rat: ["rat1", "rat2", "rat3", "rat4"], //array of frames by name
          },
        };

        console.log(atlasData, textureId);

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
      const randomElement = <T>(arr: T[]): T =>
        arr[Math.floor(Math.random() * arr.length)];

      const getSprite = (): Sprite | AnimatedSprite => {
        const textureOrArray = TEXTURE_MAP[spriteUpdate.sprite];

        if (textureOrArray instanceof Spritesheet) {
          const animatedSprite = new AnimatedSprite(
            textureOrArray.animations.anim
          );

          animatedSprite.texture.baseTexture.scaleMode = SCALE_MODES.NEAREST;

          // set the animation speed
          animatedSprite.animationSpeed = 0.1666;

          // play the animation on a loop
          animatedSprite.play();

          return animatedSprite;
        } else if (Array.isArray(textureOrArray)) {
          const sprite = new Sprite(randomElement(textureOrArray));
          sprite.zIndex = spriteUpdate.sprite
            .toLocaleLowerCase()
            .includes("floor")
            ? 0
            : 1;

          return sprite;
        } else {
          const sprite = new Sprite(textureOrArray);
          sprite.zIndex = spriteUpdate.sprite
            .toLocaleLowerCase()
            .includes("floor")
            ? 0
            : 1;
          return sprite;
        }
      };

      const newSprite = getSprite();

      // var shadowLayer = new DisplayGroup(-1, false);

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
