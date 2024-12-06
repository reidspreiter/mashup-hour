import { useState, ReactNode, useRef } from "react"
import "./styles/controllers.css"

interface TooltipProps {
    children: ReactNode,
    text: string,
}

const Tooltip: React.FC<TooltipProps> = ({ text, children }) => {
    const [showTooltip, setShowTooltip] = useState(false);
    const timeoutRef = useRef<number | null>(null);

    const onMouseEnter = () => {
        timeoutRef.current = setTimeout(() => {
            setShowTooltip(true);
        }, 800)
    }

    const onMouseLeave = () => {
        if (timeoutRef.current !== null) {
            clearTimeout(timeoutRef.current);
        }
        setShowTooltip(false);
    }

    return (
        <div className="tooltip-container" onMouseEnter={onMouseEnter} onMouseLeave={onMouseLeave} onClick={onMouseLeave}>
            {children}
            {showTooltip && <div className="tooltip">{text}</div>}
        </div>
    )
}
export default Tooltip