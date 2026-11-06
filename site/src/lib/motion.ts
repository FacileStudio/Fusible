import { gsap } from "gsap";
import { prefersReducedMotion } from "@facile/muse";

/**
 * Svelte action that reveals an element and its `[data-reveal-item]` children when scrolled into view.
 */
export function scrollReveal(node: HTMLElement, opts: { delay?: number; stagger?: number; y?: number } = {}) {
  if (typeof window === "undefined" || prefersReducedMotion()) return {};
  const items = Array.from(node.querySelectorAll<HTMLElement>("[data-reveal-item]"));
  const targets = items.length > 0 ? items : [node];
  gsap.set(targets, { opacity: 0, y: opts.y ?? 20 });
  const observer = new IntersectionObserver(
    (entries) => {
      if (!entries.some((entry) => entry.isIntersecting)) return;
      observer.disconnect();
      gsap.to(targets, {
        opacity: 1,
        y: 0,
        duration: 0.65,
        delay: opts.delay ?? 0,
        stagger: opts.stagger ?? 0.07,
        ease: "power3.out",
        clearProps: "transform,opacity",
      });
    },
    { threshold: 0.1, rootMargin: "0px 0px -40px 0px" },
  );
  observer.observe(node);
  return {
    destroy() {
      observer.disconnect();
      gsap.killTweensOf(targets);
    },
  };
}

/**
 * Slides the active navigation pill under the target link.
 */
export function moveNavPill(pill: HTMLElement | null, target: HTMLElement | undefined, animate = true) {
  if (!pill || !target) {
    if (pill) gsap.set(pill, { autoAlpha: 0 });
    return;
  }
  const to = { x: target.offsetLeft, width: target.offsetWidth, autoAlpha: 1 };
  if (!animate || prefersReducedMotion()) {
    gsap.set(pill, to);
    return;
  }
  gsap.to(pill, { ...to, duration: 0.32, ease: "power3.inOut", overwrite: "auto" });
}

/**
 * Smoothly scrolls to a section id on the current page.
 */
export function scrollToSection(id: string, event?: MouseEvent) {
  const target = document.getElementById(id);
  if (!target) return;
  event?.preventDefault();
  history.replaceState(null, "", `#${id}`);
  const top = target.getBoundingClientRect().top + window.scrollY - 76;
  window.scrollTo({ top: Math.max(0, top), behavior: prefersReducedMotion() ? "auto" : "smooth" });
}

/**
 * Returns the id of the last section whose top has passed the trigger line.
 */
function detectActiveSection(ids: string[]): string {
  const maxScroll = Math.max(1, document.documentElement.scrollHeight - window.innerHeight);
  const progress = Math.min(1, Math.max(0, window.scrollY / maxScroll));
  const triggerLine = window.innerHeight * (0.24 + progress * 0.56);
  let current = "";
  for (const id of ids) {
    const el = document.getElementById(id);
    if (el && el.getBoundingClientRect().top <= triggerLine) current = id;
  }
  return current;
}

/**
 * Reports scroll state and the active section while the window scrolls or resizes.
 */
export function watchActiveSection(
  ids: string[],
  onActive: (id: string) => void,
  onScrollState: (scrolled: boolean) => void,
) {
  const onScroll = () => {
    onScrollState(window.scrollY > 16);
    onActive(detectActiveSection(ids));
  };
  onScroll();
  window.addEventListener("scroll", onScroll, { passive: true });
  window.addEventListener("resize", onScroll, { passive: true });
  return () => {
    window.removeEventListener("scroll", onScroll);
    window.removeEventListener("resize", onScroll);
  };
}
