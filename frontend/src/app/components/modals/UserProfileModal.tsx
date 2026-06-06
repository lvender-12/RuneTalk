import { UserPlus } from "lucide-react";
import type { Adventurer, Presence } from "../../data/mock";
import { Avatar } from "../shared/Avatar";
import { PresenceDot } from "../shared/PresenceDot";
import { Dialog, DialogContent } from "../ui/dialog";
import { formatCustomStatus, getPresenceConfig } from "../../utils/customStatus";

export function UserProfileModal({
  adventurer,
  presence,
  isAlly,
  onOpenChange,
}: {
  adventurer: Adventurer | null;
  presence?: Presence;
  isAlly: boolean;
  onOpenChange: (open: boolean) => void;
}) {
  return (
    <Dialog open={Boolean(adventurer)} onOpenChange={onOpenChange}>
      <DialogContent className="overflow-hidden border-border bg-popover p-0 text-foreground">
        {adventurer && (
          <>
            <div className="h-28" style={{ background: `linear-gradient(135deg, ${adventurer.banner_color}, #151226)` }} />
            <div className="px-6 pb-6">
              <div className="-mt-8 flex items-end justify-between">
                <Avatar adventurer={adventurer} presence={presence} size="lg" />
                <button className="flex h-9 items-center gap-2 rounded-md bg-primary px-3 text-sm text-primary-foreground">
                  <UserPlus className="size-4" />
                  {isAlly ? "Ally" : "Add ally"}
                </button>
              </div>
              <h2 className="mt-4 font-[Cinzel] text-xl font-bold text-[#f3e8c7]">{adventurer.display_name}</h2>
              <p className="text-sm text-muted-foreground">@{adventurer.username}</p>
              <div className="mt-4 rounded-md border border-border bg-card/60 p-3">
                <div className="mb-2 flex items-center gap-2 text-sm capitalize">
                  <PresenceDot status={presence?.status} className="size-3 border-0" />
                  {getPresenceConfig(presence?.status).label}
                  <span className="text-muted-foreground">- {formatCustomStatus(presence)}</span>
                </div>
                <p className="text-sm leading-6 text-[#ded7e8]">{adventurer.bio}</p>
              </div>
            </div>
          </>
        )}
      </DialogContent>
    </Dialog>
  );
}
