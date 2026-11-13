import type { Content } from "./types";
import { enDocs } from "./en-docs";
import { enLanding } from "./en-landing";

export const en: Content = {
  locale: "en",
  htmlLang: "en",
  ogLocale: "en_US",
  prefix: "",
  ui: {
    skip: "Skip to content",
    themeToLight: "Switch to light theme",
    themeToDark: "Switch to dark theme",
    openMenu: "Open navigation menu",
    closeMenu: "Close navigation menu",
    language: "Language",
    copy: "Copy",
    copied: "Copied",
    code: "Code",
    menuTitle: "Menu",
    primaryNav: "Primary",
  },
  nav: { features: "Features", agents: "Agents", docs: "Docs", download: "Download" },
  meta: {
    home: {
      title: "Fusible · Boards, firmware and enclosures, verified",
      description:
        "Fusible checks boards, firmware and enclosures against electrical, physical and mechanical rules. " +
        "When the fuse blows, nothing gets exported or flashed.",
      imageAlt: "Fusible: Boards, firmware and enclosures, verified",
    },
    download: {
      title: "Download Fusible",
      description:
        "Download Fusible for Windows, macOS, Linux and Android, or install the command line tool with cargo. " +
        "Free and open source under GPL-3.0, with signed updates.",
      imageAlt: "Download Fusible",
    },
    docs: {
      title: "Fusible docs",
      description:
        "Install Fusible, write the project file, read the rules and their standards, build mechanical parts, " +
        "connect agents over MCP and pair your phone read-only.",
      imageAlt: "Fusible documentation",
    },
  },
  ...enLanding,
  download: {
    title: "Download Fusible",
    intro: "The desktop studio, the Android viewer and the command line tool.",
    heading: "Get Fusible",
    note:
      "Installers are not code-signed with Apple or Microsoft certificates, so your OS will warn you. " +
      "In-app updates are verified against Fusible's own signing key.",
    cli: [
      { label: "Cargo", command: "cargo install fusible" },
      { label: "Facile CLI", command: "facile install fusible" },
    ],
    actionLine: "GitHub Action: uses: FacileStudio/Fusible@v1",
  },
  docs: enDocs,
};
