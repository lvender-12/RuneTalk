import { useMemo, useState } from "react";
import { Bell, Hash, Pin, Search, Shield } from "lucide-react";
import { LoginPage } from "./components/auth/LoginPage";
import { RegisterPage } from "./components/auth/RegisterPage";
import { OtpPage } from "./components/auth/OtpPage";
import { ChatView } from "./components/chat/ChatView";
import { MessageInput } from "./components/chat/MessageInput";
import { AlliesView } from "./components/friends/AlliesView";
import { ChannelSidebar } from "./components/layout/ChannelSidebar";
import { DMSidebar } from "./components/layout/DMSidebar";
import { GuildSidebar } from "./components/layout/GuildSidebar";
import { MemberList } from "./components/layout/MemberList";
import { UserPanel } from "./components/layout/UserPanel";
import { CreateGuildModal } from "./components/modals/CreateGuildModal";
import { JoinGuildModal } from "./components/modals/JoinGuildModal";
import { UserProfileModal } from "./components/modals/UserProfileModal";
import {
  adventurers,
  allies,
  currentAdventurer,
  echoes as initialEchoes,
  guildMembers,
  guilds,
  pledges,
  presence as initialPresence,
  rifts,
  scrolls,
  whispers as initialWhispers,
  type Adventurer,
  type Echo,
  type Presence,
  type Whisper,
} from "./data/mock";
import { normalizePresenceMap, updateCurrentPresenceStatus, updatePresenceStatus } from "./utils/customStatus";

type AppMode = "login" | "register" | "otp" | "guild" | "dm" | "allies";

function newId(prefix: string) {
  return `${prefix}-${Date.now()}-${Math.random().toString(16).slice(2)}`;
}

