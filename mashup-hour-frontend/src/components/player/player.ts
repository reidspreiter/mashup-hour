import * as Tone from "tone";

// TODO: Create own player instead of relying on a wrapper of Tone's
// It should be straightforward to use with Tone features
// A lot of the position workarounds would not have to occur if a custom player was created

type PositionCallback = (percent: number) => void;

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
  private positionUpdateIntervalId?: number;
  private ignoreStopCounter: number = 0;
  private stopTriggeredManually: boolean = false;
  private isStarted: boolean = false;

  private _onPositionUpdate?: PositionCallback;
  private _onEndBoundUpdate?: PositionCallback;
  private _onStartBoundUpdate?: PositionCallback;

  public restartOnPause: boolean = false;
  public reverseRelativeToEnd: boolean = false;

  constructor(name: string, preview: string) {
    this.name = name;

    const binaryPreview = atob(preview);
    const arrayBuffer = new ArrayBuffer(binaryPreview.length);
    const uint8Array = new Uint8Array(arrayBuffer);
    for (let i = 0; i < binaryPreview.length; i++) {
      uint8Array[i] = binaryPreview.charCodeAt(i);
    }
    this.blob = new Blob([arrayBuffer], { type: "audio/mp3" });
    this.previewUrl = URL.createObjectURL(this.blob);

    this.playbackSpeedPitcher = new Tone.PitchShift({
      pitch: 0,
      windowSize: 0.1,
    }).toDestination();

    this.player = new Tone.Player(this.previewUrl).connect(this.playbackSpeedPitcher);
    this.player.buffer.onload = () => {
      this._endBound = this.player.buffer.duration;
    };
  }

  set onStop(onStop: () => void) {
    this.player.onstop = () => {
      if (this.ignoreStopCounter > 0) {
        this.ignoreStopCounter--;
      } else {
        onStop();
        clearInterval(this.positionUpdateIntervalId);
        this.positionUpdateIntervalId = undefined;
        this.refreshPositionStopped(this.stopTriggeredManually);
        this.stopTriggeredManually = false;
        this.isStarted = false;
      }
    };
  }

  public togglePlayer = (isEnabled: boolean) => {
    if (isEnabled) {
      Tone.start();
      this._position = this._position === 0 && this.reverse ? this._endBound * 1000 : this._position;
      this.isStarted = true;

      this.player.start(undefined, this.reverse ? this.duration - this._position / 1000 : this._position / 1000);

      this.timeReference = performance.now();
      clearInterval(this.positionUpdateIntervalId);
      this.positionUpdateIntervalId = setInterval(() => {
        this.refreshPositionStarted();
      }, 500);
    } else {
      this.stopTriggeredManually = true;
      this.player.stop();
    }
  };

  private seek(position: number) {
    // Tone.Player calls the onStop function when seeking
    // this.isSeeking is reset within the onStop callback to ensure it is called before isSeeking is reset
    this.ignoreStopCounter++;
    this.player.seek(this.reverse ? this.duration - position : position);
    this.timeReference = performance.now();
    this._position = position * 1000;
    this._onPositionUpdate?.(position);
  }

  public restart = () => {
    this.seek(this.reverse ? this._endBound : this._startBound);
  };

  public randomizeBounds = () => {
    const durationPercent = Math.random();
    const startPercent = Math.random() * (1 - durationPercent);
    const endPercent = startPercent + durationPercent;
    this.setBounds(startPercent, endPercent);
    if (this.isStarted) {
      this.seek(this.reverse ? this._endBound : this._startBound);
    }
  };

  public randomizeBoundPosition = () => {
    const durationPercent = (this._endBound - this._startBound) / this.duration;
    const startPercent = Math.random() * (1 - durationPercent);
    const endPercent = startPercent + durationPercent;
    this.setBounds(startPercent, endPercent);
    if (this.isStarted) {
      this.seek(this.reverse ? this._endBound : this._startBound);
    }
  };

  set onPositionUpdate(onPositionUpdate: PositionCallback) {
    this._onPositionUpdate = onPositionUpdate;
  }

  set onEndBoundUpdate(onEndBoundUpdate: PositionCallback) {
    this._onEndBoundUpdate = onEndBoundUpdate;
  }

  set onStartBoundUpdate(onStartBoundUpdate: PositionCallback) {
    this._onStartBoundUpdate = onStartBoundUpdate;
  }

  set loop(loop: boolean) {
    this.player.loop = loop;
  }

  get reverse(): boolean {
    return this.player.reverse;
  }

  set reverse(reverse: boolean) {
    this.player.reverse = reverse;
    if (this.reverseRelativeToEnd) {
      this._position = this.duration * 1000 - this._position;
      this.timeReference = performance.now();
      this._onPositionUpdate?.(this._position / 1000 / this.duration);
    } else {
      this.seek(this._position / 1000);
    }
  }

  set playbackRate(rate: number) {
    if (this.consistentPitch) {
      this.playbackSpeedPitcher.pitch = -12 * Math.log2(rate);
    }
    this.player.playbackRate = rate;
    this.refreshPositionStartedNoUpdate();
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

  get duration(): number {
    return this.player.buffer.duration;
  }

  set startBound(percent: number) {
    this.player.loopStart = percent * this.duration;
    this._startBound = percent * this.duration;
    this._onStartBoundUpdate?.(percent);
    this.refreshPositionStarted();
    if (!this.isStarted && !this.reverse) {
      this._position = this._startBound * 1000;
    }
  }

  set endBound(percent: number) {
    this.player.loopEnd = percent * this.duration;
    this._endBound = percent * this.duration;
    this._onEndBoundUpdate?.(percent);
    this.refreshPositionStarted();
    if (!this.isStarted && this.reverse) {
      this._position = this._endBound * 1000;
    }
  }

  public setBounds = (startPercent: number, endPercent: number) => {
    const duration = this.duration;
    this.player.setLoopPoints(startPercent * duration, endPercent * duration);
    this._startBound = startPercent * duration;
    this._endBound = endPercent * duration;
    this._onStartBoundUpdate?.(startPercent);
    this._onEndBoundUpdate?.(endPercent);
    this.refreshPositionStarted();
  };

  private refreshPositionStartedNoUpdate(force: boolean = false) {
    if (this.isStarted || force) {
      const now = performance.now();
      if (this.reverse) {
        this._position -= (now - this.timeReference) * this.player.playbackRate;
        if (this._position <= this._startBound * 1000) {
          this._position = this._endBound * 1000 - (this._startBound * 1000 - this._position);
        }
      } else {
        this._position += (now - this.timeReference) * this.player.playbackRate;
        if (this._position >= this._endBound * 1000) {
          this._position = this._startBound * 1000 + (this._position - this._endBound * 1000);
        }
      }
      this.timeReference = now;
    }
  }

  private refreshPositionStopped(triggeredManually: boolean) {
    if (triggeredManually && !this.restartOnPause) {
      this.refreshPositionStarted(true);
    } else {
      this._position = (this.reverse ? this._endBound : this._startBound) * 1000;
      this._onPositionUpdate?.(this._position / 1000 / this.duration);
    }
  }

  // probably get rid of force once bounds behavior is handled
  private refreshPositionStarted(force: boolean = false) {
    this.refreshPositionStartedNoUpdate(force);
    this._onPositionUpdate?.(this._position / 1000 / this.duration);
  }

  get position(): number {
    return this._position / 1000;
  }
}
