import { Plus, Compass, Shield, LogIn } from "lucide-react";
import type { Guild } from "../../data/mock";
import { Tooltip } from "../shared/Tooltip";

interface GuildSidebarProps {
  guilds: Guild[];
  selectedGuildId?: string;
  mode: "login" | "register" | "otp" | "guild" | "dm" | "allies";
  onHome: () => void;
  onGuildSelect: (guildId: string) => void;
  onCreateGuild: () => void;
  onJoinGuild: () => void;
  onAuthMode?: () => void;
}

function getGuildInitials(name: string): string {
  const words = name.trim().split(/\s+/);
  if (words.length === 1) return words[0].slice(0, 3).toUpperCase();
  return words
    .slice(0, 3)
    .map((w) => w[0])
    .join("")
    .toUpperCase();
}

export function GuildSidebar({
  guilds,
  selectedGuildId,
  mode,
  onHome,
  onGuildSelect,
  onCreateGuild,
  onJoinGuild,
  onAuthMode,
}: GuildSidebarProps) {
  const isHomeActive = mode === "dm" || mode === "allies";

  return (
    <aside
      className="flex h-full w-[72px] shrink-0 flex-col items-center py-3 bg-[#08060d] border-r border-[#201838] select-none z-20"
      aria-label="Guilds navigation"
    >
      {/* Home / Direct Messages Navigation */}
      <div className="relative group flex items-center justify-center w-full mb-2">
        {/* Active Pill Indicator */}
        <span
          className={`absolute left-0 w-1 bg-amber-400 rounded-r-full transition-all duration-200 ${
            isHomeActive
              ? "h-10"
              : "h-0 group-hover:h-5 opacity-0 group-hover:opacity-100"
          }`}
        />
        <Tooltip content="Direct Scrolls & Allies" side="right">
          <button
            type="button"
            onClick={onHome}
            className={`relative grid size-12 place-items-center rounded-[24px] transition-all duration-200 cursor-pointer ${
              isHomeActive
                ? "rounded-[16px] bg-gradient-to-tr from-amber-500 to-yellow-400 text-slate-950 shadow-[0_0_20px_rgba(245,158,11,0.4)]"
                : "bg-[#161128] text-amber-200/80 hover:rounded-[16px] hover:bg-gradient-to-tr hover:from-amber-600 hover:to-amber-500 hover:text-slate-950 hover:shadow-[0_0_15px_rgba(212,175,55,0.3)]"
            }`}
            aria-label="Home and Direct Messages"
          >
            <Shield className="size-6 transition-transform group-hover:scale-105" />
          </button>
        </Tooltip>
      </div>

      {/* Separator */}
      <div className="w-8 h-[2px] bg-[#241c3e] my-1.5 rounded-full" />

      {/* Guilds List */}
      <div className="flex-1 w-full overflow-y-auto overflow-x-hidden space-y-2 py-1 flex flex-col items-center">
        {guilds.map((guild) => {
          const isSelected = mode === "guild" && selectedGuildId === guild.id;
          const initials = getGuildInitials(guild.name);

          return (
            <div
              key={guild.id}
              className="relative group flex items-center justify-center w-full"
            >
              {/* Left Pill Indicator */}
              <span
                className={`absolute left-0 w-1 bg-amber-400 rounded-r-full transition-all duration-200 ${
                  isSelected
                    ? "h-10"
                    : "h-0 group-hover:h-5 opacity-0 group-hover:opacity-100"
                }`}
              />
              <Tooltip content={guild.name} side="right">
                <button
                  type="button"
                  onClick={() => onGuildSelect(guild.id)}
                  className={`relative grid size-12 place-items-center overflow-hidden transition-all duration-200 cursor-pointer ${
                    isSelected
                      ? "rounded-[16px] bg-gradient-to-br from-purple-700 via-indigo-800 to-[#1e173e] text-amber-200 ring-2 ring-amber-400/80 shadow-[0_0_20px_rgba(168,85,247,0.35)]"
                      : "rounded-[24px] bg-[#161128] text-slate-300 hover:rounded-[16px] hover:bg-[#231a42] hover:text-amber-200"
                  }`}
                  aria-label={guild.name}
                >
                  {guild.icon_url ? (
                    <img
                      src={guild.icon_url}
                      alt={guild.name}
                      className="size-full object-cover"
                    />
                  ) : (
                    <span className="font-[Cinzel] text-xs font-bold tracking-wider">
                      {initials}
                    </span>
                  )}
                </button>
              </Tooltip>
            </div>
          );
        })}

        {/* Create Guild Button */}
        <div className="relative group flex items-center justify-center w-full pt-1">
          <Tooltip content="Create Guild" side="right">
            <button
              type="button"
              onClick={onCreateGuild}
              className="grid size-12 place-items-center rounded-[24px] bg-[#161128] text-emerald-400/90 border border-emerald-500/20 hover:rounded-[16px] hover:bg-emerald-600 hover:text-white hover:border-transparent hover:shadow-[0_0_15px_rgba(16,185,129,0.4)] transition-all duration-200 cursor-pointer"
              aria-label="Create Guild"
            >
              <Plus className="size-5" />
            </button>
          </Tooltip>
        </div>

        {/* Join Guild Button */}
        <div className="relative group flex items-center justify-center w-full">
          <Tooltip content="Join Guild with Invite" side="right">
            <button
              type="button"
              onClick={onJoinGuild}
              className="grid size-12 place-items-center rounded-[24px] bg-[#161128] text-cyan-400/90 border border-cyan-500/20 hover:rounded-[16px] hover:bg-cyan-600 hover:text-white hover:border-transparent hover:shadow-[0_0_15px_rgba(6,182,212,0.4)] transition-all duration-200 cursor-pointer"
              aria-label="Join Guild"
            >
              <Compass className="size-5" />
            </button>
          </Tooltip>
        </div>
      </div>

      {/* Auth Portal Switcher Shortcut */}
      {onAuthMode && (
        <div className="relative group flex items-center justify-center w-full pt-2 border-t border-[#201838]">
          <Tooltip content="Switch to Auth Screen" side="right">
            <button
              type="button"
              onClick={onAuthMode}
              className="grid size-10 place-items-center rounded-lg bg-[#140f24] text-slate-400 hover:bg-[#20183b] hover:text-amber-300 transition-colors cursor-pointer"
              aria-label="Switch to Login / Register"
            >
              <LogIn className="size-4" />
            </button>
          </Tooltip>
        </div>
      )}
    </aside>
  );
}
