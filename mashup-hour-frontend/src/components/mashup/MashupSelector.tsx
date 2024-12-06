import { IoIosSkipBackward, IoIosSkipForward } from "react-icons/io";
import { PlayButton, Button } from "../controllers";
import { useState } from "react";

interface MashupSelectorProps {
    trackIndex: number,
    setTrackIndex: React.Dispatch<React.SetStateAction<number>>,
    trackLimit: number,
}

const MashupSelector: React.FC<MashupSelectorProps> = ({ trackIndex, setTrackIndex, trackLimit }) => {
    const [isPlaying, setIsPlaying] = useState<boolean>(false);

    const temp = (isEnabled: boolean) => {
        ;
    }

    return (
        <section className="main-column-section">
            <Button name="previous tracks" icon={IoIosSkipBackward} onClick={() => setTrackIndex((trackLimit + trackIndex - 1))} />
            <PlayButton onClick={temp} isPlaying={isPlaying} setIsPlaying={setIsPlaying} />
            <Button name="next tracks" icon={IoIosSkipForward} onClick={() => setTrackIndex((trackIndex + 1) % trackLimit)} />
        </section>
    )
}
export default MashupSelector