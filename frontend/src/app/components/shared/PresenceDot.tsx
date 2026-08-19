import type { PresenceStatus } from "../../data/mock";
import { getPresenceConfig } from "../../utils/customStatus";

const STATUS_COLOR_MAP: Record<PresenceStatus, string> = {
  online: "bg-[#10b981] shadow-[0_0_8px_rgba(16,185,129,0.6)]",
  idle: "bg-[#f59e0b] shadow-[0_0_8px_rgba(245,158,11,0.6)]",
  dnd: "bg-[#f43f5e] shadow-[0_0_8px_rgba(244,63,94,0.6)]",
  offline: "bg-[#64748b]",
};

export function PresenceDot({
  status,
  className = "",
}: {
  status?: PresenceStatus;
  className?: string;
}) {
  const presence = status ?? "offline";
  const config = getPresenceConfig(presence);
  const colorClass = STATUS_COLOR_MAP[presence];

  return (
    <span
      className={`inline-block rounded-full border-2 border-[#09080e] ${colorClass} ${className}`}
      title={config.label}
      aria-label={config.label}
    />
  );
}
