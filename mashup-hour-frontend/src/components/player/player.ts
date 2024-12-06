import * as Tone from "tone"

export class Player {
    private previewUrl: string;
    private blob: Blob;
    private player: Tone.Player;
    private playbackSpeedPitcher: Tone.PitchShift;
    private playbackSpeedPitcherEnabled: boolean = false;

    constructor(preview: string, setIsPlaying: React.Dispatch<React.SetStateAction<boolean>>) {
        console.log("Constructing")
        const binaryPreview = atob(preview);
        const arrayBuffer = new ArrayBuffer(binaryPreview.length);
        const uint8Array = new Uint8Array(arrayBuffer);
        for (let i = 0; i < binaryPreview.length; i++) {
            uint8Array[i] = binaryPreview.charCodeAt(i);
        }
        this.blob = new Blob([arrayBuffer], { type: 'audio/mp3' });
        this.previewUrl = URL.createObjectURL(this.blob);

        this.playbackSpeedPitcher = new Tone.PitchShift({
            pitch: 0,
            windowSize: 0.1,
        }).toDestination();
        this.player = new Tone.Player(this.previewUrl).connect(this.playbackSpeedPitcher);
        this.player.onstop = () => setIsPlaying(false);
    }

    public togglePlayer = (isEnabled: boolean) => {
        console.log(`toggling player ${this.previewUrl}`)
        if (isEnabled) {
            Tone.start();
            // this.player.start(undefined, this.player.loopStart, Tone.Time(this.player.loopEnd).toSeconds() - Tone.Time(this.player.loopStart).toSeconds());
            this.player.start(undefined, this.player.loopStart);
        } else {
            this.player.stop();
        }
    }

    public toggleLoop = (isEnabled: boolean) => {
        console.log("toggle loop is called")
        this.player.loop = isEnabled;
    }

    public toggleReverse = (isEnabled: boolean) => {
        this.player.reverse = isEnabled;
    }

    public adjustPlaybackSpeed = (newValue: number) => {
        if (this.playbackSpeedPitcherEnabled) {
            this.playbackSpeedPitcher.pitch = -12 * Math.log2(newValue);
            console.log(this.playbackSpeedPitcher.pitch);
            console.log(newValue);
        }
        this.player.playbackRate = newValue;

    }

    public adjustVolume = (newValue: number) => {
        this.player.volume.value = newValue;
    }

    public toggleMute = (isEnabled: boolean) => {
        if (isEnabled) {
            this.player.mute = true;
        } else {
            this.player.mute = false;
        }
    }

    public togglePitchConsistentPlayback = (isEnabled: boolean) => {
        this.playbackSpeedPitcherEnabled = isEnabled;
        if (isEnabled) {
            // this.player.connect(this.playbackSpeedPitcher);
            this.playbackSpeedPitcher.pitch = -12 * Math.log2(this.player.playbackRate);
        } else {
            // this.player.toDestination().disconnect(this.playbackSpeedPitcher)
            this.playbackSpeedPitcher.pitch = 0;
        }
    }

    public isStarted = (): boolean => {
        return this.player.state === "started"
    }

    public totalDuration = (): number => {
        return this.player.buffer.duration;
    }

    public setStart = (start: number) => {
        this.player.loopStart = start;
    }

    public setEnd = (end: number) => {
        this.player.loopEnd = end;
    }
}