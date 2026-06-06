import { makeDefaultStatusByPresence } from "../utils/customStatus";

export type PresenceStatus = "online" | "idle" | "dnd" | "offline";

export interface Adventurer {
  id: string;
  username: string;
  display_name: string;
  email: string;
  bio: string;
  avatar_url: string | null;
  banner_color: string;
  created_at: string;
}

export interface Guild {
  id: string;
  owner_id: string;
  name: string;
  description: string;
  icon_url: string | null;
  invite_code: string;
  is_public: boolean;
  created_at: string;
}

export interface GuildMember {
  guild_id: string;
  adventurer_id: string;
  role: "owner" | "admin" | "member";
  joined_at: string;
}

export interface Rift {
  id: string;
  guild_id: string;
  name: string;
  topic: string;
  category: string;
  position: number;
  created_at: string;
}

export interface Echo {
  id: string;
  rift_id: string;
  adventurer_id: string;
  content: string;
  reply_to_echo_id?: string;
  is_pinned: boolean;
  created_at: string;
}

export interface Scroll {
  id: string;
  adventurer_one_id: string;
  adventurer_two_id: string;
  created_at: string;
}

export interface Whisper {
  id: string;
  scroll_id: string;
  adventurer_id: string;
  content: string;
  reply_to_whisper_id?: string;
  created_at: string;
}

export interface Ally {
  id: string;
  adventurer_id: string;
  ally_id: string;
  created_at: string;
}

export interface Pledge {
  id: string;
  from_adventurer_id: string;
  to_adventurer_id: string;
  status: "pending" | "accepted" | "rejected";
  created_at: string;
}

export interface CustomStatus {
  presence: PresenceStatus;
  statusText: string;
  statusIcon?: string;
}

export interface PresenceStatusStatus {
  emoji?: string;
  text?: string;
}

export interface Presence {
  adventurer_id: string;
  status: PresenceStatus;
  statusByPresence?: Partial<Record<PresenceStatus, PresenceStatusStatus>>;
  custom_status?: string | CustomStatus;
  last_seen_at: string;
}

const now = "2026-06-06T06:22:00.000Z";

export const currentAdventurer: Adventurer = {
  id: "00000000-0000-4000-8000-000000000001",
  username: "zenn",
  display_name: "Zenn the Rune Keeper",
  email: "zenn@runetalk.local",
  bio: "Backend scribe, guild cartographer, and keeper of quiet channels.",
  avatar_url: null,
  banner_color: "#c9a227",
  created_at: "2026-06-01T12:00:00.000Z",
};

export const adventurers: Adventurer[] = [
  currentAdventurer,
  {
    id: "00000000-0000-4000-8000-000000000002",
    username: "maera",
    display_name: "Maera Emberhand",
    email: "maera@runetalk.local",
    bio: "Ships features fast, then writes the lore doc nobody asked for.",
    avatar_url: null,
    banner_color: "#d96b4d",
    created_at: "2026-06-01T13:00:00.000Z",
  },
  {
    id: "00000000-0000-4000-8000-000000000003",
    username: "orrin",
    display_name: "Orrin Vale",
    email: "orrin@runetalk.local",
    bio: "Moderator of the east gate and collector of suspicious stack traces.",
    avatar_url: null,
    banner_color: "#5c87d8",
    created_at: "2026-06-02T08:00:00.000Z",
  },
  {
    id: "00000000-0000-4000-8000-000000000004",
    username: "nyx",
    display_name: "Nyx Silverquill",
    email: "nyx@runetalk.local",
    bio: "Turns channel chaos into readable issue threads.",
    avatar_url: null,
    banner_color: "#8a71d6",
    created_at: "2026-06-02T10:00:00.000Z",
  },
  {
    id: "00000000-0000-4000-8000-000000000005",
    username: "kael",
    display_name: "Kael Mossforge",
    email: "kael@runetalk.local",
    bio: "Knows where every migration is buried.",
    avatar_url: null,
    banner_color: "#7ba967",
    created_at: "2026-06-03T09:00:00.000Z",
  },
  {
    id: "00000000-0000-4000-8000-000000000006",
    username: "seren",
    display_name: "Seren Ashfall",
    email: "seren@runetalk.local",
    bio: "Always online, rarely impressed by unread badges.",
    avatar_url: null,
    banner_color: "#7fd3c6",
    created_at: "2026-06-03T10:00:00.000Z",
  },
];

export const guilds: Guild[] = [
  {
    id: "10000000-0000-4000-8000-000000000001",
    owner_id: currentAdventurer.id,
    name: "Moonlit Forge",
    description: "Build notes, release chatter, and late-night artifact reviews.",
    icon_url: null,
    invite_code: "FORGE-7K",
    is_public: true,
    created_at: now,
  },
  {
    id: "10000000-0000-4000-8000-000000000002",
    owner_id: "00000000-0000-4000-8000-000000000003",
    name: "Astral Vanguard",
    description: "Expedition planning for adventurers who read the changelog.",
    icon_url: null,
    invite_code: "ASTRA-2Q",
    is_public: true,
    created_at: now,
  },
  {
    id: "10000000-0000-4000-8000-000000000003",
    owner_id: "00000000-0000-4000-8000-000000000004",
    name: "The Archive",
    description: "A quieter guild for specs, scrolls, and durable decisions.",
    icon_url: null,
    invite_code: "ARCH-11",
    is_public: false,
    created_at: now,
  },
];

