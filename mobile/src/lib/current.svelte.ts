import { type Host, loadHosts } from "./hosts";
import { MOCK_ID, mockHost } from "./fixtures";

/** Loads the paired studio named by the `?id=` query of a page. */
export class Current {
  host = $state<Host | null>(null);
  ready = $state(false);

  /** Finds the host by id; the mock host answers to its own id. */
  async load(id: string) {
    this.host = id === MOCK_ID ? mockHost : ((await loadHosts()).find((h) => h.id === id) ?? null);
    this.ready = true;
  }
}
