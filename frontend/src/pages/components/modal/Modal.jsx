import "../../../style/modal.css";
import { XIcon } from "lucide-react";
import { useEffect } from "react";

export default function Modal({ title, content, isOpen, onClose }) {
    useEffect(() => {
        const handleEscape = (e) => {
            if (e.key === "Escape") onClose();
        };

        if (!isOpen) return;

        document.addEventListener("keydown", handleEscape);
        return () => document.removeEventListener("keydown", handleEscape);
    }, [isOpen, onClose]);

    if (!isOpen) return null;

    // Prevent closing when clicking inside the modal
    const handleContentClick = (e) => {
        e.stopPropagation();
    };

    return (
        <div className="modal-overlay" onClick={onClose}>
            <div
                className="content-container"
                onClick={handleContentClick}
            >
                <button
                    onClick={onClose}
                    aria-label="Close modal"
                    className="modal-close"
                >
                    <XIcon />
                </button>

                <h2 style={{ marginBottom: "2rem" }}>{title}</h2>
                {content}
            </div>
        </div>
    );
}
