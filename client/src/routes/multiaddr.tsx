import { useState } from "react";
import { createFileRoute } from "@tanstack/react-router";
import { Button } from "@/components/ui/button";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { Link2, AlertCircle, Check } from "lucide-react";
import {
  parseMultiaddr,
  type ParsedMultiaddr,
  type ProtocolLayer,
} from "@/commands";

export const Route = createFileRoute("/multiaddr")({
  component: MultiaddrPage,
});

const LAYER_CONFIG: Record<
  ProtocolLayer,
  { label: string; color: string; bgColor: string }
> = {
  network: {
    label: "网络层",
    color: "text-blue-600 dark:text-blue-400",
    bgColor: "bg-blue-50 dark:bg-blue-950 border-blue-200 dark:border-blue-800",
  },
  transport: {
    label: "传输层",
    color: "text-green-600 dark:text-green-400",
    bgColor:
      "bg-green-50 dark:bg-green-950 border-green-200 dark:border-green-800",
  },
  security: {
    label: "安全层",
    color: "text-yellow-600 dark:text-yellow-400",
    bgColor:
      "bg-yellow-50 dark:bg-yellow-950 border-yellow-200 dark:border-yellow-800",
  },
  muxer: {
    label: "多路复用",
    color: "text-purple-600 dark:text-purple-400",
    bgColor:
      "bg-purple-50 dark:bg-purple-950 border-purple-200 dark:border-purple-800",
  },
  application: {
    label: "应用层",
    color: "text-orange-600 dark:text-orange-400",
    bgColor:
      "bg-orange-50 dark:bg-orange-950 border-orange-200 dark:border-orange-800",
  },
  identity: {
    label: "身份层",
    color: "text-pink-600 dark:text-pink-400",
    bgColor:
      "bg-pink-50 dark:bg-pink-950 border-pink-200 dark:border-pink-800",
  },
  relay: {
    label: "中继层",
    color: "text-cyan-600 dark:text-cyan-400",
    bgColor:
      "bg-cyan-50 dark:bg-cyan-950 border-cyan-200 dark:border-cyan-800",
  },
};

const EXAMPLE_ADDRS = [
  "/ip4/127.0.0.1/tcp/4001",
  "/ip4/192.168.1.100/tcp/4001/p2p/12D3KooWMgcJeCtWqiptgnn4HHY6jRkMaRzHumXrntP3Np7qUqMT",
  "/ip6/::1/udp/4001/quic-v1",
  "/dns4/example.com/tcp/443/wss/p2p/12D3KooWMgcJeCtWqiptgnn4HHY6jRkMaRzHumXrntP3Np7qUqMT",
  "/ip4/192.0.2.0/tcp/5002/p2p/QmdPU7Pf7TzNGECNqx5spEPhb7N2K5N2V9RLtpq4EXqPZP/p2p-circuit/p2p/QmVT6GYwE4Qn5PEgNBBZ9rJ3Mzqyb5HhMNqr5YP3KY8V6v",
];

