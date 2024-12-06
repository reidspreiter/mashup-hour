import { MashedTrackAsset } from "../../schemas/mashup-hour"


interface MashupInfoProps {
    mashedTrackAssets: MashedTrackAsset[],
    trackIndex: number,
}

const MashupInfo: React.FC<MashupInfoProps> = ({ mashedTrackAssets, trackIndex }) => {
    const mashedTrack = mashedTrackAssets[trackIndex]
    return (
        <section>
            <h2 className="title">{mashedTrack.title}</h2>
            <p className="artist">{mashedTrack.artist}</p>
        </section>
    )
}
export default MashupInfo