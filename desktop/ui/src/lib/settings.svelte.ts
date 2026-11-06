import { api, message } from "./api";
import { applyTheme } from "./theme";
import type { Settings } from "./types";

class SettingsStore {
  value = $state<Settings | null>(null);
  error = $state<string | null>(null);
  saved = $state(false);

  load = async (): Promise<void> => {
    this.value = await api.getSettings();
    applyTheme(this.value.theme);
  };

  save = async (patch: Partial<Settings>): Promise<void> => {
    if (!this.value) return;
    try {
      this.value = await api.setSettings({ ...this.value, ...patch });
      applyTheme(this.value.theme);
      this.error = null;
      this.saved = true;
      setTimeout(() => (this.saved = false), 1600);
    } catch (e) {
      this.error = message(e);
    }
  };
}

export const settings = new SettingsStore();
