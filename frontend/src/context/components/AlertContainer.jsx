import "../../style/alert.css";
import Alert from "./Alert";

export default function AlertContainer({ alerts, removeAlert }) {
  return (
    <div className="alert-container">
      {alerts.map(alert => (
        <Alert key={alert.id} {...alert} onClose={removeAlert} />
      ))}
    </div>
  );
}
