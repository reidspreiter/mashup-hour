import * as Tone from "tone"

// TODO: Create own player instead of relying on a wrapper of Tone's
// It should be straightforward to use with Tone features
// A lot of the position workarounds would not have to occur if a custom player was created

export class Player {
    readonly name: string;

    private previewUrl: string;
    private blob: Blob;
    private player: Tone.Player;

    private playbackSpeedPitcher: Tone.PitchShift;
    private consistentPitch: boolean = false;

    private _startBound: number = 0;
    private _endBound: number = 0;

    private _position: number = 0;
    private timeReference: number = 0;
    private _onPositionUpdate?: (position: number) => void;
    private positionUpdateIntervalId?: number;

    public restartOnPause: boolean = false;

    constructor(name: string, preview: string, setIsPlaying: React.Dispatch<React.SetStateAction<boolean>>) {
        this.name = name;

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
        this.player.onstop = () => {
            setIsPlaying(false);
            this.refreshPosition(true);
        }

        this.player.buffer.onload = () => {
            this._endBound = this.player.buffer.duration;
        }
    }

    public togglePlayer = (isEnabled: boolean) => {
        if (isEnabled) {
            Tone.start();
            if (this.restartOnPause) {
                this._position = (this.reverse ? this._endBound : this._startBound) * 1000;
            } else {
                this._position = this._position === 0 && this.reverse ? this._endBound * 1000 : this._position;
            }

            this.player.start(undefined, this.reverse ? this.duration - this._position / 1000 : this._position / 1000);

            this.timeReference = performance.now();
            if (this.positionUpdateIntervalId === undefined) {
                this.positionUpdateIntervalId = setInterval(() => {
                    this.refreshPosition();
                }, 500);
            }
        } else {
            clearInterval(this.positionUpdateIntervalId)
            this.positionUpdateIntervalId = undefined;
            this.player.stop();
        }
    }

    set onPositionUpdate(onPositionUpdate: (position: number) => void) {
        this._onPositionUpdate = onPositionUpdate;
    }

    set loop(loop: boolean) {
        this.player.loop = loop;
    }

    get reverse(): boolean {
        return this.player.reverse
    }

    set reverse(reverse: boolean) {
        this.player.reverse = reverse;
        this.refreshPosition();
    }

    set playbackRate(rate: number) {
        if (this.consistentPitch) {
            this.playbackSpeedPitcher.pitch = -12 * Math.log2(rate);
        }
        this.player.playbackRate = rate;
        this.refreshPosition();
    }

    set volume(volume: number) {
        this.player.volume.value = volume;
    }

    set mute(mute: boolean) {
        this.player.mute = mute;
    }

    set pitchConsistentPlayback(pitchConsistentPlayback: boolean) {
        this.consistentPitch = pitchConsistentPlayback;
        if (pitchConsistentPlayback) {
            // this.player.connect(this.playbackSpeedPitcher);
            this.playbackSpeedPitcher.pitch = -12 * Math.log2(this.player.playbackRate);
        } else {
            // this.player.toDestination().disconnect(this.playbackSpeedPitcher)
            this.playbackSpeedPitcher.pitch = 0;
        }
    }

    get isStarted(): boolean {
        return this.player.state === "started";
    }

    get duration(): number {
        return this.player.buffer.duration;
    }

    set startBound(start: number) {
        this.player.loopStart = start;
        this._startBound = start;
        this.refreshPosition();
    }

    set endBound(end: number) {
        this.player.loopEnd = end;
        this._endBound = end;
        this.refreshPosition();
    }

    public setBounds = (start: number, end: number) => {
        this.player.setLoopPoints(start, end);
        this._startBound = start;
        this._endBound = end;
        this.refreshPosition();
    }

    private refreshPosition(forceRefresh: boolean = false) {
        if (this.isStarted || forceRefresh) {
            const now = performance.now();
            if (this.reverse) {
                this._position -= (now - this.timeReference) * this.player.playbackRate;
                if (this._position <= this._startBound * 1000) {
                    this._position = (this._endBound * 1000) - ((this._startBound * 1000) - this._position);
                }
            } else {
                this._position += (now - this.timeReference) * this.player.playbackRate;
                if (this._position >= this._endBound * 1000) {
                    this._position = (this._startBound * 1000) + (this._position - (this._endBound * 1000));
                }
            }
            this.timeReference = now;
        } else if (!this.isStarted && this.restartOnPause) {
            this._position = (this.reverse ? this._endBound : this._startBound) * 1000;
        }
        console.log(this._position / 1000);
        this._onPositionUpdate?.(this._position / 1000);
    }

    get position(): number {
        return this._position / 1000;
    }
}

