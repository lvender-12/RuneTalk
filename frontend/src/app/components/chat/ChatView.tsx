import { useRef, useEffect } from "react";
import { Shield, Hash } from "lucide-react";
import type { Adventurer, Echo, Presence, Whisper } from "../../data/mock";
import { MessageItem } from "./MessageItem";

interface ChatViewProps {
  mode: "guild" | "dm" | "allies";
  messages: (Echo | Whisper)[];
  adventurers: Adventurer[];
  presence: Record<string, Presence>;
  echoes: Echo[];
  whispers: Whisper[];
  onReply: (message: Echo | Whisper) => void;
  onProfile: (adventurer: Adventurer) => void;
}

export function ChatView({
  mode,
  messages,
  adventurers,
  presence,
  echoes,
  whispers,
  onReply,
  onProfile,
}: ChatViewProps) {
  const bottomRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    bottomRef.current?.scrollIntoView({ behavior: "smooth" });
  }, [messages.length]);

  const getAuthor = (adventurerId: string) =>
    adventurers.find((a) => a.id === adventurerId);

  const getReplyMessage = (replyId?: string) => {
    if (!replyId) return undefined;
    return (
      echoes.find((e) => e.id === replyId) ||
      whispers.find((w) => w.id === replyId)
    );
  };

  return (
    <div className="flex-1 overflow-y-auto px-1 py-4 space-y-1">
      {/* Welcome Chamber Banner */}
      <div className="mx-4 mb-8 rounded-xl border border-[#2e2450] bg-gradient-to-br from-[#181232] via-[#120d24] to-[#0d091a] p-6 shadow-xl">
        <div className="flex items-center gap-3">
          <div className="grid size-12 place-items-center rounded-xl bg-gradient-to-tr from-amber-500 to-yellow-400 text-slate-950 shadow-[0_0_20px_rgba(245,158,11,0.3)]">
            {mode === "dm" ? <Shield className="size-6" /> : <Hash className="size-6" />}
          </div>
          <div>
            <h3 className="font-[Cinzel] text-xl font-bold tracking-wide text-amber-200">
              {mode === "dm" ? "Direct Arcane Scroll" : "Welcome to the Rift Chamber"}
            </h3>
            <p className="text-xs text-slate-400 mt-0.5">
              {mode === "dm"
                ? "This scroll is inscribed between you and your ally. Messages are sealed."
                : "Echo your thoughts across the runes. All chamber members will hear your voice."}
            </p>
          </div>
        </div>
      </div>

      {/* Message Stream */}
      {messages.map((message) => {
        const author = getAuthor(message.adventurer_id);
        const replyId =
          "reply_to_echo_id" in message
            ? message.reply_to_echo_id
            : "reply_to_whisper_id" in message
            ? message.reply_to_whisper_id
            : undefined;

        const replyMessage = getReplyMessage(replyId);
        const replyAuthor = replyMessage ? getAuthor(replyMessage.adventurer_id) : undefined;
        const isPinned = "is_pinned" in message ? message.is_pinned : false;

        return (
          <MessageItem
            key={message.id}
            message={message}
            author={author}
            presence={author ? presence[author.id] : undefined}
            replyToMessage={replyMessage}
            replyToAuthor={replyAuthor}
            isPinned={isPinned}
            onReply={() => onReply(message)}
            onProfile={author ? () => onProfile(author) : undefined}
          />
        );
      })}

      <div ref={bottomRef} />
    </div>
  );
}
