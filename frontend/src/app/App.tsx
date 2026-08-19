import { useMemo, useState, useEffect } from "react";
import { Bell, Hash, Pin, Shield, Menu, X, Users } from "lucide-react";
import { LoginPage } from "./components/auth/LoginPage";
import { RegisterPage } from "./components/auth/RegisterPage";
import { OtpPage } from "./components/auth/OtpPage";
import { ChatView } from "./components/chat/ChatView";
import { MessageInput } from "./components/chat/MessageInput";
import { AlliesView } from "./components/friends/AlliesView";
import { WelcomeGreeting } from "./components/guild/WelcomeGreeting";
import { ChannelSidebar } from "./components/layout/ChannelSidebar";
import { DMSidebar } from "./components/layout/DMSidebar";
import { GuildSidebar } from "./components/layout/GuildSidebar";
import { MemberList } from "./components/layout/MemberList";
import { UserPanel } from "./components/layout/UserPanel";
import { CreateGuildModal } from "./components/modals/CreateGuildModal";
import { JoinGuildModal } from "./components/modals/JoinGuildModal";
import { UserProfileModal } from "./components/modals/UserProfileModal";
import {
  adventurers as initialAdventurers,
  allies as initialAllies,
  echoes as initialEchoes,
  guildMembers as initialGuildMembers,
  guilds as initialGuilds,
  pledges as initialPledges,
  presence as initialPresence,
  rifts as initialRifts,
  scrolls as initialScrolls,
  whispers as initialWhispers,
  type Adventurer,
  type Ally,
  type Echo,
  type Guild,
  type GuildMember,
  type Pledge,
  type Presence,
  type Rift,
  type Scroll,
  type Whisper,
} from "./data/mock";
import {
  normalizePresenceMap,
  updateCurrentPresenceStatus,
  updatePresenceStatus,
} from "./utils/customStatus";

type AppMode = "login" | "register" | "otp" | "guild" | "dm" | "allies";

interface PendingAuth {
  username: string;
  email: string;
  password?: string;
  otp: string;
  type: "register" | "login";
  matchedUser?: Adventurer;
}

function newId(prefix: string) {
  return `${prefix}-${Date.now()}-${Math.random().toString(16).slice(2)}`;
}

