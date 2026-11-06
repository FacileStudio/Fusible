import { describe, expect, test } from "bun:test";
import { UpdaterStore, type UpdaterApi } from "./updater.svelte";
import type { UpdateCheck, UpdateProgress } from "./types";

const AVAILABLE: UpdateCheck = { status: "available", version: "1.0.0", notes: "Notes", date: null };

function fake(check: UpdateCheck | Error, install: Error | null = null) {
  const listeners: ((p: UpdateProgress) => void)[] = [];
  let restarted = 0;
  const backend: UpdaterApi = {
    checkUpdate: async () => {
      if (check instanceof Error) throw check;
      return check;
    },
    installUpdate: async () => {
      for (const cb of listeners) cb({ downloaded: 50, total: 200 });
      if (install) throw install;
    },
    restartApp: async () => {
      restarted += 1;
    },
    onProgress: async (cb) => {
      listeners.push(cb);
    },
  };
  return { store: new UpdaterStore(backend), restarts: () => restarted, listeners };
}

describe("updater store", () => {
  test("skipped maps to idle", async () => {
    const { store } = fake({ status: "skipped" });
    await store.checkNow(true);
    expect(store.state).toBe("idle");
  });
  test("up_to_date maps to upToDate", async () => {
    const { store } = fake({ status: "up_to_date" });
    await store.checkNow();
    expect(store.state).toBe("upToDate");
  });
  test("available keeps the version and notes", async () => {
    const { store } = fake(AVAILABLE);
    await store.checkNow();
    expect([store.state, store.version, store.notes]).toEqual(["available", "1.0.0", "Notes"]);
  });
  test("unsupported becomes an error with the release link", async () => {
    const { store } = fake({ status: "unsupported", url: "https://example.com/r" });
    await store.checkNow();
    expect(store.state).toBe("error");
    expect(store.errorUrl).toBe("https://example.com/r");
  });
  test("a manual failure shows the error, an automatic one stays silent", async () => {
    const manual = fake(new Error("offline"));
    await manual.store.checkNow();
    expect([manual.store.state, manual.store.error]).toEqual(["error", "offline"]);
    const auto = fake(new Error("offline"));
    await auto.store.checkNow(true);
    expect(auto.store.state).toBe("idle");
  });
  test("install goes through downloading to ready and reports percent", async () => {
    const { store, listeners } = fake(AVAILABLE);
    await store.checkNow();
    const done = store.installNow();
    expect(store.state).toBe("downloading");
    await done;
    expect(store.state).toBe("ready");
    expect(listeners.length).toBe(1);
  });
  test("percent follows progress while downloading", async () => {
    const { store } = fake(AVAILABLE);
    await store.checkNow();
    await store.installNow();
    store.progress = { downloaded: 50, total: 200 };
    expect(store.percent).toBe(25);
    store.progress = { downloaded: 5, total: null };
    expect(store.percent).toBeNull();
  });
  test("a failed install keeps the version and can retry", async () => {
    const { store } = fake(AVAILABLE, new Error("bad"));
    await store.checkNow();
    await store.installNow();
    expect([store.state, store.error, store.version]).toEqual(["error", "bad", "1.0.0"]);
  });
  test("install does nothing without an update, and a ready update survives a check", async () => {
    const idle = fake(AVAILABLE);
    await idle.store.installNow();
    expect(idle.store.state).toBe("idle");
    const { store } = fake(AVAILABLE);
    await store.checkNow();
    await store.installNow();
    await store.checkNow();
    expect(store.state).toBe("ready");
  });
  test("restart calls the backend", async () => {
    const { store, restarts } = fake(AVAILABLE);
    await store.restartNow();
    expect(restarts()).toBe(1);
  });
});
