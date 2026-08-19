import { useState } from "react";
import { Pin, Reply, Copy, Check } from "lucide-react";
import type { Adventurer, Echo, Presence, Whisper } from "../../data/mock";
import { Avatar } from "../shared/Avatar";
import { Badge } from "../shared/Badge";

interface MessageItemProps {
  message: Echo | Whisper;
  author?: Adventurer;
  presence?: Presence;
  replyToMessage?: Echo | Whisper;
  replyToAuthor?: Adventurer;
  isPinned?: boolean;
  onReply?: () => void;
  onPinToggle?: () => void;
  onProfile?: () => void;
}

function formatMessageTime(dateString: string): string {
  try {
    const date = new Date(dateString);
    return date.toLocaleTimeString([], { hour: "2-digit", minute: "2-digit" });
  } catch {
    return dateString;
  }
}

export function MessageItem({
  message,
  author,
  presence,
  replyToMessage,
  replyToAuthor,
  isPinned,
  onReply,
  onPinToggle,
  onProfile,
}: MessageItemProps) {
  const [copied, setCopied] = useState(false);

  const isOwner = author?.username === "zenn";
  const isAdmin = author?.username === "lyra" || author?.username === "eldrin";

  const handleCopy = () => {
    navigator.clipboard.writeText(message.content);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  return (
    <div className="group relative flex gap-3 px-4 py-2 hover:bg-[#151029]/60 transition-colors">
      {/* Action Bar on Hover */}
      <div className="absolute right-4 -top-3 hidden group-hover:flex items-center gap-0.5 rounded-lg border border-[#382b5e] bg-[#1a1433] px-1 py-0.5 shadow-xl shadow-black/60 z-10 animate-fade-in">
        {onReply && (
          <button
            type="button"
            onClick={onReply}
            className="grid size-7 place-items-center rounded text-slate-400 hover:bg-[#251d45] hover:text-amber-200 transition-colors cursor-pointer"
            title="Reply"
          >
            <Reply className="size-3.5" />
          </button>
        )}
        {onPinToggle && (
          <button
            type="button"
            onClick={onPinToggle}
            className={`grid size-7 place-items-center rounded transition-colors cursor-pointer ${
              isPinned
                ? "text-amber-400 bg-amber-500/20"
                : "text-slate-400 hover:bg-[#251d45] hover:text-amber-200"
            }`}
            title={isPinned ? "Unpin message" : "Pin message"}
          >
            <Pin className="size-3.5" />
          </button>
        )}
        <button
          type="button"
          onClick={handleCopy}
          className="grid size-7 place-items-center rounded text-slate-400 hover:bg-[#251d45] hover:text-amber-200 transition-colors cursor-pointer"
          title="Copy text"
        >
          {copied ? <Check className="size-3.5 text-emerald-400" /> : <Copy className="size-3.5" />}
        </button>
      </div>

      {/* Author Avatar */}
      <div className="pt-0.5">
        <Avatar
          adventurer={author}
          presence={presence}
          size="md"
          onClick={onProfile}
        />
      </div>

      {/* Message Body */}
      <div className="min-w-0 flex-1">
        {/* Reply Context Banner */}
        {replyToMessage && (
          <div className="mb-1 flex items-center gap-1.5 text-xs text-slate-400">
            <Reply className="size-3 -scale-x-100 text-amber-400" />
            <span className="font-semibold text-amber-300">
              @{replyToAuthor?.display_name || "Adventurer"}
            </span>
            <span className="truncate max-w-md italic text-slate-400">
              {replyToMessage.content}
            </span>
          </div>
        )}

        {/* Message Header */}
        <div className="flex items-center gap-2 flex-wrap">
          <button
            type="button"
            onClick={onProfile}
            className="font-semibold text-sm text-slate-100 hover:text-amber-300 transition-colors cursor-pointer"
          >
            {author?.display_name || "Unknown Adventurer"}
          </button>

          {isOwner && (
            <Badge variant="owner" size="sm">
              Guildmaster
            </Badge>
          )}
          {isAdmin && !isOwner && (
            <Badge variant="admin" size="sm">
              Elder
            </Badge>
          )}

          <span className="text-[11px] text-slate-400">
            {formatMessageTime(message.created_at)}
          </span>

          {isPinned && (
            <span className="inline-flex items-center gap-1 rounded bg-amber-500/15 px-1.5 py-0.5 text-[10px] font-bold text-amber-300 border border-amber-500/30">
              <Pin className="size-2.5" /> Pinned
            </span>
          )}
        </div>

        {/* Message Content */}
        <div className="mt-1 text-sm leading-relaxed text-slate-200 select-text whitespace-pre-wrap break-words">
          {message.content}
        </div>
      </div>
    </div>
  );
}
