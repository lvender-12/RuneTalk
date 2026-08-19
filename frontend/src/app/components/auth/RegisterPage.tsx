import React, { useState } from "react";
import { Shield, Mail, KeyRound, User, ArrowRight, LogIn } from "lucide-react";
import { Button } from "../shared/Button";
import { Input } from "../shared/Input";

interface RegisterPageProps {
  onLogin: () => void;
  onRegisterSubmit: (username: string, email: string, pass: string) => { success: boolean; error?: string };
}

export function RegisterPage({ onLogin, onRegisterSubmit }: RegisterPageProps) {
  const [username, setUsername] = useState("");
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (!username.trim() || !email.trim() || !password.trim()) {
      setError("All fields are required");
      return;
    }
    if (username.trim().length < 3) {
      setError("Username must be at least 3 characters");
      return;
    }
    if (password.length < 6) {
      setError("Password must be at least 6 characters");
      return;
    }

    setError(null);
    setLoading(true);

    setTimeout(() => {
      setLoading(false);
      const res = onRegisterSubmit(username.trim(), email.trim(), password.trim());
      if (!res.success) {
        setError(res.error || "Registration failed");
      }
    }, 400);
  };

  return (
    <div className="flex min-h-screen w-full items-center justify-center bg-[#07050d] p-4 text-slate-100 selection:bg-amber-500/30 selection:text-amber-200">
      <div className="absolute inset-0 overflow-hidden pointer-events-none">
        <div className="absolute -top-40 left-1/2 -translate-x-1/2 size-[600px] rounded-full bg-amber-500/10 blur-[120px]" />
        <div className="absolute -bottom-40 right-1/4 size-[500px] rounded-full bg-purple-600/10 blur-[140px]" />
      </div>

      <div className="relative w-full max-w-md rounded-2xl border border-[#342858] bg-[#130e26]/95 p-8 shadow-[0_15px_50px_rgba(0,0,0,0.85)] backdrop-blur-xl animate-scale-in">
        <div className="text-center space-y-2 mb-8">
          <div className="mx-auto grid size-16 place-items-center rounded-2xl bg-gradient-to-tr from-amber-500 via-amber-400 to-yellow-500 text-slate-950 shadow-[0_0_25px_rgba(245,158,11,0.4)]">
            <Shield className="size-8 stroke-[2.2]" />
          </div>
          <h1 className="font-[Cinzel] text-2xl font-bold tracking-wider text-amber-200 pt-2">
            Inscribe Adventurer
          </h1>
          <p className="text-xs text-slate-400">
            Forge your character profile and join the guild registry
          </p>
        </div>

        <form onSubmit={handleSubmit} className="space-y-4">
          <Input
            label="Adventurer Username"
            placeholder="e.g. alex, shadowkeeper..."
            value={username}
            onChange={(e) => setUsername(e.target.value)}
            leftIcon={<User className="size-4 text-slate-400" />}
            helperText="Minimum 3 characters"
            required
          />

          <Input
            label="Email Address"
            type="email"
            placeholder="adventurer@runetalk.gg"
            value={email}
            onChange={(e) => setEmail(e.target.value)}
            leftIcon={<Mail className="size-4 text-slate-400" />}
            required
          />

          <Input
            label="Password"
            type="password"
            placeholder="Minimum 6 characters"
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
            Continue to Verification
          </Button>
        </form>

        <div className="mt-6 pt-6 border-t border-[#261e40] text-center text-xs text-slate-400">
          Already forged your character?{" "}
          <button
            type="button"
            onClick={onLogin}
            className="inline-flex items-center gap-1 font-semibold text-amber-300 hover:text-amber-200 hover:underline transition-colors cursor-pointer"
          >
            <LogIn className="size-3.5" />
            Sign In
          </button>
        </div>
      </div>
    </div>
  );
}
