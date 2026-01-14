import { useState } from "preact/hooks";
import { ConnectionManager } from "./components/ConnectionManager";
import { QueryRunner } from "./components/QueryRunner";
import "./App.css";

function App() {
  const [activeConnectionId, setActiveConnectionId] = useState<string | null>(null);

  // Prevent default context menu for more app-like feel
  // useEffect(() => {
  //   document.addEventListener('contextmenu', event => event.preventDefault());
  // }, []);

  return (
    <main class="h-screen w-screen flex overflow-hidden bg-white dark:bg-zinc-950 text-zinc-900 dark:text-zinc-100 font-sans">
      <ConnectionManager
        onConnect={(id) => setActiveConnectionId(id)}
        activeId={activeConnectionId || undefined}
      />

      <div class="flex-1 flex flex-col h-full relative z-0 overflow-hidden">
        {activeConnectionId ? (
          <QueryRunner connectionId={activeConnectionId} />
        ) : (
          <div class="flex-1 flex flex-col items-center justify-center text-zinc-400 bg-zinc-50/50 dark:bg-zinc-900/50">
            <div class="w-16 h-16 mb-4 rounded-xl bg-zinc-100 dark:bg-zinc-800 flex items-center justify-center">
              <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-database-zap opacity-50"><ellipse cx="12" cy="5" rx="9" ry="3" /><path d="M3 5V19A9 3 0 0 0 15 21.84" /><path d="M21 5V8" /><path d="M21 12L18 17H22L19 22" /></svg>
            </div>
            <p class="font-medium">No Connection Active</p>
            <p class="text-sm opacity-70 mt-1">Connect to a database using the sidebar.</p>
          </div>
        )}
      </div>
    </main>
  );
}

export default App;