function MultiaddrPage() {
  const [input, setInput] = useState("");
  const [result, setResult] = useState<ParsedMultiaddr | null>(null);
  const [loading, setLoading] = useState(false);

  const handleParse = async () => {
    if (!input.trim()) return;
    setLoading(true);
    try {
      const parsed = await parseMultiaddr(input.trim());
      setResult(parsed);
    } catch (e) {
      console.error(e);
    } finally {
      setLoading(false);
    }
  };

  const handleExample = (addr: string) => {
    setInput(addr);
  };

  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === "Enter") {
      handleParse();
    }
  };

  // 按层级分组
  const groupedByLayer = result?.components.reduce(
    (acc, comp) => {
      if (!acc[comp.layer]) acc[comp.layer] = [];
      acc[comp.layer].push(comp);
      return acc;
    },
    {} as Record<ProtocolLayer, typeof result.components>
  );

  // 定义层级顺序
  const layerOrder: ProtocolLayer[] = [
    "network",
    "transport",
    "security",
    "muxer",
    "application",
    "relay",
    "identity",
  ];

  return (
    <div className="p-6 max-w-3xl space-y-4">
      <h1 className="text-xl font-bold flex items-center gap-2">
        <Link2 size={20} />
        Multiaddr 解析器
      </h1>

      <Card>
        <CardHeader>
          <CardTitle className="text-base">输入地址</CardTitle>
          <CardDescription>
            输入 Multiaddr 字符串，查看协议栈分层结构
          </CardDescription>
        </CardHeader>
        <CardContent className="space-y-4">
          <div className="flex gap-2">
            <Input
              placeholder="/ip4/127.0.0.1/tcp/4001/p2p/..."
              value={input}
              onChange={(e: React.ChangeEvent<HTMLInputElement>) =>
                setInput(e.target.value)
              }
              onKeyDown={handleKeyDown}
              className="flex-1 font-mono"
            />
            <Button onClick={handleParse} disabled={!input.trim() || loading}>
              解析
            </Button>
          </div>

          <div className="space-y-2">
            <p className="text-sm text-muted-foreground">示例地址：</p>
            <div className="flex flex-wrap gap-2">
              {EXAMPLE_ADDRS.map((addr, i) => (
                <button
                  key={i}
                  onClick={() => handleExample(addr)}
                  className="text-xs font-mono px-2 py-1 bg-muted rounded hover:bg-accent transition-colors truncate max-w-full"
                  title={addr}
                >
                  {addr.length > 40 ? addr.slice(0, 40) + "..." : addr}
                </button>
              ))}
            </div>
          </div>
        </CardContent>
      </Card>

      {result && (
        <Card>
          <CardHeader>
            <CardTitle className="text-base flex items-center gap-2">
              解析结果
              {result.valid ? (
                <Check size={16} className="text-green-500" />
              ) : (
                <AlertCircle size={16} className="text-red-500" />
              )}
            </CardTitle>
          </CardHeader>
          <CardContent className="space-y-4">
            {!result.valid ? (
              <div className="p-3 bg-red-50 dark:bg-red-950 border border-red-200 dark:border-red-800 rounded-md">
                <p className="text-sm text-red-600 dark:text-red-400">
                  解析失败: {result.error}
                </p>
              </div>
            ) : (
              <>
                {/* 原始地址 */}
                <div className="p-3 bg-muted rounded-md">
                  <p className="text-xs text-muted-foreground mb-1">
                    原始地址
                  </p>
                  <p className="font-mono text-sm break-all">{result.input}</p>
                </div>

                {/* 协议栈可视化 */}
                <div className="space-y-3">
                  <p className="text-sm font-medium">协议栈分层</p>
                  <div className="space-y-2">
                    {layerOrder.map((layer) => {
                      const components = groupedByLayer?.[layer];
                      if (!components || components.length === 0) return null;

                      const config = LAYER_CONFIG[layer];

                      return (
                        <div
                          key={layer}
                          className={`p-3 border rounded-md ${config.bgColor}`}
                        >
                          <div className="flex items-center gap-2 mb-2">
                            <span
                              className={`text-xs font-medium ${config.color}`}
                            >
                              {config.label}
                            </span>
                          </div>
                          <div className="space-y-1">
                            {components.map((comp, i) => (
                              <div
                                key={i}
                                className="flex items-center gap-2 font-mono text-sm"
                              >
                                <span className="font-semibold">
                                  {comp.name}
                                </span>
                                {comp.value && (
                                  <>
                                    <span className="text-muted-foreground">
                                      =
                                    </span>
                                    <span className="text-foreground break-all">
                                      {comp.value.length > 50
                                        ? comp.value.slice(0, 50) + "..."
                                        : comp.value}
                                    </span>
                                  </>
                                )}
                              </div>
                            ))}
                          </div>
                        </div>
                      );
                    })}
                  </div>
                </div>

                {/* 组件列表 */}
                <div className="space-y-2">
                  <p className="text-sm font-medium">组件序列</p>
                  <div className="flex flex-wrap gap-1">
                    {result.components.map((comp, i) => {
                      const config = LAYER_CONFIG[comp.layer];
                      return (
                        <span
                          key={i}
                          className={`px-2 py-1 text-xs font-mono rounded border ${config.bgColor} ${config.color}`}
                          title={comp.raw}
                        >
                          {comp.name}
                          {comp.value && (
                            <span className="opacity-70">
                              :{" "}
                              {comp.value.length > 15
                                ? comp.value.slice(0, 15) + "..."
                                : comp.value}
                            </span>
                          )}
                        </span>
                      );
                    })}
                  </div>
                </div>
              </>
            )}
          </CardContent>
        </Card>
      )}
    </div>
  );
}
