import { expect, test } from "bun:test";
import { MAX_DELAY, MIN_DELAY, nextDelay } from "./backoff";

test("nextDelay doubles from one second and stops at ten", () => {
  expect(nextDelay(MIN_DELAY / 2)).toBe(1000);
  expect(nextDelay(1000)).toBe(2000);
  expect(nextDelay(4000)).toBe(8000);
  expect(nextDelay(8000)).toBe(MAX_DELAY);
  expect(nextDelay(MAX_DELAY)).toBe(MAX_DELAY);
});
