import { useEffect, useState } from "react";
import { IoPauseCircleSharp, IoPlayCircleSharp } from "react-icons/io5";
import { Player } from "../../player";
import Tooltip from "./Tooltip";
import "./styles/controllers.css";
import { EnablerFunc } from "./types";

interface PlayButtonProps {
  player?: Player;
  onClick: EnablerFunc;
}

const PlayButton: React.FC<PlayButtonProps> = ({ onClick, player }) => {
  const [isPlaying, setIsPlaying] = useState<boolean>(false);

  useEffect(() => {
    if (player !== undefined) {
      player.onStop = () => setIsPlaying(false);
    }
  }, [player]);

  return (
    <Tooltip text={isPlaying ? "pause" : "play"}>
      <button
        className="controller"
        onClick={() => {
          setIsPlaying(!isPlaying);
          onClick(!isPlaying);
        }}
      >
        {isPlaying ? (
          <IoPauseCircleSharp className="play-button-icon controller-icon" />
        ) : (
          <IoPlayCircleSharp className="play-button-icon controller-icon" />
        )}
      </button>
    </Tooltip>
  );
};
export default PlayButton;
