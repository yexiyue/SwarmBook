import { createRootRoute, Outlet } from "@tanstack/react-router";
import { TanStackRouterDevtools } from "@tanstack/react-router-devtools";
import { AppSidebar } from "@/components/layout";

const RootLayout = () => (
  <div className="flex h-screen">
    <AppSidebar />
    <main className="flex-1 overflow-auto">
      <Outlet />
    </main>
    {/* <TanStackRouterDevtools /> */}
  </div>
);

export const Route = createRootRoute({ component: RootLayout });
