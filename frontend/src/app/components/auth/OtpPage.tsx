import React, { useState, useRef, useEffect } from "react";
import { KeyRound, ArrowLeft, RefreshCw, CheckCircle2, Sparkles } from "lucide-react";
import { Button } from "../shared/Button";

interface OtpPageProps {
  email: string;
  expectedOtp: string;
  onBack: () => void;
  onVerified: () => void;
  onResendOtp?: () => string;
}

export function OtpPage({
  email,
  expectedOtp,
  onBack,
  onVerified,
  onResendOtp,
}: OtpPageProps) {
  const [digits, setDigits] = useState(["", "", "", "", "", ""]);
  const [currentExpectedOtp, setCurrentExpectedOtp] = useState(expectedOtp);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [countdown, setCountdown] = useState(45);
  const [resending, setResending] = useState(false);
  const [resentSuccess, setResentSuccess] = useState(false);
  const inputsRef = useRef<(HTMLInputElement | null)[]>([]);

  useEffect(() => {
    inputsRef.current[0]?.focus();
    const interval = setInterval(() => {
      setCountdown((prev) => (prev > 0 ? prev - 1 : 0));
    }, 1000);
    return () => clearInterval(interval);
  }, []);

  const handleChange = (index: number, value: string) => {
    if (!/^[0-9]?$/.test(value)) return;
    setError(null);
    const newDigits = [...digits];
    newDigits[index] = value;
    setDigits(newDigits);

    if (value && index < 5) {
      inputsRef.current[index + 1]?.focus();
    }

    if (value && index === 5 && newDigits.every((d) => d !== "")) {
      handleVerify(newDigits.join(""));
    }
  };

  const handleKeyDown = (index: number, e: React.KeyboardEvent<HTMLInputElement>) => {
    if (e.key === "Backspace" && !digits[index] && index > 0) {
      inputsRef.current[index - 1]?.focus();
    }
  };

  const handlePaste = (e: React.ClipboardEvent<HTMLInputElement>) => {
    e.preventDefault();
    const pasted = e.clipboardData.getData("text").replace(/[^0-9]/g, "").slice(0, 6);
    if (!pasted) return;
    const newDigits = [...digits];
    for (let i = 0; i < pasted.length; i++) {
      newDigits[i] = pasted[i];
    }
    setDigits(newDigits);
    if (pasted.length === 6) {
      handleVerify(pasted);
    } else {
      inputsRef.current[pasted.length]?.focus();
    }
  };

  const handleQuickFill = () => {
    const chars = currentExpectedOtp.split("").slice(0, 6);
    setDigits(chars);
    handleVerify(currentExpectedOtp);
  };

  const handleVerify = (code?: string) => {
    const fullCode = code || digits.join("");
    if (fullCode.length !== 6) {
      setError("Please enter all 6 digits");
      return;
    }

    if (fullCode !== currentExpectedOtp) {
      setError("Invalid OTP code. Please enter the code shown above.");
      return;
    }

    setError(null);
    setLoading(true);
    setTimeout(() => {
      setLoading(false);
      onVerified();
    }, 500);
  };

  const handleResend = () => {
    if (countdown > 0) return;
    setResending(true);
    setTimeout(() => {
      setResending(false);
      setCountdown(45);
      const nextOtp = onResendOtp ? onResendOtp() : Math.floor(100000 + Math.random() * 900000).toString();
      setCurrentExpectedOtp(nextOtp);
      setResentSuccess(true);
      setTimeout(() => setResentSuccess(false), 3000);
    }, 600);
  };

  return (
    <div className="flex min-h-screen w-full items-center justify-center bg-[#07050d] p-4 text-slate-100 selection:bg-amber-500/30 selection:text-amber-200">
      <div className="relative w-full max-w-md rounded-2xl border border-[#342858] bg-[#130e26]/95 p-8 shadow-[0_15px_50px_rgba(0,0,0,0.85)] backdrop-blur-xl animate-scale-in">
        <div className="text-center space-y-2 mb-6">
          <div className="mx-auto grid size-16 place-items-center rounded-2xl bg-gradient-to-tr from-amber-500 via-amber-400 to-yellow-500 text-slate-950 shadow-[0_0_25px_rgba(245,158,11,0.4)]">
            <KeyRound className="size-8 stroke-[2.2]" />
          </div>
          <h1 className="font-[Cinzel] text-2xl font-bold tracking-wider text-amber-200 pt-2">
            Verify Arcane Rune
          </h1>
          <p className="text-xs text-slate-400">
            A 6-digit verification code was dispatched for:
          </p>
          <p className="text-xs font-semibold text-amber-300 font-mono">
            {email}
          </p>
        </div>

        {/* Verification Code Display Badge */}
        <div className="mb-6 rounded-xl border border-amber-500/30 bg-amber-500/10 p-3 text-center">
          <p className="text-[11px] uppercase tracking-wider text-slate-400 font-medium">
            Your Rune Code:
          </p>
          <div className="flex items-center justify-center gap-2 mt-1">
            <span className="font-mono text-xl font-bold tracking-[0.25em] text-amber-300">
              {currentExpectedOtp}
            </span>
            <button
              type="button"
              onClick={handleQuickFill}
              className="flex items-center gap-1 rounded bg-amber-500/25 px-2 py-0.5 text-[11px] font-bold text-amber-200 hover:bg-amber-500/40 transition-colors cursor-pointer"
            >
              <Sparkles className="size-3 text-amber-400" />
              Auto Fill
            </button>
          </div>
        </div>

        {/* 6 Digit Input Group */}
        <div className="flex justify-center gap-2 mb-4">
          {digits.map((digit, index) => (
            <input
              key={index}
              ref={(el) => (inputsRef.current[index] = el)}
              type="text"
              inputMode="numeric"
              maxLength={1}
              value={digit}
              onChange={(e) => handleChange(index, e.target.value)}
              onKeyDown={(e) => handleKeyDown(index, e)}
              onPaste={handlePaste}
              className="size-12 rounded-lg bg-[#191233] border border-[#392c5e] text-center font-mono text-lg font-bold text-amber-200 focus:outline-none focus:border-amber-400 focus:ring-2 focus:ring-amber-400/30 transition-all"
            />
          ))}
        </div>

        {error && (
          <div className="mb-4 rounded-lg border border-rose-500/40 bg-rose-500/10 p-2.5 text-xs text-center font-medium text-rose-300 animate-fade-in">
            {error}
          </div>
        )}

        <Button
          type="button"
          variant="primary"
          size="lg"
          loading={loading}
          disabled={digits.some((d) => d === "")}
          onClick={() => handleVerify()}
          className="w-full font-bold tracking-wide"
        >
          Confirm & Access Realm
        </Button>

        {/* Resend & Status */}
        <div className="mt-6 flex items-center justify-between text-xs text-slate-400">
          <button
            type="button"
            onClick={onBack}
            className="flex items-center gap-1 hover:text-slate-200 transition-colors cursor-pointer"
          >
            <ArrowLeft className="size-3.5" /> Back
          </button>

          <button
            type="button"
            disabled={countdown > 0 || resending}
            onClick={handleResend}
            className={`flex items-center gap-1.5 font-medium transition-colors ${
              countdown > 0
                ? "text-slate-400 cursor-not-allowed"
                : "text-amber-400 hover:text-amber-300 cursor-pointer"
            }`}
          >
            <RefreshCw className={`size-3 ${resending ? "animate-spin" : ""}`} />
            {countdown > 0 ? `Resend in ${countdown}s` : "Resend Code"}
          </button>
        </div>

        {resentSuccess && (
          <div className="mt-4 flex items-center gap-2 rounded-lg border border-emerald-500/40 bg-emerald-500/10 p-2.5 text-xs text-emerald-300 font-medium animate-fade-in">
            <CheckCircle2 className="size-4 text-emerald-400" />
            A new OTP has been dispatched.
          </div>
        )}
      </div>
    </div>
  );
}
