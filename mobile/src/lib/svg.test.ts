import { expect, test } from "bun:test";
import { fitSvg } from "./svg";

test("adds a viewBox from the width and height", () => {
  const out = fitSvg('<svg xmlns="http://www.w3.org/2000/svg" width="800" height="600"><g/></svg>');
  expect(out).toContain('viewBox="0 0 800 600"');
  expect(out).toContain('width="100%"');
});

test("leaves an SVG that already has a viewBox alone", () => {
  const svg = '<svg viewBox="0 0 10 10" width="10" height="10"></svg>';
  expect(fitSvg(svg)).toBe(svg);
});
