import { useEffect, useRef, useState } from "react";
import { Log, HoverMenu, ControlOverlay, HoverMenuProps } from "./components";
import { initializeGame } from "./game/main";
import { DialogueMap, EntityData, EntityIndex } from "./utility/types";
import { DirectionHandlers, GameInputState } from "./game/input";
import "./App.css";
import {
  NpcDialogue,
  NpcDialogueProps,
} from "./components/NpcDialogue/NpcDialogue";
import { MainTitle } from "./components/MainTitle/MainTitle";

const music = new Audio("audio/music/supersewerslug.ogg");

const gameInputState: GameInputState = { enabled: true };

let spawnHandler: () => void | undefined;

const App = () => {
  const initialized = useRef<boolean>(false);
  const canvasContainer = useRef<HTMLDivElement | null>(null);
  const logContainer = useRef<HTMLDivElement | null>(null);

  const [hoverMenu, setHoverMenu] = useState<HoverMenuProps>();
  const [npcDialogueMenu, setNpcDialogueMenu] = useState<NpcDialogueProps>();
  const [log, setLog] = useState<string[]>([]);
  const [moveCount, setMoveCount] = useState<number>();
  const [directionHandlers, setDirectionHandlers] =
    useState<DirectionHandlers>();
  const [enableMainTitle, setEnableMainTitle] = useState<boolean>(false);

  const onHover = (x: number, y: number, entityData?: EntityData) => {
    if (!entityData) {
      setHoverMenu(undefined);
    } else {
      setHoverMenu({ menuPosition: { x, y }, entityData });
    }
  };

  const addLogEntry = (logEntry: string) => {
    setLog((oldLog) => [logEntry, ...oldLog]);
  };

  const onClick = addLogEntry;
  const onDamage = addLogEntry;
  const onDeath = addLogEntry;

  const onDialogueClose = () => {
    gameInputState.enabled = true;
    setNpcDialogueMenu(undefined);
  };

  const onDialogue = (nameAndDialogueMap: {
    entity_name: string;
    dialogue_map: DialogueMap;
  }) => {
    setNpcDialogueMenu({
      nameAndDialogueMap,
      onClose: onDialogueClose,
    });

    gameInputState.enabled = false;
    // setTimeout(() => {
    //   setNpcDialogueMenu(undefined);
    // }, 2000);
  };

  // Queries the server for the game configuration (to determine the canvas size)
  // and then initializes the game.  Will only fire once (due to `initialized` check)
  // so the game state will persist during Vite dev server hot reloading
  useEffect(() => {
    if (initialized.current === false) {
      initialized.current = true;
      initializeGame(
        onHover,
        onClick,
        onDeath,
        onDamage,
        setMoveCount,
        onDialogue,
        gameInputState
      ).then(({ gameCanvas, directionHandlers: dirHandlers, spawnSlime }) => {
        spawnHandler = spawnSlime;
        setDirectionHandlers(dirHandlers);
        canvasContainer.current?.appendChild(gameCanvas);
        let canvasHeight = gameCanvas.height;
        const canvasWidth = gameCanvas.width;

        if (canvasContainer.current && logContainer.current) {
          canvasContainer.current.style.height = canvasHeight + "px";
          logContainer.current.style.width = canvasWidth + "px";

          // Log height is shorter on mobile
          if (window.matchMedia("(max-width: 600px)").matches) {
            canvasHeight = Math.floor(canvasHeight / 2);
          }

          logContainer.current.style.height = canvasHeight + "px";
        }

        gameCanvas.onmouseleave = () => {
          setHoverMenu(undefined);
        };
      });
    }
  });

  return (
    <>
      <button
        onClick={() => {
          setEnableMainTitle((oldVal) => !oldVal);
        }}
      >
        Test title sequence
      </button>
      <button
        onClick={() => {
          if (music.currentTime === 0) {
            music.play();
            music.loop = true;
          } else {
            music.pause();
          }
        }}
      >
        Test music
      </button>
      <button
        onClick={() => {
          console.log(spawnHandler);
          spawnHandler?.();
        }}
      >
        Spawn a Slime
      </button>
      {enableMainTitle && <MainTitle />}
      {!enableMainTitle && (
        <div className="game-container">
          <p>All time totals moves: {moveCount}</p>
          <div className="canvas-and-log-container">
            <div className="canvas-container" ref={canvasContainer}>
              {/* {hoverMenu && <HoverMenu {...hoverMenu} />} */}
              {npcDialogueMenu && <NpcDialogue {...npcDialogueMenu} />}
              {/* {directionHandlers && (
              <ControlOverlay directionHandlers={directionHandlers} />
            )} */}
            </div>
            <div ref={logContainer} className="log-container">
              <Log log={log} />
            </div>
          </div>
        </div>
      )}
    </>
  );
};

export default App;
