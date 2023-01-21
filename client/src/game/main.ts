/** Glue that binds together the input, canvas and connection modules */

import { connectToGameServer } from "./connection";
import { clearEverything, createGameApp, spriteMap } from "./canvas";
import {
  EntityData,
  EntityIndex,
  ServerMessageAllClients,
  ServerMessageSingleClient,
  Sound,
  SpriteTexture,
} from "../utility/types";
import { assertNever, log } from "../utility/functions";
import { GAME_CONFIG_URI } from "../utility/config";
import { addInputListeners, GameInputState } from "./input";
import { CAMERA_SIZE, setCamera, TILE_SIZE } from "./camera";

var punch = new Audio("punch.ogg");

let xPixel = 0;
let yPixel = 0;

const updateHoverMenuPosition = (x: number, y: number) => {
  xPixel = x;
  yPixel = y;
};

export const initializeGame = async (
  onHover: (x: number, y: number, entityData?: EntityData) => void,
  onClick: (log: string) => void,
  onMoveCount: (count: number) => void,
  onDialogue: ({
    entity,
    dialogue,
  }: {
    entity: EntityIndex;
    dialogue: string;
  }) => void,
  gameInputState: GameInputState
) => {
  // const mapDimensionsResponse = await fetch(GAME_CONFIG_URI, { method: "GET" });

  // if (!mapDimensionsResponse.ok) {
  //   throw Error("Failed to get initial game config");
  // }

  // gameState.dimensions = await mapDimensionsResponse.json();

  const { addSprite, gameCanvas, removeSprite, setSpritePosition } =
    await createGameApp({ width: CAMERA_SIZE, height: CAMERA_SIZE }, TILE_SIZE);

  const onMessage = (msg: MessageEvent<unknown>) => {
    if (typeof msg.data !== "string") {
      console.error("Received invalid message", msg.data);
      throw Error;
    }
    const response: ServerMessageAllClients | ServerMessageSingleClient =
      JSON.parse(msg.data);

    switch (response.type) {
      case "showDialogue":
        onDialogue(response.content);

        break;
      case "centreCamera":
        setCamera(response.content);

        for (const [entityIndex, spritePosition] of spriteMap) {
          // if (spritePosition.texture === SpriteTexture.Bunny) {
          //   log.trace("Player", entityIndex, spritePosition);
          //   log.trace("Player new position", response.content);
          // }
          setSpritePosition({
            entity: { idx: entityIndex },
            pos: spritePosition.pos,
            sprite: spritePosition.texture,
          });
        }

        break;
      case "entityPositionChange":
        setSpritePosition(response.content);
        break;
      // case "newEntity":
      //   addSprite(response.content);
      //   break;
      // case "newEntities":
      // case "existingEntities":
      //   response.content.forEach((renderData) => {
      //     addSprite(renderData);
      //   });
      //   break;
      case "updateFullGameMap":
        clearEverything();

        setCamera(response.content.camera);

        response.content.entities.forEach((renderData) => {
          addSprite(renderData);
        });
        break;
      case "removeSprite":
        removeSprite(response.content);
        break;
      case "moveCount":
        onMoveCount(response.content);
        break;
      case "tileHover":
        onHover(xPixel, yPixel, response.content);
        break;
      case "tileClick":
        onClick(response.content);
        break;
      case "playSound":
        if (response.content == Sound.Punch) {
          punch.currentTime = 0;
          punch.play();
        }
        break;
      default:
        assertNever(response);
    }
  };

  const { safeSend } = connectToGameServer({
    onOpen: () => {
      log.trace("Connected");
    },
    onClose: () => {
      log.trace("Disconnected");
    },
    onMessage,
  });

  const directionHandlers = addInputListeners(
    gameCanvas,
    updateHoverMenuPosition,
    safeSend,
    gameInputState
  );

  let interval = setInterval(() => {
    let result = safeSend({ type: "initialize" });
    if (result === "success") {
      clearInterval(interval);
    }
  }, 100);

  return { gameCanvas, directionHandlers };
};
