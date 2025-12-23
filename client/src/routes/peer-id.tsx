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
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { Copy, KeyRound } from "lucide-react";
import { generatePeerId, type KeyType, type PeerIdResult } from "@/commands";

export const Route = createFileRoute("/peer-id")({
  component: PeerIdGenerator,
});

function PeerIdGenerator() {
  const [keyType, setKeyType] = useState<KeyType>("ed25519");
  const [result, setResult] = useState<PeerIdResult | null>(null);
  const [loading, setLoading] = useState(false);
  const [copied, setCopied] = useState<string | null>(null);

  const generate = async () => {
    setLoading(true);
    try {
      const res = await generatePeerId(keyType);
      setResult(res);
    } catch (e) {
      console.error(e);
    } finally {
      setLoading(false);
    }
  };

  const copyToClipboard = async (text: string, field: string) => {
    await navigator.clipboard.writeText(text);
    setCopied(field);
    setTimeout(() => setCopied(null), 1500);
  };

  return (
    <div className="p-6 max-w-2xl space-y-4">
      <h1 className="text-xl font-bold flex items-center gap-2">
        <KeyRound size={20} />
        PeerId 生成器
      </h1>

      <Card>
        <CardHeader>
          <CardTitle className="text-base">生成配置</CardTitle>
          <CardDescription>
            选择密钥类型并生成对应的 PeerId
          </CardDescription>
        </CardHeader>
        <CardContent>
          <div className="flex gap-4">
            <Select value={keyType} onValueChange={(v) => setKeyType(v as KeyType)}>
              <SelectTrigger className="w-48">
                <SelectValue />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="ed25519">Ed25519 (推荐)</SelectItem>
                <SelectItem value="secp256k1">Secp256k1</SelectItem>
                <SelectItem value="ecdsa">ECDSA</SelectItem>
              </SelectContent>
            </Select>
            <Button onClick={generate} disabled={loading}>
              {loading ? "生成中..." : "生成"}
            </Button>
          </div>
        </CardContent>
      </Card>

      {result && (
        <Card>
          <CardHeader>
            <CardTitle className="text-base">生成结果</CardTitle>
            <CardDescription>密钥类型: {result.key_type}</CardDescription>
          </CardHeader>
          <CardContent className="space-y-4">
            <ResultField
              label="PeerId"
              value={result.peer_id}
              copied={copied === "peer_id"}
              onCopy={() => copyToClipboard(result.peer_id, "peer_id")}
            />
            <ResultField
              label="公钥 (Protobuf Hex)"
              value={result.public_key_hex}
              copied={copied === "public_key"}
              onCopy={() => copyToClipboard(result.public_key_hex, "public_key")}
              small
            />
          </CardContent>
        </Card>
      )}
    </div>
  );
}

function ResultField({
  label,
  value,
  copied,
  onCopy,
  small,
}: {
  label: string;
  value: string;
  copied: boolean;
  onCopy: () => void;
  small?: boolean;
}) {
  return (
    <div>
      <div className="flex items-center justify-between mb-1">
        <label className="text-sm font-medium text-muted-foreground">
          {label}
        </label>
        <button
          onClick={onCopy}
          className="text-xs text-muted-foreground hover:text-foreground flex items-center gap-1"
        >
          <Copy size={12} />
          {copied ? "已复制" : "复制"}
        </button>
      </div>
      <div
        className={`p-3 bg-muted rounded-md font-mono break-all ${small ? "text-xs" : "text-sm"}`}
      >
        {value}
      </div>
    </div>
  );
}
