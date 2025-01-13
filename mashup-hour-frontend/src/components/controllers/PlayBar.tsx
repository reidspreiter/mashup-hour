import { useCallback, useEffect, useRef, useState } from "react";
import { Player } from "../../player";
import { clamp } from "../util";
import "./styles/controllers.css";
import Tooltip from "./Tooltip";

interface PlayBarProps {
  player: Player;
}

enum PlayBarDraggable {
  START,
  END,
  SEEK,
}

const PlayBar: React.FC<PlayBarProps> = ({ player }) => {
  const [seekPercentage, setSeekPercentage] = useState<number>(0);
  const [startPercentage, setStartPercentage] = useState<number>(0);
  const [endPercentage, setEndPercentage] = useState<number>(1);
  const [dragging, setDragging] = useState<PlayBarDraggable | null>(null);
  const [isRightClick, setIsRightClick] = useState<boolean>(false);
  const sliderRef = useRef<HTMLDivElement | null>(null);

  console.log(seekPercentage);

  useEffect(() => {
    player.onPositionUpdate = (percent: number) => {
      if (dragging !== PlayBarDraggable.SEEK) {
        setSeekPercentage(percent);
      }
    };
    player.onEndBoundUpdate = (percent: number) => setEndPercentage(percent);
    player.onStartBoundUpdate = (percent: number) => setStartPercentage(percent);
  }, [player, dragging]);

  const getLinearGradientString = useCallback((): string => {
    return `linear-gradient(to right, #121212 0%, #121212 ${startPercentage * 100}%, transparent ${startPercentage * 100}%, transparent ${endPercentage * 100}%, #121212 ${endPercentage * 100}%, #121212 100%)`;
  }, [startPercentage, endPercentage]);

  const endDrag = useCallback(
    (e: MouseEvent) => {
      e.stopPropagation();
      if (isRightClick) {
        player.setBounds(startPercentage, endPercentage);
      } else if (dragging === PlayBarDraggable.SEEK) {
        player.seek(seekPercentage);
      } else if (dragging === PlayBarDraggable.START) {
        player.startBound = startPercentage;
      } else {
        player.endBound = endPercentage;
      }
      setDragging(null);
      document.body.classList.remove("sliding");
      document.body.classList.remove("pointing");
    },
    [isRightClick, player, endPercentage, startPercentage, dragging],
  );

  const startDrag = (e: React.MouseEvent, draggable: PlayBarDraggable) => {
    setDragging(draggable);
    setIsRightClick(e.button === 2);
    if (draggable !== PlayBarDraggable.SEEK) {
      document.body.classList.add("sliding");
    } else {
      document.body.classList.add("pointing");
    }
  };

  const onDrag = useCallback(
    (e: MouseEvent) => {
      if (dragging !== null) {
        const rect = sliderRef.current?.getBoundingClientRect();
        if (rect !== undefined) {
          const sliderWidth = rect.width;
          const mousePos = clamp(0, sliderWidth, e.clientX - rect.left);
          const newPercentage = mousePos / sliderWidth;

          if (dragging === PlayBarDraggable.SEEK) {
            setSeekPercentage(newPercentage);
          } else if (isRightClick) {
            const d = (dragging === PlayBarDraggable.END ? endPercentage : startPercentage) - newPercentage;
            const newEndPercentage = endPercentage - d;
            const newStartPercentage = startPercentage - d;
            if (newEndPercentage >= 0 && newEndPercentage <= 1 && newStartPercentage >= 0 && newStartPercentage <= 1) {
              setEndPercentage(newEndPercentage);
              setStartPercentage(newStartPercentage);
            }
          } else if (dragging === PlayBarDraggable.END && newPercentage > startPercentage) {
            setEndPercentage(newPercentage);
          } else if (dragging === PlayBarDraggable.START && newPercentage < endPercentage) {
            setStartPercentage(newPercentage);
          }
        }
      }
    },
    [dragging, endPercentage, startPercentage, isRightClick],
  );

  useEffect(() => {
    if (dragging !== null) {
      document.addEventListener("mousemove", onDrag);
      document.addEventListener("mouseup", endDrag);
    } else {
      document.removeEventListener("mousemove", onDrag);
      document.removeEventListener("mouseup", endDrag);
    }

    return () => {
      document.removeEventListener("mousemove", onDrag);
      document.removeEventListener("mouseup", endDrag);
    };
  }, [dragging, onDrag, endDrag]);

  const seek = useCallback(
    (e: React.MouseEvent) => {
      if (dragging === null) {
        const rect = sliderRef.current?.getBoundingClientRect();
        if (rect !== undefined) {
          const sliderWidth = rect.width;
          const mousePos = clamp(0, sliderWidth, e.clientX - rect.left);
          const newPercentage = mousePos / sliderWidth;
          player.seek(newPercentage);
        }
      }
    },
    [player],
  );

  return (
    <div
      ref={sliderRef}
      className="playbar"
      onMouseUp={seek}
      style={{
        backgroundImage: getLinearGradientString(),
      }}
    >
      <Tooltip
        text="seek"
        showCondition={dragging === null}
        style={{
          left: `${seekPercentage * 100}%`,
          position: "absolute",
        }}
      >
        <div
          className="playbar-pos"
          style={{
            transform: `${dragging === PlayBarDraggable.SEEK ? "scale(2)" : ""}`,
          }}
          onMouseDown={(e) => startDrag(e, PlayBarDraggable.SEEK)}
        ></div>
      </Tooltip>
      <Tooltip
        text="start"
        showCondition={dragging === null}
        style={{
          left: `${startPercentage * 100}%`,
          top: "6px",
          position: "absolute",
        }}
      >
        <div
          className="playbar-bound"
          style={{
            backgroundColor: "green",
            transform: `${dragging === PlayBarDraggable.START ? "scale(1.5)" : ""}`,
          }}
          onMouseDown={(e) => startDrag(e, PlayBarDraggable.START)}
        ></div>
      </Tooltip>
      <Tooltip
        text="end"
        showCondition={dragging === null}
        style={{
          left: `${endPercentage * 100}%`,
          top: "-6px",
          position: "absolute",
        }}
      >
        <div
          className="playbar-bound"
          style={{
            backgroundColor: "red",
            transform: `${dragging === PlayBarDraggable.END ? "scale(1.5)" : ""}`,
          }}
          onMouseDown={(e) => startDrag(e, PlayBarDraggable.END)}
        ></div>
      </Tooltip>
    </div>
  );
};
export default PlayBar;
