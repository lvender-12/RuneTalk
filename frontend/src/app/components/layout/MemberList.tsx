import type { Adventurer, Presence } from "../../data/mock";
import { Avatar } from "../shared/Avatar";
import { Badge } from "../shared/Badge";
import { formatCustomStatus, getPresenceConfig } from "../../utils/customStatus";

export function MemberList({
  members,
  presence,
  onProfile,
}: {
  members: Adventurer[];
  presence: Record<string, Presence>;
  onProfile: (adventurer: Adventurer) => void;
}) {
  const online = members.filter((member) => presence[member.id]?.status !== "offline");
  const offline = members.filter((member) => presence[member.id]?.status === "offline");

  return (
    <div className="h-full overflow-y-auto p-3 space-y-6 select-none">
      <MemberGroup
        label={`${getPresenceConfig("online").label} — ${online.length}`}
        members={online}
        presence={presence}
        onProfile={onProfile}
      />
      <MemberGroup
        label={`${getPresenceConfig("offline").label} — ${offline.length}`}
        members={offline}
        presence={presence}
        onProfile={onProfile}
      />
    </div>
  );
}

function MemberGroup({
  label,
  members,
  presence,
  onProfile,
}: {
  label: string;
  members: Adventurer[];
  presence: Record<string, Presence>;
  onProfile: (adventurer: Adventurer) => void;
}) {
  if (members.length === 0) return null;

  return (
    <div>
      <p className="mb-2 px-2 text-[11px] font-bold uppercase tracking-widest text-slate-400">
        {label}
      </p>
      <div className="space-y-0.5">
        {members.map((member) => {
          const userPresence = presence[member.id];
          const isOwner = member.username === "zenn";
          const isAdmin = member.username === "lyra" || member.username === "eldrin";

          return (
            <button
              key={member.id}
              type="button"
              className="group flex w-full items-center gap-2.5 rounded-lg p-2 text-left transition-all hover:bg-[#1c1538] cursor-pointer"
              onClick={() => onProfile(member)}
            >
              <Avatar adventurer={member} presence={userPresence} size="sm" />
              <div className="min-w-0 flex-1">
                <div className="flex items-center gap-1.5">
                  <span className="truncate text-xs font-semibold text-slate-200 group-hover:text-amber-200 transition-colors">
                    {member.display_name}
                  </span>
                  {isOwner && (
                    <Badge variant="owner" size="sm">
                      Owner
                    </Badge>
                  )}
                  {isAdmin && !isOwner && (
                    <Badge variant="admin" size="sm">
                      Admin
                    </Badge>
                  )}
                </div>
                <span className="block truncate text-[10px] text-slate-400">
                  {formatCustomStatus(userPresence)}
                </span>
              </div>
            </button>
          );
        })}
      </div>
    </div>
  );
}
