import { MashupInfo, MashupSelector, MashupVisualizer } from ".";
import { MashedTrackAsset } from "../../schemas/mashup-hour";

interface MashupProps {
  mashedTrackAssets: MashedTrackAsset[];
  trackIndex: number;
  setTrackIndex: React.Dispatch<React.SetStateAction<number>>;
  trackLimit: number;
}

const Mashup: React.FC<MashupProps> = ({ mashedTrackAssets, trackIndex, setTrackIndex, trackLimit }) => {
  return (
    <div className="main-column">
      <MashupInfo mashedTrackAssets={mashedTrackAssets} trackIndex={trackIndex} />
      <MashupVisualizer />
      <MashupSelector trackIndex={trackIndex} setTrackIndex={setTrackIndex} trackLimit={trackLimit} />
    </div>
  );
};
export default Mashup;
