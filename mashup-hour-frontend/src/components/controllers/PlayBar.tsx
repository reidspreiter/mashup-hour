import { useCallback, useEffect, useRef, useState } from "react"
import { Player } from "../player";
import Tooltip from "./Tooltip";
import { clamp } from "../util";
import "./styles/controllers.css"

interface PlayBarProps {
    player: Player,
}

enum PlayBarDraggable {
    START,
    END,
    POS,
}

const posWidth = 16;
const boundWidth = 10;

const PlayBar: React.FC<PlayBarProps> = ({ player }) => {
    const totalDuration = player.totalDuration();
    console.log(totalDuration);
    const [isHovering, setIsHovering] = useState<boolean>(false);
    const [playPos, setPlayPos] = useState<number>(0);
    const [startPos, setStartPos] = useState<number>(0);
    const [endPos, setEndPos] = useState<number>(player.totalDuration());
    console.log(endPos);
    const [dragging, setDragging] = useState<PlayBarDraggable | null>(null);
    const sliderRef = useRef<HTMLDivElement | null>(null);

    // track the players position, WIP I guess this might take a miracle to do accurately
    // useEffect(() => {
    //     const updatePlayPos = () => {
    //     }
    // })

    const stopDrag = () => {
        setDragging(null);
        document.body.classList.remove("sliding");
    }

    const startDrag = (draggable: PlayBarDraggable) => {
        setDragging(draggable);
        document.body.classList.add("sliding");
    }

    const onMouseEnter = () => {
        setIsHovering(true);
    }

    const onMouseLeave = () => {
        setIsHovering(false);
    }

    const onDrag = useCallback((e: MouseEvent) => {
        if (dragging !== null) {
            console.log(dragging)
            const widthOffset = (dragging === PlayBarDraggable.POS ? posWidth : boundWidth) / 2;
            const rect = sliderRef.current?.getBoundingClientRect();
            if (rect !== undefined) {
                const sliderWidth = rect.width;
                const mousePos = clamp(0 - widthOffset, sliderWidth - widthOffset, e.clientX - rect.left - widthOffset);
                const newPosition = (mousePos / sliderWidth) * totalDuration;

                if (dragging === PlayBarDraggable.POS) {
                    setPlayPos(newPosition);
                } else if (dragging === PlayBarDraggable.END && newPosition > startPos) {
                    player.setEnd(newPosition);
                    setEndPos(newPosition);
                } else if (dragging === PlayBarDraggable.START && newPosition < endPos) {
                    player.setStart(newPosition);
                    setStartPos(newPosition);
                }
            }
        }
    }, [dragging, totalDuration, endPos, startPos, player]);

    useEffect(() => {
        if (dragging !== null) {
            document.addEventListener('mousemove', onDrag);
            document.addEventListener('mouseup', stopDrag);
        } else {
            document.removeEventListener('mousemove', onDrag);
            document.removeEventListener('mouseup', stopDrag);
        }

        // remove event listeners when component unmounts
        return () => {
            document.removeEventListener('mousemove', onDrag);
            document.removeEventListener('mouseup', stopDrag);
        };
    }, [dragging, onDrag]);



    return (
        <div
            ref={sliderRef}
            className="playbar"
            onMouseEnter={onMouseEnter}
            onMouseLeave={onMouseLeave}
        >
            {(isHovering || dragging === PlayBarDraggable.POS) && (
                <Tooltip text="seek">
                    <div className="playbar-pos"
                        style={{
                            left: `${(playPos / totalDuration) * 100}%`,
                            width: `${posWidth}px`,
                            height: `${posWidth}px`,
                            transform: `${dragging === PlayBarDraggable.POS ? "scale(var(--scale-increase))" : ""}`
                        }}
                        onMouseDown={() => setDragging(PlayBarDraggable.POS)}
                    ></div>
                </Tooltip>
            )
            }
            <Tooltip text="start">
                <div
                    className="playbar-bound"
                    style={{
                        left: `${(startPos / totalDuration) * 100}%`,
                        top: "4px",
                        backgroundColor: 'green',
                        width: `${boundWidth}px`,
                        height: `${posWidth / 2}px`,
                        transform: `${dragging === PlayBarDraggable.START ? "scale(var(--scale-increase))" : ""}`
                    }}
                    onMouseDown={() => startDrag(PlayBarDraggable.START)}
                ></div>
            </Tooltip>
            <Tooltip text="end">
                <div
                    className="playbar-bound"
                    style={{
                        left: `${(endPos / totalDuration) * 100}%`,
                        top: "-4px",
                        backgroundColor: 'red',
                        width: `${boundWidth}px`,
                        height: `${posWidth / 2}px`,
                        transform: `${dragging === PlayBarDraggable.END ? "scale(var(--scale-increase))" : ""}`
                    }}
                    onMouseDown={() => startDrag(PlayBarDraggable.END)}
                ></div>
            </Tooltip>

            {/* <div
                playbar if I ever figure out how to do it accurately
            /> */}
        </div >
    );
}
export default PlayBar