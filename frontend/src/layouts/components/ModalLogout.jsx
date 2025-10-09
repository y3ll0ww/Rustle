import { BanIcon, LogOutIcon } from "lucide-react";
import { useAuth } from "../../context/AuthContext";
import Modal from "../../components/Modal";

export default function LogoutModal({ isOpen, onClose }) {
    const { logout } = useAuth();
    
    function ModalContent() {
        return <div
            className="modal-buttons"
            style={{ justifyContent: "center", marginTop: "-0.5rem" }}
        >
            <button className="btn-secondary" onClick={onClose} >
                <BanIcon />
                Cancel
            </button>
            <button className="btn-abort" onClick={logout}>
                <LogOutIcon />
                Logout
            </button>
        </div>
    }

    return <Modal
        title="Are you sure you want to log out?"
        content={<ModalContent />}
        isOpen={isOpen}
        onClose={onClose}
    />;
}
