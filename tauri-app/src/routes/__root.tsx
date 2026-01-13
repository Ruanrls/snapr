import * as React from "react";
import { Outlet, createRootRoute } from "@tanstack/react-router";
import { TanStackRouterDevtools } from "@tanstack/react-router-devtools";
import { Toaster } from "@/components/ui/sonner";

export const Route = createRootRoute({
  component: RootComponent,
});

function RootComponent() {
  return (
    <React.Fragment>
      <main className="dark w-full h-screen bg-background text-foreground overflow-x-hidden">
        <Outlet />
        <TanStackRouterDevtools />
        <Toaster position="bottom-right" />
      </main>
    </React.Fragment>
  );
}
