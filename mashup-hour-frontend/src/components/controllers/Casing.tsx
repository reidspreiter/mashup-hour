import { ReactNode } from "react"

interface CasingProps {
    children: ReactNode
    sub1?: ReactNode
    sub2?: ReactNode
    sub3?: ReactNode
}

const Casing: React.FC<CasingProps> = ({ children, sub1, sub2, sub3 }) => {
    return (
        <div className="casing">
            <div className="subcasing">
                {sub1}
                {sub2}
                {sub3}
            </div>
            {children}
        </div>
    )
}
export default Casing