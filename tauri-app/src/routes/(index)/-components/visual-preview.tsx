import { ScreenPositions } from "@/context/commands";

type VisualPreviewProps = {
  type: ScreenPositions;
};

export const VisualPreview = ({ type }: VisualPreviewProps) => {
  const activeClasses = "bg-accent";
  const inactiveClasses = "bg-muted/50";

  const renderPreview = () => {
    switch (type) {
      // Halves - Vertical splits
      case "Left":
        return (
          <div className="w-12 h-8 grid grid-cols-2 gap-0.5 border border-border rounded-sm overflow-hidden bg-background p-0.5">
            <div className={activeClasses} />
            <div className={inactiveClasses} />
          </div>
        );

      case "Right":
        return (
          <div className="w-12 h-8 grid grid-cols-2 gap-0.5 border border-border rounded-sm overflow-hidden bg-background p-0.5">
            <div className={inactiveClasses} />
            <div className={activeClasses} />
          </div>
        );

      // Halves - Horizontal splits
      case "Top":
        return (
          <div className="w-12 h-8 grid grid-rows-2 gap-0.5 border border-border rounded-sm overflow-hidden bg-background p-0.5">
            <div className={activeClasses} />
            <div className={inactiveClasses} />
          </div>
        );

      case "Bottom":
        return (
          <div className="w-12 h-8 grid grid-rows-2 gap-0.5 border border-border rounded-sm overflow-hidden bg-background p-0.5">
            <div className={inactiveClasses} />
            <div className={activeClasses} />
          </div>
        );

      // Quarters - 2x2 grid
      case "TopLeft":
        return (
          <div className="w-12 h-8 grid grid-cols-2 grid-rows-2 gap-0.5 border border-border rounded-sm overflow-hidden bg-background p-0.5">
            <div className={activeClasses} />
            <div className={inactiveClasses} />
            <div className={inactiveClasses} />
            <div className={inactiveClasses} />
          </div>
        );

      case "TopRight":
        return (
          <div className="w-12 h-8 grid grid-cols-2 grid-rows-2 gap-0.5 border border-border rounded-sm overflow-hidden bg-background p-0.5">
            <div className={inactiveClasses} />
            <div className={activeClasses} />
            <div className={inactiveClasses} />
            <div className={inactiveClasses} />
          </div>
        );

      case "BottomLeft":
        return (
          <div className="w-12 h-8 grid grid-cols-2 grid-rows-2 gap-0.5 border border-border rounded-sm overflow-hidden bg-background p-0.5">
            <div className={inactiveClasses} />
            <div className={inactiveClasses} />
            <div className={activeClasses} />
            <div className={inactiveClasses} />
          </div>
        );

      case "BottomRight":
        return (
          <div className="w-12 h-8 grid grid-cols-2 grid-rows-2 gap-0.5 border border-border rounded-sm overflow-hidden bg-background p-0.5">
            <div className={inactiveClasses} />
            <div className={inactiveClasses} />
            <div className={inactiveClasses} />
            <div className={activeClasses} />
          </div>
        );

      // Center (quarter-sized, centered on screen)
      case "Center":
        return (
          <div className="w-12 h-8 relative border border-border rounded-sm overflow-hidden bg-background p-0.5">
            <div className={`${inactiveClasses} w-full h-full`} />
            <div
              className={`${activeClasses} absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-3/5 h-3/5 rounded-[1px]`}
            />
          </div>
        );

      // Maximize (full screen)
      case "Maximize":
        return (
          <div className="w-12 h-8 border border-border rounded-sm overflow-hidden bg-background p-0.5">
            <div className={`${activeClasses} w-full h-full`} />
          </div>
        );

      default:
        return null;
    }
  };

  return (
    <div className="transition-transform duration-200 group-hover:scale-105">
      {renderPreview()}
    </div>
  );
};
