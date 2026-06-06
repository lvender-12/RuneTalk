import type { Adventurer, Presence } from "../../data/mock";
import { Avatar } from "../shared/Avatar";
import { formatCustomStatus, getPresenceConfig } from "../../utils/customStatus";

export function MemberList({ members, presence, onProfile }: { members: Adventurer[]; presence: Record<string, Presence>; onProfile: (adventurer: Adventurer) => void }) {
  const online = members.filter((member) => presence[member.id]?.status !== "offline");
  const offline = members.filter((member) => presence[member.id]?.status === "offline");

  return (
    <div className="h-full overflow-y-auto p-4">
      <MemberGroup label={`${getPresenceConfig("online").label} - ${online.length}`} members={online} presence={presence} onProfile={onProfile} />
      <MemberGroup label={`${getPresenceConfig("offline").label} - ${offline.length}`} members={offline} presence={presence} onProfile={onProfile} />
    </div>
  );
}

function MemberGroup({ label, members, presence, onProfile }: { label: string; members: Adventurer[]; presence: Record<string, Presence>; onProfile: (adventurer: Adventurer) => void }) {
  return (
    <div className="mb-6">
      <p className="mb-3 text-xs font-semibold uppercase tracking-[0.18em] text-muted-foreground">{label}</p>
      <div className="space-y-2">
        {members.map((member) => (
          <button key={member.id} className="flex w-full items-center gap-3 rounded-md p-2 text-left transition hover:bg-card/70" onClick={() => onProfile(member)}>
            <Avatar adventurer={member} presence={presence[member.id]} size="sm" />
            <span className="min-w-0">
              <span className="block truncate text-sm text-foreground">{member.display_name}</span>
              <span className="block truncate text-xs text-muted-foreground">{formatCustomStatus(presence[member.id])}</span>
            </span>
          </button>
        ))}
      </div>
    </div>
  );
}
