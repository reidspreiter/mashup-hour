import { MashupAssets } from "../../schemas/mashup-hour";
import MashupInfo from "./MashupInfo";
import MashupSelector from "./MashupSelector";
import MashupVisualizer from "./MashupVisualizer";

interface MashupProps {
    assets: MashupAssets[],
    trackIndex: number,
    setTrackIndex: React.Dispatch<React.SetStateAction<number>>,
    trackLimit: number,
}

const Mashup: React.FC<MashupProps> = ({ assets, trackIndex, setTrackIndex, trackLimit }) => {
    const mashedTrackAssets = assets.map((asset) => asset.mashedTrack);
    return (
        <div className="main-column">
            <MashupInfo mashedTrackAssets={mashedTrackAssets} trackIndex={trackIndex} />
            <MashupVisualizer />
            <MashupSelector trackIndex={trackIndex} setTrackIndex={setTrackIndex} trackLimit={trackLimit} />
        </div>
    );
};
export default Mashup;