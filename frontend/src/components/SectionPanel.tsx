import { PropsWithChildren, ReactNode } from "react";

interface SectionPanelProps extends PropsWithChildren {
  title: string;
  subtitle?: string;
  headerAction?: ReactNode;
}

export function SectionPanel({ title, subtitle, headerAction, children }: SectionPanelProps) {
  return (
    <section className="section-panel">
      <div className="section-header">
        <div className="section-heading">
          <h3>{title}</h3>
          {subtitle ? <p>{subtitle}</p> : null}
        </div>
        {headerAction ? <div className="section-action">{headerAction}</div> : null}
      </div>
      {children}
    </section>
  );
}
