import { api, message } from "./api";
import { baseName, suggestName } from "./path";
import type { Project } from "./types";

/** Orders projects by last opened, most recent first; never-opened ones go last. */
export function sortRecent(projects: Project[]): Project[] {
  const stamp = (p: Project) => (p.last_opened ? Date.parse(p.last_opened) || 0 : 0);
  return [...projects].sort((a, b) => stamp(b) - stamp(a));
}

/** Sorts by recency and keeps projects whose name or path contains the query. */
export function filterProjects(projects: Project[], query: string): Project[] {
  const needle = query.trim().toLowerCase();
  const list = sortRecent(projects);
  if (!needle) return list;
  return list.filter((p) => p.name.toLowerCase().includes(needle) || p.path.toLowerCase().includes(needle));
}

export type Failure = { title: string; log: string };

export type Prefill = { name: string; location: string };

export class ProjectsStore {
  all = $state<Project[]>([]);
  query = $state("");
  loaded = $state(false);
  loadError = $state<string | null>(null);
  opening = $state<string | null>(null);
  failure = $state<Failure | null>(null);
  prefill = $state<Prefill | null>(null);

  visible = $derived(filterProjects(this.all, this.query));

  load = async (): Promise<void> => {
    try {
      this.all = await api.listProjects();
      this.loadError = null;
    } catch (error) {
      this.loadError = message(error);
    }
    this.loaded = true;
  };

  open = async (path: string, name = baseName(path)): Promise<void> => {
    if (this.opening) return;
    this.opening = path;
    this.failure = null;
    try {
      await api.openProject(path);
    } catch (error) {
      const log = (await api.lastError().catch(() => null)) || message(error);
      this.failure = { title: `Could not open ${name}`, log };
    }
    this.opening = null;
    await this.load();
  };

  forget = async (path: string): Promise<void> => {
    await api.forgetProject(path).catch(() => undefined);
    await this.load();
  };

  reveal = async (path: string): Promise<void> => {
    await api.revealProject(path).catch((error) => {
      this.failure = { title: "Could not open the folder", log: message(error) };
    });
  };

  openFolder = async (): Promise<boolean> => {
    const picked = await api.pickFolder("Open a project folder").catch(() => null);
    if (!picked) return false;
    const info = await api.inspectFolder(picked).catch(() => null);
    if (info?.has_manifest) {
      await this.open(picked);
      return false;
    }
    this.prefill = { name: suggestName(picked), location: picked };
    return true;
  };
}

export const projects = new ProjectsStore();
