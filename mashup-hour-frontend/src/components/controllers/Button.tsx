import { IconType } from "react-icons"
import Tooltip from "./Tooltip"
import React from "react"
import "./styles/controllers.css"

interface ButtonProps {
    name: string,
    icon: IconType,
    onClick: () => void,
}

const Button: React.FC<ButtonProps> = ({ name, icon, onClick }) => {
    return (
        <Tooltip text={name}>
            <button className="controller" onClick={() => onClick()}>
                {React.createElement(icon, { className: "controller-icon icon-medium" })}
            </button>
        </Tooltip>
    )
}
export default Button