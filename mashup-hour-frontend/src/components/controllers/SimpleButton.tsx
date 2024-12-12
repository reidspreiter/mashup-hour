import Tooltip from "./Tooltip";
import "./styles/controllers.css";
import { ButtonFunc } from "./types";

interface SimpleButtonProps {
  name: string;
  onClick: ButtonFunc;
}

const SimpleButton: React.FC<SimpleButtonProps> = ({ name, onClick }) => {
  return (
    <Tooltip text={name}>
      <button
        className="controller circle-button enabled-on-hold"
        onClick={() => {
          onClick();
        }}
      ></button>
    </Tooltip>
  );
};
export default SimpleButton;
