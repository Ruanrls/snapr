import type { ScreenPositions } from "@/context/commands";

type VisualPreviewProps = {
	type: ScreenPositions;
};

export const VisualPreview = ({ type }: VisualPreviewProps) => {
	const activeClasses = "bg-accent-blue rounded-sm";
	const inactiveClasses = "bg-muted/30 rounded-sm";

	const containerClasses =
		"w-14 h-10 rounded-lg overflow-hidden bg-background/50 p-1 ring-1 ring-border/50";

	const renderPreview = () => {
		switch (type) {
			case "Left":
				return (
					<div className={`${containerClasses} grid grid-cols-2 gap-1`}>
						<div className={activeClasses} />
						<div className={inactiveClasses} />
					</div>
				);

			case "Right":
				return (
					<div className={`${containerClasses} grid grid-cols-2 gap-1`}>
						<div className={inactiveClasses} />
						<div className={activeClasses} />
					</div>
				);

			case "Top":
				return (
					<div className={`${containerClasses} grid grid-rows-2 gap-1`}>
						<div className={activeClasses} />
						<div className={inactiveClasses} />
					</div>
				);

			case "Bottom":
				return (
					<div className={`${containerClasses} grid grid-rows-2 gap-1`}>
						<div className={inactiveClasses} />
						<div className={activeClasses} />
					</div>
				);

			case "TopLeft":
				return (
					<div
						className={`${containerClasses} grid grid-cols-2 grid-rows-2 gap-1`}
					>
						<div className={activeClasses} />
						<div className={inactiveClasses} />
						<div className={inactiveClasses} />
						<div className={inactiveClasses} />
					</div>
				);

			case "TopRight":
				return (
					<div
						className={`${containerClasses} grid grid-cols-2 grid-rows-2 gap-1`}
					>
						<div className={inactiveClasses} />
						<div className={activeClasses} />
						<div className={inactiveClasses} />
						<div className={inactiveClasses} />
					</div>
				);

			case "BottomLeft":
				return (
					<div
						className={`${containerClasses} grid grid-cols-2 grid-rows-2 gap-1`}
					>
						<div className={inactiveClasses} />
						<div className={inactiveClasses} />
						<div className={activeClasses} />
						<div className={inactiveClasses} />
					</div>
				);

			case "BottomRight":
				return (
					<div
						className={`${containerClasses} grid grid-cols-2 grid-rows-2 gap-1`}
					>
						<div className={inactiveClasses} />
						<div className={inactiveClasses} />
						<div className={inactiveClasses} />
						<div className={activeClasses} />
					</div>
				);

			case "Center":
				return (
					<div className={`${containerClasses} relative`}>
						<div className={`${inactiveClasses} w-full h-full`} />
						<div
							className={`${activeClasses} absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-3/5 h-3/5`}
						/>
					</div>
				);

			case "Maximize":
				return (
					<div className={containerClasses}>
						<div className={`${activeClasses} w-full h-full`} />
					</div>
				);

			default:
				return null;
		}
	};

	return renderPreview();
};
