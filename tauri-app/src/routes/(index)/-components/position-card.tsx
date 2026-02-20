import { Command, ScreenPositions } from "@/context/commands";
import { InputData, ShortcutInput } from "./shortcut-input";
import { VisualPreview } from "./visual-preview";

type PositionCardProps = {
  id: ScreenPositions;
  label: string;
  inputData?: Command;
  onRecord: (data: InputData) => void;
};

export const PositionCard = ({
  id,
  label,
  inputData,
  onRecord,
}: PositionCardProps) => {
  return (
    <div className="group flex flex-col gap-3 p-4 border border-border rounded-lg bg-card hover:border-accent transition-all duration-200 hover:shadow-md">
      <label className="text-sm font-medium text-foreground text-center">
        {label}
      </label>

      <div className="flex justify-center">
        <VisualPreview type={id} />
      </div>

      <div className="flex justify-center">
        <ShortcutInput
          modifiers={inputData?.key_binding.modifiers ?? 0}
          actionKey={inputData?.key_binding.key ?? 0}
          onRecord={onRecord}
        />
      </div>
    </div>
  );
};
