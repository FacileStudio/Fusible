import { expect, test } from "bun:test";
import { parsePairUri } from "./pair";

const token = "a".repeat(64);

test("parses a good link and decodes the name", () => {
  const r = parsePairUri(`fusible://pair?h=192.168.1.20:3020&t=${token}&n=My%20board`);
  expect(r).toEqual({ ok: true, host: "http://192.168.1.20:3020", token, name: "My board" });
});

test("falls back to a default name", () => {
  const r = parsePairUri(`fusible://pair?h=10.0.0.2:3020&t=${token}`);
  expect(r.ok && r.name).toBe("Fusible");
});

test("refuses a public host, whatever the token", () => {
  expect(parsePairUri(`fusible://pair?h=8.8.8.8:3020&t=${token}`)).toEqual({ ok: false, error: "not_private" });
  expect(parsePairUri(`fusible://pair?h=example.com:80&t=${token}`).ok).toBe(false);
});

test("refuses other schemes and bad tokens", () => {
  expect(parsePairUri("https://example.com").ok).toBe(false);
  expect(parsePairUri("fusible://pair?h=10.0.0.2:3020&t=zz")).toEqual({ ok: false, error: "bad_token" });
});
