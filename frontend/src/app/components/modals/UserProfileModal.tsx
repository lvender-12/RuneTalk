import { UserPlus, UserCheck, MessageSquare } from "lucide-react";
import type { Adventurer, Presence } from "../../data/mock";
import { Avatar } from "../shared/Avatar";
import { PresenceDot } from "../shared/PresenceDot";
import { Badge } from "../shared/Badge";
import { Button } from "../shared/Button";
import { Dialog, DialogContent } from "../ui/dialog";
import { formatCustomStatus, getPresenceConfig } from "../../utils/customStatus";

export function UserProfileModal({
  adventurer,
  presence,
  isAlly,
  onOpenChange,
  onMessage,
}: {
  adventurer: Adventurer | null;
  presence?: Presence;
  isAlly: boolean;
  onOpenChange: (open: boolean) => void;
  onMessage?: (adventurerId: string) => void;
}) {
  if (!adventurer) return null;

  const isOwner = adventurer.username === "zenn";
  const isAdmin = adventurer.username === "lyra" || adventurer.username === "eldrin";
  const presenceConfig = getPresenceConfig(presence?.status);

  return (
    <Dialog open={Boolean(adventurer)} onOpenChange={onOpenChange}>
      <DialogContent className="overflow-hidden border-[#382b5e] bg-[#140f26] p-0 text-slate-100 shadow-[0_15px_50px_rgba(0,0,0,0.85)]">
        {/* Banner Header with dynamic gradient */}
        <div
          className="h-32 w-full relative"
          style={{
            background: `linear-gradient(135deg, ${adventurer.banner_color}dd 0%, #15102a 70%, #0c0818 100%)`,
          }}
        >
          <div className="absolute inset-0 bg-black/20" />
        </div>

        {/* Profile Card Body */}
        <div className="px-6 pb-6 pt-0">
          {/* Avatar & Action Button Row */}
          <div className="-mt-10 flex items-end justify-between relative z-10">
            <Avatar
              adventurer={adventurer}
              presence={presence}
              size="lg"
              className="ring-4 ring-[#140f26]"
            />

            <div className="flex items-center gap-2">
              {onMessage && (
                <Button
                  type="button"
                  variant="secondary"
                  size="sm"
                  onClick={() => {
                    onMessage(adventurer.id);
                    onOpenChange(false);
                  }}
                  leftIcon={<MessageSquare className="size-3.5 text-amber-300" />}
                >
                  Direct Scroll
                </Button>
              )}

              <Button
                type="button"
                variant={isAlly ? "secondary" : "primary"}
                size="sm"
                leftIcon={
                  isAlly ? (
                    <UserCheck className="size-3.5 text-emerald-400" />
                  ) : (
                    <UserPlus className="size-3.5" />
                  )
                }
              >
                {isAlly ? "Allies" : "Add Ally"}
              </Button>
            </div>
          </div>

          {/* User Details */}
          <div className="mt-4 space-y-1">
            <div className="flex items-center gap-2 flex-wrap">
              <h2 className="font-[Cinzel] text-xl font-bold text-amber-200">
                {adventurer.display_name}
              </h2>
              {isOwner && <Badge variant="owner">Guildmaster</Badge>}
              {isAdmin && !isOwner && <Badge variant="admin">Elder</Badge>}
            </div>
            <p className="text-xs text-slate-400">@{adventurer.username}</p>
          </div>

          {/* Presence & Bio Details Box */}
          <div className="mt-4 space-y-3 rounded-xl border border-[#2e2450] bg-[#0f0c1e] p-4">
            {/* Status */}
            <div className="flex items-center gap-2 text-xs font-semibold text-slate-200">
              <PresenceDot status={presence?.status} className="size-2.5 border-0" />
              <span>{presenceConfig.label}</span>
              {presence && (
                <span className="text-slate-400 font-normal">
                  — {formatCustomStatus(presence)}
                </span>
              )}
            </div>

            {/* Bio */}
            {adventurer.bio && (
              <div className="border-t border-[#241c3d] pt-3">
                <p className="text-[11px] font-bold uppercase tracking-wider text-slate-400 mb-1">
                  Arcane Inscription (Bio)
                </p>
                <p className="text-xs text-slate-300 leading-relaxed">
                  {adventurer.bio}
                </p>
              </div>
            )}

            {/* Member Since */}
            <div className="border-t border-[#241c3d] pt-3 flex items-center justify-between text-[11px] text-slate-400">
              <span>Adventurer since</span>
              <span className="text-slate-300 font-mono">
                {new Date(adventurer.created_at).toLocaleDateString(undefined, {
                  month: "short",
                  year: "numeric",
                })}
              </span>
            </div>
          </div>
        </div>
      </DialogContent>
    </Dialog>
  );
}
