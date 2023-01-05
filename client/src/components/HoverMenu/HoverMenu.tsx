import { PlayerDetails, Position } from "../../utility/types";
import "./HoverMenu.css";

export interface HoverMenuProps {
  menuPosition: Position;
  playerDetails: PlayerDetails;
}

export const HoverMenu: React.FC<HoverMenuProps> = ({
  menuPosition,
  playerDetails,
}) => {
  return (
    <div
      className="hover-menu"
      style={{
        left: menuPosition.x,
        top: menuPosition.y,
      }}
    >
      <p>{playerDetails.name}</p>
    </div>
  );
};
