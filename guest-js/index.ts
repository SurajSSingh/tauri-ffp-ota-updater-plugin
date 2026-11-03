import { invoke } from '@tauri-apps/api/core'

export async function ping(value: string): Promise<string | null> {
  return await invoke<{ value: string | null }>('plugin:ffp-ota-updater|ping', {
    payload: {
      value,
    },
  }).then((r) => r.value);
}
