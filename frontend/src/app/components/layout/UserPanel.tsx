import { Settings, LogOut, Sparkles } from "lucide-react";
import { useState } from "react";
import type { Adventurer, Presence, PresenceStatus } from "../../data/mock";
import { Avatar } from "../shared/Avatar";
import { PresenceDot } from "../shared/PresenceDot";
import {
  formatCustomStatus,
  getPresenceConfig,
  getStatusForPresence,
  PRESENCE_STATUSES,
  STATUS_ICON_OPTIONS,
} from "../../utils/customStatus";

interface UserPanelProps {
  adventurer: Adventurer;
  presence?: Presence;
  onPresenceChange: (status: PresenceStatus, statusText: string, statusIcon: string) => void;
  onStatusChange: (statusIcon: string, statusText: string) => void;
  onProfile: () => void;
  onLogout: () => void;
}

export function UserPanel({
  adventurer,
  presence,
  onPresenceChange,
  onStatusChange,
  onProfile,
  onLogout,
}: UserPanelProps) {
  const [pickerOpen, setPickerOpen] = useState(false);
  const currentPresence = presence ?? {
    adventurer_id: adventurer.id,
    status: "online",
    last_seen_at: new Date().toISOString(),
  };
  const currentStatus = getStatusForPresence(currentPresence, currentPresence.status);
  const updateStatusText = (text: string) => onStatusChange(currentStatus.emoji, text);

  return (
    <div className="relative shrink-0 border-t border-[#241c3c] bg-[#0c0918] p-3 select-none z-30">
      {/* Adventurer Identity Bar */}
      <div className="flex items-center gap-2.5">
        <Avatar
          adventurer={adventurer}
          presence={currentPresence}
          size="sm"
          onClick={onProfile}
        />
        <button
          type="button"
          className="min-w-0 flex-1 text-left cursor-pointer group"
          onClick={onProfile}
        >
          <span className="block truncate text-xs font-bold text-slate-100 group-hover:text-amber-200 transition-colors">
            {adventurer.display_name}
          </span>
          <span className="block truncate text-[11px] text-slate-400">
            {formatCustomStatus(currentPresence) || `@${adventurer.username}`}
          </span>
        </button>

        {/* Settings Button */}
        <button
          type="button"
          onClick={onProfile}
          className="grid size-8 place-items-center rounded-lg text-slate-400 hover:bg-[#1e1738] hover:text-amber-200 transition-colors cursor-pointer"
          title="Adventurer Profile & Settings"
          aria-label="Profile & Settings"
        >
          <Settings className="size-4" />
        </button>

        {/* Explicit Logout Button */}
        <button
          type="button"
          onClick={onLogout}
          className="grid size-8 place-items-center rounded-lg text-slate-400 hover:bg-rose-500/20 hover:text-rose-400 transition-colors cursor-pointer"
          title="Sign Out"
          aria-label="Sign Out"
        >
          <LogOut className="size-4" />
        </button>
      </div>

      {/* Presence Quick Selectors */}
      <div className="mt-2.5 grid grid-cols-2 gap-1.5">
        {PRESENCE_STATUSES.map((status) => {
          const isSelected = currentPresence.status === status;
          return (
            <button
              key={status}
              type="button"
              className={`flex items-center justify-center gap-1.5 rounded-md px-2 py-1.5 text-[11px] font-medium border transition-all cursor-pointer ${
                isSelected
                  ? "border-amber-400/60 bg-amber-500/15 text-amber-200 shadow-sm"
                  : "border-[#271f45] bg-[#141026] text-slate-400 hover:bg-[#1c1634] hover:text-slate-200"
              }`}
              onClick={() => {
                const nextStatus = getStatusForPresence(currentPresence, status);
                onPresenceChange(status, nextStatus.text, nextStatus.emoji);
              }}
            >
              <PresenceDot status={status} className="size-2 border-0" />
              <span>{getPresenceConfig(status).label}</span>
            </button>
          );
        })}
      </div>

      {/* Custom Status Input & Icon Tag Selector */}
      <div className="relative mt-2 flex items-center gap-1.5">
        <button
          type="button"
          className="flex h-7 px-2 shrink-0 items-center justify-center rounded-md border border-[#2c234a] bg-[#141026] text-[10px] font-bold text-amber-300 hover:border-amber-400/60 transition-colors cursor-pointer"
          onClick={() => setPickerOpen((prev) => !prev)}
          title="Choose status tag"
          aria-label="Choose status tag"
        >
          {currentStatus.emoji || <Sparkles className="size-3 text-amber-400" />}
        </button>

        <input
          type="text"
          className="h-7 min-w-0 flex-1 rounded-md border border-[#2c234a] bg-[#141026] px-2 text-xs text-slate-100 placeholder-slate-400 outline-none focus:border-amber-400/60 transition-colors"
          placeholder="Set custom status..."
          aria-label="Custom status text"
          value={currentStatus.text || ""}
          onChange={(e) => updateStatusText(e.target.value)}
        />
      </div>

      {/* Status Tag Picker Modal / Dropdown */}
      {pickerOpen && (
        <div className="absolute bottom-full left-3 right-3 mb-2 z-50 rounded-xl border border-[#382c5e] bg-[#18122d] p-3 shadow-2xl shadow-black/90 animate-slide-up">
          <div className="flex items-center justify-between pb-2 mb-2 border-b border-[#292044]">
            <span className="text-[11px] font-bold uppercase tracking-wider text-amber-300">
              Select Status Tag
            </span>
            <button
              type="button"
              onClick={() => setPickerOpen(false)}
              className="text-xs text-slate-400 hover:text-white"
            >
              Close
            </button>
          </div>
          <div className="grid grid-cols-5 gap-1.5">
            {STATUS_ICON_OPTIONS.map((icon) => (
              <button
                key={icon}
                type="button"
                className={`flex h-8 items-center justify-center rounded text-[10px] font-semibold border transition-colors cursor-pointer ${
                  currentStatus.emoji === icon
                    ? "border-amber-400/80 bg-amber-500/25 text-amber-200"
                    : "border-transparent bg-[#110d22] text-slate-300 hover:bg-[#221a3f] hover:text-amber-200"
                }`}
                onClick={() => {
                  onStatusChange(icon, currentStatus.text || "");
                  setPickerOpen(false);
                }}
              >
                {icon}
              </button>
            ))}
          </div>
        </div>
      )}
    </div>
  );
}
