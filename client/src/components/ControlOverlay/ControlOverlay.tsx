import { DirectionHandlers } from "../../game/input";
import "./ControlOverlay.css";

export interface ControlOverlayProps {
  directionHandlers: DirectionHandlers;
}

export const ControlOverlay: React.FC<ControlOverlayProps> = ({
  directionHandlers,
}) => {
  const { up, left, right, down } = directionHandlers;

  return (
    <div className="control-overlay">
      <button className="controller-button up" onClick={up}>
        {"ʌ"}
      </button>
      <button className="controller-button left" onClick={left}>
        {"<"}
      </button>
      <button className="controller-button right" onClick={right}>
        {">"}
      </button>
      <button className="controller-button down" onClick={down}>
        {"v"}
      </button>
    </div>
  );
};
