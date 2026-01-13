import { PositionId } from "./lib/types";

type VisualPreviewProps = {
  type: PositionId;
};

export const VisualPreview = ({ type }: VisualPreviewProps) => {
  const activeClasses = "bg-accent";
  const inactiveClasses = "bg-muted/50";

  const renderPreview = () => {
    switch (type) {
      // Halves - Vertical splits
      case "half-left":
        return (
          <div className="w-12 h-8 grid grid-cols-2 gap-0.5 border border-border rounded-sm overflow-hidden bg-background p-0.5">
            <div className={activeClasses} />
            <div className={inactiveClasses} />
          </div>
        );

      case "half-right":
        return (
          <div className="w-12 h-8 grid grid-cols-2 gap-0.5 border border-border rounded-sm overflow-hidden bg-background p-0.5">
            <div className={inactiveClasses} />
            <div className={activeClasses} />
          </div>
        );

      // Halves - Horizontal splits
      case "half-top":
        return (
          <div className="w-12 h-8 grid grid-rows-2 gap-0.5 border border-border rounded-sm overflow-hidden bg-background p-0.5">
            <div className={activeClasses} />
            <div className={inactiveClasses} />
          </div>
        );

      case "half-bottom":
        return (
          <div className="w-12 h-8 grid grid-rows-2 gap-0.5 border border-border rounded-sm overflow-hidden bg-background p-0.5">
            <div className={inactiveClasses} />
            <div className={activeClasses} />
          </div>
        );

      // Quarters - 2x2 grid
      case "quarter-top-left":
        return (
          <div className="w-12 h-8 grid grid-cols-2 grid-rows-2 gap-0.5 border border-border rounded-sm overflow-hidden bg-background p-0.5">
            <div className={activeClasses} />
            <div className={inactiveClasses} />
            <div className={inactiveClasses} />
            <div className={inactiveClasses} />
          </div>
        );

      case "quarter-top-right":
        return (
          <div className="w-12 h-8 grid grid-cols-2 grid-rows-2 gap-0.5 border border-border rounded-sm overflow-hidden bg-background p-0.5">
            <div className={inactiveClasses} />
            <div className={activeClasses} />
            <div className={inactiveClasses} />
            <div className={inactiveClasses} />
          </div>
        );

      case "quarter-bottom-left":
        return (
          <div className="w-12 h-8 grid grid-cols-2 grid-rows-2 gap-0.5 border border-border rounded-sm overflow-hidden bg-background p-0.5">
            <div className={inactiveClasses} />
            <div className={inactiveClasses} />
            <div className={activeClasses} />
            <div className={inactiveClasses} />
          </div>
        );

      case "quarter-bottom-right":
        return (
          <div className="w-12 h-8 grid grid-cols-2 grid-rows-2 gap-0.5 border border-border rounded-sm overflow-hidden bg-background p-0.5">
            <div className={inactiveClasses} />
            <div className={inactiveClasses} />
            <div className={inactiveClasses} />
            <div className={activeClasses} />
          </div>
        );

      // Thirds - Horizontal columns
      case "third-left":
        return (
          <div className="w-12 h-8 grid grid-cols-3 gap-0.5 border border-border rounded-sm overflow-hidden bg-background p-0.5">
            <div className={activeClasses} />
            <div className={inactiveClasses} />
            <div className={inactiveClasses} />
          </div>
        );

      case "third-center":
        return (
          <div className="w-12 h-8 grid grid-cols-3 gap-0.5 border border-border rounded-sm overflow-hidden bg-background p-0.5">
            <div className={inactiveClasses} />
            <div className={activeClasses} />
            <div className={inactiveClasses} />
          </div>
        );

      case "third-right":
        return (
          <div className="w-12 h-8 grid grid-cols-3 gap-0.5 border border-border rounded-sm overflow-hidden bg-background p-0.5">
            <div className={inactiveClasses} />
            <div className={inactiveClasses} />
            <div className={activeClasses} />
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
