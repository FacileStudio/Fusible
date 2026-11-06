import { expect, test } from "bun:test";
import { en } from "./i18n-en";
import { fr } from "./i18n-fr";

test("both catalogs have the same keys", () => {
  expect(Object.keys(fr).sort()).toEqual(Object.keys(en).sort());
});

test("no French string is left as the English text", () => {
  const same = Object.keys(en).filter((k) => (en as Record<string, string>)[k] === (fr as Record<string, string>)[k]);
  expect(same.sort()).toEqual(["appName", "total", "view3d", "viewPcb"]);
});
