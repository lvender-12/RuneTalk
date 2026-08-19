import React from "react";
import { Crown, Shield, Sparkles } from "lucide-react";

export interface BadgeProps {
  variant?: "owner" | "admin" | "member" | "gold" | "cyan" | "success" | "danger" | "neutral";
  size?: "sm" | "md";
  children: React.ReactNode;
  className?: string;
  icon?: React.ReactNode;
}

const VARIANTS = {
  owner:
    "bg-amber-500/15 text-amber-300 border-amber-500/30 shadow-[0_0_10px_rgba(245,158,11,0.15)]",
  admin:
    "bg-purple-500/15 text-purple-300 border-purple-500/30 shadow-[0_0_10px_rgba(168,85,247,0.15)]",
  member:
    "bg-slate-700/30 text-slate-300 border-slate-600/40",
  gold:
    "bg-amber-500/15 text-amber-300 border-amber-500/30",
  cyan:
    "bg-cyan-500/15 text-cyan-300 border-cyan-500/30 shadow-[0_0_10px_rgba(6,182,212,0.15)]",
  success:
    "bg-emerald-500/15 text-emerald-300 border-emerald-500/30",
  danger:
    "bg-rose-500/15 text-rose-300 border-rose-500/30",
  neutral:
    "bg-[#1c1733] text-slate-300 border-[#312854]",
};

const SIZES = {
  sm: "text-[10px] px-1.5 py-0.5 gap-1 rounded",
  md: "text-xs px-2 py-0.5 gap-1.5 rounded-md",
};

export function Badge({
  variant = "neutral",
  size = "md",
  children,
  className = "",
  icon,
}: BadgeProps) {
  const defaultIcon =
    variant === "owner" ? (
      <Crown className="size-3 text-amber-400 shrink-0" />
    ) : variant === "admin" ? (
      <Shield className="size-3 text-purple-400 shrink-0" />
    ) : variant === "cyan" ? (
      <Sparkles className="size-3 text-cyan-400 shrink-0" />
    ) : null;

  return (
    <span
      className={`inline-flex items-center font-medium border select-none ${VARIANTS[variant]} ${SIZES[size]} ${className}`}
    >
      {icon || defaultIcon}
      <span>{children}</span>
    </span>
  );
}
