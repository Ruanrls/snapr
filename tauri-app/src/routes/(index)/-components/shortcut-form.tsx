import { toast } from "sonner";
import { SectionContainer } from "./section-container";
import { PositionRow } from "./position-row";
import type { InputData } from "./shortcut-input";
import { POSITION_CONFIGS, SECTION_META } from "./lib/constants";
import { serializeShortcut } from "./lib/utils";
import { type ScreenPositions, useCommands } from "@/context/commands";

export const ShortcutForm = () => {
	const { commands, positionToCommand, insertCommand, removeCommand } =
		useCommands();

	const createRecordHandler = (positionId: ScreenPositions, label: string) => {
		return (data: InputData) => {
			const existingCommand = positionToCommand.get(positionId);
			const existingPosition = commands.get(serializeShortcut(data));

			if (existingCommand && existingPosition) {
				const isSameCommand =
					existingCommand === positionId &&
					serializeShortcut(data) ===
						serializeShortcut({
							actionKey: existingPosition.key_binding.key,
							modifiers: existingPosition.key_binding.modifiers,
						});

				if (isSameCommand) {
					return;
				}

				toast.error("Duplicate Shortcut", {
					description: `This shortcut is already assigned to ${existingPosition.position}`,
				});

				return;
			}

			insertCommand(serializeShortcut(data), {
				key_binding: {
					key: data.actionKey,
					modifiers: data.modifiers,
				},
				position: positionId,
			});

			toast.success("Shortcut Saved", {
				description: `Assigned to ${label}`,
			});
		};
	};

	const sections = ["halves", "quarters", "maximize"] as const;

	return (
		<form className="space-y-10 w-full">
			{sections.map((section) => {
				const positions = POSITION_CONFIGS.filter(
					(p) => p.section === section,
				);
				const meta = SECTION_META[section];

				return (
					<SectionContainer
						key={section}
						title={meta.title}
						description={meta.description}
					>
						<div className="divide-y divide-border/50">
							{positions.map((pos) => {
								const commandKey = positionToCommand.get(pos.id);
								const inputData = commandKey
									? commands.get(commandKey)
									: undefined;

								return (
									<PositionRow
										key={pos.id}
										id={pos.id}
										label={pos.label}
										inputData={inputData}
										onRecord={createRecordHandler(pos.id, pos.label)}
										onRemove={
											commandKey
												? () => {
														removeCommand(commandKey);
														toast.success("Shortcut Removed", {
															description: `Removed shortcut from ${pos.label}`,
														});
													}
												: undefined
										}
									/>
								);
							})}
						</div>
					</SectionContainer>
				);
			})}
		</form>
	);
};
