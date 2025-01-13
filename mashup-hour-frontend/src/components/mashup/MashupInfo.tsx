import { MashedTrackAsset } from "../../schemas/mashup-hour";
import { TextScrollOnOverflow } from "../common";

interface MashupInfoProps {
  mashedTrackAssets: MashedTrackAsset[];
  trackIndex: number;
}

const MashupInfo: React.FC<MashupInfoProps> = ({ mashedTrackAssets, trackIndex }) => {
  const mashedTrack = mashedTrackAssets[trackIndex];
  return (
    <section>
      <TextScrollOnOverflow textClass="title">{mashedTrack.title}</TextScrollOnOverflow>
      <TextScrollOnOverflow textClass="subtitle">{mashedTrack.artist}</TextScrollOnOverflow>
    </section>
  );
};
export default MashupInfo;
