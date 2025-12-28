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
import { MessageSquare, Play, Square, Send, X } from "lucide-react";
import {
  startReqRespNode,
  sendReqRespCommand,
  type ReqRespEvent,
} from "@/commands";

export const Route = createFileRoute("/reqresp")({
  component: ReqRespPage,
});

interface PeerInfo {
  peerId: string;
  agentVersion?: string;
  lastRtt?: number;
}

interface PendingRequest {
  requestId: number;
  peerId: string;
  name: string;
  age: number;
}

function ReqRespPage() {
  const [peerId, setPeerId] = useState<string | null>(null);
  const [running, setRunning] = useState(false);
  const [dialAddr, setDialAddr] = useState("");
  const [events, setEvents] = useState<ReqRespEvent[]>([]);
  const [peers, setPeers] = useState<Map<string, PeerInfo>>(new Map());
  const [pendingRequests, setPendingRequests] = useState<PendingRequest[]>([]);
  const [selectedPeer, setSelectedPeer] = useState<string | null>(null);
  const [reqName, setReqName] = useState("");
  const [reqAge, setReqAge] = useState("");
  const [responseText, setResponseText] = useState<Record<number, string>>({});

  const handleEvent = (event: ReqRespEvent) => {
    setEvents((prev) => [...prev.slice(-49), event]);

    if (event.type === "connected") {
      setPeers((prev) => new Map(prev).set(event.peerId, { peerId: event.peerId }));
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
    } else if (event.type === "identified") {
      setPeers((prev) => {
        const next = new Map(prev);
        const peer = next.get(event.peerId) || { peerId: event.peerId };
        next.set(event.peerId, { ...peer, agentVersion: event.agentVersion });
        return next;
      });
    } else if (event.type === "inboundRequest") {
      setPendingRequests((prev) => [...prev, {
        requestId: event.requestId,
        peerId: event.peerId,
        name: event.name,
        age: event.age,
      }]);
    }
  };

  const start = async () => {
    try {
      const id = await startReqRespNode(handleEvent);
      setPeerId(id);
      setRunning(true);
    } catch (e) {
      console.error(e);
    }
  };

  const stop = async () => {
    await sendReqRespCommand({ command: "stop" });
    setRunning(false);
    setPeerId(null);
    setPeers(new Map());
    setPendingRequests([]);
  };

  const dial = async () => {
    if (!dialAddr) return;
    await sendReqRespCommand({ command: "dial", addr: dialAddr });
  };

  const disconnect = async (peerId: string) => {
    await sendReqRespCommand({ command: "disconnect", peerId });
  };

  const sendRequest = async () => {
    if (!selectedPeer || !reqName || !reqAge) return;
    await sendReqRespCommand({
      command: "sendRequest",
      peerId: selectedPeer,
      name: reqName,
      age: parseInt(reqAge) || 0,
    });
    setReqName("");
    setReqAge("");
  };

  const sendResponse = async (requestId: number) => {
    const message = responseText[requestId] || "";
    await sendReqRespCommand({ command: "sendResponse", requestId, message });
    setPendingRequests((prev) => prev.filter((r) => r.requestId !== requestId));
    setResponseText((prev) => {
      const next = { ...prev };
      delete next[requestId];
      return next;
    });
  };

  return (
    <div className="p-6 max-w-3xl space-y-4">
      <h1 className="text-xl font-bold flex items-center gap-2">
        <MessageSquare size={20} />
        Request-Response 协议
      </h1>

      <Card>
        <CardHeader>
          <CardTitle className="text-base">节点控制</CardTitle>
          <CardDescription>启动节点，发送请求并响应其他节点的请求</CardDescription>
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
                onChange={(e) => setDialAddr(e.target.value)}
                className="flex-1"
              />
              <Button onClick={dial} disabled={!dialAddr}>连接</Button>
            </div>
          )}
        </CardContent>
      </Card>

      {pendingRequests.length > 0 && (
        <Card className="border-orange-500">
          <CardHeader>
            <CardTitle className="text-base text-orange-500">📨 待响应请求</CardTitle>
          </CardHeader>
          <CardContent className="space-y-3">
            {pendingRequests.map((req) => (
              <div key={req.requestId} className="p-3 bg-muted rounded-md space-y-2">
                <div className="text-sm">
                  <span className="text-muted-foreground">来自: </span>
                  <span className="font-mono">{req.peerId.slice(0, 16)}...</span>
                </div>
                <div className="text-sm">
                  <span className="text-muted-foreground">内容: </span>
                  name={req.name}, age={req.age}
                </div>
                <div className="flex gap-2">
                  <Input
                    placeholder="输入响应消息"
                    value={responseText[req.requestId] || ""}
                    onChange={(e) => setResponseText((prev) => ({ ...prev, [req.requestId]: e.target.value }))}
                    className="flex-1"
                  />
                  <Button size="sm" onClick={() => sendResponse(req.requestId)}>
                    <Send size={14} className="mr-1" />
                    响应
                  </Button>
                </div>
              </div>
            ))}
          </CardContent>
        </Card>
      )}

      {peers.size > 0 && (
        <Card>
          <CardHeader>
            <CardTitle className="text-base">已连接节点</CardTitle>
          </CardHeader>
          <CardContent className="space-y-3">
            {[...peers.values()].map((peer) => (
              <div
                key={peer.peerId}
                className={`p-3 rounded-md cursor-pointer ${selectedPeer === peer.peerId ? "bg-primary/10 border border-primary" : "bg-muted"}`}
                onClick={() => setSelectedPeer(peer.peerId)}
              >
                <div className="flex items-center justify-between">
                  <div className="font-mono text-xs truncate flex-1">{peer.peerId}</div>
                  <div className="flex items-center gap-2">
                    {peer.lastRtt !== undefined && (
                      <span className="text-xs text-muted-foreground">{peer.lastRtt}ms</span>
                    )}
                    <Button
                      size="sm"
                      variant="ghost"
                      className="h-7 w-7 p-0 hover:bg-destructive hover:text-destructive-foreground"
                      onClick={(e) => { e.stopPropagation(); disconnect(peer.peerId); }}
                    >
                      <X size={14} />
                    </Button>
                  </div>
                </div>
                {peer.agentVersion && (
                  <div className="text-xs text-muted-foreground mt-1">{peer.agentVersion}</div>
                )}
              </div>
            ))}

            {selectedPeer && (
              <div className="pt-3 border-t space-y-2">
                <div className="text-sm font-medium">发送请求到: {selectedPeer.slice(0, 16)}...</div>
                <div className="flex gap-2">
                  <Input placeholder="Name" value={reqName} onChange={(e) => setReqName(e.target.value)} className="flex-1" />
                  <Input placeholder="Age" type="number" value={reqAge} onChange={(e) => setReqAge(e.target.value)} className="w-24" />
                  <Button onClick={sendRequest} disabled={!reqName || !reqAge}>
                    <Send size={14} className="mr-1" />
                    发送
                  </Button>
                </div>
              </div>
            )}
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
                <div key={i} className={getEventColor(event.type)}>{formatEvent(event)}</div>
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
    case "listening": return "text-blue-500";
    case "connected": return "text-green-500";
    case "disconnected": return "text-yellow-500";
    case "ping": return "text-foreground";
    case "identified": return "text-purple-500";
    case "inboundRequest": return "text-orange-500";
    case "response": return "text-green-600";
    case "requestFailed":
    case "error": return "text-red-500";
    default: return "";
  }
}

function formatEvent(event: ReqRespEvent): string {
  switch (event.type) {
    case "listening": return `[监听] ${event.addr}`;
    case "connected": return `[连接] ${event.peerId}`;
    case "disconnected": return `[断开] ${event.peerId}`;
    case "ping": return `[Ping] ${event.peerId.slice(0, 20)}... RTT: ${event.rttMs}ms`;
    case "identified": return `[识别] ${event.peerId.slice(0, 20)}... ${event.agentVersion}`;
    case "inboundRequest": return `[请求] 来自 ${event.peerId.slice(0, 16)}... name=${event.name}, age=${event.age}`;
    case "response": return `[响应] ${event.peerId.slice(0, 16)}... ${event.message}`;
    case "requestFailed": return `[失败] ${event.peerId.slice(0, 16)}... ${event.error}`;
    case "error": return `[错误] ${event.message}`;
  }
}
