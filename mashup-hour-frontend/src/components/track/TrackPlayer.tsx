import { Switch, Knob, PlayButton, SimpleSwitch, Casing, PressAndHold, PlayBar, InvisibleCasing } from "../controllers"
import { IoRepeatSharp, IoArrowBackSharp, IoVolumeMedium, IoSpeedometer } from "react-icons/io5"
import { Player } from "../player"

interface TrackPlayerProps {
    player: Player;
    isPlaying: boolean;
    setIsPlaying: React.Dispatch<React.SetStateAction<boolean>>;
}

const TrackPlayer: React.FC<TrackPlayerProps> = ({ player, isPlaying, setIsPlaying }) => {
    console.log("Player rerendering")
    return (
        <section>
            <div className="main-column-section" style={{ marginBottom: "10px" }}><PlayBar player={player} /></div>
            <div className="main-column-section">
                <Casing sub1={<SimpleSwitch name="preserve pitch" onClick={player.togglePitchConsistentPlayback} />}><Knob name="speed" min={0.1} max={3} mid={1} onChange={player.adjustPlaybackSpeed} icon={IoSpeedometer} /></Casing>
                <Casing sub2={<SimpleSwitch name="lock playbar bounds" onClick={() => { }} />}><Switch name="reverse" icon={IoArrowBackSharp} onClick={player.toggleReverse} /></Casing>
                <InvisibleCasing><PlayButton onClick={player.togglePlayer} isPlaying={isPlaying} setIsPlaying={setIsPlaying} /></InvisibleCasing>
                <InvisibleCasing><Switch name="loop" icon={IoRepeatSharp} onClick={player.toggleLoop} /></InvisibleCasing>
                <Casing sub1={<SimpleSwitch name="mute" onClick={player.toggleMute} enabledName="unmute" />} sub2={<PressAndHold name="mute on hold" onClick={player.toggleMute} />}><Knob name="volume" min={-40} mid={-4} max={20} onChange={player.adjustVolume} icon={IoVolumeMedium} /></Casing>

            </div>
        </section >
    )
}
export default TrackPlayer