import { Settings } from "lucide-react";
import { useState } from "react";
import type { Adventurer, Presence, PresenceStatus } from "../../data/mock";
import { Avatar } from "../shared/Avatar";
import { formatCustomStatus, getPresenceConfig, getStatusForPresence, PRESENCE_STATUSES, STATUS_ICON_OPTIONS } from "../../utils/customStatus";

export function UserPanel({
  adventurer,
  presence,
  onPresenceChange,
  onStatusChange,
  onProfile,
}: {
  adventurer: Adventurer;
  presence: Presence;
  onPresenceChange: (status: PresenceStatus, statusText: string, statusIcon: string) => void;
  onStatusChange: (statusIcon: string, statusText: string) => void;
  onProfile: () => void;
}) {
  const [pickerOpen, setPickerOpen] = useState(false);
  const currentStatus = getStatusForPresence(presence, presence.status);
  const updateStatusText = (text: string) => onStatusChange(currentStatus.emoji, text);

  return (
    <div className="border-t border-sidebar-border bg-[#0d0b16] p-3">
      <div className="flex items-center gap-3">
        <Avatar adventurer={adventurer} presence={presence} size="sm" onClick={onProfile} />
        <button className="min-w-0 flex-1 text-left" onClick={onProfile}>
          <span className="block truncate text-sm font-semibold text-foreground">{adventurer.display_name}</span>
          <span className="block truncate text-xs text-muted-foreground">{formatCustomStatus(presence)}</span>
        </button>
        <button className="grid size-8 place-items-center rounded-md text-muted-foreground transition hover:bg-sidebar-accent hover:text-foreground">
          <Settings className="size-4" />
        </button>
      </div>
      <div className="mt-3 grid grid-cols-2 gap-1">
        {PRESENCE_STATUSES.map((status) => (
          <button
            key={status}
            className={`flex min-w-0 items-center justify-center gap-1 rounded-md border px-1.5 py-1.5 text-[11px] transition ${presence.status === status ? "border-primary bg-primary/15 text-primary" : "border-border bg-background/30 text-muted-foreground hover:text-foreground"}`}
            onClick={() => {
              const nextStatus = getStatusForPresence(presence, status);
              onPresenceChange(status, nextStatus.text, nextStatus.emoji);
            }}
          >
            <span className="shrink-0" aria-hidden="true">{getPresenceConfig(status).toggleEmoji}</span>
            <span className="min-w-0 overflow-hidden text-ellipsis whitespace-nowrap">{getPresenceConfig(status).label}</span>
          </button>
        ))}
      </div>
      <div className="relative mt-2 flex gap-1">
        <button
          className="grid h-8 w-9 shrink-0 place-items-center rounded-md border border-border bg-input-background text-sm transition hover:border-primary/50"
          onClick={() => setPickerOpen((value) => !value)}
          type="button"
          aria-label="Choose custom status icon"
        >
          {currentStatus.emoji}
        </button>
        {pickerOpen && (
          <div className="absolute bottom-9 left-0 z-30 grid grid-cols-5 gap-1 rounded-md border border-border bg-popover p-2 shadow-xl">
            {STATUS_ICON_OPTIONS.map((icon) => (
              <button
                key={icon}
                className={`grid size-8 place-items-center rounded-md text-sm transition hover:bg-accent ${currentStatus.emoji === icon ? "bg-primary/15 text-primary" : ""}`}
                onClick={() => {
                  onStatusChange(icon, currentStatus.text);
                  setPickerOpen(false);
                }}
                type="button"
              >
                {icon}
              </button>
            ))}
          </div>
        )}
        <input
          className="h-8 min-w-0 flex-1 rounded-md border border-border bg-input-background px-2 text-xs text-foreground outline-none focus:border-primary/50"
          aria-label="Custom status text"
          value={currentStatus.text}
          onChange={(event) => updateStatusText(event.currentTarget.value)}
          onInput={(event) => updateStatusText(event.currentTarget.value)}
        />
      </div>
    </div>
  );
}
