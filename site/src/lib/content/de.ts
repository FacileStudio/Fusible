import type { Content } from "./types";
import { enDocs } from "./en-docs";
import { deLanding } from "./de-landing";

export const de: Content = {
  locale: "de",
  htmlLang: "de",
  ogLocale: "de_DE",
  prefix: "/de",
  docsPrefix: "",
  ui: {
    skip: "Zum Inhalt springen",
    themeToLight: "Zum hellen Design wechseln",
    themeToDark: "Zum dunklen Design wechseln",
    openMenu: "Navigationsmenü öffnen",
    closeMenu: "Navigationsmenü schließen",
    language: "Sprache",
    copy: "Kopieren",
    copied: "Kopiert",
    code: "Code",
    menuTitle: "Menü",
    primaryNav: "Hauptnavigation",
  },
  nav: { features: "Funktionen", agents: "Agenten", docs: "Doku", download: "Download" },
  meta: {
    home: {
      title: "Fusible · Prüfen Sie Ihre Platine vor der Bestellung",
      description:
        "Beschreiben Sie Ihre Platine in Code. Fusible prüft jede Regel und sperrt den Export, solange ein " +
        "Fehler bleibt. Kostenlos, Open Source, unter GPL-3.0.",
      imageAlt: "Fusible: Prüfen Sie Ihre Platine vor der Bestellung",
    },
    download: {
      title: "Fusible herunterladen",
      description:
        "Laden Sie Fusible für Windows, macOS, Linux und Android herunter, oder installieren Sie das " +
        "Kommandozeilen-Tool mit cargo. Kostenlos und Open Source (GPL-3.0).",
      imageAlt: "Fusible herunterladen",
    },
    docs: {
      title: "Fusible-Dokumentation",
      description:
        "Installieren Sie Fusible, schreiben Sie die Projektdatei, lesen Sie Regeln und Normen, bauen Sie " +
        "Mechanikteile und binden Sie Ihre Agenten per MCP an.",
      imageAlt: "Fusible-Dokumentation",
    },
  },
  ...deLanding,
  download: {
    title: "Fusible herunterladen",
    intro: "Das Desktop-Studio, der Android-Viewer und das Kommandozeilen-Tool.",
    heading: "Fusible installieren",
    note:
      "Die Installer sind nicht mit Apple- oder Microsoft-Zertifikaten signiert, daher warnt Ihr System. " +
      "Updates in der App werden mit Fusibles eigenem Signaturschlüssel geprüft.",
    cli: [
      { label: "Cargo", command: "cargo install fusible" },
      { label: "Facile CLI", command: "facile install fusible" },
    ],
    actionLine: "GitHub Action:",
  },
  docs: enDocs,
};
