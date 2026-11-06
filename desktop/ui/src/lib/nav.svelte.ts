export type View = "projects" | "settings";
export type SettingsTab = "general" | "toolchain" | "updates" | "phone";

const TABS: SettingsTab[] = ["general", "toolchain", "updates", "phone"];

class Nav {
  view = $state<View>("projects");
  tab = $state<SettingsTab>("general");

  go = (view: View, tab?: SettingsTab): void => {
    this.view = view;
    if (tab) this.tab = tab;
  };

  fromHash = (hash: string): void => {
    const [view, tab] = hash.replace(/^#\/?/, "").split("/");
    this.view = view === "settings" ? "settings" : "projects";
    if (TABS.includes(tab as SettingsTab)) this.tab = tab as SettingsTab;
  };
}

export const nav = new Nav();
