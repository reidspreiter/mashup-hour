import React, { useCallback, useEffect, useRef, useState } from "react";
import { IconType } from "react-icons";
import Tooltip from "./Tooltip";
import "./styles/controllers.css";
import { ChangerFunc } from "./types";

interface KnobProps {
  name: string;
  min: number;
  mid: number;
  max: number;
  onChange: ChangerFunc;
  icon: IconType;
}

const Knob: React.FC<KnobProps> = ({ name, min, mid, max, onChange, icon }) => {
  const [percent, setPercent] = useState<number>(0.5);
  const [isDragging, setIsDragging] = useState<boolean>(false);
  const [startY, setStartY] = useState<number>(0);
  const knobRef = useRef<SVGSVGElement | null>(null);

  const reset = () => {
    setPercent(0.5);
    onChange(mid);
  };

  const startDrag = (e: React.MouseEvent) => {
    setIsDragging(true);
    setStartY(e.clientY);
    e.preventDefault();
    document.body.classList.add("dragging");
  };

  const stopDrag = () => {
    setIsDragging(false);
    document.body.classList.remove("dragging");
  };

  const sensitivity = 0.02;
  const onDrag = useCallback(
    (e: MouseEvent) => {
      if (isDragging && knobRef.current) {
        const y = e.clientY;

        // make negative so knob turns right when mouse goes up
        const deltaY = -(y - startY);
        const newPercent = Math.min(Math.max(deltaY * sensitivity + percent, 0), 1);
        let newValue;
        if (newPercent > 0.5) {
          newValue = mid + ((newPercent - 0.5) / 0.5) * (max - mid);
        } else if (newPercent < 0.5) {
          newValue = min + (newPercent / 0.5) * (mid - min);
        } else {
          newValue = mid;
        }
        setPercent(newPercent);
        setStartY(y);
        onChange(newValue);
      }
    },
    [isDragging, startY, percent, mid, min, max, onChange],
  );

  useEffect(() => {
    if (isDragging) {
      document.addEventListener("mousemove", onDrag);
      document.addEventListener("mouseup", stopDrag);
    } else {
      document.removeEventListener("mousemove", onDrag);
      document.removeEventListener("mouseup", stopDrag);
    }

    // remove event listeners when component unmounts
    return () => {
      document.removeEventListener("mousemove", onDrag);
      document.removeEventListener("mouseup", stopDrag);
    };
  }, [isDragging, onDrag]);

  return (
    <Tooltip text={name}>
      <div className="controller" onMouseDown={startDrag} onDoubleClick={reset}>
        <svg
          ref={knobRef}
          viewBox="0 0 100 100"
          xmlns="http://www.w3.org/2000/svg"
          className="knob"
          style={isDragging ? { transform: "scale(var(--scale-increase))" } : {}}
        >
          <circle
            cx="50"
            cy="50"
            r="45"
            fill={isDragging ? "var(--white)" : "var(--par-color)"}
            stroke="var(--bg-color)"
            strokeWidth="3"
          />
          <rect
            x="47"
            y="50"
            width="6"
            height="6"
            fill="var(--pink)"
            transform={`rotate(${-120 + 240 * percent}, 50, 50) translate(0, -32)`}
          />
        </svg>
        {React.createElement(icon, { className: "knob-icon ignore-pointer" })}
      </div>
    </Tooltip>
  );
};
export default Knob;
