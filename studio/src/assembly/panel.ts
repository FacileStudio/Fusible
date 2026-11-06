import type { Report, Violation } from "./model";

export interface Panel {
  status: HTMLDivElement;
  list: HTMLDivElement;
}

let buildHandler: () => void = () => undefined;

/** Registers what the build button of the assembly tab does. */
export function setAssemblyBuildHandler(handler: () => void): void {
  buildHandler = handler;
}

/** Creates the empty-state line and the violation list inside `host`. */
export function mountPanel(host: HTMLElement): Panel {
  const status = document.createElement("div");
  status.className = "empty-state assembly-status";
  const list = document.createElement("div");
  list.className = "assembly-list";
  list.hidden = true;
  host.append(status, list);
  return { status, list };
}

function buildButton(): HTMLButtonElement {
  const button = document.createElement("button");
  button.className = "btn assembly-build";
  button.textContent = "Build assembly";
  button.addEventListener("click", () => buildHandler());
  return button;
}

/** Shows `message` centred over the stage, with a build button when a build would help; empty hides it. */
export function setStatus(panel: Panel, message: string, buildable = false): void {
  panel.status.replaceChildren(document.createTextNode(message));
  if (message && buildable) panel.status.append(buildButton());
  panel.status.style.display = message ? "flex" : "none";
}

function row(violation: Violation, onPick: (violation: Violation) => void): HTMLButtonElement {
  const item = document.createElement("button");
  item.className = "assembly-item";
  const rule = document.createElement("span");
  rule.className = "assembly-rule";
  rule.textContent = violation.rule;
  const message = document.createElement("span");
  message.textContent = violation.message;
  item.append(rule, message);
  item.addEventListener("click", () => onPick(violation));
  return item;
}

function heading(report: Report): HTMLDivElement {
  const head = document.createElement("div");
  head.className = "assembly-head";
  const count = report.violations.length;
  const noun = count === 1 ? "violation" : "violations";
  head.textContent = report.fuse === "blown" ? `Fuse blown, ${count} ${noun}` : "Fuse intact";
  head.classList.toggle("is-blown", report.fuse === "blown");
  return head;
}

/** Lists the violations of `report`; each row calls `onPick`. A null report hides the list. */
export function setReport(panel: Panel, report: Report | null, onPick: (violation: Violation) => void): void {
  panel.list.hidden = !report;
  if (!report) return;
  panel.list.replaceChildren(heading(report), ...report.violations.map((v) => row(v, onPick)));
}

/** Marks the row at `index` as the picked one. */
export function markPicked(panel: Panel, index: number): void {
  panel.list.querySelectorAll(".assembly-item").forEach((item, i) => item.classList.toggle("active", i === index));
}
