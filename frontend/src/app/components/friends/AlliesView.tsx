import React, { useState } from "react";
import { Users, UserPlus, Check, X, MessageSquare, Search, Shield, Sparkles } from "lucide-react";
import type { Adventurer, Ally, Pledge, Presence } from "../../data/mock";
import { Avatar } from "../shared/Avatar";
import { Button } from "../shared/Button";
import { Input } from "../shared/Input";
import { formatCustomStatus, getPresenceConfig } from "../../utils/customStatus";

interface AlliesViewProps {
  allies: Ally[];
  pledges: Pledge[];
  adventurers: Adventurer[];
  currentAdventurer: Adventurer;
  presence: Record<string, Presence>;
  onProfile: (adventurer: Adventurer) => void;
  onOpenDM?: (adventurerId: string) => void;
}

type TabType = "online" | "all" | "pending" | "add";

export function AlliesView({
  allies,
  pledges,
  adventurers,
  currentAdventurer,
  presence,
  onProfile,
  onOpenDM,
}: AlliesViewProps) {
  const [activeTab, setActiveTab] = useState<TabType>("online");
  const [search, setSearch] = useState("");
  const [addUsername, setAddUsername] = useState("");
  const [addedMessage, setAddedMessage] = useState<string | null>(null);

  // Get full adventurer records for allies
  const allyAdventurers = allies
    .map((ally) => {
      const otherId =
        ally.adventurer_id === currentAdventurer.id ? ally.ally_id : ally.adventurer_id;
      return adventurers.find((a) => a.id === otherId);
    })
    .filter((a): a is Adventurer => Boolean(a));

  const onlineAllies = allyAdventurers.filter(
    (a) => presence[a.id]?.status && presence[a.id].status !== "offline"
  );

  const pendingPledges = pledges.filter(
    (p) => p.to_adventurer_id === currentAdventurer.id && p.status === "pending"
  );

  const filteredAllies = (activeTab === "online" ? onlineAllies : allyAdventurers).filter(
    (a) =>
      a.display_name.toLowerCase().includes(search.toLowerCase()) ||
      a.username.toLowerCase().includes(search.toLowerCase())
  );

  const handleSendPledge = (e: React.FormEvent) => {
    e.preventDefault();
    if (!addUsername.trim()) return;
    setAddedMessage(`Pledge request sent to @${addUsername.trim()}`);
    setAddUsername("");
    setTimeout(() => setAddedMessage(null), 4000);
  };

  return (
    <div className="flex flex-1 flex-col min-h-0 bg-[#0b0817] select-none">
      {/* Top Allies Navigation Header */}
      <header className="flex h-16 shrink-0 items-center justify-between border-b border-[#241c3c] bg-[#110d22] px-6">
        <div className="flex items-center gap-3">
          <div className="flex items-center gap-2 border-r border-[#2c234a] pr-4">
            <Users className="size-5 text-amber-400" />
            <h2 className="font-[Cinzel] text-base font-bold text-amber-200">
              Allies & Pledges
            </h2>
          </div>

          <div className="flex items-center gap-1">
            <button
              type="button"
              onClick={() => setActiveTab("online")}
              className={`rounded-md px-3 py-1.5 text-xs font-semibold transition-all cursor-pointer ${
                activeTab === "online"
                  ? "bg-amber-500/20 text-amber-300 border border-amber-500/40 shadow-sm"
                  : "text-slate-400 hover:text-slate-200 hover:bg-[#1a1433]"
              }`}
            >
              Online ({onlineAllies.length})
            </button>
            <button
              type="button"
              onClick={() => setActiveTab("all")}
              className={`rounded-md px-3 py-1.5 text-xs font-semibold transition-all cursor-pointer ${
                activeTab === "all"
                  ? "bg-amber-500/20 text-amber-300 border border-amber-500/40 shadow-sm"
                  : "text-slate-400 hover:text-slate-200 hover:bg-[#1a1433]"
              }`}
            >
              All ({allyAdventurers.length})
            </button>
            <button
              type="button"
              onClick={() => setActiveTab("pending")}
              className={`relative rounded-md px-3 py-1.5 text-xs font-semibold transition-all cursor-pointer ${
                activeTab === "pending"
                  ? "bg-amber-500/20 text-amber-300 border border-amber-500/40 shadow-sm"
                  : "text-slate-400 hover:text-slate-200 hover:bg-[#1a1433]"
              }`}
            >
              Pending ({pendingPledges.length})
              {pendingPledges.length > 0 && (
                <span className="ml-1.5 inline-block size-2 rounded-full bg-rose-500 animate-pulse" />
              )}
            </button>
            <button
              type="button"
              onClick={() => setActiveTab("add")}
              className={`ml-2 flex items-center gap-1.5 rounded-md px-3 py-1.5 text-xs font-bold transition-all cursor-pointer ${
                activeTab === "add"
                  ? "bg-emerald-500 text-slate-950 shadow-[0_0_15px_rgba(16,185,129,0.4)]"
                  : "bg-emerald-500/20 text-emerald-400 hover:bg-emerald-500 hover:text-slate-950"
              }`}
            >
              <UserPlus className="size-3.5" />
              Summon Ally
            </button>
          </div>
        </div>
      </header>

      {/* Main Content Area */}
      <div className="flex-1 overflow-y-auto p-6">
        {activeTab === "add" ? (
          /* Summon Ally Tab */
          <div className="max-w-xl mx-auto space-y-6 animate-slide-up">
            <div className="rounded-xl border border-[#2e234e] bg-[#140f26] p-6 shadow-xl">
              <h3 className="font-[Cinzel] text-lg font-bold text-amber-200">
                Summon an Adventurer
              </h3>
              <p className="mt-1 text-xs text-slate-400">
                Send a pledge of friendship using their secret adventurer username.
              </p>

              <form onSubmit={handleSendPledge} className="mt-5 space-y-4">
                <Input
                  label="Adventurer Username"
                  placeholder="e.g. zenn, lyra, eldrin..."
                  value={addUsername}
                  onChange={(e) => setAddUsername(e.target.value)}
                  leftIcon={<Sparkles className="size-4 text-amber-400" />}
                />
                <Button
                  type="submit"
                  variant="primary"
                  size="md"
                  disabled={!addUsername.trim()}
                  className="w-full"
                >
                  Send Pledge Request
                </Button>
              </form>

              {addedMessage && (
                <div className="mt-4 rounded-lg border border-emerald-500/40 bg-emerald-500/10 p-3 text-xs text-emerald-300 font-medium animate-fade-in">
                  {addedMessage}
                </div>
              )}
            </div>
          </div>
        ) : activeTab === "pending" ? (
          /* Pending Pledges Tab */
          <div className="max-w-2xl mx-auto space-y-4 animate-slide-up">
            <h3 className="text-xs font-bold uppercase tracking-wider text-slate-400">
              Pending Incoming Pledges ({pendingPledges.length})
            </h3>
            {pendingPledges.length === 0 ? (
              <div className="rounded-xl border border-[#2a2245] bg-[#130f25] p-8 text-center text-slate-400">
                <Shield className="mx-auto size-10 text-slate-500 mb-2 opacity-50" />
                <p className="text-sm">No pending friend pledges at the moment.</p>
              </div>
            ) : (
              pendingPledges.map((pledge) => {
                const requester = adventurers.find((a) => a.id === pledge.from_adventurer_id);
                if (!requester) return null;

                return (
                  <div
                    key={pledge.id}
                    className="flex items-center justify-between rounded-xl border border-[#2d244f] bg-[#16112a] p-4 shadow-md"
                  >
                    <div className="flex items-center gap-3">
                      <Avatar
                        adventurer={requester}
                        presence={presence[requester.id]}
                        size="md"
                        onClick={() => onProfile(requester)}
                      />
                      <div>
                        <p className="text-sm font-semibold text-slate-100">
                          {requester.display_name}
                        </p>
                        <p className="text-xs text-slate-400">@{requester.username}</p>
                      </div>
                    </div>

                    <div className="flex items-center gap-2">
                      <button
                        type="button"
                        className="grid size-9 place-items-center rounded-lg bg-emerald-600 text-white hover:bg-emerald-500 transition-colors shadow-sm cursor-pointer"
                        title="Accept pledge"
                      >
                        <Check className="size-4" />
                      </button>
                      <button
                        type="button"
                        className="grid size-9 place-items-center rounded-lg bg-rose-600/30 text-rose-300 hover:bg-rose-600 hover:text-white transition-colors cursor-pointer"
                        title="Decline pledge"
                      >
                        <X className="size-4" />
                      </button>
                    </div>
                  </div>
                );
              })
            )}
          </div>
        ) : (
          /* Online / All Allies List */
          <div className="max-w-4xl mx-auto space-y-4">
            {/* Search Filter */}
            <div className="relative mb-4">
              <Search className="absolute left-3 size-4 text-slate-400 pointer-events-none top-1/2 -translate-y-1/2" />
              <input
                type="text"
                value={search}
                onChange={(e) => setSearch(e.target.value)}
                placeholder="Search across all allies..."
                className="w-full h-10 rounded-lg bg-[#140f26] border border-[#2d244e] pl-9 pr-4 text-sm text-slate-100 placeholder-slate-400 focus:outline-none focus:border-amber-400/80 focus:ring-2 focus:ring-amber-400/20 transition-all"
              />
            </div>

            <div className="text-xs font-bold uppercase tracking-wider text-slate-400 mb-2">
              {activeTab === "online" ? "Online Allies" : "All Allies"} ({filteredAllies.length})
            </div>

            {filteredAllies.length === 0 ? (
              <div className="rounded-xl border border-[#2a2245] bg-[#130f25] p-8 text-center text-slate-400">
                <Users className="mx-auto size-10 text-slate-500 mb-2 opacity-50" />
                <p className="text-sm">No allies found.</p>
              </div>
            ) : (
              <div className="grid gap-2">
                {filteredAllies.map((ally) => {
                  const userPresence = presence[ally.id];
                  const statusText = formatCustomStatus(userPresence);

                  return (
                    <div
                      key={ally.id}
                      className="group flex items-center justify-between rounded-xl border border-[#282046] bg-[#140f26] px-4 py-3 hover:border-amber-400/40 hover:bg-[#1a1433] transition-all"
                    >
                      <div
                        className="flex items-center gap-3.5 flex-1 min-w-0 cursor-pointer"
                        onClick={() => onProfile(ally)}
                      >
                        <Avatar
                          adventurer={ally}
                          presence={userPresence}
                          size="md"
                        />
                        <div className="min-w-0 flex-1">
                          <div className="flex items-center gap-2">
                            <span className="truncate text-sm font-semibold text-slate-100 group-hover:text-amber-200 transition-colors">
                              {ally.display_name}
                            </span>
                            <span className="text-xs text-slate-400">
                              @{ally.username}
                            </span>
                          </div>
                          <p className="truncate text-xs text-slate-400 mt-0.5">
                            {statusText || getPresenceConfig(userPresence?.status).label}
                          </p>
                        </div>
                      </div>

                      <div className="flex items-center gap-2">
                        {onOpenDM && (
                          <button
                            type="button"
                            onClick={() => onOpenDM(ally.id)}
                            className="grid size-9 place-items-center rounded-lg bg-[#20183b] text-slate-300 hover:bg-amber-500 hover:text-slate-950 transition-all cursor-pointer shadow-sm"
                            title="Direct Message"
                          >
                            <MessageSquare className="size-4" />
                          </button>
                        )}
                      </div>
                    </div>
                  );
                })}
              </div>
            )}
          </div>
        )}
      </div>
    </div>
  );
}
