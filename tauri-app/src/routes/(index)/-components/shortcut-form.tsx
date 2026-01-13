import { useState } from "react";
import { toast } from "sonner";
import { SectionContainer } from "./section-container";
import { PositionCard } from "./position-card";
import { InputData } from "./shortcut-input";
import { ShortcutData, PositionId } from "./lib/types";
import { POSITION_CONFIGS, SECTION_META } from "./lib/constants";
import { findDuplicate } from "./lib/utils";

export const ShortcutForm = () => {
  const [shortcuts, setShortcuts] = useState<ShortcutData>({});

  const createRecordHandler = (positionId: PositionId, label: string) => {
    return (data: InputData) => {
      const { isDuplicate, conflictingPosition } = findDuplicate(
        data,
        positionId,
        shortcuts
      );

      if (isDuplicate) {
        toast.error("Duplicate Shortcut", {
          description: `This shortcut is already assigned to ${conflictingPosition}`,
        });
        return;
      }

      setShortcuts((prev) => ({ ...prev, [positionId]: data }));
      toast.success("Shortcut Saved", {
        description: `Assigned to ${label}`,
      });
    };
  };

  const getGridClasses = (section: "halves" | "quarters" | "thirds") => {
    switch (section) {
      case "halves":
        return "grid grid-cols-2 md:grid-cols-4 gap-4 mt-4";
      case "quarters":
        return "grid grid-cols-2 gap-4 mt-4";
      case "thirds":
        return "grid grid-cols-1 md:grid-cols-3 gap-4 mt-4";
    }
  };

  const sections = ["halves", "quarters", "thirds"] as const;

  return (
    <form className="space-y-8 w-full max-w-6xl mx-auto py-6">
      {sections.map((section, index) => {
        const positions = POSITION_CONFIGS.filter((p) => p.section === section);
        const meta = SECTION_META[section];

        return (
          <SectionContainer
            key={section}
            title={meta.title}
            description={meta.description}
            isLast={index === sections.length - 1}
          >
            <div className={getGridClasses(section)}>
              {positions.map((pos) => (
                <PositionCard
                  key={pos.id}
                  id={pos.id}
                  label={pos.label}
                  inputData={
                    shortcuts[pos.id] || { modifiers: [], actionKey: "" }
                  }
                  onRecord={createRecordHandler(pos.id, pos.label)}
                />
              ))}
            </div>
          </SectionContainer>
        );
      })}
    </form>
  );
};
