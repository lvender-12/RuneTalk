import { useState } from "react";
import { Users, Search } from "lucide-react";
import type { Adventurer, Ally, Presence, Scroll } from "../../data/mock";
import { Avatar } from "../shared/Avatar";
import { formatCustomStatus } from "../../utils/customStatus";

interface DMSidebarProps {
  allies: Ally[];
  adventurers: Adventurer[];
  currentAdventurerId: string;
  presence: Record<string, Presence>;
  scrolls: Scroll[];
  selectedScrollId: string;
  onAllies: () => void;
  onScrollSelect: (scrollId: string) => void;
  onProfile: (adventurer: Adventurer) => void;
}

export function DMSidebar({
  allies,
  adventurers,
  currentAdventurerId,
  presence,
  scrolls,
  selectedScrollId,
  onAllies,
  onScrollSelect,
  onProfile,
}: DMSidebarProps) {
  const [search, setSearch] = useState("");

  // Get recipient for a scroll
  const getRecipient = (scroll: Scroll): Adventurer | undefined => {
    const otherId =
      scroll.adventurer_one_id === currentAdventurerId
        ? scroll.adventurer_two_id
        : scroll.adventurer_one_id;
    return adventurers.find((a) => a.id === otherId);
  };

  const filteredScrolls = scrolls.filter((scroll) => {
    const recipient = getRecipient(scroll);
    if (!recipient) return false;
    if (!search.trim()) return true;
    const query = search.toLowerCase();
    return (
      recipient.display_name.toLowerCase().includes(query) ||
      recipient.username.toLowerCase().includes(query)
    );
  });

  return (
    <div className="flex flex-1 flex-col min-h-0 bg-[#0d0a18] select-none">
      {/* Header with Search */}
      <div className="p-3 border-b border-[#241c3c] bg-[#110d20]">
        <button
          type="button"
          onClick={onAllies}
          className="flex w-full items-center justify-between gap-2.5 rounded-lg bg-gradient-to-r from-purple-900/40 to-amber-900/30 border border-[#382b5c] px-3 py-2.5 text-sm font-semibold text-amber-200 hover:border-amber-400/50 hover:shadow-[0_0_15px_rgba(212,175,55,0.15)] transition-all cursor-pointer mb-2"
        >
          <div className="flex items-center gap-2.5">
            <Users className="size-4 text-amber-400" />
            <span className="font-[Cinzel] tracking-wide">Allies & Pledges</span>
          </div>
          <span className="rounded-full bg-amber-400/20 px-2 py-0.5 text-[11px] font-bold text-amber-300">
            {allies.length}
          </span>
        </button>

        {/* Quick Search */}
        <div className="relative flex items-center">
          <Search className="absolute left-2.5 size-3.5 text-slate-400 pointer-events-none" />
          <input
            type="text"
            value={search}
            onChange={(e) => setSearch(e.target.value)}
            placeholder="Find scroll or ally..."
            className="w-full h-8 rounded-md bg-[#161129] border border-[#2c234a] pl-8 pr-3 text-xs text-slate-200 placeholder-slate-400 focus:outline-none focus:border-amber-400/60 focus:ring-1 focus:ring-amber-400/20 transition-colors"
          />
        </div>
      </div>

      {/* Direct Scrolls List */}
      <div className="flex-1 overflow-y-auto px-2 py-3 space-y-1">
        <div className="px-2 pb-1 text-[11px] font-bold uppercase tracking-widest text-slate-400">
          Direct Scrolls ({filteredScrolls.length})
        </div>

        {filteredScrolls.map((scroll) => {
          const recipient = getRecipient(scroll);
          if (!recipient) return null;

          const isSelected = selectedScrollId === scroll.id;
          const userPresence = presence[recipient.id];
          const statusText = formatCustomStatus(userPresence);

          return (
            <div
              key={scroll.id}
              onClick={() => onScrollSelect(scroll.id)}
              className={`group flex w-full items-center gap-2.5 rounded-lg px-2.5 py-2 text-left transition-all duration-150 cursor-pointer ${
                isSelected
                  ? "bg-gradient-to-r from-amber-500/20 via-purple-600/20 to-transparent border-l-2 border-amber-400 shadow-sm"
                  : "hover:bg-[#1a1433]"
              }`}
            >
              <Avatar
                adventurer={recipient}
                presence={userPresence}
                size="sm"
                onClick={() => onProfile(recipient)}
              />
              <div className="min-w-0 flex-1">
                <p
                  className={`truncate text-xs font-semibold ${
                    isSelected ? "text-amber-200" : "text-slate-200 group-hover:text-white"
                  }`}
                >
                  {recipient.display_name}
                </p>
                <p className="truncate text-[11px] text-slate-400">{statusText}</p>
              </div>
            </div>
          );
        })}
      </div>
    </div>
  );
}
