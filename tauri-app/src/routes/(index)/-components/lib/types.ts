import { InputData } from "../shortcut-input";

export type PositionId =
  | "half-left"
  | "half-right"
  | "half-top"
  | "half-bottom"
  | "quarter-top-left"
  | "quarter-top-right"
  | "quarter-bottom-left"
  | "quarter-bottom-right"
  | "third-left"
  | "third-center"
  | "third-right";

export type PositionConfig = {
  id: PositionId;
  label: string;
  section: "halves" | "quarters" | "thirds";
};

export type ShortcutData = {
  [key in PositionId]?: InputData;
};
