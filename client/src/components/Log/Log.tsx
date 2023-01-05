import "./Log.css";

export interface LogProps {
  log: string[];
}

export const Log: React.FC<LogProps> = ({ log }) => {
  return (
    <ol className="log">
      <li>Log 👇</li>
      {log.map((entry, idx) => (
        <li key={`${idx}-${entry}`}>{entry}</li>
      ))}
    </ol>
  );
};
