import { IoPauseCircleSharp, IoPlayCircleSharp } from "react-icons/io5";
import { EnablerFunc } from "./types";
import Tooltip from "./Tooltip";
import "./styles/controllers.css"

interface PlayButtonProps {
    isPlaying: boolean,
    setIsPlaying: React.Dispatch<React.SetStateAction<boolean>>,
    onClick: EnablerFunc,
}

const PlayButton: React.FC<PlayButtonProps> = ({ onClick, isPlaying, setIsPlaying }) => {
    return (
        <Tooltip text={isPlaying ? "pause" : "play"}>
            <button className="controller" onClick={() => {
                setIsPlaying(!isPlaying);
                onClick(!isPlaying);
            }}>
                {isPlaying ? (
                    <IoPauseCircleSharp className="play-button-icon controller-icon" />
                ) : (
                    <IoPlayCircleSharp className="play-button-icon controller-icon" />
                )}</button>
        </Tooltip>
    )
}
export default PlayButton