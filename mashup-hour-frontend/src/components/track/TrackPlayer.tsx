import { IoArrowBackSharp, IoRepeatSharp, IoSpeedometer, IoVolumeMedium } from "react-icons/io5";
import { Player } from "../../player";
import { Casing, Knob, PlayBar, PlayButton, PressAndHold, SimpleButton, SimpleSwitch, Switch } from "../controllers";
interface TrackPlayerProps {
  player: Player;
}

const TrackPlayer: React.FC<TrackPlayerProps> = ({ player }) => {
  return (
    <section>
      <div className="main-column-section" style={{ marginBottom: "10px" }}>
        <PlayBar player={player} />
      </div>
      <div className="main-column-section">
        <Casing
          sub1={
            <SimpleSwitch
              name="preserve pitch"
              onClick={(isEnabled: boolean) => (player.pitchConsistentPlayback = isEnabled)}
            />
          }
        >
          <Knob
            name="speed"
            min={0.1}
            max={3}
            mid={1}
            onChange={(newValue: number) => (player.playbackRate = newValue)}
            icon={IoSpeedometer}
          />
        </Casing>
        <Casing sub2={<SimpleSwitch name="reflect" onClick={(isEnabled) => (player.reflect = isEnabled)} />}>
          <Switch
            name="reverse"
            icon={IoArrowBackSharp}
            onClick={(isEnabled: boolean) => (player.reverse = isEnabled)}
          />
        </Casing>
        <Casing
          sub1={<SimpleButton name="restart" onClick={player.restart} />}
          sub2={
            <SimpleSwitch
              name="restart on pause"
              onClick={(isEnabled: boolean) => (player.restartOnPause = isEnabled)}
            />
          }
        >
          <PlayButton onClick={player.togglePlayer} player={player} />
        </Casing>
        <Casing
          sub1={<SimpleButton name="randomize playbar bounds" onClick={player.randomizeBounds} />}
          sub2={<SimpleButton name="randomize bound position" onClick={player.randomizeBoundPosition} />}
        >
          <Switch name="loop" icon={IoRepeatSharp} onClick={(isEnabled: boolean) => (player.loop = isEnabled)} />
        </Casing>
        <Casing
          sub1={
            <SimpleSwitch
              name="mute"
              onClick={(isEnabled: boolean) => (player.mute = isEnabled)}
              enabledName="unmute"
            />
          }
          sub2={<PressAndHold name="mute on hold" onClick={(isEnabled: boolean) => (player.mute = isEnabled)} />}
        >
          <Knob
            name="volume"
            min={-40}
            mid={-4}
            max={20}
            onChange={(newValue: number) => (player.volume = newValue)}
            icon={IoVolumeMedium}
          />
        </Casing>
      </div>
    </section>
  );
};
export default TrackPlayer;
