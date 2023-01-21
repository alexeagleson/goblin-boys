import { Position } from "../../utility/types";
import "./NpcDialogue.css";

export interface NpcDialogueProps {
  dialogue: string;
  menuPosition: Position;
}

export const NpcDialogue: React.FC<NpcDialogueProps> = ({
  dialogue,
  menuPosition,
}) => {
  return (
    <div
      className="npc-dialogue-menu"
      style={{
        left: menuPosition.x,
        top: menuPosition.y,
      }}
    >
      <p>{dialogue}</p>
    </div>
  );
};
