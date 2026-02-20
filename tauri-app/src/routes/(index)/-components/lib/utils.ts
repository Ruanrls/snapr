import { InputData } from "../shortcut-input";

export function serializeShortcut(data: InputData): string {
  return `${data.modifiers}+${data.actionKey}`;
}
