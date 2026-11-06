import { expect, test } from "bun:test";
import { remaining } from "./pair";

test("remaining counts whole seconds and stops at zero", () => {
  const now = Date.parse("2026-11-04T10:00:00Z");
  expect(remaining("2026-11-04T10:01:30Z", now)).toBe(90);
  expect(remaining("2026-11-04T09:59:00Z", now)).toBe(0);
});
