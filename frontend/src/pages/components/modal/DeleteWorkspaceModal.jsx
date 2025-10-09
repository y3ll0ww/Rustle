import { BanIcon, TrashIcon } from "lucide-react";
import Modal from "../../../components/Modal";

export default function DeleteWorkspaceModal({ isOpen, onClose, onSubmit }) {
    function ModalContent() {
        return <div>
            <p>Deleting this workspace will result in irrecoverable loss of data.</p>
            <div className="modal-buttons">
                <button className="btn-secondary" onClick={onClose}>
                    <BanIcon />
                    Cancel
                </button>
                <button className="btn-abort" onClick={onSubmit}>
                    <TrashIcon />
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
