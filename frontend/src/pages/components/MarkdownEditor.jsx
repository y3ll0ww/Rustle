import { Ban, Save } from "lucide-react";
import "../../style/markdown.css";
import { useRef } from "react";
import ReactMarkdown from "react-markdown";
import remarkGfm from "remark-gfm";

export default function MarkdownEditor({ value, onChange, onSave, onCancel, }) {
  const inputRef = useRef(null);
  const previewRef = useRef(null);

  const handleScroll = () => {
    if (!inputRef.current || !previewRef.current) return;

    const input = inputRef.current;
    const preview = previewRef.current;

    // Ratio of scroll in textarea
    const ratio = input.scrollTop / (input.scrollHeight - input.clientHeight);

    // Apply ratio to preview
    preview.scrollTop = ratio * (preview.scrollHeight - preview.clientHeight);
  };

  return (
    <div>
      <div className="markdown-tools">
        <button className="btn-primary" onClick={onSave}>
          <Save size={16} />
          <span> Save</span>
        </button>
        <button className="btn-primary" onClick={onCancel}>
          <Ban size={16} />
          <span> Cancel</span>
        </button>
      </div>
      <div className="markdown-container">
        {/* Input area */}
        <textarea
          ref={inputRef}
          className="markdown-input"
          value={value}
          onChange={(e) => onChange(e.target.value)}
          onScroll={handleScroll}
          placeholder="Write your markdown here..."
        />

        {/* Preview */}
        <div ref={previewRef} className="markdown-preview markdown">
          <ReactMarkdown remarkPlugins={[remarkGfm]}>
            {value}
          </ReactMarkdown>
        </div>
      </div>
    </div>
  );
}
