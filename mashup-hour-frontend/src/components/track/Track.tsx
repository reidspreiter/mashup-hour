import { MashupAssets } from "../../schemas/mashup-hour"
import { TrackInfo, TrackManipulator, TrackSampler, TrackPlayer, TrackSide } from ".";
import { useState, useEffect, useMemo } from "react"
import { Player } from "../player"

interface TrackProps {
    assets: MashupAssets[],
    trackIndex: number,
    trackSide: TrackSide,
}

const Track: React.FC<TrackProps> = ({ assets, trackIndex, trackSide }) => {
    const [isPlaying, setIsPlaying] = useState<boolean>(false);

    const trackAssets = useMemo(() => {
        return assets.map((asset) => trackSide === TrackSide.LEFT ? asset.track1 : asset.track2);
    }, [assets, trackSide]);

    const player = useMemo(() => {
        return new Player(trackAssets[trackIndex].title, trackAssets[trackIndex].preview, setIsPlaying);
    }, [trackAssets, trackIndex]);

    useEffect(() => {
        return () => {
            player.togglePlayer(false);
            setIsPlaying(false);
            console.log("cleaning");
        }
    }, [player]);

    return (
        <div className="main-column">
            <TrackInfo trackAssets={trackAssets} trackIndex={trackIndex} />
            <TrackSampler />
            <TrackManipulator />
            <TrackPlayer player={player} isPlaying={isPlaying} setIsPlaying={setIsPlaying} />
        </div>
    )
}
export default Track