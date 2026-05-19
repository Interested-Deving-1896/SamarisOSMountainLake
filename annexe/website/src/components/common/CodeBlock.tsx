import { Check, Copy } from "lucide-react";
import { useState } from "react";
import { Button } from "@/components/ui/button";

interface Props {
  code: string;
  language?: string;
}

export const CodeBlock = ({ code, language = "bash" }: Props) => {
  const [copied, setCopied] = useState(false);

  const onCopy = async () => {
    await navigator.clipboard.writeText(code);
    setCopied(true);
    setTimeout(() => setCopied(false), 1800);
  };

  return (
    <div className="glass-strong rounded-xl overflow-hidden">
      <div className="flex items-center justify-between px-4 py-2 border-b border-border/60">
        <span className="text-xs font-mono uppercase tracking-wider text-muted-foreground">{language}</span>
        <Button
          size="sm"
          variant="ghost"
          onClick={onCopy}
          aria-label="Copy code"
          className="h-7 px-2 text-xs gap-1.5 text-muted-foreground hover:text-primary"
        >
          {copied ? <Check className="size-3.5" /> : <Copy className="size-3.5" />}
          {copied ? "Copied" : "Copy"}
        </Button>
      </div>
      <pre className="p-4 text-sm overflow-x-auto">
        <code className="font-mono text-foreground">{code}</code>
      </pre>
    </div>
  );
};
