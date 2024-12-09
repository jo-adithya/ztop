import { commands } from "@lib/utils/bindings";
import { wrapTauriCommand } from "@lib/utils/tauri";

export const getCpuStatsCommand = wrapTauriCommand(commands.getCpuStats);

export function getCpuStatsService(
  ...args: Parameters<typeof getCpuStatsCommand>
) {
  return getCpuStatsCommand(...args);
}
