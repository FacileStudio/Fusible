/** Whole seconds left until an RFC 3339 expiry; never negative. */
export function remaining(expiresAt: string, now: number): number {
  return Math.max(0, Math.ceil((Date.parse(expiresAt) - now) / 1000));
}
