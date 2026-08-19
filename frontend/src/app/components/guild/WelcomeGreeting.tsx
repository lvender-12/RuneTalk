import { Shield, Plus, Compass, Sparkles, MessageSquare, ArrowRight } from "lucide-react";
import { Button } from "../shared/Button";
import type { Adventurer, Guild } from "../../data/mock";

interface WelcomeGreetingProps {
  currentUser: Adventurer;
  publicGuilds: Guild[];
  onCreateGuild: () => void;
  onJoinGuild: () => void;
  onQuickJoin: (inviteCode: string) => void;
  onOpenDMs: () => void;
}

export function WelcomeGreeting({
  currentUser,
  publicGuilds,
  onCreateGuild,
  onJoinGuild,
  onQuickJoin,
  onOpenDMs,
}: WelcomeGreetingProps) {
  return (
    <div className="flex-1 overflow-y-auto p-6 md:p-12 flex flex-col items-center justify-center text-center relative selection:bg-amber-500/30 selection:text-amber-200">
      {/* Background ambient magic aura */}
      <div className="absolute inset-0 overflow-hidden pointer-events-none">
        <div className="absolute top-10 left-1/2 -translate-x-1/2 size-[650px] rounded-full bg-amber-500/10 blur-[150px]" />
        <div className="absolute bottom-10 right-1/4 size-[500px] rounded-full bg-purple-600/10 blur-[160px]" />
      </div>

      <div className="relative z-10 max-w-2xl w-full space-y-8 animate-fade-in">
        {/* Fantasy Seal Icon */}
        <div className="relative mx-auto size-24">
          <div className="absolute inset-0 rounded-3xl bg-gradient-to-tr from-amber-500 via-yellow-400 to-amber-600 blur-lg opacity-40 animate-pulse" />
          <div className="relative grid size-24 place-items-center rounded-3xl bg-gradient-to-tr from-amber-500 via-amber-400 to-yellow-500 text-slate-950 shadow-2xl">
            <Shield className="size-12 stroke-[2.2]" />
          </div>
        </div>

        {/* Title & Lore Text */}
        <div className="space-y-3">
          <div className="inline-flex items-center gap-2 rounded-full border border-amber-500/30 bg-amber-500/10 px-3.5 py-1 text-xs font-semibold tracking-wider text-amber-300 uppercase">
            <Sparkles className="size-3.5" />
            Unpledged Adventurer
          </div>
          <h1 className="font-[Cinzel] text-3xl md:text-4xl font-bold tracking-wide text-amber-100">
            Hail, {currentUser.display_name}
          </h1>
          <p className="text-sm md:text-base text-slate-300 leading-relaxed font-sans max-w-lg mx-auto">
            Your name has been inscribed upon the eternal scrolls of RuneTalk, but you have not yet pledged allegiance to an order. The Great Realm awaits your legend.
          </p>
        </div>

        {/* Main Action Cards */}
        <div className="grid grid-cols-1 sm:grid-cols-2 gap-4 text-left pt-2">
          {/* Forge Order Card */}
          <div
            onClick={onCreateGuild}
            className="group relative rounded-2xl border border-[#342759] bg-[#140e28]/90 p-5 shadow-xl hover:border-amber-400/80 hover:bg-[#1a1236] transition-all duration-200 cursor-pointer flex flex-col justify-between"
          >
            <div className="space-y-2.5">
              <div className="grid size-11 place-items-center rounded-xl bg-amber-500/20 text-amber-300 border border-amber-500/40 group-hover:scale-105 transition-transform">
                <Plus className="size-6" />
              </div>
              <h3 className="font-[Cinzel] text-base font-bold text-amber-200 group-hover:text-amber-100">
                Forge a Guild
              </h3>
              <p className="text-xs text-slate-400 leading-relaxed">
                Found your own order, establish rift chambers, and summon adventurers under your banner.
              </p>
            </div>
            <div className="mt-4 flex items-center gap-1.5 text-xs font-bold text-amber-300 group-hover:translate-x-1 transition-transform">
              <span>Found Order</span>
              <ArrowRight className="size-3.5" />
            </div>
          </div>

          {/* Join Order Card */}
          <div
            onClick={onJoinGuild}
            className="group relative rounded-2xl border border-[#342759] bg-[#140e28]/90 p-5 shadow-xl hover:border-cyan-400/80 hover:bg-[#131b36] transition-all duration-200 cursor-pointer flex flex-col justify-between"
          >
            <div className="space-y-2.5">
              <div className="grid size-11 place-items-center rounded-xl bg-cyan-500/20 text-cyan-300 border border-cyan-500/40 group-hover:scale-105 transition-transform">
                <Compass className="size-6" />
              </div>
              <h3 className="font-[Cinzel] text-base font-bold text-cyan-200 group-hover:text-cyan-100">
                Enter with Rune Code
              </h3>
              <p className="text-xs text-slate-400 leading-relaxed">
                Inscribe an 8-character invite code dispatched by a Guildmaster to enter an existing sanctuary.
              </p>
            </div>
            <div className="mt-4 flex items-center gap-1.5 text-xs font-bold text-cyan-300 group-hover:translate-x-1 transition-transform">
              <span>Redeem Code</span>
              <ArrowRight className="size-3.5" />
            </div>
          </div>
        </div>

        {/* Existing Public Realms */}
        {publicGuilds.length > 0 && (
          <div className="rounded-2xl border border-[#2a2046] bg-[#0f0b1e]/80 p-5 text-left space-y-3">
            <div className="flex items-center justify-between">
              <span className="text-xs font-bold uppercase tracking-wider text-amber-300 font-[Cinzel]">
                Public Sanctuaries in the Realm
              </span>
              <span className="text-[11px] text-slate-400">Click to enter immediately</span>
            </div>

            <div className="grid grid-cols-1 sm:grid-cols-2 gap-2.5">
              {publicGuilds.map((g) => (
                <div
                  key={g.id}
                  className="flex items-center justify-between rounded-xl border border-[#281e42] bg-[#140f28] p-3 hover:border-amber-400/50 hover:bg-[#1c1539] transition-all"
                >
                  <div className="min-w-0 flex-1 pr-2">
                    <h4 className="font-[Cinzel] text-xs font-bold text-slate-100 truncate">
                      {g.name}
                    </h4>
                    <p className="text-[11px] text-slate-400 truncate">{g.description}</p>
                  </div>
                  <Button
                    size="sm"
                    variant="secondary"
                    onClick={() => onQuickJoin(g.invite_code)}
                    className="text-xs shrink-0 font-bold"
                  >
                    Join
                  </Button>
                </div>
              ))}
            </div>
          </div>
        )}

        {/* Footer Alternative Link */}
        <div className="pt-2 text-xs text-slate-400">
          Prefer private conversation first?{" "}
          <button
            type="button"
            onClick={onOpenDMs}
            className="inline-flex items-center gap-1 font-semibold text-amber-300 hover:text-amber-200 hover:underline transition-colors cursor-pointer"
          >
            <MessageSquare className="size-3.5" />
            Open Direct Scrolls & Allies
          </button>
        </div>
      </div>
    </div>
  );
}
