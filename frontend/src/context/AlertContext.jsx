import { createContext, useContext, useState, useCallback } from "react";
import AlertContainer from "./components/AlertContainer";

const AlertContext = createContext();

export function AlertProvider({ children }) {
    const [alerts, setAlerts] = useState([]);

    const addAlert = useCallback((type, title, description, duration = 4000) => {
        const id = Date.now() + Math.random();
        setAlerts(prev => [...prev, { id, type, title, description, duration }]);
    }, []);

    const alertSuccess = useCallback((title, description, duration) => {
        addAlert("success", title, description, duration);
    }, [addAlert]);

    const alertError = useCallback((title, description, duration) => {
        addAlert("error", title, description, duration);
    }, [addAlert]);

    const alertInfo = useCallback((title, description, duration) => {
        addAlert("info", title, description, duration);
    }, [addAlert]);

    const removeAlert = useCallback((id) => {
        setAlerts(prev => prev.filter(a => a.id !== id));
    }, []);

    return (
        <AlertContext.Provider value={{ alertSuccess, alertError, alertInfo }}>
            {children}
            <AlertContainer alerts={alerts} removeAlert={removeAlert} />
        </AlertContext.Provider>
    );
}

export function useAlert() {
    return useContext(AlertContext);
}
