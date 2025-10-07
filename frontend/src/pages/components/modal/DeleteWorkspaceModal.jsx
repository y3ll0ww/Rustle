import { BanIcon, SaveIcon } from "lucide-react";
import Modal from "./Modal";

export default function DeleteWorkspaceModal({ isOpen, onClose, onSubmit }) {
    function ModalContent() {
        return <div>
            <p>Deleting this workspace will result in irrecoverable loss of data.</p>
            <div className="modal-buttons">
                <button className="btn-secondary" onClick={onClose}>
                    <BanIcon />
                    Cancel
                </button>
                <button className="btn-primary" onClick={onSubmit}>
                    <SaveIcon />
                    Delete
                </button>
            </div>
        </div>
    }

    return <Modal
        title="Are you sure you want to delete this workspace?"
        content={<ModalContent />}
        isOpen={isOpen}
        onClose={onClose}
    />;
}
