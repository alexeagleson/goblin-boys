import { useEffect, useRef, useState } from "react";
import { Log, HoverMenu, ControlOverlay, HoverMenuProps } from "./components";
import { initializeGame } from "./game/main";
import {
  DialogueMap,
  EntityData,
  EntityIndex,
  ServerMessageSingleClient,
} from "./utility/types";
import { DirectionHandlers, GameInputState } from "./game/input";
import "./App.css";
import {
  NpcDialogue,
  NpcDialogueProps,
} from "./components/NpcDialogue/NpcDialogue";
import { MainTitle } from "./components/MainTitle/MainTitle";
import { DebugMenu, DebugMenuProps } from "./components/DebugMenu/DebugMenu";
import {
  DamageNumber,
  DamageNumberProps,
} from "./components/DamageNumber/DamageNumber";
import { PlayerStats } from "./components/PlayerStats/PlayerStats";
import { GAME_CONFIG, setGameConfig } from "./game/camera";

const music = new Audio("audio/music/supersewerslug.ogg");

const gameInputState: GameInputState = { enabled: true };

let spawnSlimeHandler: () => void | undefined;
let spawnKingRatHandler: () => void | undefined;

const PLAYER_SPRITE_NAMES = [
  "KidZilla",
  "Ghost Boy",
  "Boney Boy",
  "Ant Boy",
  "Sewer Kid",
] as const;

export type PlayerSpriteName = typeof PLAYER_SPRITE_NAMES[number];

export type PlayerStats = Extract<
  ServerMessageSingleClient,
  { type: "showDamage" }
>["content"];

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

  const [debugMenuProps, setDebugMenuProps] = useState<DebugMenuProps>();

  const [damageNumbers, setDamageNumbers] = useState<Array<DamageNumberProps>>(
    []
  );

  const [startGame, setStartGame] = useState(false);

  const randomSprite =
    PLAYER_SPRITE_NAMES[Math.floor(Math.random() * PLAYER_SPRITE_NAMES.length)];

  const [playerSprite, setPlayerSprite] =
    useState<PlayerSpriteName>(randomSprite);

  const [playerName, setPlayerName] = useState<string>("Player");

  const [playerStats, setPlayerStats] = useState<PlayerStats>();

  const [cameraRadius, setCameraRadius] = useState(GAME_CONFIG.CAMERA_RADIUS);
  const [spriteScale, setSpriteScale] = useState(GAME_CONFIG.SPRITE_SCALE);

  const [logOn, setLogOn] = useState(true);

  const [mobileControls, setMobileControls] = useState(false);

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

  const onDialogue = (
    nameAndDialogueMap: NpcDialogueProps["nameAndDialogueMap"]
  ) => {
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
    if (startGame) {
      if (initialized.current === false) {
        initialized.current = true;
        initializeGame(
          onHover,
          onClick,
          onDeath,
          onDamage,
          setMoveCount,
          onDialogue,
          gameInputState,
          setDebugMenuProps,
          (payload) => setDamageNumbers((prev) => [...prev, payload]),
          playerSprite,
          playerName,
          setPlayerStats
        ).then(
          ({
            gameCanvas,
            directionHandlers: dirHandlers,
            spawnSlime,
            spawnRatKing,
          }) => {
            spawnSlimeHandler = spawnSlime;
            spawnKingRatHandler = spawnRatKing;
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
          }
        );
      }
    }
  }, [startGame]);

  const onChangeValue: React.FormEventHandler<HTMLDivElement> = (event) => {
    const target = event.target as HTMLInputElement;
    setPlayerSprite(target.value as PlayerSpriteName);
  };

  return (
    <>
      {startGame === false ? (
        <div>
          <div onChange={onChangeValue}>
            {PLAYER_SPRITE_NAMES.map((spriteName, idx) => {
              return (
                <div key={idx}>
                  <label htmlFor={spriteName}>{spriteName}</label>
                  <input
                    type="radio"
                    id={spriteName}
                    value={spriteName}
                    name="playerSprite"
                    // checked={playerSprite === spriteName}
                    defaultChecked={playerSprite === spriteName}
                  />
                </div>
              );
            })}
          </div>
          <input
            type="text"
            value={playerName}
            onChange={(e) => setPlayerName(e.target.value)}
          />
          <div>
            <input
              type="range"
              id="sprite-scale"
              name="sprite-scale"
              min="1"
              max="5"
              step="1"
              value={spriteScale}
              onChange={(e) => {
                const scale = Number(e.target.value);

                setSpriteScale(scale);
                setGameConfig(scale, GAME_CONFIG.CAMERA_RADIUS);
              }}
            />
            <label htmlFor="sprite-scale">Sprite Scale {spriteScale}</label>
          </div>

          <div>
            <input
              type="range"
              id="camera-radius"
              name="camera-radius"
              min="1"
              max="10"
              value={cameraRadius}
              step="1"
              onChange={(e) => {
                const radius = Number(e.target.value);

                setCameraRadius(radius);
                setGameConfig(GAME_CONFIG.SPRITE_SCALE, radius);
              }}
            />
            <label htmlFor="camera-radius">Camera Radius {cameraRadius}</label>
          </div>
          <button onClick={() => setStartGame(true)}>OK GO</button>
        </div>
      ) : (
        <>
          {debugMenuProps && <DebugMenu {...debugMenuProps} />}
          <div style={{ display: "flex", columnGap: "8px", margin: "8px" }}>
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
                spawnSlimeHandler?.();
              }}
            >
              Spawn a Slime
            </button>
            <button
              onClick={() => {
                spawnKingRatHandler?.();
              }}
            >
              Spawn King Rat
            </button>
            <button
              onClick={() => {
                setLogOn(false);
              }}
            >
              Disable Log
            </button>
            <button
              onClick={() => {
                setMobileControls(true);
              }}
            >
              Enable Control Overlay
            </button>
          </div>
          {enableMainTitle && <MainTitle />}
          {!enableMainTitle && (
            <div className="game-container">
              <p>All time totals moves: {moveCount}</p>

              <div className="canvas-and-log-container">
                <div className="canvas-container" ref={canvasContainer}>
                  {damageNumbers.map((damProps, idx) => (
                    <DamageNumber key={idx} {...damProps} />
                  ))}
                  {/* {hoverMenu && <HoverMenu {...hoverMenu} />} */}
                  {npcDialogueMenu && <NpcDialogue {...npcDialogueMenu} />}
                  {mobileControls && directionHandlers && (
                    <ControlOverlay directionHandlers={directionHandlers} />
                  )}
                </div>

                {logOn && (
                  <div ref={logContainer} className="log-container">
                    <Log log={log} />
                  </div>
                )}
              </div>
              <div style={{ display: "flex", flexDirection: "row" }}>
                Hp: {playerStats && <PlayerStats playerStats={playerStats} />}
              </div>
            </div>
          )}
        </>
      )}
    </>
  );
};

export default App;
