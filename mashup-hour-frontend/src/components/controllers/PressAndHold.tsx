import { useState } from "react";
import "./styles/controllers.css";
import Tooltip from "./Tooltip";
import { EnablerFunc } from "./types";

interface PressAndHoldProps {
  name: string;
  onClick: EnablerFunc;
}

const PressAndHold: React.FC<PressAndHoldProps> = ({ name, onClick }) => {
  const [isEnabled, setIsEnabled] = useState<boolean>(false);
  return (
    <Tooltip text={name}>
      <button
        className={`controller circle-button ${isEnabled ? " enabled-bg" : ""}`}
        onMouseDown={() => {
          onClick(true);
          setIsEnabled(true);
        }}
        onMouseUp={() => {
          onClick(false);
          setIsEnabled(false);
        }}
      ></button>
    </Tooltip>
  );
};
export default PressAndHold;
