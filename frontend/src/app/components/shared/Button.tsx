import React from "react";

export interface ButtonProps extends React.ButtonHTMLAttributes<HTMLButtonElement> {
  variant?: "primary" | "secondary" | "danger" | "ghost" | "outline" | "rune";
  size?: "xs" | "sm" | "md" | "lg" | "icon";
  loading?: boolean;
  leftIcon?: React.ReactNode;
  rightIcon?: React.ReactNode;
}

const VARIANTS = {
  primary:
    "bg-gradient-to-r from-amber-500 via-amber-400 to-yellow-500 text-slate-950 font-semibold hover:from-amber-400 hover:to-yellow-400 shadow-[0_0_15px_rgba(245,158,11,0.25)] hover:shadow-[0_0_20px_rgba(245,158,11,0.4)] border-transparent",
  secondary:
    "bg-[#1b1630] hover:bg-[#261f42] text-slate-200 border-[#362d59] hover:border-amber-500/40 hover:text-amber-200",
  danger:
    "bg-gradient-to-r from-rose-600 to-red-600 hover:from-rose-500 hover:to-red-500 text-white shadow-[0_0_15px_rgba(244,63,94,0.25)] border-transparent",
  ghost:
    "bg-transparent hover:bg-white/5 text-slate-300 hover:text-white border-transparent",
  outline:
    "bg-transparent border-[#362d59] text-slate-300 hover:border-amber-500/50 hover:bg-amber-500/10 hover:text-amber-300",
  rune:
    "bg-[#141026] border-amber-500/40 text-amber-300 hover:bg-amber-500/15 hover:border-amber-400 shadow-[0_0_12px_rgba(212,175,55,0.2)]",
};

const SIZES = {
  xs: "h-7 px-2.5 text-xs gap-1.5 rounded",
  sm: "h-8 px-3 text-xs gap-2 rounded-md",
  md: "h-9 px-4 text-sm gap-2 rounded-md",
  lg: "h-11 px-5 text-base gap-2.5 rounded-lg",
  icon: "size-9 p-0 justify-center rounded-md",
};

export const Button = React.forwardRef<HTMLButtonElement, ButtonProps>(
  (
    {
      variant = "primary",
      size = "md",
      loading = false,
      leftIcon,
      rightIcon,
      className = "",
      disabled,
      children,
      ...props
    },
    ref
  ) => {
    const isDisabled = disabled || loading;

    return (
      <button
        ref={ref}
        disabled={isDisabled}
        className={`relative inline-flex items-center justify-center font-medium border transition-all duration-150 active:scale-[0.98] disabled:opacity-50 disabled:pointer-events-none disabled:active:scale-100 select-none cursor-pointer focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-amber-500/50 ${VARIANTS[variant]} ${SIZES[size]} ${className}`}
        {...props}
      >
        {loading ? (
          <span className="inline-block size-4 border-2 border-current border-t-transparent rounded-full animate-spin shrink-0" />
        ) : (
          leftIcon && <span className="shrink-0">{leftIcon}</span>
        )}
        {children && <span>{children}</span>}
        {!loading && rightIcon && <span className="shrink-0">{rightIcon}</span>}
      </button>
    );
  }
);

Button.displayName = "Button";
