import { InputData, ShortcutInput } from "./shortcut-input";
import { VisualPreview } from "./visual-preview";
import { PositionId } from "./lib/types";

type PositionCardProps = {
  id: PositionId;
  label: string;
  inputData: InputData;
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
          modifiers={inputData.modifiers}
          actionKey={inputData.actionKey}
          onRecord={onRecord}
        />
      </div>
    </div>
  );
};
