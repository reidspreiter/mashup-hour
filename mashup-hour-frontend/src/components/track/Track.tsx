import { useEffect, useMemo, useState } from "react";
import { TrackInfo, TrackManipulator, TrackPlayer, TrackSide } from ".";
import { MashupAssets } from "../../schemas/mashup-hour";
import { Player } from "../player";

interface TrackProps {
  assets: MashupAssets[];
  trackIndex: number;
  trackSide: TrackSide;
}

const Track: React.FC<TrackProps> = ({ assets, trackIndex, trackSide }) => {
  const [isPlaying, setIsPlaying] = useState<boolean>(false);

  const trackAssets = useMemo(() => {
    return assets.map((asset) => (trackSide === TrackSide.LEFT ? asset.track1 : asset.track2));
  }, [assets, trackSide]);

  const player = useMemo(() => {
    return new Player(trackAssets[trackIndex].title, trackAssets[trackIndex].preview, setIsPlaying);
  }, [trackAssets, trackIndex]);

  useEffect(() => {
    return () => {
      player.togglePlayer(false);
      setIsPlaying(false);
    };
  }, [player]);

  return (
    <div className="main-column" onContextMenu={(e) => e.preventDefault()}>
      <TrackInfo trackAssets={trackAssets} trackIndex={trackIndex} />
      <TrackManipulator />
      <TrackPlayer player={player} isPlaying={isPlaying} setIsPlaying={setIsPlaying} />
    </div>
  );
};
export default Track;
