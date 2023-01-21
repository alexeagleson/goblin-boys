import { Position } from "../../utility/types";
import "./NpcDialogue.css";

export interface NpcDialogueProps {
  dialogue: string;
  menuPosition: Position;
  onDialogueClick: () => void;
}

export const NpcDialogue: React.FC<NpcDialogueProps> = ({
  dialogue,
  menuPosition,
  onDialogueClick,
}) => {
  return (
    <div className="dialogue-container">
      <div className="sprite-and-name">
        <img src="sprites/v2/npcRat.png" />
        <p>Rat</p>
      </div>
      <p className="dialogue-text">
        …a string of break-ins and robberies in the area has created an
        atmosphere of anxiety for local residents, some of whom are demanding
        action by police and officials. When pressed for comment constable
        Martha Pizenski told this reporter that the alleged crimewave was
        nothing to worry about. "We have everything under control. Rest assured
        these criminals will be brought to justice…
      </p>

      <div className="button-container">
        <button onClick={onDialogueClick}>Option 1</button>
        <button onClick={onDialogueClick}>Option 2</button>
      </div>
    </div>
  );
};
