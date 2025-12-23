import { createFileRoute } from "@tanstack/react-router";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { MessageCircle } from "lucide-react";

export const Route = createFileRoute("/chat")({
  component: Chat,
});

function Chat() {
  return (
    <div className="p-6">
      <Card>
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <MessageCircle size={20} />
            P2P 聊天
          </CardTitle>
        </CardHeader>
        <CardContent>
          <p className="text-muted-foreground">即将推出...</p>
          <p className="text-sm text-muted-foreground mt-2">
            基于 GossipSub 的去中心化聊天室，配合教程第五篇「消息传播」使用。
          </p>
        </CardContent>
      </Card>
    </div>
  );
}
