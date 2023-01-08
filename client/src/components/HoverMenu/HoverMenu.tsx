import { EntityData, Position } from "../../utility/types";
import "./HoverMenu.css";

export interface HoverMenuProps {
  menuPosition: Position;
  entityData: EntityData;
}

export const HoverMenu: React.FC<HoverMenuProps> = ({
  menuPosition,
  entityData,
}) => {
  return (
    <div
      className="hover-menu"
      style={{
        left: menuPosition.x,
        top: menuPosition.y,
      }}
    >
      <p>{entityData.name}</p>
      <p>Blocks Light: {entityData.blocksLight ? "Yes" : "No"}</p>
      <p>Can See: {entityData.visibleToPlayer ? "Yes" : "No"}</p>
    </div>
  );
};
