import { expect, test } from "bun:test";
import { handleKey, type KeyActions } from "./keys";

function actions(log: string[], assembly = true): KeyActions {
  return {
    setView: (view) => log.push(`view:${view}`),
    toggleSidebar: () => log.push("sidebar"),
    toggleTree: () => log.push("tree"),
    closeOverlay: () => (log.push("close"), true),
    hasAssembly: () => assembly,
    hasProject: () => true,
  };
}

const press = (key: string, extra: Partial<KeyboardEvent> = {}) => ({ key, target: null, ...extra }) as KeyboardEvent;

test("digits switch views and the assembly key needs an assembly", () => {
  const log: string[] = [];
  handleKey(press("2"), actions(log));
  handleKey(press("5"), actions(log, false));
  handleKey(press("5"), actions(log, true));
  expect(log).toEqual(["view:pcb", "view:assembly"]);
});

test("backslash, bracket and escape reach their actions, modifiers do not", () => {
  const log: string[] = [];
  handleKey(press("\\"), actions(log));
  handleKey(press("["), actions(log));
  handleKey(press("Escape"), actions(log));
  handleKey(press("1", { ctrlKey: true }), actions(log));
  expect(log).toEqual(["sidebar", "tree", "close"]);
});
