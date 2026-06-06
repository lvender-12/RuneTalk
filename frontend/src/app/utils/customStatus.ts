import type { CustomStatus, Presence, PresenceStatus, PresenceStatusStatus } from "../data/mock";

interface PresenceConfig {
  label: string;
  toggleEmoji: string;
  defaultStatusEmoji: string;
  defaultStatusText: string;
  dotClass: string;
}

export const PRESENCE_CONFIG: Record<PresenceStatus, PresenceConfig> = {
  online: {
    label: "Online",
    toggleEmoji: "🟢",
    defaultStatusEmoji: "🗺️",
    defaultStatusText: "Mapping the runes",
    dotClass: "bg-[#48c78e]",
  },
  idle: {
    label: "Idle",
    toggleEmoji: "🌙",
    defaultStatusEmoji: "📜",
    defaultStatusText: "Studying old scrolls",
    dotClass: "bg-[#d9b44a]",
  },
  dnd: {
    label: "Dnd",
    toggleEmoji: "⛔",
    defaultStatusEmoji: "🛡️",
    defaultStatusText: "Guarding the gate",
    dotClass: "bg-[#d96b4d]",
  },
  offline: {
    label: "Offline",
    toggleEmoji: "⚫",
    defaultStatusEmoji: "💤",
    defaultStatusText: "Away from the forge",
    dotClass: "bg-[#6f6a80]",
  },
};

export const PRESENCE_STATUSES = Object.keys(PRESENCE_CONFIG) as PresenceStatus[];

export const STATUS_ICON_OPTIONS = ["🗺️", "✍️", "⚒️", "📜", "🔮", "🛡️", "🕯️", "💤", "🔥", "⚡"];

function isPresenceStatus(presence: unknown): presence is PresenceStatus {
  return typeof presence === "string" && presence in PRESENCE_CONFIG;
}

export function getPresenceConfig(presence?: PresenceStatus | string | null) {
  return isPresenceStatus(presence) ? PRESENCE_CONFIG[presence] : PRESENCE_CONFIG.offline;
}

export function makeCustomStatus(
  presence: PresenceStatus,
  statusText: string,
  statusIcon = getPresenceConfig(presence).defaultStatusEmoji,
): CustomStatus {
  return {
    presence,
    statusText: statusText || getPresenceConfig(presence).defaultStatusText,
    statusIcon: statusIcon || getPresenceConfig(presence).defaultStatusEmoji,
  };
}

export function makeDefaultStatusByPresence(): Record<PresenceStatus, Required<PresenceStatusStatus>> {
  return PRESENCE_STATUSES.reduce(
    (statuses, presence) => {
      const config = getPresenceConfig(presence);
      statuses[presence] = {
        emoji: config.defaultStatusEmoji,
        text: config.defaultStatusText,
      };
      return statuses;
    },
    {} as Record<PresenceStatus, Required<PresenceStatusStatus>>,
  );
}

function legacyCustomStatusFromObject(value: unknown, fallbackPresence: PresenceStatus): CustomStatus | null {
  if (!value || typeof value !== "object") return null;

  const legacy = value as Record<string, unknown>;
  const presence = isPresenceStatus(legacy.presence) ? legacy.presence : fallbackPresence;
  const statusText = typeof legacy.statusText === "string"
    ? legacy.statusText
    : typeof legacy.text === "string"
      ? legacy.text
      : "";
  const statusIcon = typeof legacy.statusIcon === "string"
    ? legacy.statusIcon
    : typeof legacy.emoji === "string"
      ? legacy.emoji
      : undefined;

  return makeCustomStatus(presence, statusText, statusIcon);
}

function normalizeLegacyCustomStatus(legacy: string | CustomStatus, fallbackPresence: PresenceStatus): CustomStatus {
  if (typeof legacy !== "string") {
    return legacyCustomStatusFromObject(legacy, fallbackPresence) ?? makeCustomStatus(fallbackPresence, "");
  }

  try {
    const parsed = JSON.parse(legacy) as unknown;
    const parsedStatus = legacyCustomStatusFromObject(parsed, fallbackPresence);
    if (parsedStatus) return parsedStatus;
  } catch {
    // Plain legacy strings are treated as the status text for the current presence.
  }

  return makeCustomStatus(fallbackPresence, legacy);
}

export function getStatusForPresence(
  userPresence: Pick<Presence, "status" | "statusByPresence" | "custom_status"> | undefined,
  presence: PresenceStatus,
): Required<PresenceStatusStatus> {
  const config = getPresenceConfig(presence);
  const saved = userPresence?.statusByPresence?.[presence];

  if (saved) {
    return {
      emoji: saved.emoji || config.defaultStatusEmoji,
      text: saved.text || config.defaultStatusText,
    };
  }

  if (userPresence?.custom_status) {
    const fallbackPresence = isPresenceStatus(userPresence.status) ? userPresence.status : presence;
    const legacyStatus = normalizeLegacyCustomStatus(userPresence.custom_status, fallbackPresence);

    if ((legacyStatus.presence ?? userPresence.status) === presence) {
      return {
        emoji: legacyStatus.statusIcon || config.defaultStatusEmoji,
        text: legacyStatus.statusText || config.defaultStatusText,
      };
    }
  }

  return {
    emoji: config.defaultStatusEmoji,
    text: config.defaultStatusText,
  };
}

export function normalizePresence(userPresence: Presence): Presence {
  const status = isPresenceStatus(userPresence.status) ? userPresence.status : "offline";
  const statusByPresence = makeDefaultStatusByPresence();

  PRESENCE_STATUSES.forEach((presence) => {
    statusByPresence[presence] = getStatusForPresence(userPresence, presence);
  });

  return {
    ...userPresence,
    status,
    statusByPresence,
  };
}

export function normalizePresenceMap(presenceByAdventurer: Record<string, Presence>): Record<string, Presence> {
  return Object.fromEntries(
    Object.entries(presenceByAdventurer).map(([adventurerId, userPresence]) => [adventurerId, normalizePresence(userPresence)]),
  );
}

export function updatePresenceStatus(userPresence: Presence, status: PresenceStatus): Presence {
  return {
    ...normalizePresence(userPresence),
    status,
  };
}

export function updateCurrentPresenceStatus(
  userPresence: Presence,
  emoji: string,
  text: string,
): Presence {
  const normalizedPresence = normalizePresence(userPresence);

  return {
    ...normalizedPresence,
    statusByPresence: {
      ...normalizedPresence.statusByPresence,
      [normalizedPresence.status]: {
        emoji: emoji || getPresenceConfig(normalizedPresence.status).defaultStatusEmoji,
        text: text || getPresenceConfig(normalizedPresence.status).defaultStatusText,
      },
    },
  };
}

export function normalizeCustomStatus(presence?: Pick<Presence, "status" | "statusByPresence" | "custom_status">): CustomStatus {
  if (!presence) {
    return makeCustomStatus("offline", PRESENCE_CONFIG.offline.defaultStatusText, PRESENCE_CONFIG.offline.defaultStatusEmoji);
  }

  const status = getStatusForPresence(presence, presence.status);
  return makeCustomStatus(presence.status, status.text, status.emoji);
}

export function formatCustomStatus(presence?: Pick<Presence, "status" | "statusByPresence" | "custom_status">) {
  const currentPresence = presence?.status ?? "offline";
  const status = getStatusForPresence(presence, currentPresence);
  return `${status.emoji} ${status.text}`;
}
