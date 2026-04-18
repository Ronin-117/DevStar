import { useStore } from '../store';

export function SettingsView() {
  const settings = useStore((s: any) => s.settings);
  const updateSettings = useStore((s: any) => s.updateSettings);

  if (!settings) return null;

  return (
    <div className="p-6">
      <h2 className="text-xl font-semibold mb-6">Settings</h2>
      <div className="bg-white border rounded-xl p-6">
        <div className="flex items-center justify-between">
          <div>
            <h3 className="font-medium text-gray-800">Enable MCP Server</h3>
            <p className="text-sm text-gray-500">Allow AI agents to interact with your projects.</p>
          </div>
          <button
            onClick={() => updateSettings({ mcp_enabled: !settings.mcp_enabled })}
            className={`w-12 h-6 rounded-full transition-colors ${
              settings.mcp_enabled ? 'bg-indigo-600' : 'bg-gray-200'
            }`}
          >
            <div className={`w-4 h-4 rounded-full bg-white transition-transform ${
              settings.mcp_enabled ? 'translate-x-7' : 'translate-x-1'
            }`} />
          </button>
        </div>
      </div>
    </div>
  );
}
