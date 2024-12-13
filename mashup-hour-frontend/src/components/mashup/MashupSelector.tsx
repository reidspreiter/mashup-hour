import { IoIosSkipBackward, IoIosSkipForward } from "react-icons/io";
import { Button, PlayButton } from "../controllers";

interface MashupSelectorProps {
  trackIndex: number;
  setTrackIndex: React.Dispatch<React.SetStateAction<number>>;
  trackLimit: number;
}

const MashupSelector: React.FC<MashupSelectorProps> = ({ trackIndex, setTrackIndex, trackLimit }) => {
  const temp = (isEnabled: boolean) => {};

  return (
    <section className="main-column-section">
      <Button
        name="previous tracks"
        icon={IoIosSkipBackward}
        onClick={() => setTrackIndex(trackLimit + trackIndex - 1)}
      />
      <PlayButton onClick={temp} />
      <Button name="next tracks" icon={IoIosSkipForward} onClick={() => setTrackIndex((trackIndex + 1) % trackLimit)} />
    </section>
  );
};
export default MashupSelector;
