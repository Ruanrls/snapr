import type { ReactNode } from "react";

type SectionContainerProps = {
	title: string;
	description: string;
	children: ReactNode;
};

export const SectionContainer = ({
	title,
	description,
	children,
}: SectionContainerProps) => {
	return (
		<section className="space-y-4">
			<div className="space-y-1">
				<h3 className="text-sm font-medium uppercase tracking-wider text-muted-foreground">
					{title}
				</h3>
				<p className="text-xs text-muted-foreground/70">{description}</p>
			</div>

			<div className="rounded-xl bg-card/40">{children}</div>
		</section>
	);
};
