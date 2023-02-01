import { ServerMessageSingleClient } from "../../utility/types";
import "./PlayerStats.css";

export interface PlayerStatsProps {
  playerStats: Extract<
    ServerMessageSingleClient,
    { type: "showDamage" }
  >["content"];
}

export const PlayerStats: React.FC<PlayerStatsProps> = ({ playerStats }) => {
  const health_percent = Math.floor(playerStats.currentHp / playerStats.maxHp * 100);

  return (
    <div>
      <p>
        {playerStats.currentHp} / {playerStats.maxHp}
      </p>
      <div className="health-bar-wrapper">
        <div className="health-bar" style={{ width: `${health_percent}%` }} />
      </div>
    </div>
  );
};
