import { createContext, useState, useEffect } from 'react';
import { MashupAssetsSchema, MashupAssets } from "./schemas/mashup-hour";
import Mashup from './components/mashup/Mashup';
import { Track, TrackSide } from "./components/track";
import { data } from './sampleData';
import "./app.css";

function App() {

  const [assets, setAssets] = useState<MashupAssets[]>(data);
  const [trackIndex, setTrackIndex] = useState<number>(0);
  const [loading, setLoading] = useState<boolean>(true); // New state to track loading
  const trackLimit = 3;

  const retrieveAssets = async () => {
    try {
      // const response = await fetch('http://127.0.0.1:8080/retrieve-assets');
      // const data = await response.json();
      // console.log(data);
      // const _assets = MashupAssetsSchema.array().parse(data);
      // setAssets(_assets);
      setLoading(false);
    } catch (error) {
      console.error("Error fetching data", error);
    }
  };

  useEffect(() => {
    retrieveAssets();
  }, [])

  if (loading) {
    return <div>Loading...</div>
  }


  return (
    <div className="app-container">
      <Track assets={assets} trackIndex={trackIndex} trackSide={TrackSide.LEFT} />
      <Mashup assets={assets} trackIndex={trackIndex} trackLimit={trackLimit} setTrackIndex={setTrackIndex} />
      <Track assets={assets} trackIndex={trackIndex} trackSide={TrackSide.RIGHT} />
    </div>
  )
}

export default App
