import { useState } from "preact/hooks";
import { invoke } from "@tauri-apps/api/core";
import { Button } from "./ui/Button";
import { Input } from "./ui/Input";
import { Database } from "lucide-preact";

interface ConnectionManagerProps {
  onConnect: (id: string, name: string) => void;
  activeId?: string;
}

export function ConnectionManager({ onConnect, activeId }: ConnectionManagerProps) {
  const [connString, setConnString] = useState("postgres://postgres:password@localhost:5432/postgres");
  const [loading, setLoading] = useState(false);
  
  async function handleConnect() {
    setLoading(true);
    try {
      const id = await invoke("connect", { connString }) as string;
      let name = "Database";
      if (connString.startsWith("postgres")) name = "PostgreSQL";
      else if (connString.startsWith("mysql")) name = "MySQL";
      else if (connString.startsWith("sqlite")) name = "SQLite";
      
      onConnect(id, name);
    } catch (e) {
      console.error(e);
      alert("Failed to connect: " + e);
    } finally {
      setLoading(false);
    }
  }

  return (
    <div class="w-72 border-r border-zinc-200 dark:border-zinc-800 bg-zinc-50 dark:bg-zinc-900 flex flex-col h-full">
      <div class="p-4 border-b border-zinc-200 dark:border-zinc-800">
        <h2 class="font-semibold flex items-center gap-2 mb-4 text-zinc-800 dark:text-zinc-200">
          <Database class="w-4 h-4" /> 
          <span>Connections</span>
        </h2>
        
        <div class="space-y-3">
            <div>
                <label class="text-xs font-medium text-zinc-500 mb-1 block">Connection String</label>
                <Input 
                    value={connString} 
                    onInput={(e) => setConnString(e.currentTarget.value)}
                    placeholder="postgres://..." 
                    class="text-xs font-mono"
                />
            </div>
            
            <Button onClick={handleConnect} disabled={loading} class="w-full" size="sm">
                {loading ? "Connecting..." : "Connect"}
            </Button>
        </div>
      </div>
      
      <div class="flex-1 overflow-auto p-4">
         {activeId ? (
            <div class="p-3 bg-white dark:bg-zinc-800/50 rounded-lg border border-zinc-200 dark:border-zinc-700/50 shadow-sm">
                <div class="flex items-center gap-2 mb-1">
                    <div class="w-2 h-2 rounded-full bg-green-500 animate-pulse"></div>
                    <div class="font-medium text-sm text-zinc-800 dark:text-zinc-200">Active Session</div>
                </div>
                <div class="text-xs font-mono text-zinc-500 truncate" title={activeId}>{activeId}</div>
            </div>
         ) : (
             <div class="text-xs text-zinc-400 text-center mt-10">
                 No active connection
             </div>
         )}
      </div>
    </div>
  );
}
