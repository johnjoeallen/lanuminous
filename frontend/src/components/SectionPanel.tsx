import { PropsWithChildren } from "react";

interface SectionPanelProps extends PropsWithChildren {
  title: string;
  subtitle?: string;
}

export function SectionPanel({ title, subtitle, children }: SectionPanelProps) {
  return (
    <section className="section-panel">
      <div className="section-heading">
        <h3>{title}</h3>
        {subtitle ? <p>{subtitle}</p> : null}
      </div>
      {children}
    </section>
  );
}

