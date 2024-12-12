import { TrackAsset } from "../../schemas/mashup-hour";

interface TrackInfoProps {
  trackAssets: TrackAsset[];
  trackIndex: number;
}

const TrackInfo: React.FC<TrackInfoProps> = ({ trackAssets, trackIndex }) => {
  const track = trackAssets[trackIndex];
  return (
    <section>
      <h2>{track.title}</h2>
      <p>{track.artist}</p>
    </section>
  );
};
export default TrackInfo;
