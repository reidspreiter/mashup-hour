import { useRef, useState } from "react";

interface TitleProps {
  textClass?: string;
  children: string;
}

const TextScrollOnOverflow: React.FC<TitleProps> = ({ textClass, children }) => {
  const [translateTime, setTranslateTime] = useState<number>(0);
  const [translateVal, setTranslateVal] = useState<number>(0);
  const textBoxRef = useRef<HTMLDivElement | null>(null);

  const transitionTimePerPixel = 0.01;

  const scrollForwardOnMouseEnter = () => {
    if (textBoxRef.current !== null) {
      const boxWidth = textBoxRef.current.clientWidth;
      const textWidth = textBoxRef.current.children[0].scrollWidth;
      const translateVal = Math.min(boxWidth - textWidth, 0);
      setTranslateVal(translateVal);
      setTranslateTime(-transitionTimePerPixel * translateVal);
    }
  };

  const scrollBackwardOnMouseLeave = () => {
    setTranslateTime(0.6);
    setTranslateVal(0);
  };

  return (
    <div
      ref={textBoxRef}
      className="text-no-overflow"
      onMouseEnter={() => scrollForwardOnMouseEnter()}
      onMouseLeave={() => scrollBackwardOnMouseLeave()}
    >
      {" "}
      <span
        className={textClass}
        style={{ transitionDuration: `${translateTime}s`, transform: `translateX(${translateVal}px)` }}
      >
        {children}
      </span>
    </div>
  );
};
export default TextScrollOnOverflow;
