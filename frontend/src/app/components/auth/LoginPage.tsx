import React, { useState } from "react";
import { Shield, KeyRound, User, ArrowRight, UserPlus } from "lucide-react";
import { Button } from "../shared/Button";
import { Input } from "../shared/Input";
import type { Adventurer } from "../../data/mock";

interface LoginPageProps {
  onRegister: () => void;
  onLoginAttempt: (identifier: string, pass: string) => { success: boolean; error?: string; user?: Adventurer };
  existingUsers: Adventurer[];
}

export function LoginPage({
  onRegister,
  onLoginAttempt,
  existingUsers,
}: LoginPageProps) {
  const [identifier, setIdentifier] = useState("");
  const [password, setPassword] = useState("");
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (!identifier.trim() || !password.trim()) {
      setError("Please fill in both identifier and password");
      return;
    }

    setError(null);
    setLoading(true);

    setTimeout(() => {
      setLoading(false);
      const result = onLoginAttempt(identifier.trim(), password.trim());
      if (!result.success) {
        setError(result.error || "Invalid username or password");
      }
    }, 400);
  };

  return (
    <div className="flex min-h-screen w-full items-center justify-center bg-[#07050d] p-4 text-slate-100 selection:bg-amber-500/30 selection:text-amber-200">
      {/* Background ambient glow */}
      <div className="absolute inset-0 overflow-hidden pointer-events-none">
        <div className="absolute -top-40 left-1/2 -translate-x-1/2 size-[600px] rounded-full bg-amber-500/10 blur-[120px]" />
        <div className="absolute -bottom-40 left-1/4 size-[500px] rounded-full bg-purple-600/10 blur-[140px]" />
      </div>

      <div className="relative w-full max-w-md rounded-2xl border border-[#342858] bg-[#130e26]/95 p-8 shadow-[0_15px_50px_rgba(0,0,0,0.85)] backdrop-blur-xl animate-scale-in">
        {/* Header Rune Emblem */}
        <div className="text-center space-y-2 mb-8">
          <div className="mx-auto grid size-16 place-items-center rounded-2xl bg-gradient-to-tr from-amber-500 via-amber-400 to-yellow-500 text-slate-950 shadow-[0_0_25px_rgba(245,158,11,0.4)]">
            <Shield className="size-8 stroke-[2.2]" />
          </div>
          <h1 className="font-[Cinzel] text-2xl font-bold tracking-wider text-amber-200 pt-2">
            RuneTalk
          </h1>
          <p className="text-xs text-slate-400">
            Enter your adventurer credentials to access the realm
          </p>
        </div>

        {/* Login Form */}
        <form onSubmit={handleSubmit} className="space-y-4">
          <Input
            label="Username or Email"
            placeholder="Enter your registered username or email"
            value={identifier}
            onChange={(e) => setIdentifier(e.target.value)}
            leftIcon={<User className="size-4 text-slate-400" />}
            required
          />

          <Input
            label="Password"
            type="password"
            placeholder="••••••••••••"
            value={password}
            onChange={(e) => setPassword(e.target.value)}
            leftIcon={<KeyRound className="size-4 text-slate-400" />}
            required
          />

          {error && (
            <div className="rounded-lg border border-rose-500/40 bg-rose-500/10 p-3 text-xs font-medium text-rose-300 animate-fade-in">
              {error}
            </div>
          )}

          <Button
            type="submit"
            variant="primary"
            size="lg"
            loading={loading}
            className="w-full mt-2 font-bold tracking-wide"
            rightIcon={<ArrowRight className="size-4" />}
          >
            Sign In to Realm
          </Button>
        </form>

        {/* Existing Accounts Helper (If any) */}
        {existingUsers.length > 0 && (
          <div className="mt-4 rounded-xl border border-[#2c244c] bg-[#0e0a1b] p-3 text-xs text-slate-400">
            <p className="font-semibold text-amber-300 mb-1">Registered Accounts:</p>
            <div className="flex flex-wrap gap-1.5 mt-1">
              {existingUsers.map((u) => (
                <button
                  key={u.id}
                  type="button"
                  onClick={() => {
                    setIdentifier(u.username);
                    setPassword("password123");
                    setError(null);
                  }}
                  className="rounded bg-[#1a1333] border border-[#352857] px-2 py-0.5 text-[11px] text-slate-200 hover:border-amber-400/60 hover:text-amber-200 transition-colors"
                >
                  @{u.username}
                </button>
              ))}
            </div>
          </div>
        )}

        {/* Footer Register Link */}
        <div className="mt-6 pt-6 border-t border-[#261e40] text-center text-xs text-slate-400">
          Not yet inscribed in the registry?{" "}
          <button
            type="button"
            onClick={onRegister}
            className="inline-flex items-center gap-1 font-semibold text-amber-300 hover:text-amber-200 hover:underline transition-colors cursor-pointer"
          >
            <UserPlus className="size-3.5" />
            Create an Account
          </button>
        </div>
      </div>
    </div>
  );
}
