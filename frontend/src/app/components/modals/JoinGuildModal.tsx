import { useState } from "react";
import { KeyRound } from "lucide-react";
import { Modal } from "../shared/Modal";
import { Button } from "../shared/Button";
import { Input } from "../shared/Input";

interface JoinGuildModalProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  onJoin?: (inviteCode: string) => void;
}

export function JoinGuildModal({
  open,
  onOpenChange,
  onJoin,
}: JoinGuildModalProps) {
  const [inviteCode, setInviteCode] = useState("");
  const [error, setError] = useState<string | null>(null);

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    const code = inviteCode.trim().toUpperCase();
    if (!code) {
      setError("Please enter an invite code");
      return;
    }
    if (code.length < 6) {
      setError("Invite codes are typically 8 characters long");
      return;
    }
    setError(null);
    onJoin?.(code);
    setInviteCode("");
    onOpenChange(false);
  };

  return (
    <Modal
      open={open}
      onOpenChange={onOpenChange}
      title="Join an Arcane Guild"
      description="Enter the sacred 8-character invite code provided by a Guildmaster"
      maxWidth="md"
    >
      <form onSubmit={handleSubmit} className="space-y-4">
        <Input
          label="Invite Code"
          placeholder="e.g. ARCANUM7"
          value={inviteCode}
          onChange={(e) => setInviteCode(e.target.value.toUpperCase())}
          leftIcon={<KeyRound className="size-4 text-slate-400" />}
          error={error || undefined}
          helperText="Format: 8 uppercase characters"
          required
        />

        <div className="rounded-lg border border-[#2c244c] bg-[#120d22] p-3 text-xs text-slate-400">
          <p className="font-semibold text-amber-300 mb-1">Looking for an invite?</p>
          <p>
            Guildmasters can generate invite scrolls directly from their guild settings.
          </p>
        </div>

        <div className="mt-6 flex justify-end gap-2 pt-2">
          <Button
            type="button"
            variant="ghost"
            onClick={() => onOpenChange(false)}
          >
            Cancel
          </Button>
          <Button type="submit" variant="primary">
            Join Order
          </Button>
        </div>
      </form>
    </Modal>
  );
}
