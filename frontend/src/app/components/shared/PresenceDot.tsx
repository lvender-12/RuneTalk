import type { PresenceStatus } from "../../data/mock";
import { getPresenceConfig } from "../../utils/customStatus";

export function PresenceDot({ status, className = "" }: { status?: PresenceStatus; className?: string }) {
  const presence = status ?? "offline";

  return (
    <span
      className={`inline-block rounded-full border-2 border-background ${getPresenceConfig(presence).dotClass} ${className}`}
      title={getPresenceConfig(presence).label}
    />
  );
}
