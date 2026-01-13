import {
  loadSettings as loadSettingsFromBackend,
  saveSettings as saveSettingsToBackend,
  getSystemTheme,
  setWindowTheme,
  type Settings as BackendSettings,
} from "../tauri";

export interface Settings {
  fontSize: number;
  theme: "light" | "dark" | "system";
  autoPreview: boolean;
  previewDelay: number;
  activeTemplateId: string | null;
  autoSave: boolean;
}

const defaultSettings: Settings = {
  fontSize: 14,
  theme: "system",
  autoPreview: true,
  previewDelay: 300,
  activeTemplateId: null,
  autoSave: true,
};

// Get effective theme considering system preference
async function getEffectiveTheme(
  theme: "light" | "dark" | "system",
): Promise<"light" | "dark"> {
  if (theme === "system") {
    try {
      const systemTheme = await getSystemTheme();
      return systemTheme === "dark" ? "dark" : "light";
    } catch (e) {
      console.warn(
        "Failed to get system theme, falling back to media query:",
        e,
      );
      return window.matchMedia("(prefers-color-scheme: dark)").matches
        ? "dark"
        : "light";
    }
  }
  return theme;
}

// Apply theme to HTML element
async function applyTheme(theme: "light" | "dark" | "system") {
  const effectiveTheme = await getEffectiveTheme(theme);
  const html = document.documentElement;

  const themeMap = {
    light: "caramellatte",
    dark: "coffee",
  };

  html.setAttribute("data-theme", themeMap[effectiveTheme]);

  try {
    localStorage.setItem("saga-scribe-theme", theme);
  } catch (e) {
    console.warn("Failed to cache theme in localStorage:", e);
  }

  try {
    await setWindowTheme(theme);
  } catch (e) {
    console.warn("Failed to set window theme:", e);
  }
}

class SettingsStore {
  #state = $state<Settings>(defaultSettings);
  #saveTimeout: ReturnType<typeof setTimeout> | null = null;

  get fontSize() { return this.#state.fontSize; }
  get theme() { return this.#state.theme; }
  get autoPreview() { return this.#state.autoPreview; }
  get previewDelay() { return this.#state.previewDelay; }
  get activeTemplateId() { return this.#state.activeTemplateId; }
  get autoSave() { return this.#state.autoSave; }

  async load() {
    try {
      const loaded = await loadSettingsFromBackend();
      this.#state = {
        fontSize: loaded.fontSize,
        theme: (loaded.theme === "dark"
          ? "dark"
          : loaded.theme === "system"
            ? "system"
            : "light") as "light" | "dark" | "system",
        autoPreview: loaded.autoPreview,
        previewDelay: loaded.previewDelay,
        activeTemplateId: loaded.activeTemplateId,
        autoSave: (loaded as any).autoSave ?? true, // Handle migration for existing settings
      };
      await applyTheme(this.#state.theme);
    } catch (e) {
      console.error("Failed to load settings, using defaults:", e);
      this.#state = { ...defaultSettings };
      await applyTheme(this.#state.theme);
    }
  }

  update(partial: Partial<Settings>) {
    this.#state = { ...this.#state, ...partial };
    if (partial.theme !== undefined) {
      applyTheme(this.#state.theme).catch((e) =>
        console.error("Failed to apply theme:", e),
      );
    }
    this.#debouncedSave();
  }

  reset() {
    this.#state = { ...defaultSettings };
    applyTheme(this.#state.theme).catch((e) =>
      console.error("Failed to apply theme:", e),
    );
    this.#debouncedSave();
  }

  #debouncedSave() {
    if (this.#saveTimeout) {
      clearTimeout(this.#saveTimeout);
    }
    this.#saveTimeout = setTimeout(async () => {
      try {
        const backendSettings: BackendSettings = {
          ...this.#state,
          theme: this.#state.theme === "system" ? "light" : this.#state.theme,
        };
        await saveSettingsToBackend(backendSettings);
      } catch (e) {
        console.error("Failed to save settings:", e);
      }
    }, 500);
  }
}

export const settings = new SettingsStore();

// Listen for system theme changes when using 'system' theme
if (typeof window !== "undefined") {
  const mediaQuery = window.matchMedia("(prefers-color-scheme: dark)");
  mediaQuery.addEventListener("change", () => {
    if (settings.theme === "system") {
      applyTheme("system").catch((e) =>
        console.error("Failed to apply theme on system change:", e),
      );
    }
  });
}
