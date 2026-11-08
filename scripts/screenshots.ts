import { chromium, type Page } from "playwright";
import { mkdirSync } from "node:fs";

const out = process.env.SHOTS_DIR ?? "assets/screenshots";
const urls = {
  studio: process.env.STUDIO_URL ?? "http://127.0.0.1:3020",
  mech: process.env.MECH_URL ?? "http://127.0.0.1:3021",
  desktop: process.env.DESKTOP_URL ?? "http://127.0.0.1:5174",
  mobile: process.env.MOBILE_URL ?? "http://127.0.0.1:5175",
  site: process.env.SITE_URL ?? "http://127.0.0.1:8090",
};
const MOBILE_HOST = process.env.MOBILE_HOST_ID ?? "mock";

async function open(page: Page, url: string, wait = 1500): Promise<void> {
  await page.goto(url, { waitUntil: "domcontentloaded" });
  await page.waitForTimeout(wait);
}

async function shot(page: Page, name: string): Promise<void> {
  await page.screenshot({ path: `${out}/${name}.png` });
  console.log(`wrote ${out}/${name}.png`);
}

async function studio(browser: import("playwright").Browser): Promise<void> {
  const page = await browser.newPage({ viewport: { width: 1600, height: 1000 } });
  await open(page, urls.studio, 9000);
  for (const [key, name] of [["2", "studio-pcb"], ["1", "studio-schematic"], ["3", "studio-3d"]] as const) {
    await page.keyboard.press(key);
    await page.waitForTimeout(7000);
    await shot(page, name);
  }
  await page.keyboard.press("2");
  await page.waitForTimeout(2500);
  await page.getByRole("tab", { name: "Checks", exact: true }).click().catch(() => {});
  await page.waitForTimeout(1500);
  await shot(page, "studio-checks");
  const mech = await browser.newPage({ viewport: { width: 1600, height: 1000 } });
  await open(mech, urls.mech, 8000);
  await mech.keyboard.press("4");
  await mech.waitForTimeout(25000);
  await shot(mech, "studio-bodies");
}

async function apps(browser: import("playwright").Browser): Promise<void> {
  const desktop = await browser.newPage({ viewport: { width: 1280, height: 800 } });
  await open(desktop, urls.desktop);
  await shot(desktop, "desktop-onboarding");
  await open(desktop, `${urls.desktop}/?skip-onboarding`);
  await shot(desktop, "desktop");
  const phone = await browser.newPage({
    viewport: { width: 390, height: 844 },
    deviceScaleFactor: 2,
  });
  await open(phone, `${urls.mobile}/?mock=1`);
  await shot(phone, "mobile-hosts");
  await open(phone, `${urls.mobile}/host?id=${MOBILE_HOST}`, 2500);
  await shot(phone, "mobile");
  await open(phone, `${urls.mobile}/host/board?id=${MOBILE_HOST}&board=main`, 5000);
  await shot(phone, "mobile-board");
  const site = await browser.newPage({ viewport: { width: 1440, height: 900 } });
  await open(site, urls.site, 3000);
  await shot(site, "site");
}

mkdirSync(out, { recursive: true });
const browser = await chromium.launch({ executablePath: process.env.CHROME_PATH });
await studio(browser);
await apps(browser);
await browser.close();
