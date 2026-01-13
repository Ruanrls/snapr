import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { createFileRoute } from "@tanstack/react-router";
import { Keyboard } from "lucide-react";
import { ShortcutForm } from "./-components/shortcut-form";

export const Route = createFileRoute("/(index)/")({
  component: RouteComponent,
});

function RouteComponent() {
  return (
    <Tabs
      defaultValue="shortcuts"
      className="flex flex-col container mx-auto pt-12 px-4"
    >
      <TabsList className="mx-auto">
        <TabsTrigger value="shortcuts">
          <Keyboard />
          shortcuts
        </TabsTrigger>
      </TabsList>
      <TabsContent value="shortcuts">
        <ShortcutForm />
      </TabsContent>
    </Tabs>
  );
}