export default function App() {
  const [mode, setMode] = useState<AppMode>("guild");
  const [afterOtpMode, setAfterOtpMode] = useState<AppMode>("guild");
  const [selectedGuildId, setSelectedGuildId] = useState(guilds[0].id);
  const [selectedRiftId, setSelectedRiftId] = useState(rifts.find((rift) => rift.guild_id === guilds[0].id)?.id ?? rifts[0].id);
  const [selectedScrollId, setSelectedScrollId] = useState(scrolls[0].id);
  const [echoes, setEchoes] = useState<Echo[]>(() => [...initialEchoes]);
  const [whispers, setWhispers] = useState<Whisper[]>(() => [...initialWhispers]);
  const [presence, setPresence] = useState<Record<string, Presence>>(() => normalizePresenceMap(initialPresence));
  const [replyingTo, setReplyingTo] = useState<Echo | Whisper | null>(null);
  const [pinnedOpen, setPinnedOpen] = useState(false);
  const [createOpen, setCreateOpen] = useState(false);
  const [joinOpen, setJoinOpen] = useState(false);
  const [profile, setProfile] = useState<Adventurer | null>(null);

  const selectedGuild = guilds.find((guild) => guild.id === selectedGuildId) ?? guilds[0];
  const guildRifts = rifts.filter((rift) => rift.guild_id === selectedGuild.id);
  const selectedRift = guildRifts.find((rift) => rift.id === selectedRiftId) ?? guildRifts[0];
  const selectedScroll = scrolls.find((scroll) => scroll.id === selectedScrollId) ?? scrolls[0];

  const members = useMemo(() => {
    const ids = guildMembers.filter((member) => member.guild_id === selectedGuild.id).map((member) => member.adventurer_id);
    return adventurers.filter((adventurer) => ids.includes(adventurer.id));
  }, [selectedGuild.id]);

  const messages = mode === "dm"
    ? whispers.filter((whisper) => whisper.scroll_id === selectedScroll.id)
    : echoes.filter((echo) => echo.rift_id === selectedRift.id);

  const pinnedEchoes = echoes.filter((echo) => echo.rift_id === selectedRift.id && echo.is_pinned);

  function enterOtp(targetMode: AppMode) {
    setAfterOtpMode(targetMode);
    setMode("otp");
  }

  function selectGuild(guildId: string) {
    const nextRift = rifts.find((rift) => rift.guild_id === guildId);
    setSelectedGuildId(guildId);
    if (nextRift) setSelectedRiftId(nextRift.id);
    setMode("guild");
    setReplyingTo(null);
  }

  function sendMessage(content: string) {
    if (mode === "dm") {
      const whisper: Whisper = {
        id: newId("whisper"),
        scroll_id: selectedScroll.id,
        adventurer_id: currentAdventurer.id,
        content,
        reply_to_whisper_id: replyingTo?.id,
        created_at: new Date().toISOString(),
      };
      setWhispers((items) => [...items, whisper]);
      setReplyingTo(null);
      return;
    }

    const echo: Echo = {
      id: newId("echo"),
      rift_id: selectedRift.id,
      adventurer_id: currentAdventurer.id,
      content,
      reply_to_echo_id: replyingTo?.id,
      is_pinned: false,
      created_at: new Date().toISOString(),
    };
    setEchoes((items) => [...items, echo]);
    setReplyingTo(null);
  }

  if (mode === "login") {
    return <LoginPage onRegister={() => setMode("register")} onSubmit={() => enterOtp("guild")} />;
  }

  if (mode === "register") {
    return <RegisterPage onLogin={() => setMode("login")} onSubmit={() => enterOtp("guild")} />;
  }

  if (mode === "otp") {
    return <OtpPage onBack={() => setMode("login")} onVerified={() => setMode(afterOtpMode)} />;
  }

  return (
    <div className="h-full bg-background text-foreground">
      <div className="flex h-full min-h-0 bg-[radial-gradient(circle_at_30%_-20%,rgba(201,162,39,0.16),transparent_34%),linear-gradient(135deg,#0e0c17_0%,#151226_42%,#0b1018_100%)]">
        <GuildSidebar
          guilds={guilds}
          selectedGuildId={selectedGuild.id}
          mode={mode}
          onHome={() => {
            setMode("dm");
            setReplyingTo(null);
          }}
          onGuildSelect={selectGuild}
          onCreateGuild={() => setCreateOpen(true)}
          onJoinGuild={() => setJoinOpen(true)}
        />

        <div className="hidden min-h-0 w-[248px] shrink-0 border-r border-sidebar-border bg-sidebar/95 lg:flex lg:flex-col">
          {mode === "dm" || mode === "allies" ? (
            <DMSidebar
              allies={allies}
              adventurers={adventurers}
              currentAdventurerId={currentAdventurer.id}
              presence={presence}
              scrolls={scrolls}
              selectedScrollId={selectedScroll.id}
              onAllies={() => setMode("allies")}
              onScrollSelect={(id) => {
                setSelectedScrollId(id);
                setMode("dm");
                setReplyingTo(null);
              }}
              onProfile={setProfile}
            />
          ) : (
            <ChannelSidebar
              guild={selectedGuild}
              rifts={guildRifts}
              selectedRiftId={selectedRift.id}
              onRiftSelect={(id) => {
                setSelectedRiftId(id);
                setReplyingTo(null);
              }}
            />
          )}
          <UserPanel
            adventurer={currentAdventurer}
            presence={presence[currentAdventurer.id]}
            onPresenceChange={(status, statusText, statusIcon) =>
              setPresence((value) => ({
                ...value,
                [currentAdventurer.id]: updatePresenceStatus({
                  ...value[currentAdventurer.id],
                  statusByPresence: {
                    ...value[currentAdventurer.id].statusByPresence,
                    [status]: { emoji: statusIcon, text: statusText },
                  },
                }, status),
              }))
            }
            onStatusChange={(emoji, text) =>
              setPresence((value) => ({
                ...value,
                [currentAdventurer.id]: updateCurrentPresenceStatus(value[currentAdventurer.id], emoji, text),
              }))
            }
            onProfile={() => setProfile(currentAdventurer)}
          />
        </div>

        <main className="flex min-w-0 flex-1 flex-col">
          {mode === "allies" ? (
            <AlliesView
              allies={allies}
              pledges={pledges}
              adventurers={adventurers}
              currentAdventurer={currentAdventurer}
              presence={presence}
              onProfile={setProfile}
            />
          ) : (
            <>
              <header className="flex h-16 shrink-0 items-center gap-3 border-b border-border bg-background/70 px-4 backdrop-blur-xl">
                <div className="flex min-w-0 flex-1 items-center gap-3">
                  {mode === "dm" ? <Shield className="size-5 text-primary" /> : <Hash className="size-5 text-primary" />}
                  <div className="min-w-0">
                    <div className="truncate font-[Cinzel] text-sm font-bold tracking-wide text-[#f3e8c7]">
                      {mode === "dm" ? selectedScrollTitle(selectedScroll, currentAdventurer.id) : selectedRift.name}
                    </div>
                    <p className="truncate text-xs text-muted-foreground">
                      {mode === "dm" ? "Private scroll between adventurers" : selectedRift.topic}
                    </p>
                  </div>
                </div>

                {mode === "guild" && (
                  <div className="relative">
                    <button
                      className="flex h-9 items-center gap-2 rounded-md border border-border bg-card/60 px-3 text-sm text-muted-foreground transition hover:border-primary/30 hover:text-foreground"
                      onClick={() => setPinnedOpen((value) => !value)}
                    >
                      <Pin className="size-4" />
                      <span className="hidden sm:inline">{pinnedEchoes.length} pinned</span>
                    </button>
                    {pinnedOpen && (
                      <div className="absolute right-0 top-11 z-20 w-80 rounded-md border border-border bg-popover p-3 shadow-2xl shadow-black/40">
                        <p className="mb-2 text-xs font-semibold uppercase tracking-[0.2em] text-primary">Pinned echoes</p>
                        <div className="space-y-2">
                          {pinnedEchoes.map((echo) => (
                            <button
                              key={echo.id}
                              className="w-full rounded-md bg-muted/70 p-2 text-left text-sm text-foreground transition hover:bg-accent"
                              onClick={() => setPinnedOpen(false)}
                            >
                              {echo.content}
                            </button>
                          ))}
                        </div>
                      </div>
                    )}
                  </div>
                )}

                <button className="hidden h-9 items-center gap-2 rounded-md border border-border bg-card/60 px-3 text-sm text-muted-foreground md:flex">
                  <Search className="size-4" />
                  Search
                </button>
                <button className="grid size-9 place-items-center rounded-md border border-border bg-card/60 text-muted-foreground transition hover:text-foreground">
                  <Bell className="size-4" />
                </button>
              </header>

              <div className="flex min-h-0 flex-1">
                <section className="flex min-w-0 flex-1 flex-col">
                  <ChatView
                    mode={mode}
                    messages={messages}
                    adventurers={adventurers}
                    presence={presence}
                    echoes={echoes}
                    whispers={whispers}
                    onReply={setReplyingTo}
                    onProfile={setProfile}
                  />
                  <MessageInput
                    label={mode === "dm" ? selectedScrollTitle(selectedScroll, currentAdventurer.id) : `#${selectedRift.name}`}
                    replyingTo={replyingTo}
                    onCancelReply={() => setReplyingTo(null)}
                    onSend={sendMessage}
                  />
                </section>

                {mode === "guild" && (
                  <aside className="hidden w-[244px] shrink-0 border-l border-border bg-background/55 xl:block">
                    <MemberList members={members} presence={presence} onProfile={setProfile} />
                  </aside>
                )}
              </div>
            </>
          )}
        </main>
      </div>

      <CreateGuildModal open={createOpen} onOpenChange={setCreateOpen} />
      <JoinGuildModal open={joinOpen} onOpenChange={setJoinOpen} />
      <UserProfileModal
        adventurer={profile}
        presence={profile ? presence[profile.id] : undefined}
        isAlly={Boolean(profile && allies.some((ally) => ally.adventurer_id === currentAdventurer.id && ally.ally_id === profile.id))}
        onOpenChange={(open) => !open && setProfile(null)}
      />
    </div>
  );
}

function selectedScrollTitle(scroll: { adventurer_one_id: string; adventurer_two_id: string }, currentId: string) {
  const otherId = scroll.adventurer_one_id === currentId ? scroll.adventurer_two_id : scroll.adventurer_one_id;
  return adventurers.find((adventurer) => adventurer.id === otherId)?.display_name ?? "Unknown Adventurer";
}
