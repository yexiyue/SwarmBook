import { createFileRoute, Link } from "@tanstack/react-router";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Fingerprint, KeyRound, Link2, MessageCircle, Radio, Wrench } from "lucide-react";

export const Route = createFileRoute("/")({
  component: Index,
});

const tools = [
  {
    to: "/peer-id",
    icon: KeyRound,
    title: "PeerId 生成器",
    description: "生成不同密钥类型的 libp2p PeerId，用于理解节点身份机制",
    chapter: "第三章：节点身份",
  },
  {
    to: "/multiaddr",
    icon: Link2,
    title: "Multiaddr 解析",
    description: "解析多地址格式，理解 libp2p 的地址抽象层",
    chapter: "第二章：核心概念",
  },
  {
    to: "/ping",
    icon: Radio,
    title: "Ping 测试",
    description: "启动 libp2p 节点并测试与其他节点的连通性",
    chapter: "第四章：网络行为",
  },
  {
    to: "/identify",
    icon: Fingerprint,
    title: "Identify 协议",
    description: "连接节点后自动交换身份信息，查看协议版本和监听地址",
    chapter: "第四章：网络行为",
  },
  {
    to: "/chat",
    icon: MessageCircle,
    title: "P2P 聊天",
    description: "基于 GossipSub 的去中心化聊天室，体验消息传播机制",
    chapter: "第五篇：消息传播",
    comingSoon: true,
  },
];

function Index() {
  return (
    <div className="p-6 max-w-4xl mx-auto space-y-6">
      <div className="space-y-2">
        <h1 className="text-2xl font-bold flex items-center gap-2">
          <Wrench size={24} />
          SwarmBook 配套工具
        </h1>
        <p className="text-muted-foreground">
          这是{" "}
          <a
            href="https://github.com/yexiyue/SwarmBook"
            target="_blank"
            rel="noopener noreferrer"
            className="underline hover:text-foreground"
          >
            SwarmBook
          </a>{" "}
          教程的配套工具集，帮助你在学习 libp2p 的过程中进行交互式实验和调试。
        </p>
      </div>

      <div className="grid gap-4 md:grid-cols-2">
        {tools.map((tool) => {
          const Icon = tool.icon;
          return (
            <Link key={tool.to} to={tool.to}>
              <Card className="h-full hover:bg-accent/50 transition-colors cursor-pointer">
                <CardHeader>
                  <CardTitle className="flex items-center gap-2 text-lg">
                    <Icon size={18} />
                    {tool.title}
                    {tool.comingSoon && (
                      <span className="text-xs bg-muted px-2 py-0.5 rounded">
                        即将推出
                      </span>
                    )}
                  </CardTitle>
                  <CardDescription>{tool.description}</CardDescription>
                </CardHeader>
                <CardContent>
                  <span className="text-xs text-muted-foreground">
                    {tool.chapter}
                  </span>
                </CardContent>
              </Card>
            </Link>
          );
        })}
      </div>

      <Card>
        <CardHeader>
          <CardTitle className="text-lg">关于本工具</CardTitle>
        </CardHeader>
        <CardContent className="space-y-2 text-sm text-muted-foreground">
          <p>
            本工具使用 Tauri + React 构建，后端直接调用 rust-libp2p
            库，确保与教程代码行为一致。
          </p>
          <p>
            每个工具都对应教程中的特定章节，你可以在阅读教程的同时使用这些工具进行实践。
          </p>
        </CardContent>
      </Card>
    </div>
  );
}
