import { useState } from "preact/hooks";
import { invoke } from "@tauri-apps/api/core";
import { Button } from "./ui/Button";
import { Play } from "lucide-preact";

interface QueryRunnerProps {
  connectionId: string;
}

export function QueryRunner({ connectionId }: QueryRunnerProps) {
  const [sql, setSql] = useState("SELECT * FROM information_schema.tables LIMIT 10;");
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  const [results, setResults] = useState<any[]>([]);
  const [columns, setColumns] = useState<string[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  async function runQuery() {
    setLoading(true);
    setError(null);
    try {
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      const data = await invoke("execute", { id: connectionId, sql }) as any[];
      setResults(data);
      if (data.length > 0) {
        setColumns(Object.keys(data[0]));
      } else {
        setColumns([]);
      }
    } catch (e) {
      setError(String(e));
    } finally {
      setLoading(false);
    }
  }

  return (
    <div class="flex flex-col h-full bg-white dark:bg-zinc-950">
        <div class="p-2 border-b border-zinc-200 dark:border-zinc-800 flex gap-2 bg-zinc-50 dark:bg-zinc-900 justify-between items-center">
            <h3 class="text-sm font-semibold text-zinc-600 dark:text-zinc-400 px-2">Query Editor</h3>
            <Button size="sm" onClick={runQuery} disabled={loading}>
                <Play class="w-3 h-3 mr-2" /> 
                {loading ? "Running..." : "Run Query"}
            </Button>
        </div>
        
        <div class="h-48 border-b border-zinc-200 dark:border-zinc-800 relative bg-zinc-900">
             <textarea 
                class="w-full h-full p-4 font-mono text-sm bg-zinc-50 dark:bg-zinc-950 text-zinc-800 dark:text-zinc-100 outline-none resize-none"
                value={sql}
                onInput={(e) => setSql(e.currentTarget.value)}
                placeholder="SELECT * FROM ..."
                spellcheck={false}
             />
        </div>
        
        <div class="flex-1 overflow-auto p-0 bg-zinc-50/50 dark:bg-zinc-900/50">
            {error && <div class="text-red-600 dark:text-red-400 font-mono text-sm p-4 bg-red-50 dark:bg-red-900/10 border-b border-red-100 dark:border-red-900/20">{error}</div>}
            
            {!error && results.length === 0 && !loading && (
                <div class="text-zinc-400 text-center mt-20 text-sm">
                    No results to display. Run a query to see data.
                </div>
            )}

            {results.length > 0 && (
                <div class="w-full">
                <table class="w-full text-left text-sm border-collapse">
                    <thead class="bg-zinc-100 dark:bg-zinc-800 text-zinc-600 dark:text-zinc-300 sticky top-0 z-10 shadow-sm">
                        <tr>
                            {columns.map(col => <th class="p-2 font-medium border-b border-r dark:border-zinc-700 last:border-r-0 whitespace-nowrap">{col}</th>)}
                        </tr>
                    </thead>
                    <tbody class="divide-y divide-zinc-100 dark:divide-zinc-800 bg-white dark:bg-zinc-950">
                        {results.map((row) => (
                            <tr class="hover:bg-blue-50 dark:hover:bg-blue-900/10 transition-colors">
                                {columns.map(col => (
                                    <td class="p-2 border-r dark:border-zinc-800 last:border-r-0 font-mono text-xs text-zinc-600 dark:text-zinc-400 max-w-[300px] truncate whitespace-nowrap" title={String(row[col])}>
                                        {row[col] === null ? <span class="text-zinc-300 italic">null</span> : String(row[col])}
                                    </td>
                                ))}
                            </tr>
                        ))}
                    </tbody>
                </table>
                </div>
            )}
        </div>
    </div>
  );
}
