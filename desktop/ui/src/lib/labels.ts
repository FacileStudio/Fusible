import type { Project } from "./types";

export const CURRENCIES = ["EUR", "USD", "GBP"];

/** The offered currencies, plus a saved one that is not in the short list. */
export function currencyChoices(current: string): string[] {
  return CURRENCIES.includes(current) ? CURRENCIES : [...CURRENCIES, current];
}

function plural(count: number, one: string, many: string): string {
  return `${count} ${count === 1 ? one : many}`;
}

export function countsLabel(p: Pick<Project, "boards" | "firmware" | "bodies">): string {
  const parts: string[] = [];
  if (p.boards) parts.push(plural(p.boards, "board", "boards"));
  if (p.firmware) parts.push(`${p.firmware} firmware`);
  if (p.bodies) parts.push(plural(p.bodies, "body", "bodies"));
  return parts.length ? parts.join(", ") : "No boards yet";
}
