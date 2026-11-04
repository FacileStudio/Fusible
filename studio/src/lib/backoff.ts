export const MIN_DELAY = 1000;
export const MAX_DELAY = 10000;

/** The wait before the next reconnect attempt: doubles from one second up to ten. */
export function nextDelay(current: number): number {
  return Math.min(MAX_DELAY, Math.max(MIN_DELAY, current * 2));
}
