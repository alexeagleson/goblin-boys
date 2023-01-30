import "./DebugMenu.css";

export interface DebugMenuProps {
  numEnemies: number;
  // numEnemySprites: number
}

export const DebugMenu: React.FC<DebugMenuProps> = ({ numEnemies }) => {
  return <div className="debug-menu">Enemies Engine: {numEnemies}</div>;
};
