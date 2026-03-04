import { X } from "lucide-react";
import type { Command, ScreenPositions } from "@/context/commands";
import { type InputData, ShortcutInput } from "./shortcut-input";
import { VisualPreview } from "./visual-preview";

type PositionRowProps = {
	id: ScreenPositions;
	label: string;
	inputData?: Command;
	onRecord: (data: InputData) => void;
	onRemove?: () => void;
};

export const PositionRow = ({
	id,
	label,
	inputData,
	onRecord,
	onRemove,
}: PositionRowProps) => {
	const hasShortcut = inputData !== undefined;

	return (
		<div className="group flex items-center gap-4 px-4 py-3 hover:bg-card/50 transition-colors rounded-lg">
			<div className="flex items-center gap-3">
				<VisualPreview type={id} />
				<span className="text-sm text-foreground">{label}</span>
			</div>

			<div className="ml-auto flex items-center gap-1.5">
				<ShortcutInput
					key={
						inputData
							? `${inputData.key_binding.modifiers}-${inputData.key_binding.key}`
							: "empty"
					}
					modifiers={inputData?.key_binding.modifiers ?? 0}
					actionKey={inputData?.key_binding.key ?? 0}
					onRecord={onRecord}
				/>
				{hasShortcut && (
					<button
						type="button"
						onClick={onRemove}
						className="flex items-center justify-center w-6 h-6 rounded-md text-muted-foreground/50 hover:text-destructive hover:bg-destructive/10 transition-all duration-200 cursor-pointer opacity-0 group-hover:opacity-100"
						title="Remove shortcut"
					>
						<X className="w-3.5 h-3.5" />
					</button>
				)}
			</div>
		</div>
	);
};
