import React from "react";
import { IconType } from "react-icons";
import "./styles/controllers.css";
import Tooltip from "./Tooltip";

interface ButtonProps {
  name: string;
  icon: IconType;
  onClick: () => void;
}

const Button: React.FC<ButtonProps> = ({ name, icon, onClick }) => {
  return (
    <Tooltip text={name}>
      <button className="controller" onClick={() => onClick()}>
        {React.createElement(icon, { className: "controller-icon icon-medium" })}
      </button>
    </Tooltip>
  );
};
export default Button;
