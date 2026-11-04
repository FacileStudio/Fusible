import { App } from "@capacitor/app";
import { get, UnpairedError } from "./client";
import type { Host } from "./hosts";

export type BoardSummary = { name: string; fuse_blown: boolean; errors: number; warnings: number };
export type Summary = {
  project: string;
  revision: string;
  boards: BoardSummary[];
  bom_total: { amount: number; currency: string } | null;
  bodies: { name: string; kind: string }[];
  last_run: { label?: string; status?: string } | null;
};

const EVERY_MS = 5000;

/** Polls the summary route while the app is visible and exposes the latest result. */
export class Poller {
  summary = $state<Summary | null>(null);
  error = $state<string | null>(null);
  revision = $state("");
  private timer: ReturnType<typeof setInterval> | undefined;
  private handle: { remove: () => Promise<void> } | undefined;

  constructor(private host: Host) {}

  private async tick() {
    try {
      const next = await get<Summary>(this.host, "/api/mobile/summary");
      this.error = null;
      this.summary = next;
      this.revision = next.revision;
    } catch (e) {
      this.error = e instanceof UnpairedError ? "unpaired" : "offline";
    }
  }

  private resume = () => {
    void this.tick();
    clearInterval(this.timer);
    this.timer = setInterval(() => void this.tick(), EVERY_MS);
  };

  /** Starts polling and pauses it when the app goes to the background. */
  async start() {
    this.resume();
    this.handle = await App.addListener("appStateChange", ({ isActive }) => {
      if (isActive) this.resume();
      else clearInterval(this.timer);
    });
  }

  /** Stops polling and drops the app listener. */
  stop() {
    clearInterval(this.timer);
    void this.handle?.remove();
  }
}
