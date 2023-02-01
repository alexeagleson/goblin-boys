import { TILE_SIZE } from "../../game/camera";
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
  showDamage: { damage, targetIsUser },
}) => {
  const modifier = Math.floor((TILE_SIZE * Math.random()) / 2);

  return (
    <span
      className="damage-number"
      style={{
        left: x - TILE_SIZE / 2 + modifier,
        top: y - TILE_SIZE / 2 - modifier,
        color: targetIsUser ? "red" : "white",
        WebkitTextStrokeWidth: "1px",
        WebkitTextStrokeColor: targetIsUser ? "white" : "black",
      }}
    >
      {damage}
    </span>
  );
};
