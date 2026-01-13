import { PositionConfig } from "./types";

export const POSITION_CONFIGS: PositionConfig[] = [
  // Halves
  { id: "half-left", label: "Left Half", section: "halves" },
  { id: "half-right", label: "Right Half", section: "halves" },
  { id: "half-top", label: "Top Half", section: "halves" },
  { id: "half-bottom", label: "Bottom Half", section: "halves" },

  // Quarters
  {
    id: "quarter-top-left",
    label: "Top-Left",
    section: "quarters",
  },
  {
    id: "quarter-top-right",
    label: "Top-Right",
    section: "quarters",
  },
  {
    id: "quarter-bottom-left",
    label: "Bottom-Left",
    section: "quarters",
  },
  {
    id: "quarter-bottom-right",
    label: "Bottom-Right",
    section: "quarters",
  },

  // Thirds
  { id: "third-left", label: "Left Third", section: "thirds" },
  { id: "third-center", label: "Center Third", section: "thirds" },
  { id: "third-right", label: "Right Third", section: "thirds" },
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
} as const;
