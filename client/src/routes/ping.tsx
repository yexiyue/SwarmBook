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
import { Radio, Play, Square, X } from "lucide-react";
import { startPingNode, sendPingCommand, type PingEvent } from "@/commands";

export const Route = createFileRoute("/ping")({
  component: PingPage,
});

interface PeerInfo {
  peerId: string;
  lastRtt?: number;
}

function PingPage() {
  const [peerId, setPeerId] = useState<string | null>(null);
  const [running, setRunning] = useState(false);
  const [dialAddr, setDialAddr] = useState("");
  const [events, setEvents] = useState<PingEvent[]>([]);
  const [peers, setPeers] = useState<Map<string, PeerInfo>>(new Map());

  const handleEvent = (event: PingEvent) => {
    setEvents((prev) => [...prev.slice(-49), event]);

    if (event.type === "connected") {
      setPeers((prev) =>
        new Map(prev).set(event.peerId, { peerId: event.peerId })
      );
    } else if (event.type === "disconnected") {
      setPeers((prev) => {
        const next = new Map(prev);
        next.delete(event.peerId);
        return next;
      });
    } else if (event.type === "ping") {
      setPeers((prev) => {
        const next = new Map(prev);
        const peer = next.get(event.peerId);
        if (peer) next.set(event.peerId, { ...peer, lastRtt: event.rttMs });
        return next;
      });
    }
  };

  const start = async () => {
    try {
      const id = await startPingNode(handleEvent);
      setPeerId(id);
      setRunning(true);
    } catch (e) {
      console.error(e);
    }
  };

  const stop = async () => {
    await sendPingCommand({ command: "stop" });
    setRunning(false);
    setPeerId(null);
    setPeers(new Map());
  };

  const dial = async () => {
    if (!dialAddr) return;
    await sendPingCommand({ command: "dial", addr: dialAddr });
  };

  const disconnect = async (peerId: string) => {
    await sendPingCommand({ command: "disconnect", addr: peerId });
  };

  return (
    <div className="p-6 max-w-3xl space-y-4">
      <h1 className="text-xl font-bold flex items-center gap-2">
        <Radio size={20} />
        Ping 测试
      </h1>

      <Card>
        <CardHeader>
          <CardTitle className="text-base">节点控制</CardTitle>
          <CardDescription>启动 libp2p ping 节点并连接其他节点</CardDescription>
        </CardHeader>
        <CardContent className="space-y-4">
          <div className="flex gap-4 items-center">
            {!running ? (
              <Button onClick={start}>
                <Play size={16} className="mr-2" />
                启动节点
              </Button>
            ) : (
              <Button variant="destructive" onClick={stop}>
                <Square size={16} className="mr-2" />
                停止节点
              </Button>
            )}
            {peerId && (
              <span className="text-sm text-muted-foreground">
                PeerId: {peerId.slice(0, 20)}...
              </span>
            )}
          </div>

          {running && (
            <div className="flex gap-2">
              <Input
                placeholder="/ip4/127.0.0.1/tcp/9696/p2p/12D3..."
                value={dialAddr}
                onChange={(e: React.ChangeEvent<HTMLInputElement>) =>
                  setDialAddr(e.target.value)
                }
                className="flex-1"
              />
              <Button onClick={dial} disabled={!dialAddr}>
                连接
              </Button>
            </div>
          )}
        </CardContent>
      </Card>

      {peers.size > 0 && (
        <Card>
          <CardHeader>
            <CardTitle className="text-base">已连接节点</CardTitle>
          </CardHeader>
          <CardContent className="space-y-2">
            {[...peers.values()].map((peer) => (
              <div
                key={peer.peerId}
                className="flex items-center justify-between p-2 bg-muted rounded-md"
              >
                <div className="font-mono text-xs truncate flex-1">
                  {peer.peerId}
                </div>
                <div className="flex items-center gap-2">
                  {peer.lastRtt !== undefined && (
                    <span className="text-xs text-muted-foreground">
                      {peer.lastRtt}ms
                    </span>
                  )}
                  <Button
                    size="sm"
                    variant="ghost"
                    className="h-7 w-7 p-0 hover:bg-destructive hover:text-destructive-foreground"
                    onClick={() => disconnect(peer.peerId)}
                  >
                    <X size={14} />
                  </Button>
                </div>
              </div>
            ))}
          </CardContent>
        </Card>
      )}

      {events.length > 0 && (
        <Card>
          <CardHeader>
            <CardTitle className="text-base">事件日志</CardTitle>
          </CardHeader>
          <CardContent>
            <div className="h-64 overflow-auto font-mono text-xs space-y-1 bg-muted p-3 rounded-md">
              {events.map((event, i) => (
                <div key={i} className={getEventColor(event.type)}>
                  {formatEvent(event)}
                </div>
              ))}
            </div>
          </CardContent>
        </Card>
      )}
    </div>
  );
}

function getEventColor(type: string) {
  switch (type) {
    case "listening":
      return "text-blue-500";
    case "connected":
      return "text-green-500";
    case "disconnected":
      return "text-yellow-500";
    case "ping":
      return "text-foreground";
    case "error":
      return "text-red-500";
    default:
      return "";
  }
}

function formatEvent(event: PingEvent): string {
  console.log(event);
  switch (event.type) {
    case "listening":
      return `[监听] ${event.addr}`;
    case "connected":
      return `[连接] ${event.peerId}`;
    case "disconnected":
      return `[断开] ${event.peerId}`;
    case "ping":
      return `[Ping] ${event.peerId}... RTT: ${event.rttMs}ms`;
    case "error":
      return `[错误] ${event.message}`;
  }
}