export const guildMembers: GuildMember[] = [
  { guild_id: guilds[0].id, adventurer_id: currentAdventurer.id, role: "owner", joined_at: now },
  { guild_id: guilds[0].id, adventurer_id: adventurers[1].id, role: "admin", joined_at: now },
  { guild_id: guilds[0].id, adventurer_id: adventurers[2].id, role: "member", joined_at: now },
  { guild_id: guilds[0].id, adventurer_id: adventurers[3].id, role: "member", joined_at: now },
  { guild_id: guilds[0].id, adventurer_id: adventurers[4].id, role: "member", joined_at: now },
  { guild_id: guilds[1].id, adventurer_id: currentAdventurer.id, role: "member", joined_at: now },
  { guild_id: guilds[1].id, adventurer_id: adventurers[2].id, role: "owner", joined_at: now },
  { guild_id: guilds[1].id, adventurer_id: adventurers[5].id, role: "admin", joined_at: now },
  { guild_id: guilds[2].id, adventurer_id: currentAdventurer.id, role: "member", joined_at: now },
  { guild_id: guilds[2].id, adventurer_id: adventurers[3].id, role: "owner", joined_at: now },
  { guild_id: guilds[2].id, adventurer_id: adventurers[4].id, role: "member", joined_at: now },
];

export const rifts: Rift[] = [
  { id: "20000000-0000-4000-8000-000000000001", guild_id: guilds[0].id, name: "great-hall", topic: "Daily coordination and announcements", category: "Council", position: 1, created_at: now },
  { id: "20000000-0000-4000-8000-000000000002", guild_id: guilds[0].id, name: "forge-logs", topic: "Backend, schema, and release work", category: "Craft", position: 2, created_at: now },
  { id: "20000000-0000-4000-8000-000000000003", guild_id: guilds[0].id, name: "rune-ui", topic: "Frontend screenshots and interaction polish", category: "Craft", position: 3, created_at: now },
  { id: "20000000-0000-4000-8000-000000000004", guild_id: guilds[1].id, name: "expeditions", topic: "Plan the next campaign", category: "Operations", position: 1, created_at: now },
  { id: "20000000-0000-4000-8000-000000000005", guild_id: guilds[1].id, name: "watchtower", topic: "Incidents, alerts, and live monitoring", category: "Operations", position: 2, created_at: now },
  { id: "20000000-0000-4000-8000-000000000006", guild_id: guilds[2].id, name: "index", topic: "Specs worth keeping", category: "Library", position: 1, created_at: now },
  { id: "20000000-0000-4000-8000-000000000007", guild_id: guilds[2].id, name: "quiet-review", topic: "Review threads without noise", category: "Library", position: 2, created_at: now },
];

export const echoes: Echo[] = [
  { id: "30000000-0000-4000-8000-000000000001", rift_id: rifts[0].id, adventurer_id: adventurers[1].id, content: "The invite flow is stable. I pinned the OTP checklist so new adventurers do not wander into half-built auth.", is_pinned: true, created_at: "2026-06-06T05:40:00.000Z" },
  { id: "30000000-0000-4000-8000-000000000002", rift_id: rifts[0].id, adventurer_id: currentAdventurer.id, content: "Good. The backend names are staying as Adventurer, Guild, Rift, Echo, Scroll, Whisper, Ally, Pledge, and Presence.", reply_to_echo_id: "30000000-0000-4000-8000-000000000001", is_pinned: false, created_at: "2026-06-06T05:43:00.000Z" },
  { id: "30000000-0000-4000-8000-000000000003", rift_id: rifts[0].id, adventurer_id: adventurers[2].id, content: "I like that the mock data mirrors table names. Swapping in API calls later should be boring.", is_pinned: false, created_at: "2026-06-06T05:47:00.000Z" },
  { id: "30000000-0000-4000-8000-000000000004", rift_id: rifts[1].id, adventurer_id: adventurers[4].id, content: "Migration 011 added indexes for rift ordering and whisper lookup. The UI can assume fast channel transitions.", is_pinned: true, created_at: "2026-06-06T05:51:00.000Z" },
  { id: "30000000-0000-4000-8000-000000000005", rift_id: rifts[1].id, adventurer_id: currentAdventurer.id, content: "Next backend pass should expose guild summaries with member counts and latest echo timestamps.", is_pinned: false, created_at: "2026-06-06T05:54:00.000Z" },
  { id: "30000000-0000-4000-8000-000000000006", rift_id: rifts[2].id, adventurer_id: adventurers[3].id, content: "The left rail should feel like a magical command surface, but still scan like Discord. No decorative clutter.", is_pinned: true, created_at: "2026-06-06T05:58:00.000Z" },
  { id: "30000000-0000-4000-8000-000000000007", rift_id: rifts[2].id, adventurer_id: adventurers[5].id, content: "Hover actions are live: reply, pin placeholder, react placeholder. The reply preview is the important one.", reply_to_echo_id: "30000000-0000-4000-8000-000000000006", is_pinned: false, created_at: "2026-06-06T06:01:00.000Z" },
  { id: "30000000-0000-4000-8000-000000000008", rift_id: rifts[3].id, adventurer_id: adventurers[2].id, content: "Expedition roster opens at dusk. Bring short briefs, not ten-page legends.", is_pinned: false, created_at: "2026-06-06T06:03:00.000Z" },
  { id: "30000000-0000-4000-8000-000000000009", rift_id: rifts[4].id, adventurer_id: adventurers[5].id, content: "Watchtower clear. No broken sockets in the last hour.", is_pinned: false, created_at: "2026-06-06T06:06:00.000Z" },
  { id: "30000000-0000-4000-8000-000000000010", rift_id: rifts[5].id, adventurer_id: adventurers[3].id, content: "Spec archive started with auth DTOs, presence status, and pledge lifecycle.", is_pinned: true, created_at: "2026-06-06T06:09:00.000Z" },
  { id: "30000000-0000-4000-8000-000000000011", rift_id: rifts[6].id, adventurer_id: adventurers[4].id, content: "Review note: the app should be readable at a glance before it becomes clever.", is_pinned: false, created_at: "2026-06-06T06:12:00.000Z" },
];

