import "../../style/alert.css";
import { useEffect, useState } from "react";

export default function Alert({ id, type, title, description, duration, onClose }) {
  const [closing, setClosing] = useState(false);
  const [hovering, setHovering] = useState(false);
  const [progress, setProgress] = useState(100);

  useEffect(() => {
    if (hovering) return;
    const interval = setInterval(() => {
      setProgress(p => {
        if (p <= 0) {
          handleClose();
          return 0;
        }
        return p - 100 / (duration / 100);
      });
    }, 100);
    return () => clearInterval(interval);
  }, [hovering, duration]);

  const handleClose = () => {
    setClosing(true);
    setTimeout(() => onClose(id), 300);
  };

  return (
    <div
      className={`alert alert-${type} ${closing ? "slide-out" : ""}`}
      onMouseEnter={() => setHovering(true)}
      onMouseLeave={() => setHovering(false)}
    >
      <span style={{ fontWeight: "bold" }}>{title}</span>
      <br/>
      <span>{description}</span>

      <button className="alert-close" onClick={handleClose}>
        ×
      </button>
      <div
        className="alert-progress"
        style={{ width: `${progress}%` }}
      />
    </div>
  );
}
