/** Glue that binds together the input, canvas and connection modules */

import { connectToGameServer } from "./connection";
import {
  clearEverything,
  createGameApp,
  getSpritePositionUnsafe,
  spriteMap,
} from "./canvas";
import {
  DialogueMap,
  EntityData,
  EntityIndex,
  ServerMessageAllClients,
  ServerMessageSingleClient,
  Sound,
  SpawnableEnemy,
  SpriteTexture,
} from "../utility/types";
import { assertNever, log } from "../utility/functions";
import { PLAYER_STATS_URI } from "../utility/config";
import { addInputListeners, GameInputState } from "./input";
import { mapPosToScreenPos, setCamera, GAME_CONFIG } from "./camera";
import { DebugMenuProps } from "../components/DebugMenu/DebugMenu";
import { DamageNumberProps } from "../components/DamageNumber/DamageNumber";
import { PlayerSpriteName, PlayerStats } from "../App";

var punch = new Audio("audio/sfx/punch.ogg");
var eatBones = new Audio("audio/sfx/bones.ogg");

// punch.loop = true;
// punch.loop = () => (punch.volume = 0);
// eatBones.onended = () => (eatBones.currentTime = 0);

// // var audioCtx, analyser, bufferLength, dataArray;

// let audioCtx: AudioContext;
// let analyser: AnalyserNode;
// // window.addEventListener("load", initAudio, false);

// let audioButton = document.createElement("button");
// audioButton.innerText = "ENABLE AUDIO";
// audioButton.onclick = () => punch.play();
// document.body.appendChild(audioButton);

// let punch: AudioBufferSourceNode;

// function initAudio() {
//   try {
//     // window.audioCtx = window.AudioContext || window.webkitAudioContext;
//     audioCtx = new AudioContext();
//     analyser = audioCtx.createAnalyser();
//   } catch (e) {
//     alert("Web Audio API is not supported in this browser");
//   }
//   // load the audio file
//   punch = audioCtx.createBufferSource();
//   var request = new XMLHttpRequest();
//   request.open("GET", "audio/sfx/punch.ogg", true);
//   request.responseType = "arraybuffer";
//   request.onload = function () {
//     var audioData = request.response;
//     audioCtx.decodeAudioData(
//       audioData,
//       function (buffer) {
//         punch.buffer = buffer;
//         punch.connect(analyser);
//         analyser.connect(audioCtx.destination);
//         punch.loop = true;
//         punch.start(0);
//       },
//       function (e) {
//         console.log(e);
//       }
//     );
//   };
//   request.send();
// }

let xPixel = 0;
let yPixel = 0;

const updateHoverMenuPosition = (x: number, y: number) => {
  xPixel = x;
  yPixel = y;
};

const SPRITE_NAME_TO_TEXTURE: Record<PlayerSpriteName, SpriteTexture> = {
  "Sewer Kid": SpriteTexture.PcSewerKidFrames6,
  "Ant Boy": SpriteTexture.PcAntBoiFrames4,
  "Boney Boy": SpriteTexture.PcBoneyBoiFrames4,
  "Ghost Boy": SpriteTexture.PcGhostBoyFrames8,
  KidZilla: SpriteTexture.PcKidZilla,
};

export const initializeGame = async (
  onHover: (x: number, y: number, entityData?: EntityData) => void,
  onClick: (log: string) => void,
  onDeath: (log: string) => void,
  onDamage: (log: string) => void,
  onMoveCount: (count: number) => void,
  onDialogue: (
    payload: Extract<
      ServerMessageSingleClient,
      { type: "showDialogue" }
    >["content"]
  ) => void,
  gameInputState: GameInputState,
  setDebugMenuProps: (props: DebugMenuProps) => void,
  setDamageNumbers: (payload: DamageNumberProps) => void,
  playerSpriteName: PlayerSpriteName,
  playerName: string,
  setPlayerStats: (payload: PlayerStats) => void,
  addLogEntry: (log: string) => void
) => {
  const {
    addSprite,
    gameCanvas,
    removeSprite,
    setSpritePosition,
    showAttackAnimation,
    tileToPx,
  } = await createGameApp(
    { width: GAME_CONFIG.CAMERA_SIZE, height: GAME_CONFIG.CAMERA_SIZE },
    GAME_CONFIG.TILE_SIZE
  );

  const onMessage = (msg: MessageEvent<unknown>) => {
    if (typeof msg.data !== "string") {
      console.error("Received invalid message", msg.data);
      throw Error;
    }
    const response: ServerMessageAllClients | ServerMessageSingleClient =
      JSON.parse(msg.data);

    switch (response.type) {
      case "showAnimation":
        showAttackAnimation(response.content.position, response.content.time);
        break;
      case "showDialogue":
        onDialogue(response.content);
        break;
      case "log":
        addLogEntry(response.content);
        break;
      case "centreCamera":
        setCamera(response.content);

        for (const [entityIndex, spritePosition] of spriteMap) {
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
      case "addSprite":
        addSprite(response.content);
        break;

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
      case "damage":
        onDamage(response.content);
        break;
      case "death":
        onDeath(response.content);
        break;
      case "playSound":
        if (response.content == Sound.Punch) {
          // punch.currentTime = 0;
          // punch.play();
          // punch?.
          // if (punch) {
          //   punch.playbackRate.value = 1;
          //   setTimeout(() => {
          //     punch.playbackRate.value = 0;
          //   }, 100);
          // }
          // console.log("Request to play sound was received:", response.content);
          // punch.play();
        } else if (response.content == Sound.EatBones) {
          // eatBones.currentTime = 0;
          eatBones.play();
        }
        break;
      case "debug":
        // console.log(response.content);
        setDebugMenuProps(response.content);
        break;
      case "showDamage":
        // console.log(response.content);
        // setDebugMenuProps(response.content);
        // console.log(response.content);
        let spritePosition = getSpritePositionUnsafe(response.content.entity);
        let screenPos = mapPosToScreenPos(spritePosition.pos);
        // console.log(screenPos);
        setDamageNumbers({
          pixelPos: tileToPx(screenPos),
          showDamage: response.content,
        });
        if (response.content.targetIsMe) {
          setPlayerStats(response.content);
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
    let result = safeSend({
      type: "initialize",
      content: {
        name: playerName,
        sprite: SPRITE_NAME_TO_TEXTURE[playerSpriteName],
      },
    });
    if (result === "success") {
      clearInterval(interval);
    }
  }, 100);

  const spawnSlime = () => {
    safeSend({ type: "spawn", content: SpawnableEnemy.Slime });
  };

  const spawnRatKing = () => {
    safeSend({ type: "spawn", content: SpawnableEnemy.RatKing });
  };

  return { gameCanvas, directionHandlers, spawnSlime, spawnRatKing };
};