export const scrolls: Scroll[] = [
  { id: "40000000-0000-4000-8000-000000000001", adventurer_one_id: currentAdventurer.id, adventurer_two_id: adventurers[1].id, created_at: now },
  { id: "40000000-0000-4000-8000-000000000002", adventurer_one_id: currentAdventurer.id, adventurer_two_id: adventurers[3].id, created_at: now },
  { id: "40000000-0000-4000-8000-000000000003", adventurer_one_id: currentAdventurer.id, adventurer_two_id: adventurers[5].id, created_at: now },
];

export const whispers: Whisper[] = [
  { id: "50000000-0000-4000-8000-000000000001", scroll_id: scrolls[0].id, adventurer_id: adventurers[1].id, content: "Can you check the create guild modal before I wire the API?", created_at: "2026-06-06T06:13:00.000Z" },
  { id: "50000000-0000-4000-8000-000000000002", scroll_id: scrolls[0].id, adventurer_id: currentAdventurer.id, content: "Yes. It has name, description, icon placeholder, public toggle, and join code tab.", reply_to_whisper_id: "50000000-0000-4000-8000-000000000001", created_at: "2026-06-06T06:14:00.000Z" },
  { id: "50000000-0000-4000-8000-000000000003", scroll_id: scrolls[1].id, adventurer_id: adventurers[3].id, content: "Profile modal needs the bio and presence. Ally action can be inert for the demo.", created_at: "2026-06-06T06:17:00.000Z" },
  { id: "50000000-0000-4000-8000-000000000004", scroll_id: scrolls[2].id, adventurer_id: adventurers[5].id, content: "I set myself idle so the dot states are visible in screenshots.", created_at: "2026-06-06T06:19:00.000Z" },
];

export const allies: Ally[] = [
  { id: "60000000-0000-4000-8000-000000000001", adventurer_id: currentAdventurer.id, ally_id: adventurers[1].id, created_at: now },
  { id: "60000000-0000-4000-8000-000000000002", adventurer_id: currentAdventurer.id, ally_id: adventurers[3].id, created_at: now },
  { id: "60000000-0000-4000-8000-000000000003", adventurer_id: currentAdventurer.id, ally_id: adventurers[5].id, created_at: now },
];

export const pledges: Pledge[] = [
  { id: "70000000-0000-4000-8000-000000000001", from_adventurer_id: adventurers[2].id, to_adventurer_id: currentAdventurer.id, status: "pending", created_at: now },
  { id: "70000000-0000-4000-8000-000000000002", from_adventurer_id: currentAdventurer.id, to_adventurer_id: adventurers[4].id, status: "pending", created_at: now },
];

export const presence: Record<string, Presence> = {
  [currentAdventurer.id]: {
    adventurer_id: currentAdventurer.id,
    status: "online",
    statusByPresence: makeDefaultStatusByPresence(),
    last_seen_at: now,
  },
  [adventurers[1].id]: {
    adventurer_id: adventurers[1].id,
    status: "online",
    last_seen_at: now,
  },
  [adventurers[2].id]: {
    adventurer_id: adventurers[2].id,
    status: "dnd",
    last_seen_at: now,
  },
  [adventurers[3].id]: {
    adventurer_id: adventurers[3].id,
    status: "online",
    last_seen_at: now,
  },
  [adventurers[4].id]: {
    adventurer_id: adventurers[4].id,
    status: "offline",
    last_seen_at: "2026-06-06T04:42:00.000Z",
  },
  [adventurers[5].id]: {
    adventurer_id: adventurers[5].id,
    status: "idle",
    last_seen_at: now,
  },
};
