import { useEffect, useMemo } from "react";
import { TrackInfo, TrackManipulator, TrackPlayer } from ".";
import { Player } from "../../player";
import { TrackAsset } from "../../schemas/mashup-hour";

interface TrackProps {
  trackAssets: TrackAsset[];
  trackIndex: number;
}

const Track: React.FC<TrackProps> = ({ trackAssets, trackIndex }) => {
  const player = useMemo(() => {
    return new Player(trackAssets[trackIndex].title, trackAssets[trackIndex].preview);
  }, [trackAssets, trackIndex]);

  useEffect(() => {
    return () => {
      player.togglePlayer(false);
    };
  }, [player]);

  return (
    <div className="main-column" onContextMenu={(e) => e.preventDefault()}>
      <TrackInfo trackAssets={trackAssets} trackIndex={trackIndex} />
      <TrackManipulator />
      <TrackPlayer player={player} />
    </div>
  );
};
export default Track;
