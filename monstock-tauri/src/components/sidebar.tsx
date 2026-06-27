import { Link, useLocation } from "@tanstack/react-router";
import { NAV_ITEMS } from "../lib/constants";

interface SidebarProps {
  open: boolean;
  onClose: () => void;
  onNavigate: () => void;
}

export function Sidebar({ open, onNavigate }: SidebarProps) {
  const location = useLocation();

  const isActive = (path: string) => {
    if (path === "/") return location.pathname === "/";
    return location.pathname.startsWith(path);
  };

  return (
    <aside
      className={`
        fixed lg:static inset-y-0 left-0 z-50
        w-[220px] flex flex-col
        transition-transform duration-200
        ${open ? "translate-x-0" : "-translate-x-full lg:translate-x-0"}
      `}
      style={{
        background: "var(--color-surface)",
        borderRight: "1px solid var(--color-border)",
      }}
    >
      {/* Logo */}
      <div
        className="px-6 py-7 pb-5"
        style={{ borderBottom: "1px solid var(--color-border)" }}
      >
        <div className="flex items-center gap-2.5">
          <div
            className="w-7 h-7 flex items-center justify-center rounded-sm"
            style={{
              border: "1.5px solid var(--color-border-strong)",
            }}
          >
            <div
              className="w-2.5 h-2.5 rounded-[2px]"
              style={{ background: "var(--color-text)" }}
            />
          </div>
          <div>
            <div
              className="text-[15px] font-semibold tracking-tight"
              style={{ color: "var(--color-text)" }}
            >
              MonStock
            </div>
            <div
              className="text-[11px] mt-0.5 font-normal"
              style={{ color: "var(--color-text-dim)" }}
            >
              Inventory & Sales
            </div>
          </div>
        </div>
      </div>

      {/* Navigation */}
      <nav className="flex-1 overflow-y-auto px-2.5 py-3">
        {NAV_ITEMS.map((item) => {
          const active = isActive(item.path);
          return (
            <Link
              key={item.path}
              to={item.path}
              onClick={onNavigate}
              className={`
                flex items-center gap-2.5 px-3 py-[7px] rounded-sm text-[12.5px] font-[450]
                transition-all duration-150 mb-px relative
                ${active
                  ? "text-text bg-raised"
                  : "text-text-sec hover:text-text hover:bg-raised"
                }
              `}
              style={{
                border: active ? "1px solid var(--color-border-strong)" : "1px solid transparent",
              }}
            >
              {active && (
                <div
                  className="absolute left-0 top-1/2 -translate-y-1/2 w-[3px] h-4 rounded-r-sm"
                  style={{ background: "var(--color-text)" }}
                />
              )}
              <span
                className="w-[18px] text-center text-[11px] font-mono transition-colors"
                style={{ color: active ? "var(--color-text-sec)" : "var(--color-text-dim)" }}
              >
                {item.num}
              </span>
              <span>{item.label}</span>
            </Link>
          );
        })}
      </nav>

      {/* Footer */}
      <div
        className="px-[18px] py-3.5 flex items-center gap-2 text-[11px]"
        style={{
          color: "var(--color-text-dim)",
          borderTop: "1px solid var(--color-border)",
        }}
      >
        <div
          className="w-[5px] h-[5px] rounded-full"
          style={{ background: "var(--color-good)" }}
        />
        <span>Offline mode</span>
      </div>
    </aside>
  );
}