export default function App() {
  // Session State: Default to null so user must register/log in first
  const [currentUser, setCurrentUser] = useState<Adventurer | null>(() => {
    try {
      const saved = localStorage.getItem("runetalk_user");
      return saved ? JSON.parse(saved) : null;
    } catch {
      return null;
    }
  });

  const [mode, setMode] = useState<AppMode>(() => (currentUser ? "guild" : "login"));
  const [pendingAuth, setPendingAuth] = useState<PendingAuth | null>(null);

  // Dynamic Data Stores
  const [adventurers, setAdventurers] = useState<Adventurer[]>(() => {
    try {
      const saved = localStorage.getItem("runetalk_adventurers");
      return saved ? JSON.parse(saved) : initialAdventurers;
    } catch {
      return initialAdventurers;
    }
  });

  const [guilds, setGuilds] = useState<Guild[]>(() => {
    try {
      const saved = localStorage.getItem("runetalk_guilds");
      return saved ? JSON.parse(saved) : initialGuilds;
    } catch {
      return initialGuilds;
    }
  });

  const [guildMembers, setGuildMembers] = useState<GuildMember[]>(() => {
    try {
      const saved = localStorage.getItem("runetalk_guild_members");
      return saved ? JSON.parse(saved) : initialGuildMembers;
    } catch {
      return initialGuildMembers;
    }
  });

  const [rifts, setRifts] = useState<Rift[]>(() => {
    try {
      const saved = localStorage.getItem("runetalk_rifts");
      return saved ? JSON.parse(saved) : initialRifts;
    } catch {
      return initialRifts;
    }
  });

  const [scrolls, setScrolls] = useState<Scroll[]>(() => [...initialScrolls]);
  const [allies] = useState<Ally[]>(() => [...initialAllies]);
  const [pledges] = useState<Pledge[]>(() => [...initialPledges]);
  const [echoes, setEchoes] = useState<Echo[]>(() => [...initialEchoes]);
  const [whispers, setWhispers] = useState<Whisper[]>(() => [...initialWhispers]);
  const [presence, setPresence] = useState<Record<string, Presence>>(() =>
    normalizePresenceMap(initialPresence)
  );

  // Persistence
  useEffect(() => {
    try {
      localStorage.setItem("runetalk_adventurers", JSON.stringify(adventurers));
    } catch {
      // ignore
    }
  }, [adventurers]);

  useEffect(() => {
    try {
      localStorage.setItem("runetalk_guilds", JSON.stringify(guilds));
    } catch {
      // ignore
    }
  }, [guilds]);

  useEffect(() => {
    try {
      localStorage.setItem("runetalk_guild_members", JSON.stringify(guildMembers));
    } catch {
      // ignore
    }
  }, [guildMembers]);

  useEffect(() => {
    try {
      localStorage.setItem("runetalk_rifts", JSON.stringify(rifts));
    } catch {
      // ignore
    }
  }, [rifts]);

  useEffect(() => {
    try {
      if (currentUser) {
        localStorage.setItem("runetalk_user", JSON.stringify(currentUser));
      } else {
        localStorage.removeItem("runetalk_user");
      }
    } catch {
      // ignore
    }
  }, [currentUser]);

  // Compute Guilds Joined by Current User
  const userGuilds = useMemo(() => {
    if (!currentUser) return [];
    return guilds.filter((g) =>
      guildMembers.some((m) => m.guild_id === g.id && m.adventurer_id === currentUser.id)
    );
  }, [guilds, guildMembers, currentUser]);

  // UI Selection State
  const [selectedGuildId, setSelectedGuildId] = useState<string>("");
  const [selectedRiftId, setSelectedRiftId] = useState<string>("");
  const [selectedScrollId, setSelectedScrollId] = useState<string>(() => scrolls[0]?.id || "");
  const [replyingTo, setReplyingTo] = useState<Echo | Whisper | null>(null);
  const [pinnedOpen, setPinnedOpen] = useState(false);
  const [createOpen, setCreateOpen] = useState(false);
  const [joinOpen, setJoinOpen] = useState(false);
  const [profile, setProfile] = useState<Adventurer | null>(null);
  const [mobileSidebarOpen, setMobileSidebarOpen] = useState(false);
  const [mobileMembersOpen, setMobileMembersOpen] = useState(false);
  const [notificationNote, setNotificationNote] = useState<string | null>(null);

  // Sync selected guild to user guilds
  useEffect(() => {
    if (userGuilds.length > 0) {
      if (!selectedGuildId || !userGuilds.some((g) => g.id === selectedGuildId)) {
        setSelectedGuildId(userGuilds[0].id);
      }
    } else {
      setSelectedGuildId("");
    }
  }, [userGuilds, selectedGuildId]);

  // Active Guild & Channel computations
  const selectedGuild = userGuilds.find((g) => g.id === selectedGuildId) || null;

  const guildRifts = useMemo(() => {
    if (!selectedGuild) return [];
    return rifts.filter((r) => r.guild_id === selectedGuild.id);
  }, [selectedGuild, rifts]);

  useEffect(() => {
    if (guildRifts.length > 0) {
      if (!selectedRiftId || !guildRifts.some((r) => r.id === selectedRiftId)) {
        setSelectedRiftId(guildRifts[0].id);
      }
    } else {
      setSelectedRiftId("");
    }
  }, [guildRifts, selectedRiftId]);

  const selectedRift = guildRifts.find((r) => r.id === selectedRiftId) || null;

  const selectedScroll =
    scrolls.find((s) => s.id === selectedScrollId) || scrolls[0] || {
      id: "default-scroll",
      adventurer_one_id: currentUser?.id || "a1",
      adventurer_two_id: "a2",
      created_at: new Date().toISOString(),
    };

  const members = useMemo(() => {
    if (!selectedGuild) return [];
    const memberIds = guildMembers
      .filter((m) => m.guild_id === selectedGuild.id)
      .map((m) => m.adventurer_id);
    return adventurers.filter((a) => memberIds.includes(a.id));
  }, [selectedGuild, guildMembers, adventurers]);

  const messages =
    mode === "dm"
      ? whispers.filter((w) => w.scroll_id === selectedScroll.id)
      : selectedRift
      ? echoes.filter((e) => e.rift_id === selectedRift.id)
      : [];

  const pinnedEchoes = selectedRift
    ? echoes.filter((e) => e.rift_id === selectedRift.id && e.is_pinned)
    : [];

  // Authentication Handlers
  const handleRegisterSubmit = (username: string, email: string, pass: string) => {
    const existing = adventurers.find(
      (a) =>
        a.username.toLowerCase() === username.toLowerCase() ||
        a.email.toLowerCase() === email.toLowerCase()
    );
    if (existing) {
      return { success: false, error: "Username or email is already taken" };
    }

    const otp = Math.floor(100000 + Math.random() * 900000).toString();
    setPendingAuth({
      username,
      email,
      password: pass,
      otp,
      type: "register",
    });
    setMode("otp");
    return { success: true };
  };

  const handleLoginAttempt = (identifier: string, pass: string) => {
    const matched = (adventurers as (Adventurer & { password?: string })[]).find(
      (a) =>
        a.username.toLowerCase() === identifier.toLowerCase() ||
        a.email.toLowerCase() === identifier.toLowerCase()
    );

    if (!matched) {
      return {
        success: false,
        error: "Adventurer not found. Please create an account first.",
      };
    }

    if (matched.password && matched.password !== pass) {
      return {
        success: false,
        error: "Incorrect password for this adventurer.",
      };
    }

    // Generate login OTP
    const otp = Math.floor(100000 + Math.random() * 900000).toString();
    setPendingAuth({
      username: matched.username,
      email: matched.email,
      password: pass,
      otp,
      type: "login",
      matchedUser: matched,
    });
    setMode("otp");
    return { success: true, user: matched };
  };

  const handleOtpVerified = () => {
    if (!pendingAuth) return;

    if (pendingAuth.type === "register") {
      const newAdventurer: Adventurer & { password?: string } = {
        id: `adv-${Date.now()}`,
        username: pendingAuth.username.toLowerCase(),
        display_name: pendingAuth.username,
        email: pendingAuth.email,
        password: pendingAuth.password || "password123",
        bio: "A wandering adventurer newly arrived in RuneTalk.",
        avatar_url: null,
        banner_color: "#d4af37",
        created_at: new Date().toISOString(),
      };

      // Add to adventurers list
      setAdventurers((prev) => [...prev, newAdventurer]);

      // DO NOT add to any guild! Leave their guild membership completely empty.
      // Add initial presence
      setPresence((prev) => ({
        ...prev,
        [newAdventurer.id]: {
          adventurer_id: newAdventurer.id,
          status: "online",
          last_seen_at: new Date().toISOString(),
        },
      }));

      setCurrentUser(newAdventurer);
      setMode("guild");
      setPendingAuth(null);
    } else if (pendingAuth.type === "login" && pendingAuth.matchedUser) {
      setCurrentUser(pendingAuth.matchedUser);
      // Ensure presence
      setPresence((prev) => ({
        ...prev,
        [pendingAuth.matchedUser!.id]: {
          adventurer_id: pendingAuth.matchedUser!.id,
          status: "online",
          last_seen_at: new Date().toISOString(),
        },
      }));
      setMode("guild");
      setPendingAuth(null);
    }
  };

  const handleLogout = () => {
    setCurrentUser(null);
    setMode("login");
    setPendingAuth(null);
    setMobileSidebarOpen(false);
  };

  // Channel & Chat Actions
  function selectGuild(guildId: string) {
    const nextRift = rifts.find((r) => r.guild_id === guildId);
    setSelectedGuildId(guildId);
    if (nextRift) setSelectedRiftId(nextRift.id);
    setMode("guild");
    setReplyingTo(null);
    setMobileSidebarOpen(false);
  }

  function openDirectMessageWith(adventurerId: string) {
    if (!currentUser) return;
    let existingScroll = scrolls.find(
      (s) =>
        (s.adventurer_one_id === currentUser.id && s.adventurer_two_id === adventurerId) ||
        (s.adventurer_two_id === currentUser.id && s.adventurer_one_id === adventurerId)
    );

    if (!existingScroll) {
      existingScroll = {
        id: newId("scroll"),
        adventurer_one_id: currentUser.id,
        adventurer_two_id: adventurerId,
        created_at: new Date().toISOString(),
      };
      setScrolls((prev) => [...prev, existingScroll!]);
    }

    setSelectedScrollId(existingScroll.id);
    setMode("dm");
    setReplyingTo(null);
    setMobileSidebarOpen(false);
  }

  function sendMessage(content: string) {
    if (!currentUser) return;

    if (mode === "dm") {
      const whisper: Whisper = {
        id: newId("whisper"),
        scroll_id: selectedScroll.id,
        adventurer_id: currentUser.id,
        content,
        reply_to_whisper_id: replyingTo?.id,
        created_at: new Date().toISOString(),
      };
      setWhispers((items) => [...items, whisper]);
      setReplyingTo(null);
      return;
    }

    if (selectedRift) {
      const echo: Echo = {
        id: newId("echo"),
        rift_id: selectedRift.id,
        adventurer_id: currentUser.id,
        content,
        reply_to_echo_id: replyingTo?.id,
        is_pinned: false,
        created_at: new Date().toISOString(),
      };
      setEchoes((items) => [...items, echo]);
      setReplyingTo(null);
    }
  }

  function togglePinEcho(echoId: string) {
    setEchoes((prev) =>
      prev.map((e) => (e.id === echoId ? { ...e, is_pinned: !e.is_pinned } : e))
    );
  }

  function handleCreateGuild(name: string, description: string, isPublic: boolean) {
    if (!currentUser) return;
    const newGuild: Guild = {
      id: newId("guild"),
      owner_id: currentUser.id,
      name,
      description,
      icon_url: null,
      invite_code: name.slice(0, 4).toUpperCase() + "-" + Math.floor(10 + Math.random() * 90),
      is_public: isPublic,
      created_at: new Date().toISOString(),
    };

    const defaultRift: Rift = {
      id: newId("rift"),
      guild_id: newGuild.id,
      name: "general",
      topic: `${name} sanctuary chamber`,
      category: "Council",
      position: 1,
      created_at: new Date().toISOString(),
    };

    setGuilds((prev) => [...prev, newGuild]);
    setRifts((prev) => [...prev, defaultRift]);
    setGuildMembers((prev) => [
      ...prev,
      {
        guild_id: newGuild.id,
        adventurer_id: currentUser.id,
        role: "owner",
        joined_at: new Date().toISOString(),
      },
    ]);

    setSelectedGuildId(newGuild.id);
    setSelectedRiftId(defaultRift.id);
    setMode("guild");
  }

  function handleJoinGuild(inviteCode: string) {
    const matched = guilds.find((g) => g.invite_code.toUpperCase() === inviteCode.toUpperCase());
    if (!matched || !currentUser) {
      setNotificationNote("Invalid invite rune code. Please verify and try again.");
      setTimeout(() => setNotificationNote(null), 3000);
      return;
    }

    if (!guildMembers.some((m) => m.guild_id === matched.id && m.adventurer_id === currentUser.id)) {
      setGuildMembers((prev) => [
        ...prev,
        {
          guild_id: matched.id,
          adventurer_id: currentUser.id,
          role: "member",
          joined_at: new Date().toISOString(),
        },
      ]);
    }

    selectGuild(matched.id);
    setNotificationNote(`Pledged allegiance to ${matched.name}!`);
    setTimeout(() => setNotificationNote(null), 3000);
  }

  // Render Auth Views if not logged in or in Auth Modes
  if (mode === "register") {
    return (
      <RegisterPage
        onLogin={() => setMode("login")}
        onRegisterSubmit={handleRegisterSubmit}
      />
    );
  }

  if (mode === "otp" && pendingAuth) {
    return (
      <OtpPage
        email={pendingAuth.email}
        expectedOtp={pendingAuth.otp}
        onBack={() => setMode(pendingAuth.type === "register" ? "register" : "login")}
        onVerified={handleOtpVerified}
        onResendOtp={() => {
          const next = Math.floor(100000 + Math.random() * 900000).toString();
          setPendingAuth((prev) => (prev ? { ...prev, otp: next } : null));
          return next;
        }}
      />
    );
  }

  if (mode === "login" || !currentUser) {
    return (
      <LoginPage
        onRegister={() => setMode("register")}
        onLoginAttempt={handleLoginAttempt}
        existingUsers={adventurers}
      />
    );
  }

  return (
    <div className="h-screen w-screen overflow-hidden bg-[#09080e] text-slate-100 flex flex-col font-sans">
      <div className="flex h-full min-h-0 relative bg-[radial-gradient(circle_at_30%_-20%,rgba(212,175,55,0.12),transparent_40%),linear-gradient(135deg,#09080e_0%,#110d22_45%,#08060f_100%)]">
        {/* Left Rail: Guilds Sidebar (Shows only guilds user belongs to) */}
        <GuildSidebar
          guilds={userGuilds}
          selectedGuildId={selectedGuild?.id}
          mode={mode}
          onHome={() => {
            setMode("dm");
            setReplyingTo(null);
            setMobileSidebarOpen(false);
          }}
          onGuildSelect={selectGuild}
          onCreateGuild={() => setCreateOpen(true)}
          onJoinGuild={() => setJoinOpen(true)}
          onAuthMode={handleLogout}
        />

        {/* Secondary Rail: Channel Sidebar or DM Sidebar */}
        <div
          className={`fixed inset-y-0 left-[72px] z-30 w-[260px] bg-[#0d0a18] border-r border-[#241c3c] flex flex-col transition-transform duration-200 md:static md:translate-x-0 ${
            mobileSidebarOpen ? "translate-x-0" : "-translate-x-full md:translate-x-0"
          }`}
        >
          {mode === "dm" || mode === "allies" ? (
            <DMSidebar
              allies={allies}
              adventurers={adventurers}
              currentAdventurerId={currentUser.id}
              presence={presence}
              scrolls={scrolls}
              selectedScrollId={selectedScroll.id}
              onAllies={() => {
                setMode("allies");
                setMobileSidebarOpen(false);
              }}
              onScrollSelect={(id) => {
                setSelectedScrollId(id);
                setMode("dm");
                setReplyingTo(null);
                setMobileSidebarOpen(false);
              }}
              onProfile={setProfile}
            />
          ) : (
            <ChannelSidebar
              guild={selectedGuild}
              rifts={guildRifts}
              selectedRiftId={selectedRift?.id}
              onRiftSelect={(id) => {
                setSelectedRiftId(id);
                setReplyingTo(null);
                setMobileSidebarOpen(false);
              }}
              onCreateGuild={() => setCreateOpen(true)}
              onJoinGuild={() => setJoinOpen(true)}
            />
          )}

          {/* User Profile Bar Docked at Bottom with Logout */}
          <UserPanel
            adventurer={currentUser}
            presence={presence[currentUser.id]}
            onPresenceChange={(status, statusText, statusIcon) =>
              setPresence((value) => ({
                ...value,
                [currentUser.id]: updatePresenceStatus(
                  {
                    ...value[currentUser.id],
                    statusByPresence: {
                      ...value[currentUser.id]?.statusByPresence,
                      [status]: { emoji: statusIcon, text: statusText },
                    },
                  },
                  status
                ),
              }))
            }
            onStatusChange={(statusIcon, statusText) =>
              setPresence((value) => ({
                ...value,
                [currentUser.id]: updateCurrentPresenceStatus(
                  value[currentUser.id],
                  statusIcon,
                  statusText
                ),
              }))
            }
            onProfile={() => setProfile(currentUser)}
            onLogout={handleLogout}
          />
        </div>

        {/* Mobile backdrop for drawer */}
        {mobileSidebarOpen && (
          <div
            className="fixed inset-0 z-25 bg-black/60 backdrop-blur-sm md:hidden"
            onClick={() => setMobileSidebarOpen(false)}
          />
        )}

        {/* Main Workspace Area */}
        <main className="flex min-w-0 flex-1 flex-col bg-[#09080e]/60 relative z-10">
          {mode === "allies" ? (
            <AlliesView
              allies={allies}
              pledges={pledges}
              adventurers={adventurers}
              currentAdventurer={currentUser}
              presence={presence}
              onProfile={setProfile}
              onOpenDM={openDirectMessageWith}
            />
          ) : mode === "guild" && userGuilds.length === 0 ? (
            /* Fantasy Greeting View When Adventurer Has No Guilds */
            <WelcomeGreeting
              currentUser={currentUser}
              publicGuilds={guilds.filter((g) => g.is_public)}
              onCreateGuild={() => setCreateOpen(true)}
              onJoinGuild={() => setJoinOpen(true)}
              onQuickJoin={handleJoinGuild}
              onOpenDMs={() => setMode("dm")}
            />
          ) : (
            <>
              {/* Header Bar */}
              <header className="flex h-16 shrink-0 items-center justify-between gap-3 border-b border-[#241c3c] bg-[#0e0a1b]/80 px-4 backdrop-blur-xl select-none">
                <div className="flex min-w-0 flex-1 items-center gap-3">
                  <button
                    type="button"
                    onClick={() => setMobileSidebarOpen((prev) => !prev)}
                    className="grid size-9 place-items-center rounded-lg border border-[#2d244e] bg-[#140f26] text-slate-300 md:hidden hover:text-amber-200 cursor-pointer"
                    aria-label="Toggle navigation"
                  >
                    <Menu className="size-4" />
                  </button>

                  <div className="grid size-8 place-items-center rounded-lg bg-amber-500/15 border border-amber-500/30 text-amber-300 shrink-0">
                    {mode === "dm" ? <Shield className="size-4" /> : <Hash className="size-4" />}
                  </div>

                  <div className="min-w-0">
                    <h2 className="truncate font-[Cinzel] text-sm font-bold tracking-wide text-amber-200">
                      {mode === "dm"
                        ? selectedScrollTitle(selectedScroll, currentUser.id, adventurers)
                        : selectedRift?.name || "Sanctuary"}
                    </h2>
                    <p className="truncate text-xs text-slate-400">
                      {mode === "dm"
                        ? "Private Arcane Scroll"
                        : selectedRift?.topic || "Sacred gathering chamber"}
                    </p>
                  </div>
                </div>

                {/* Right Header Actions */}
                <div className="flex items-center gap-2">
                  {mode === "guild" && selectedRift && (
                    <div className="relative">
                      <button
                        type="button"
                        className={`flex h-9 items-center gap-1.5 rounded-lg border px-3 text-xs font-semibold transition-all cursor-pointer ${
                          pinnedOpen || pinnedEchoes.length > 0
                            ? "border-amber-400/50 bg-amber-500/15 text-amber-300"
                            : "border-[#2c234a] bg-[#130f25] text-slate-400 hover:text-slate-200"
                        }`}
                        onClick={() => setPinnedOpen((value) => !value)}
                      >
                        <Pin className="size-3.5" />
                        <span className="hidden sm:inline">
                          {pinnedEchoes.length} Pinned
                        </span>
                      </button>

                      {/* Pinned Echoes Popover */}
                      {pinnedOpen && (
                        <div className="absolute right-0 top-11 z-40 w-80 rounded-xl border border-[#382b5e] bg-[#16112a] p-3 shadow-2xl shadow-black/80 animate-slide-up">
                          <div className="flex items-center justify-between pb-2 border-b border-[#292044] mb-2">
                            <span className="text-[11px] font-bold uppercase tracking-wider text-amber-300">
                              Pinned Echoes ({pinnedEchoes.length})
                            </span>
                            <button
                              type="button"
                              onClick={() => setPinnedOpen(false)}
                              className="text-slate-400 hover:text-white"
                            >
                              <X className="size-3.5" />
                            </button>
                          </div>

                          <div className="space-y-1.5 max-h-64 overflow-y-auto">
                            {pinnedEchoes.length === 0 ? (
                              <p className="text-xs text-slate-400 p-2 text-center">
                                No pinned echoes in this rift yet.
                              </p>
                            ) : (
                              pinnedEchoes.map((echo) => (
                                <div
                                  key={echo.id}
                                  className="flex items-start justify-between gap-2 rounded-lg bg-[#110d22] p-2 text-left text-xs text-slate-200 border border-[#241c3d]"
                                >
                                  <span className="line-clamp-2">{echo.content}</span>
                                  <button
                                    type="button"
                                    onClick={() => togglePinEcho(echo.id)}
                                    className="text-amber-400 hover:text-rose-400 text-[10px] font-medium shrink-0 pt-0.5"
                                  >
                                    Unpin
                                  </button>
                                </div>
                              ))
                            )}
                          </div>
                        </div>
                      )}
                    </div>
                  )}

                  {/* Notification Bell */}
                  <button
                    type="button"
                    onClick={() => {
                      setNotificationNote("All arcane runes are active and in harmony.");
                      setTimeout(() => setNotificationNote(null), 3000);
                    }}
                    className="grid size-9 place-items-center rounded-lg border border-[#2c234a] bg-[#130f25] text-slate-400 hover:bg-[#1f173b] hover:text-amber-200 transition-colors cursor-pointer"
                    title="Arcane notifications"
                    aria-label="Arcane notifications"
                  >
                    <Bell className="size-4" />
                  </button>

                  {/* Mobile Members Toggle Button */}
                  {mode === "guild" && selectedGuild && (
                    <button
                      type="button"
                      onClick={() => setMobileMembersOpen((prev) => !prev)}
                      className="grid size-9 place-items-center rounded-lg border border-[#2c234a] bg-[#130f25] text-slate-400 xl:hidden hover:text-amber-200 cursor-pointer"
                      title="Chamber Adventurers"
                      aria-label="Chamber Adventurers"
                    >
                      <Users className="size-4" />
                    </button>
                  )}
                </div>
              </header>

              {/* Notification Banner */}
              {notificationNote && (
                <div className="bg-amber-500/20 border-b border-amber-500/30 px-4 py-2 text-xs text-amber-200 flex items-center justify-between animate-slide-up">
                  <span>{notificationNote}</span>
                  <button
                    type="button"
                    onClick={() => setNotificationNote(null)}
                    className="text-amber-300 hover:text-white"
                  >
                    <X className="size-3.5" />
                  </button>
                </div>
              )}

              {/* Chat & Right Member List Grid */}
              <div className="flex min-h-0 flex-1 relative">
                <section className="flex min-w-0 flex-1 flex-col">
                  <ChatView
                    mode={mode as "guild" | "dm" | "allies"}
                    messages={messages}
                    adventurers={adventurers}
                    presence={presence}
                    echoes={echoes}
                    whispers={whispers}
                    onReply={setReplyingTo}
                    onProfile={setProfile}
                  />
                  <MessageInput
                    label={
                      mode === "dm"
                        ? selectedScrollTitle(selectedScroll, currentUser.id, adventurers)
                        : `#${selectedRift?.name || "chamber"}`
                    }
                    replyingTo={replyingTo}
                    onCancelReply={() => setReplyingTo(null)}
                    onSend={sendMessage}
                  />
                </section>

                {/* Right Member List */}
                {mode === "guild" && selectedGuild && (
                  <>
                    <aside
                      className={`fixed inset-y-0 right-0 z-30 w-[240px] bg-[#0c0918] border-l border-[#241c3c] transition-transform duration-200 xl:static xl:translate-x-0 ${
                        mobileMembersOpen ? "translate-x-0" : "translate-x-full xl:translate-x-0"
                      }`}
                    >
                      <MemberList
                        members={members}
                        presence={presence}
                        onProfile={setProfile}
                      />
                    </aside>

                    {mobileMembersOpen && (
                      <div
                        className="fixed inset-0 z-25 bg-black/60 backdrop-blur-sm xl:hidden"
                        onClick={() => setMobileMembersOpen(false)}
                      />
                    )}
                  </>
                )}
              </div>
            </>
          )}
        </main>
      </div>

      {/* Global Modals */}
      <CreateGuildModal
        open={createOpen}
        onOpenChange={setCreateOpen}
        onCreate={handleCreateGuild}
      />
      <JoinGuildModal
        open={joinOpen}
        onOpenChange={setJoinOpen}
        onJoin={handleJoinGuild}
      />
      <UserProfileModal
        adventurer={profile}
        presence={profile ? presence[profile.id] : undefined}
        isAlly={Boolean(
          profile &&
            currentUser &&
            allies.some(
              (ally) =>
                (ally.adventurer_id === currentUser.id && ally.ally_id === profile.id) ||
                (ally.ally_id === currentUser.id && ally.adventurer_id === profile.id)
            )
        )}
        onOpenChange={(open) => !open && setProfile(null)}
        onMessage={openDirectMessageWith}
      />
    </div>
  );
}

function selectedScrollTitle(
  scroll: { adventurer_one_id: string; adventurer_two_id: string },
  currentId: string,
  adventurersList: Adventurer[]
) {
  const otherId =
    scroll.adventurer_one_id === currentId
      ? scroll.adventurer_two_id
      : scroll.adventurer_one_id;
  return (
    adventurersList.find((adventurer) => adventurer.id === otherId)?.display_name ??
    "Unknown Adventurer"
  );
}
