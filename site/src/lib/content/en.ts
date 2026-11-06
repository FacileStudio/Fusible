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
  nav: { features: "Features", agents: "Agents", mobile: "Mobile", docs: "Docs", download: "Download" },
  meta: {
    home: {
      title: "Fusible: boards, firmware and enclosures in one checked project",
      description:
        "Fusible checks boards, firmware and enclosures against electrical, physical and mechanical rules. " +
        "When the fuse blows, nothing is exported, built or flashed.",
      imageAlt: "Fusible: one project for your boards, firmware and enclosure",
    },
    download: {
      title: "Download Fusible",
      description:
        "Download Fusible for Windows, macOS, Linux and Android, or install the command line tool with cargo. " +
        "Free and open source under the MIT license, verified updates.",
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
    intro: "Pick your system. Every button also works without release data: it opens the latest release on GitHub.",
    platforms: [
      { key: "windows", title: "Windows", hint: "Installer (.msi or .exe)" },
      { key: "macos", title: "macOS", hint: "Universal disk image (.dmg)" },
      { key: "linux", title: "Linux", hint: "AppImage, .deb or .rpm" },
      { key: "android", title: "Android", hint: "Read-only companion app (.apk)" },
    ],
    button: "Download",
    checksum: "SHA-256",
    allReleases: "All releases on GitHub",
    versionLabel: "Latest release",
    cliTitle: "Command line",
    cliLines: ["cargo install fusible", "facile install fusible"],
    actionLine: "GitHub Action: uses: FacileStudio/Fusible@v1",
    androidNote: "Allow installs from this source when Android asks.",
    desktopNote: "Installers are not code-signed with Apple or Microsoft certificates, so your OS will warn you. " +
      "In-app updates are verified against Fusible's own signing key.",
  },
  docs: enDocs,
};
