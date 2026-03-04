import { createFileRoute } from "@tanstack/react-router";
import { Keyboard } from "lucide-react";
import { ShortcutForm } from "./-components/shortcut-form";

export const Route = createFileRoute("/(index)/")({
	component: RouteComponent,
});

function RouteComponent() {
	return (
		<div className="flex flex-col container mx-auto max-w-5xl px-6 py-10">
			<header className="mb-10">
				<div className="flex items-center gap-3 mb-2">
					<div className="flex items-center justify-center w-9 h-9 rounded-lg bg-accent-blue-muted/30">
						<Keyboard className="w-5 h-5 text-accent-blue" />
					</div>
					<h1 className="text-2xl font-semibold tracking-tight text-foreground">
						Snapr
					</h1>
				</div>
				<p className="text-sm text-muted-foreground ml-12">
					Configure keyboard shortcuts for window snapping
				</p>
			</header>

			<ShortcutForm />
		</div>
	);
}
