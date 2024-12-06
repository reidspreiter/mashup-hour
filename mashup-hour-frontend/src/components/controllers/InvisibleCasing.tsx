import { ReactNode } from "react"
import "./styles/controllers.css"

interface InvisibleCasingProps {
    children: ReactNode
}

const InvisibleCasing: React.FC<InvisibleCasingProps> = ({ children }) => {
    return (
        <div className="casing invisible-casing">
            <div className="subcasing">
                <div className="controller circle-button invisible"></div>
            </div>
            {children}
        </div>
    )
}
export default InvisibleCasing