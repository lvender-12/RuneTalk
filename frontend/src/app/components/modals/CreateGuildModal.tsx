import { useState } from "react";
import { Globe, Lock } from "lucide-react";
import { Modal } from "../shared/Modal";
import { Button } from "../shared/Button";
import { Input } from "../shared/Input";

interface CreateGuildModalProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  onCreate?: (name: string, description: string, isPublic: boolean) => void;
}

export function CreateGuildModal({
  open,
  onOpenChange,
  onCreate,
}: CreateGuildModalProps) {
  const [name, setName] = useState("");
  const [description, setDescription] = useState("");
  const [isPublic, setIsPublic] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (!name.trim()) {
      setError("Guild name is required");
      return;
    }
    if (name.trim().length < 4) {
      setError("Guild name must be at least 4 characters");
      return;
    }
    setError(null);
    onCreate?.(name.trim(), description.trim(), isPublic);
    setName("");
    setDescription("");
    onOpenChange(false);
  };

  const monogram = name.trim().slice(0, 3).toUpperCase() || "NEW";

  return (
    <Modal
      open={open}
      onOpenChange={onOpenChange}
      title="Forge a New Guild"
      description="Create a sacred community for your fellow adventurers"
      maxWidth="md"
    >
      <form onSubmit={handleSubmit} className="space-y-4">
        {/* Guild Icon Preview */}
        <div className="flex flex-col items-center justify-center p-3 mb-2">
          <div className="grid size-16 place-items-center rounded-2xl bg-gradient-to-br from-purple-700 via-indigo-800 to-[#1e173e] text-amber-200 ring-2 ring-amber-400/60 shadow-[0_0_25px_rgba(168,85,247,0.35)]">
            <span className="font-[Cinzel] text-base font-bold tracking-widest">
              {monogram}
            </span>
          </div>
          <span className="text-[11px] text-slate-400 mt-2">Guild Icon Preview</span>
        </div>

        <Input
          label="Guild Name"
          placeholder="e.g. Order of the Sunken Rune"
          value={name}
          onChange={(e) => setName(e.target.value)}
          helperText="Minimum 4 characters"
          error={error || undefined}
          required
        />

        <Input
          label="Guild Description"
          placeholder="What quest or craft unites this order?"
          value={description}
          onChange={(e) => setDescription(e.target.value)}
        />

        {/* Visibility Switch */}
        <div className="space-y-1.5 pt-1">
          <label className="block text-xs font-semibold uppercase tracking-wider text-slate-300">
            Guild Visibility
          </label>
          <div className="grid grid-cols-2 gap-2">
            <button
              type="button"
              onClick={() => setIsPublic(true)}
              className={`flex items-center gap-2 rounded-lg border p-3 text-left transition-all cursor-pointer ${
                isPublic
                  ? "border-amber-400/80 bg-amber-500/15 text-amber-200"
                  : "border-[#2d244f] bg-[#141026] text-slate-400 hover:bg-[#1c1634]"
              }`}
            >
              <Globe className="size-4 shrink-0 text-amber-400" />
              <div>
                <p className="text-xs font-bold">Public Order</p>
                <p className="text-[10px] text-slate-400">Discoverable by all</p>
              </div>
            </button>

            <button
              type="button"
              onClick={() => setIsPublic(false)}
              className={`flex items-center gap-2 rounded-lg border p-3 text-left transition-all cursor-pointer ${
                !isPublic
                  ? "border-amber-400/80 bg-amber-500/15 text-amber-200"
                  : "border-[#2d244f] bg-[#141026] text-slate-400 hover:bg-[#1c1634]"
              }`}
            >
              <Lock className="size-4 shrink-0 text-purple-400" />
              <div>
                <p className="text-xs font-bold">Private Sanctuary</p>
                <p className="text-[10px] text-slate-400">Invite-code only</p>
              </div>
            </button>
          </div>
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
            Forge Guild
          </Button>
        </div>
      </form>
    </Modal>
  );
}
