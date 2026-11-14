import type { Content } from "./types";

type Landing = Pick<Content, "hero" | "demos" | "agents" | "suite" | "dev" | "closing" | "footer">;

export const deLanding: Landing = {
  hero: {
    title: "Prüfen Sie Ihre Platine vor der Bestellung.",
    sub:
      "Beschreiben Sie Ihre Platine in Code. Fusible prüft jede Regel und sperrt den Export, " +
      "solange ein Fehler bleibt.",
    note: "Kostenlos und Open Source (GPL-3.0). Kein Facile-Konto nötig.",
    download: "Herunterladen",
    copyLabel: "Installationsbefehl kopieren",
  },
  demos: {
    hero: {
      label: "Das Fusible-Studio sperrt einen Export, bis eine zu schmale Leiterbahn korrigiert ist",
      intact: "Sicherung intakt",
      blown: "Sicherung durch",
      check: "Alles prüfen",
      export: "Exportieren",
      checks: "Prüfungen",
      violation: "VBUS führt 2 A. Eine 0,2-mm-Bahn wird zu heiß, sie braucht 0,8 mm.",
      refused: "Export gesperrt: Die Sicherung ist durch.",
      views: ["Schaltplan", "PCB", "3D", "Gehäuse", "Teile"],
    },
    agent: {
      prompt: "Füge der Platine eine USB-Versorgung hinzu.",
      placeholder: "Nachricht an den Agenten",
      answer:
        "Erledigt. Die erste VBUS-Bahn war zu schmal, ich habe sie auf 0,8 mm verbreitert. Die Sicherung ist intakt.",
      blown: "Sicherung durch",
      intact: "Sicherung intakt",
    },
    title: "Nichts geht raus, solange die Sicherung nicht hält",
    check: {
      title: "Eine durchgebrannte Sicherung stoppt den Export",
      text: "Exit-Code 1, und keine Gerber-, STEP- oder Firmware-Datei wird geschrieben.",
    },
    firmware: {
      title: "Die Firmware folgt der Platine",
      text: "Verschieben Sie einen Pin im TSX, und die Rust-Konstanten folgen.",
    },
    cost: {
      title: "Kosten immer aktuell",
      text: "Bauteile, Drucke und Preise in einer einzigen Stückliste.",
      label: "Summe",
    },
    loop: {
      title: "Im Entwurf erkannt, nicht auf dem Labortisch",
      short: {
        title: "Kurzschlüsse erreichen die Fertigung nie",
        text: "Zwei Netze, die sich berühren, lösen die Sicherung aus, bevor etwas geroutet wird.",
      },
      route: { title: "Leiterbahnen routen sich selbst", text: "fusible route macht aus den Luftlinien Kupfer." },
      sync: {
        title: "Änderungen aus KiCad kommen zurück",
        text: "Verschieben Sie ein Bauteil in KiCad, fusible sync übernimmt es.",
      },
    },
    board: {
      title: "Pastille in 3D",
      text: "Die echte Pastille-Platine, mit Fusible entworfen. Tippen Sie auf In 3D erkunden und drehen Sie sie.",
      link: "Pastille ansehen",
      pcb: "3D-PCB",
      enclosure: "Gehäuse",
      presets: { iso: "iso", top: "oben", side: "seitlich" },
      auto: "Auto",
      explore: "In 3D erkunden",
      done: "Fertig",
      hint: "Ziehen zum Drehen · Pinch oder Scrollen zum Zoomen",
      loading: "3D-Geometrie wird geladen",
      failed: "3D-Vorschau nicht verfügbar",
      failedText: "Das 3D-Modell konnte nicht geladen werden. Laden Sie die Seite neu.",
      status: "Sicherung intakt · 0 DRC-Verstöße",
    },
  },
  agents: {
    title: "Agenten arbeiten an Ihren Platinen",
    text:
      "Fusible spricht MCP, also erhalten Bulle, Claude Code und Codex bei jeder Änderung das Urteil der Sicherung.",
  },
  suite: {
    description: "Fusible arbeitet mit drei Facile-Apps zusammen.",
    nuage: "Senden Sie die Fertigungsdaten mit einem Befehl ans Team.",
    bulle: "Ein Assistent, der Ihre Platine ändert und prüft.",
    pastille: "Eine echte Platine, mit Fusible gebaut.",
    suite: "Fusible gehört zur Facile-Suite.",
    suiteLink: "Alle Apps ansehen",
  },
  dev: {
    description:
      "Installieren Sie die Desktop-App oder nutzen Sie cargo, und binden Sie Bulle oder Ihre Agenten per MCP an.",
    mcp: "Bulle, Claude Code und Codex lesen das Urteil der Sicherung und ändern die Platine mit 15 Werkzeugen.",
    actionTitle: "GitHub Action",
    action: "Die Sicherung löst in der CI bei jedem Pull Request aus.",
    source: "GPL-3.0. Code lesen, Issue melden, Fix einreichen.",
  },
  closing: { title: "Ihre nächste Platine, geprüft vor der Fertigung." },
  footer: {
    tagline: "Prüfen Sie Ihre Platine, bevor Sie sie bestellen.",
    license: "GPL-3.0-Lizenz",
    github: "GitHub",
    crates: "crates.io",
    studio: "Facile Studio",
  },
};
