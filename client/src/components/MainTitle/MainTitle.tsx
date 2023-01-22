import "./MainTitle.css";

export interface MainTitleProps {}

export const MainTitle: React.FC<MainTitleProps> = ({}) => {
  return (
    <div className="main-title-wrapper">
      <img className="title-image" src="titles/GoblinBoysTitle1.png"></img>
      <img className="title-text" src="titles/GoblinBoysTitle2.png"></img>
    </div>
  );
};
