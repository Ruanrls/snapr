import { ScreenPositions } from "@/context/commands";
import { InputData } from "../shortcut-input";

export type PositionConfig = {
  id: ScreenPositions;
  label: string;
  section: "halves" | "quarters" | "thirds" | "maximize";
};

export type ShortcutData = {
  [key in ScreenPositions]?: InputData;
};
