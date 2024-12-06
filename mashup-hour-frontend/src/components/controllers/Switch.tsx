import React, { useState } from "react"
import { EnablerFunc } from "./types"
import { IconType } from "react-icons"
import Tooltip from "./Tooltip";
import "./styles/controllers.css"

interface SwitchProps {
    name: string,
    icon: IconType,
    onClick: EnablerFunc,
}

const Switch: React.FC<SwitchProps> = ({ name, icon, onClick }) => {
    const [isEnabled, setIsEnabled] = useState(false)

    return (
        <Tooltip text={isEnabled ? `disable ${name}` : name}>
            <button className="controller" onClick={() => {
                onClick(!isEnabled);
                setIsEnabled(!isEnabled);

            }}>
                {React.createElement(icon, { className: `controller-icon ${isEnabled ? "enabled" : ""}` })}
            </button>
        </Tooltip>
    )
}
export default Switch