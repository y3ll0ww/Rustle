import "../../style/markdown.css";
import { useState } from "react";
import ReactMarkdown from "react-markdown";

export default function MarkdownEditor() {
  const [value, setValue] = useState("");

  return (
    <div className="markdown-container">
      {/* Input area */}
      <textarea
        className="markdown-input"
        value={value}
        onChange={(e) => setValue(e.target.value)}
        placeholder="Write your markdown here..."
      />

      {/* Preview */}
      <div className="markdown-preview">
        <ReactMarkdown>{value}</ReactMarkdown>
      </div>
    </div>
  );
}
