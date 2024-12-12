import { ReactNode } from "react";
import "./styles/controllers.css";

interface EmptyCasing {
  children: ReactNode;
}

const EmptyCasing: React.FC<EmptyCasing> = ({ children }) => {
  return (
    <div className="casing invisible-casing">
      <div className="subcasing">
        <div className="controller circle-button invisible"></div>
      </div>
      {children}
    </div>
  );
};
export default EmptyCasing;
