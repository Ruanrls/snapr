import { Separator } from "@/components/ui/separator";
import { ReactNode } from "react";

type SectionContainerProps = {
  title: string;
  description: string;
  children: ReactNode;
  isLast?: boolean;
};

export const SectionContainer = ({
  title,
  description,
  children,
  isLast = false,
}: SectionContainerProps) => {
  return (
    <section className="space-y-2">
      <div className="space-y-1">
        <h3 className="text-xl font-extrabold text-foreground">{title}</h3>
        <p className="text-sm text-muted-foreground">{description}</p>
      </div>

      {children}

      {!isLast && <Separator className="mt-6" />}
    </section>
  );
};
