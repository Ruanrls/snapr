import { InputData } from "../shortcut-input";
import { PositionId, ShortcutData } from "./types";
import { POSITION_CONFIGS } from "./constants";

export function serializeShortcut(data: InputData): string {
  const sortedModifiers = [...data.modifiers].sort();
  return `${sortedModifiers.join("+")}${data.actionKey}`;
}

export function findDuplicate(
  newData: InputData,
  currentPositionId: PositionId,
  allShortcuts: ShortcutData
): { isDuplicate: boolean; conflictingPosition?: string } {
  const newSerialized = serializeShortcut(newData);

  for (const [posId, shortcut] of Object.entries(allShortcuts)) {
    if (posId === currentPositionId) continue;
    if (!shortcut || !shortcut.actionKey) continue;

    const existingSerialized = serializeShortcut(shortcut);
    if (newSerialized === existingSerialized) {
      const config = POSITION_CONFIGS.find((c) => c.id === posId);
      return {
        isDuplicate: true,
        conflictingPosition: config?.label || posId,
      };
    }
  }

  return { isDuplicate: false };
}
