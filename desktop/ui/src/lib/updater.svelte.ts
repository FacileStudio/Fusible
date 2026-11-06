import { api, message, onUpdateProgress } from "./api";
import { percentOf } from "./updateText";
import type { UpdateCheck, UpdateProgress } from "./types";

export type UpdaterState = "idle" | "checking" | "available" | "downloading" | "ready" | "error" | "upToDate";

export type UpdaterApi = {
  checkUpdate: (auto: boolean) => Promise<UpdateCheck>;
  installUpdate: () => Promise<void>;
  restartApp: () => Promise<void>;
  onProgress: (cb: (progress: UpdateProgress) => void) => Promise<void>;
};

const real: UpdaterApi = {
  checkUpdate: api.checkUpdate,
  installUpdate: api.installUpdate,
  restartApp: api.restartApp,
  onProgress: onUpdateProgress,
};

export class UpdaterStore {
  state = $state<UpdaterState>("idle");
  version = $state<string | null>(null);
  notes = $state("");
  progress = $state<UpdateProgress>({ downloaded: 0, total: null });
  error = $state<string | null>(null);
  errorUrl = $state<string | null>(null);

  percent = $derived(percentOf(this.progress.downloaded, this.progress.total));

  #api: UpdaterApi;
  #subscribed = false;

  constructor(backend: UpdaterApi = real) {
    this.#api = backend;
  }

  checkNow = async (auto = false): Promise<void> => {
    if (this.state === "checking" || this.state === "downloading" || this.state === "ready") return;
    this.error = null;
    this.errorUrl = null;
    if (!auto) this.state = "checking";
    try {
      this.#apply(await this.#api.checkUpdate(auto));
    } catch (error) {
      if (auto) this.state = "idle";
      else this.#fail(message(error));
    }
  };

  installNow = async (): Promise<void> => {
    if (this.state !== "available" && !(this.state === "error" && this.version)) return;
    this.error = null;
    this.progress = { downloaded: 0, total: null };
    this.state = "downloading";
    try {
      await this.#listen();
      await this.#api.installUpdate();
      this.state = "ready";
    } catch (error) {
      this.#fail(message(error));
    }
  };

  restartNow = async (): Promise<void> => {
    try {
      await this.#api.restartApp();
    } catch (error) {
      this.#fail(message(error));
    }
  };

  #apply(result: UpdateCheck): void {
    if (result.status === "available") {
      this.version = result.version;
      this.notes = result.notes;
      this.state = "available";
    } else if (result.status === "up_to_date") {
      this.state = "upToDate";
    } else if (result.status === "unsupported") {
      this.#fail("This install cannot update itself. Get the new version from the release page.");
      this.errorUrl = result.url;
    } else {
      this.state = "idle";
    }
  }

  #fail(text: string): void {
    this.error = text;
    this.state = "error";
  }

  async #listen(): Promise<void> {
    if (this.#subscribed) return;
    this.#subscribed = true;
    await this.#api.onProgress((progress) => {
      if (this.state === "downloading") this.progress = progress;
    });
  }
}

export const updater = new UpdaterStore();
