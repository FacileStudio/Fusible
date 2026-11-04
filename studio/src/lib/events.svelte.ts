import { MIN_DELAY, nextDelay } from "./backoff";

export const events = $state({ connected: false });

/** Opens the /events stream, reconnects with backoff on failure and returns a function that closes it. */
export function connectEvents(onMessage: (data: Record<string, unknown>) => void): () => void {
  let source: EventSource | null = null;
  let timer: ReturnType<typeof setTimeout> | undefined;
  let delay = MIN_DELAY / 2;
  let closed = false;

  const open = () => {
    source = new EventSource("/events");
    source.onopen = () => {
      events.connected = true;
      delay = MIN_DELAY / 2;
    };
    source.onmessage = (event) => {
      try {
        onMessage(JSON.parse(event.data));
      } catch (err) {
        console.error(err);
      }
    };
    source.onerror = () => {
      events.connected = false;
      source?.close();
      if (closed) return;
      delay = nextDelay(delay);
      timer = setTimeout(open, delay);
    };
  };

  open();
  return () => {
    closed = true;
    clearTimeout(timer);
    source?.close();
  };
}
