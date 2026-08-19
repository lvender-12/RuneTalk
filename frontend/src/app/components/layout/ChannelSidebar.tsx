import React, { useState } from "react";
import { Hash, Volume2, ShieldCheck, ChevronDown, Copy, Check, Sparkles, Plus, Compass } from "lucide-react";
import type { Guild, Rift } from "../../data/mock";

interface ChannelSidebarProps {
  guild?: Guild | null;
  rifts: Rift[];
  selectedRiftId?: string;
  onRiftSelect: (riftId: string) => void;
  onCreateGuild?: () => void;
  onJoinGuild?: () => void;
}

export function ChannelSidebar({
  guild,
  rifts,
  selectedRiftId,
  onRiftSelect,
  onCreateGuild,
  onJoinGuild,
}: ChannelSidebarProps) {
  const [copied, setCopied] = useState(false);
  const [menuOpen, setMenuOpen] = useState(false);

  if (!guild) {
    return (
      <div className="flex flex-1 flex-col min-h-0 bg-[#0d0a18] select-none">
        <div className="flex h-16 w-full items-center border-b border-[#241c3c] bg-[#110d20] px-4">
          <h2 className="font-[Cinzel] text-sm font-bold tracking-wide text-amber-200">
            Guild Sanctuary
          </h2>
        </div>
        <div className="flex-1 p-4 flex flex-col items-center justify-center text-center space-y-4">
          <div className="size-12 rounded-2xl bg-amber-500/10 border border-amber-500/20 grid place-items-center text-amber-300">
            <Sparkles className="size-6" />
          </div>
          <div className="space-y-1">
            <h3 className="font-[Cinzel] text-xs font-bold text-slate-200">
              No Active Order
            </h3>
            <p className="text-[11px] text-slate-400 leading-relaxed">
              Forge a new guild or enter an invite code to unlock rift chambers.
            </p>
          </div>
          <div className="w-full space-y-2 pt-2">
            {onCreateGuild && (
              <button
                type="button"
                onClick={onCreateGuild}
                className="w-full flex items-center justify-center gap-1.5 rounded-lg border border-amber-500/40 bg-amber-500/15 py-2 text-xs font-bold text-amber-200 hover:bg-amber-500/25 transition-all cursor-pointer"
              >
                <Plus className="size-3.5" />
                Forge Guild
              </button>
            )}
            {onJoinGuild && (
              <button
                type="button"
                onClick={onJoinGuild}
                className="w-full flex items-center justify-center gap-1.5 rounded-lg border border-[#2e234c] bg-[#150f28] py-2 text-xs font-medium text-slate-300 hover:bg-[#1f163b] hover:text-amber-200 transition-all cursor-pointer"
              >
                <Compass className="size-3.5" />
                Join Order
              </button>
            )}
          </div>
        </div>
      </div>
    );
  }

  const handleCopyInvite = (e: React.MouseEvent) => {
    e.stopPropagation();
    navigator.clipboard.writeText(guild.invite_code);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  return (
    <div className="flex flex-1 flex-col min-h-0 bg-[#0d0a18] select-none">
      {/* Guild Header Dropdown Trigger */}
      <div className="relative">
        <button
          type="button"
          onClick={() => setMenuOpen((prev) => !prev)}
          className="flex h-16 w-full items-center justify-between border-b border-[#241c3c] bg-[#110d20] px-4 text-left transition-colors hover:bg-[#191330] cursor-pointer"
        >
          <div className="min-w-0 flex-1 pr-2">
            <div className="flex items-center gap-1.5">
              <h2 className="truncate font-[Cinzel] text-base font-bold tracking-wide text-amber-200">
                {guild.name}
              </h2>
              <ShieldCheck className="size-4 text-amber-400 shrink-0" />
            </div>
            <p className="truncate text-xs text-slate-400">{guild.description}</p>
          </div>
          <ChevronDown
            className={`size-4 text-slate-400 transition-transform duration-200 ${
              menuOpen ? "rotate-180" : ""
            }`}
          />
        </button>

        {/* Dropdown Menu */}
        {menuOpen && (
          <div className="absolute left-3 right-3 top-16 z-30 mt-1 rounded-lg border border-[#362b5a] bg-[#17122b] p-2 shadow-2xl shadow-black/80 animate-slide-up">
            <div className="flex items-center justify-between rounded-md bg-[#1f183b] p-2.5">
              <div>
                <p className="text-[11px] font-semibold uppercase tracking-wider text-slate-400">
                  Invite Code
                </p>
                <p className="font-mono text-sm font-bold text-amber-300">
                  {guild.invite_code}
                </p>
              </div>
              <button
                type="button"
                onClick={handleCopyInvite}
                className="flex items-center gap-1 rounded bg-amber-500/20 px-2 py-1 text-xs font-semibold text-amber-200 hover:bg-amber-500/30 transition-colors cursor-pointer"
              >
                {copied ? <Check className="size-3.5" /> : <Copy className="size-3.5" />}
                {copied ? "Copied" : "Copy"}
              </button>
            </div>
          </div>
        )}
      </div>

      {/* Rifts Channel List */}
      <div className="flex-1 overflow-y-auto px-2 py-3 space-y-4">
        <div>
          <div className="flex items-center justify-between px-2 pb-1.5">
            <span className="text-[11px] font-bold uppercase tracking-wider text-amber-300/70 font-[Cinzel]">
              Sacred Rifts
            </span>
          </div>

          <div className="space-y-0.5">
            {rifts.map((rift) => {
              const isSelected = selectedRiftId === rift.id;
              const isVoice = rift.category?.toLowerCase() === "voice";

              return (
                <button
                  key={rift.id}
                  type="button"
                  onClick={() => onRiftSelect(rift.id)}
                  className={`group flex w-full items-center gap-2.5 rounded-lg px-2.5 py-2 text-left text-sm font-medium transition-all duration-150 cursor-pointer ${
                    isSelected
                      ? "bg-gradient-to-r from-amber-500/20 via-purple-600/20 to-transparent text-amber-200 border-l-2 border-amber-400 shadow-sm"
                      : "text-slate-300 hover:bg-[#1a1433] hover:text-slate-100"
                  }`}
                >
                  {isVoice ? (
                    <Volume2
                      className={`size-4 ${
                        isSelected
                          ? "text-amber-400"
                          : "text-slate-400 group-hover:text-slate-200"
                      }`}
                    />
                  ) : (
                    <Hash
                      className={`size-4 ${
                        isSelected
                          ? "text-amber-400"
                          : "text-slate-400 group-hover:text-slate-200"
                      }`}
                    />
                  )}
                  <span className="truncate">{rift.name}</span>
                </button>
              );
            })}
          </div>
        </div>
      </div>
    </div>
  );
}
