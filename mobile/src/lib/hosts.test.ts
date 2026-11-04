import { expect, test } from "bun:test";
import { isPrivateHost } from "./hosts";

test("accepts RFC 1918 addresses with a port", () => {
  const good = [
    "http://10.0.0.5:3020",
    "http://172.16.0.1:3020",
    "http://172.31.255.254:3021",
    "http://192.168.1.20:3020/",
  ];
  for (const url of good) {
    expect(isPrivateHost(url)).toBe(true);
  }
});

test("refuses public, loopback, link-local and out-of-range addresses", () => {
  const bad = ["http://8.8.8.8:3020", "http://127.0.0.1:3020", "http://169.254.1.1:3020", "http://172.32.0.1:3020"];
  for (const url of bad) {
    expect(isPrivateHost(url)).toBe(false);
  }
  expect(isPrivateHost("http://192.168.1.256:3020")).toBe(false);
});

test("refuses names, missing ports, https, paths and credentials", () => {
  const bad = [
    "http://192.168.1.20",
    "https://192.168.1.20:3020",
    "http://localhost:3020",
    "http://studio.local:3020",
    "http://192.168.1.20:3020/api",
    "http://user@192.168.1.20:3020",
    "http://192.168.1.20:0",
    "http://192.168.1.20:99999",
    "http://192.168.01.20:3020",
    "http://192.168.1.20.evil.com:3020",
    "",
  ];
  for (const url of bad) expect(isPrivateHost(url)).toBe(false);
});
