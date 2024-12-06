import { useState } from "react"
import { EnablerFunc } from "./types"
import Tooltip from "./Tooltip"
import "./styles/controllers.css"

interface SimpleSwitchProps {
    name: string,
    onClick: EnablerFunc,
    enabledName?: string,
}

const SimpleSwitch: React.FC<SimpleSwitchProps> = ({ name, onClick, enabledName }) => {
    const [isEnabled, setIsEnabled] = useState<boolean>(false);
    const nameWhenEnabled = enabledName || `disable ${name}`

    return (
        <Tooltip text={isEnabled ? nameWhenEnabled : name}>
            <button className={`controller circle-button ${isEnabled ? " enabled-bg" : ""}`} onClick={() => {
                onClick(!isEnabled);
                setIsEnabled(!isEnabled);
            }}>
            </button>
        </Tooltip>
    )
}
export default SimpleSwitch