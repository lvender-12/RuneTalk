import type { Adventurer, Presence } from "../../data/mock";
import { PresenceDot } from "./PresenceDot";

interface AvatarProps {
  adventurer?: Pick<Adventurer, "display_name" | "username" | "avatar_url" | "banner_color"> | null;
  presence?: Presence;
  size?: "xs" | "sm" | "md" | "lg" | "xl";
  className?: string;
  showPresence?: boolean;
  onClick?: () => void;
}

const SIZE_MAP = {
  xs: "size-6 text-[10px]",
  sm: "size-8 text-xs",
  md: "size-10 text-sm",
  lg: "size-14 text-lg",
  xl: "size-20 text-2xl font-bold",
};

const DOT_SIZE_MAP = {
  xs: "size-2 -bottom-0.5 -right-0.5 border",
  sm: "size-2.5 -bottom-0.5 -right-0.5 border-[1.5px]",
  md: "size-3.5 bottom-0 right-0 border-2",
  lg: "size-4.5 bottom-0.5 right-0.5 border-2",
  xl: "size-6 bottom-1 right-1 border-[3px]",
};

function getInitials(name?: string): string {
  if (!name) return "A";
  const parts = name.trim().split(/\s+/);
  if (parts.length === 1) return parts[0].slice(0, 2).toUpperCase();
  return (parts[0][0] + parts[parts.length - 1][0]).toUpperCase();
}

export function Avatar({
  adventurer,
  presence,
  size = "md",
  className = "",
  showPresence = true,
  onClick,
}: AvatarProps) {
  const initials = getInitials(adventurer?.display_name || adventurer?.username);
  const bannerColor = adventurer?.banner_color || "#c9a227";
  const isClickable = Boolean(onClick);

  return (
    <div
      className={`relative inline-flex shrink-0 items-center justify-center select-none ${
        isClickable ? "cursor-pointer transition-transform duration-150 active:scale-95" : ""
      } ${className}`}
      onClick={onClick}
      role={isClickable ? "button" : undefined}
      tabIndex={isClickable ? 0 : undefined}
      onKeyDown={
        isClickable
          ? (e) => {
              if (e.key === "Enter" || e.key === " ") {
                e.preventDefault();
                onClick?.();
              }
            }
          : undefined
      }
    >
      <div
        className={`relative overflow-hidden rounded-full font-[Cinzel] font-bold text-white shadow-md ring-1 ring-border/50 ${SIZE_MAP[size]} flex items-center justify-center`}
        style={{
          background: adventurer?.avatar_url
            ? undefined
            : `linear-gradient(135deg, ${bannerColor}dd 0%, #151226 100%)`,
        }}
      >
        {adventurer?.avatar_url ? (
          <img
            src={adventurer.avatar_url}
            alt={adventurer.display_name || adventurer.username}
            className="h-full w-full object-cover"
          />
        ) : (
          <span className="tracking-wider text-amber-100/90 drop-shadow-sm">{initials}</span>
        )}
      </div>

      {showPresence && presence && (
        <PresenceDot
          status={presence.status}
          className={`absolute ${DOT_SIZE_MAP[size]}`}
        />
      )}
    </div>
  );
}
