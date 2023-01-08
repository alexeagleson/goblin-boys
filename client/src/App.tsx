import { useEffect, useRef, useState } from "react";
import { Log, HoverMenu, ControlOverlay, HoverMenuProps } from "./components";
import { initializeGame } from "./game/main";
import { EntityData } from "./utility/types";
import { DirectionHandlers } from "./game/input";
import "./App.css";

const App = () => {
  const initialized = useRef<boolean>(false);
  const canvasContainer = useRef<HTMLDivElement | null>(null);
  const logContainer = useRef<HTMLDivElement | null>(null);

  const [hoverMenu, setHoverMenu] = useState<HoverMenuProps>();
  const [log, setLog] = useState<string[]>([]);
  const [moveCount, setMoveCount] = useState<number>();
  const [directionHandlers, setDirectionHandlers] =
    useState<DirectionHandlers>();

  const onHover = (x: number, y: number, entityData?: EntityData) => {
    if (!entityData) {
      setHoverMenu(undefined);
    } else {
      setHoverMenu({ menuPosition: { x, y }, entityData });
    }
  };

  const onClick = (logEntry: string) => {
    setLog((oldLog) => [logEntry, ...oldLog]);
  };

  // Queries the server for the game configuration (to determine the canvas size)
  // and then initializes the game.  Will only fire once (due to `initialized` check)
  // so the game state will persist during Vite dev server hot reloading
  useEffect(() => {
    if (initialized.current === false) {
      initialized.current = true;
      initializeGame(onHover, onClick, setMoveCount).then(
        ({ gameCanvas, directionHandlers: dirHandlers }) => {
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
  });

  return (
    <div className="game-container">
      <p>All time totals moves: {moveCount}</p>
      <div className="canvas-and-log-container">
        <div className="canvas-container" ref={canvasContainer}>
          {hoverMenu && <HoverMenu {...hoverMenu} />}
          {directionHandlers && (
            <ControlOverlay directionHandlers={directionHandlers} />
          )}
        </div>
        <div ref={logContainer} className="log-container">
          <Log log={log} />
        </div>
      </div>
    </div>
  );
};

export default App;
