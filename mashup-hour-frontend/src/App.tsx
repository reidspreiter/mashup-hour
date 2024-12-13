import { useEffect, useState } from "react";
import "./app.css";
import Mashup from "./components/mashup/Mashup";
import { Track } from "./components/track";
import { data } from "./sampleData";
import { MashupAssets } from "./schemas/mashup-hour";

function App() {
  const [assets, setAssets] = useState<MashupAssets[]>(data);
  const [trackIndex, setTrackIndex] = useState<number>(0);
  const [loading, setLoading] = useState<boolean>(true); // New state to track loading
  const trackLimit = 3;

  const retrieveAssets = async () => {
    try {
      // const response = await fetch('http://127.0.0.1:8080/retrieve-assets');
      // const data = await response.json();
      // const _assets = MashupAssetsSchema.array().parse(data);
      // setAssets(_assets);
      setLoading(false);
    } catch (error) {
      console.error("Error fetching data", error);
    }
  };

  useEffect(() => {
    retrieveAssets();
  }, []);

  if (loading) {
    return <div>Loading...</div>;
  }

  return (
    <div className="app-container">
      <Track trackAssets={assets.map((asset) => asset.track1)} trackIndex={trackIndex} />
      <Mashup
        mashedTrackAssets={assets.map((asset) => asset.mashedTrack)}
        trackIndex={trackIndex}
        trackLimit={trackLimit}
        setTrackIndex={setTrackIndex}
      />
      <Track trackAssets={assets.map((asset) => asset.track2)} trackIndex={trackIndex} />
    </div>
  );
}

export default App;
