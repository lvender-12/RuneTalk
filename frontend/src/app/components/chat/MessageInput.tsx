import React, { useState, useRef, useEffect } from "react";
import { Send, X, Paperclip } from "lucide-react";
import type { Echo, Whisper } from "../../data/mock";

interface MessageInputProps {
  label: string;
  replyingTo: Echo | Whisper | null;
  onCancelReply: () => void;
  onSend: (content: string) => void;
}

export function MessageInput({
  label,
  replyingTo,
  onCancelReply,
  onSend,
}: MessageInputProps) {
  const [text, setText] = useState("");
  const textareaRef = useRef<HTMLTextAreaElement>(null);

  useEffect(() => {
    textareaRef.current?.focus();
  }, [replyingTo]);

  const handleSubmit = (e?: React.FormEvent) => {
    if (e) e.preventDefault();
    if (!text.trim()) return;
    onSend(text.trim());
    setText("");
    if (textareaRef.current) {
      textareaRef.current.style.height = "auto";
    }
  };

  const handleKeyDown = (e: React.KeyboardEvent<HTMLTextAreaElement>) => {
    if (e.key === "Enter" && !e.shiftKey) {
      e.preventDefault();
      handleSubmit();
    }
  };

  const handleInput = (e: React.ChangeEvent<HTMLTextAreaElement>) => {
    setText(e.target.value);
    e.target.style.height = "auto";
    e.target.style.height = `${Math.min(e.target.scrollHeight, 160)}px`;
  };

  return (
    <div className="p-4 bg-[#0a0714] border-t border-[#201838] select-none">
      {/* Replying Context Bar */}
      {replyingTo && (
        <div className="mb-2 flex items-center justify-between rounded-t-lg bg-[#191333] border-t border-x border-[#382b5e] px-3 py-1.5 text-xs text-amber-200 animate-slide-up">
          <div className="flex items-center gap-2 truncate">
            <span className="font-semibold text-amber-400">Replying to message:</span>
            <span className="truncate italic text-slate-400 max-w-sm">
              {replyingTo.content}
            </span>
          </div>
          <button
            type="button"
            onClick={onCancelReply}
            className="grid size-5 place-items-center rounded text-slate-400 hover:bg-white/10 hover:text-white transition-colors cursor-pointer"
          >
            <X className="size-3.5" />
          </button>
        </div>
      )}

      {/* Input Container */}
      <div
        className={`relative flex items-end gap-2 rounded-xl border border-[#2e234e] bg-[#130e25] p-2 focus-within:border-amber-400/80 focus-within:ring-2 focus-within:ring-amber-400/20 transition-all ${
          replyingTo ? "rounded-t-none" : ""
        }`}
      >
        <button
          type="button"
          className="grid size-9 place-items-center rounded-lg text-slate-400 hover:bg-[#20183b] hover:text-amber-300 transition-colors cursor-pointer shrink-0"
          title="Attach rune artifact"
        >
          <Paperclip className="size-4" />
        </button>

        <textarea
          ref={textareaRef}
          rows={1}
          value={text}
          onChange={handleInput}
          onKeyDown={handleKeyDown}
          placeholder={`Inscribe message in ${label}...`}
          className="max-h-40 min-h-[36px] w-full resize-none bg-transparent py-1.5 text-sm text-slate-100 placeholder-slate-400 outline-none leading-relaxed"
        />

        <button
          type="button"
          onClick={() => handleSubmit()}
          disabled={!text.trim()}
          className="grid size-9 place-items-center rounded-lg bg-gradient-to-tr from-amber-500 to-yellow-400 text-slate-950 font-bold hover:from-amber-400 hover:to-yellow-300 shadow-[0_0_12px_rgba(245,158,11,0.3)] transition-all active:scale-95 disabled:opacity-40 disabled:pointer-events-none disabled:shadow-none cursor-pointer shrink-0"
          title="Send message (Enter)"
        >
          <Send className="size-4" />
        </button>
      </div>
    </div>
  );
}
