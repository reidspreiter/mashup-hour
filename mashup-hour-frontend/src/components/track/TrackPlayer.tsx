import { Switch, Knob, PlayButton, SimpleSwitch, Casing, PressAndHold, PlayBar, SimpleButton } from "../controllers"
import { IoRepeatSharp, IoArrowBackSharp, IoVolumeMedium, IoSpeedometer } from "react-icons/io5"
import { Player } from "../player"

interface TrackPlayerProps {
    player: Player;
    isPlaying: boolean;
    setIsPlaying: React.Dispatch<React.SetStateAction<boolean>>;
}

const TrackPlayer: React.FC<TrackPlayerProps> = ({ player, isPlaying, setIsPlaying }) => {
    return (
        <section>
            <div className="main-column-section" style={{ marginBottom: "10px" }}><PlayBar player={player} /></div>
            <div className="main-column-section">
                <Casing sub1={<SimpleSwitch name="preserve pitch" onClick={(isEnabled: boolean) => player.pitchConsistentPlayback = isEnabled} />}><Knob name="speed" min={0.1} max={3} mid={1} onChange={(newValue: number) => player.playbackRate = newValue} icon={IoSpeedometer} /></Casing>
                <Casing sub2={<SimpleSwitch name="lock playbar bounds" onClick={() => { }} />}><Switch name="reverse" icon={IoArrowBackSharp} onClick={(isEnabled: boolean) => player.reverse = isEnabled} /></Casing>
                <Casing sub1={<SimpleButton name="restart" onClick={() => { }} />} sub2={<SimpleSwitch name="restart on pause" onClick={() => { }} />}><PlayButton onClick={player.togglePlayer} isPlaying={isPlaying} setIsPlaying={setIsPlaying} /></Casing>
                <Casing sub1={<SimpleButton name="randomize playbar bounds" onClick={() => { }} />}><Switch name="loop" icon={IoRepeatSharp} onClick={(isEnabled: boolean) => player.loop = isEnabled} /></Casing>
                <Casing sub1={<SimpleSwitch name="mute" onClick={(isEnabled: boolean) => player.mute = isEnabled} enabledName="unmute" />} sub2={<PressAndHold name="mute on hold" onClick={(isEnabled: boolean) => player.mute = isEnabled} />}><Knob name="volume" min={-40} mid={-4} max={20} onChange={(newValue: number) => player.volume = newValue} icon={IoVolumeMedium} /></Casing>

            </div>
        </section >
    )
}
export default TrackPlayer