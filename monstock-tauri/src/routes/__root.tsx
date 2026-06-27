import { createRootRoute, Outlet } from "@tanstack/react-router";
import { Sidebar } from "../components/sidebar";
import { TopBar } from "../components/topbar";
import { useState } from "react";

export const Route = createRootRoute({
  component: RootLayout,
});

function RootLayout() {
  const [sidebarOpen, setSidebarOpen] = useState(false);
  const [pageTitle, setPageTitle] = useState("Dashboard");

  return (
    <div className="flex h-screen" style={{ background: "var(--color-bg)" }}>
      {/* Mobile overlay */}
      {sidebarOpen && (
        <div
          className="fixed inset-0 bg-black/40 z-40 lg:hidden"
          onClick={() => setSidebarOpen(false)}
        />
      )}

      {/* Sidebar */}
      <Sidebar
        open={sidebarOpen}
        onClose={() => setSidebarOpen(false)}
        onNavigate={() => setSidebarOpen(false)}
      />

      {/* Main */}
      <div className="flex-1 flex flex-col min-w-0">
        <TopBar
          title={pageTitle}
          onMenuClick={() => setSidebarOpen(true)}
        />
        <main className="flex-1 overflow-auto p-6 lg:p-7 max-w-7xl">
          <Outlet />
        </main>
      </div>
    </div>
  );
}