import { GAME_CONFIG } from "../../game/camera";
import { Position, ServerMessageSingleClient } from "../../utility/types";
import "./DamageNumber.css";

export interface DamageNumberProps {
  showDamage: Extract<
    ServerMessageSingleClient,
    { type: "showDamage" }
  >["content"];
  pixelPos: Position;
}

export const DamageNumber: React.FC<DamageNumberProps> = ({
  pixelPos: { x, y },
  showDamage: { damage, targetIsUser, isHealing },
}) => {
  const modifier = Math.floor((GAME_CONFIG.TILE_SIZE * Math.random()) / 2);

  return (
    <span
      className="damage-number"
      style={{
        left: x - GAME_CONFIG.TILE_SIZE / 2 + modifier,
        top: y - GAME_CONFIG.TILE_SIZE / 2 - modifier,
        color: isHealing ? "lightgreen" : targetIsUser ? "red" : "white",
        WebkitTextStrokeWidth: "1px",
        WebkitTextStrokeColor: targetIsUser ? "white" : "black",
      }}
    >
      {damage}
    </span>
  );
};
