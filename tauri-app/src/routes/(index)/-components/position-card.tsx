import { Command, ScreenPositions } from "@/context/commands";
import { InputData, ShortcutInput } from "./shortcut-input";
import { VisualPreview } from "./visual-preview";

type PositionCardProps = {
  id: ScreenPositions;
  label: string;
  inputData?: Command;
  onRecord: (data: InputData) => void;
  onRemove?: () => void;
};

export const PositionCard = ({
  id,
  label,
  inputData,
  onRecord,
  onRemove,
}: PositionCardProps) => {
  const hasShortcut = inputData !== undefined;

  return (
    <div className="group flex flex-col gap-3 p-4 border border-border rounded-lg bg-card hover:border-accent transition-all duration-200 hover:shadow-md">
      <label className="text-sm font-medium text-foreground text-center">
        {label}
      </label>

      <div className="flex justify-center">
        <VisualPreview type={id} />
      </div>

      <div className="flex justify-center items-center gap-1">
        <ShortcutInput
          key={inputData ? `${inputData.key_binding.modifiers}-${inputData.key_binding.key}` : "empty"}
          modifiers={inputData?.key_binding.modifiers ?? 0}
          actionKey={inputData?.key_binding.key ?? 0}
          onRecord={onRecord}
        />
        {hasShortcut && (
          <button
            type="button"
            onClick={onRemove}
            className="flex items-center justify-center w-7 h-7 rounded-sm border border-accent text-muted-foreground hover:text-destructive hover:border-destructive transition-colors cursor-pointer"
            title="Remove shortcut"
          >
            <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
              <line x1="18" y1="6" x2="6" y2="18" />
              <line x1="6" y1="6" x2="18" y2="18" />
            </svg>
          </button>
        )}
      </div>
    </div>
  );
};
