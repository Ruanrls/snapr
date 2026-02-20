import { PositionConfig } from "./types";

export const POSITION_CONFIGS: PositionConfig[] = [
  // Halves
  { id: "Left", label: "Left Half", section: "halves" },
  { id: "Right", label: "Right Half", section: "halves" },
  { id: "Top", label: "Top Half", section: "halves" },
  { id: "Bottom", label: "Bottom Half", section: "halves" },

  // Quarters
  { id: "TopLeft", label: "Top-Left", section: "quarters" },
  { id: "TopRight", label: "Top-Right", section: "quarters" },
  { id: "BottomLeft", label: "Bottom-Left", section: "quarters" },
  { id: "BottomRight", label: "Bottom-Right", section: "quarters" },
  { id: "Center", label: "Center", section: "quarters" },

  // Maximize
  { id: "Maximize", label: "Maximize", section: "maximize" },
];

export const SECTION_META = {
  halves: {
    title: "Halves",
    description:
      "Split the screen into halves - snap windows to the left, right, top, and bottom sides of your monitor",
  },
  quarters: {
    title: "Quarters",
    description:
      "Snap windows to the four corners of your screen for efficient multitasking",
  },
  thirds: {
    title: "Thirds (Horizontal)",
    description:
      "Divide the screen into three equal vertical columns for organized workspace layouts",
  },
  maximize: {
    title: "Maximize",
    description:
      "Maximize the active window to fill the entire screen",
  },
} as const;
