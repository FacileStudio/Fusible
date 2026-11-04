import { expect, test } from "bun:test";
import { circuitUrl, firmwareUrl, getJson, ApiError } from "./api";

test("circuitUrl and firmwareUrl encode the name", () => {
  expect(circuitUrl("main board")).toBe("/api/circuit?board=main%20board");
  expect(firmwareUrl("fw/1")).toBe("/api/firmware?name=fw%2F1");
});

test("getJson turns a non-2xx reply into an ApiError with the status", async () => {
  const real = globalThis.fetch;
  const reply = () => new Response(JSON.stringify({ error: "nope" }), { status: 404 });
  globalThis.fetch = (async () => reply()) as unknown as typeof fetch;
  try {
    const failure = (await getJson("/api/rules").catch((err) => err)) as ApiError;
    expect(failure).toBeInstanceOf(ApiError);
    expect(failure.status).toBe(404);
    expect(failure.message).toBe("nope");
  } finally {
    globalThis.fetch = real;
  }
});
