import { EntityInfo, Position } from "../../utility/types";
import "./HoverMenu.css";

export interface HoverMenuProps {
  menuPosition: Position;
  entityInfo: EntityInfo;
}

export const HoverMenu: React.FC<HoverMenuProps> = ({
  menuPosition,
  entityInfo,
}) => {
  return (
    <div
      className="hover-menu"
      style={{
        left: menuPosition.x,
        top: menuPosition.y,
      }}
    >
      <p>{entityInfo.name}</p>
      <p>Blocks Light: {entityInfo.blocksLight ? "Yes" : "No"}</p>
    </div>
  );
};
