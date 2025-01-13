import { TrackAsset } from "../../schemas/mashup-hour";
import { TextScrollOnOverflow } from "../common";

interface TrackInfoProps {
  trackAssets: TrackAsset[];
  trackIndex: number;
}

const TrackInfo: React.FC<TrackInfoProps> = ({ trackAssets, trackIndex }) => {
  const track = trackAssets[trackIndex];
  return (
    <section>
      <TextScrollOnOverflow textClass="title">{track.title}</TextScrollOnOverflow>
      <TextScrollOnOverflow textClass="subtitle">{track.artist}</TextScrollOnOverflow>
    </section>
  );
};
export default TrackInfo;
