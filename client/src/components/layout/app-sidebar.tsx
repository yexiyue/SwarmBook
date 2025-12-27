import { useState } from "react";
import { Link, useLocation } from "@tanstack/react-router";
import {
  ChevronLeft,
  ChevronRight,
  Fingerprint,
  Home,
  KeyRound,
  Link2,
  MessageCircle,
  Radio,
  type LucideIcon,
} from "lucide-react";
import { cn } from "@/lib/utils";
import {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger,
} from "@/components/ui/tooltip";

interface NavItem {
  to: string;
  icon: LucideIcon;
  label: string;
}

const navItems: NavItem[] = [
  { to: "/", icon: Home, label: "概览" },
  { to: "/peer-id", icon: KeyRound, label: "PeerId 生成器" },
  { to: "/multiaddr", icon: Link2, label: "Multiaddr 解析" },
  { to: "/ping", icon: Radio, label: "Ping 测试" },
  { to: "/identify", icon: Fingerprint, label: "Identify 协议" },
  { to: "/chat", icon: MessageCircle, label: "P2P 聊天" },
];

export function AppSidebar() {
  const [collapsed, setCollapsed] = useState(false);
  const location = useLocation();

  return (
    <TooltipProvider delayDuration={0}>
      <aside
        className={cn(
          "h-screen border-r bg-background flex flex-col transition-all duration-300",
          collapsed ? "w-16" : "w-56"
        )}
      >
        <div className="flex items-center justify-between p-4 border-b">
          {!collapsed && (
            <span className="font-semibold text-sm">SwarmBook Tools</span>
          )}
          <button
            onClick={() => setCollapsed(!collapsed)}
            className={cn(
              "p-1.5 rounded-md hover:bg-accent",
              collapsed && "mx-auto"
            )}
          >
            {collapsed ? <ChevronRight size={18} /> : <ChevronLeft size={18} />}
          </button>
        </div>

        <nav className="flex-1 p-2 space-y-1">
          {navItems.map((item) => {
            const isActive = location.pathname === item.to;
            const Icon = item.icon;

            const linkContent = (
              <Link
                to={item.to}
                className={cn(
                  "flex items-center gap-3 px-3 py-2 rounded-md text-sm transition-colors",
                  isActive
                    ? "bg-accent text-accent-foreground"
                    : "hover:bg-accent/50 text-muted-foreground hover:text-foreground"
                )}
              >
                <Icon size={18} />
                {!collapsed && <span>{item.label}</span>}
              </Link>
            );

            if (collapsed) {
              return (
                <Tooltip key={item.to}>
                  <TooltipTrigger asChild>{linkContent}</TooltipTrigger>
                  <TooltipContent side="right">{item.label}</TooltipContent>
                </Tooltip>
              );
            }

            return <div key={item.to}>{linkContent}</div>;
          })}
        </nav>

        <div className="p-4 border-t text-xs text-muted-foreground">
          {!collapsed && <span>教程配套工具</span>}
        </div>
      </aside>
    </TooltipProvider>
  );
}
