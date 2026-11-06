const HEAD = /<svg([^>]*?)\swidth="([\d.]+)"\s+height="([\d.]+)"/;

/** Gives a fixed-size SVG a viewBox so CSS can scale it to its container. */
export function fitSvg(svg: string): string {
  if (/<svg[^>]*viewBox=/.test(svg)) return svg;
  return svg.replace(HEAD, (_all, rest, w, h) => `<svg${rest} viewBox="0 0 ${w} ${h}" width="100%" height="100%"`);
}
