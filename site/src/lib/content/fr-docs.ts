import type { DocsContent } from "./types";
import { BULLE_SNIPPET, CODEX_SNIPPET, MANIFEST_YAML } from "./shared";
import { frTools } from "./fr-tools";

export const frDocs: DocsContent = {
  title: "Documentation Fusible",
  intro: "Tout ce qu'il faut pour lancer votre premier projet vérifié.",
  toc: "Sur cette page",
  sections: [
    {
      id: "install",
      title: "Installation",
      blocks: [
        { kind: "code", code: "cargo install fusible\nfacile install fusible" },
        {
          kind: "p",
          text:
            "Vérifier du Circuit JSON, des Gerbers et du SVG ne demande rien de plus. " +
            "Pour compiler des cartes .tsx ou exporter du STEP, installez tscircuit et contrôlez les outils.",
        },
        { kind: "code", code: "bun add -g tscircuit\nfusible doctor\nfusible doctor --install" },
        {
          kind: "p",
          text:
            "L'application de bureau pour Windows, macOS et Linux est jointe à chaque version GitHub. " +
            "Les installateurs ne sont pas signés avec des certificats Apple ou Microsoft : votre système vous avertira. " +
          "Les mises à jour intégrées sont vérifiées avec la clé de signature de Fusible.",
        },
      ],
    },
    {
      id: "quick-start",
      title: "Démarrage rapide",
      blocks: [
        { kind: "code", code: "fusible init --name sensor-node\nfusible check\nfusible studio" },
        {
          kind: "p",
          text:
            "init écrit un fusible.yml. check vérifie chaque carte. " +
            "studio ouvre la vue en direct.",
        },
        {
          kind: "p",
          text:
            "Codes de sortie : 0 fusible intact, 1 fusible sauté, 2 erreur d'outil. " +
            "fusible check --json affiche le verdict et chaque diagnostic en JSON.",
        },
      ],
    },
    {
      id: "project-file",
      title: "Le fichier de projet",
      blocks: [
        { kind: "code", code: MANIFEST_YAML },
        {
          kind: "list",
          items: [
            "boards, firmware et bodies sont des éléments nommés. Les noms sont uniques entre les trois.",
            "nets déclare la tension et le courant d'un réseau. Les règles physiques le lisent.",
            "fab choisit un profil de fabricant : jlcpcb-2l, jlcpcb-4l, ou vos propres limites.",
            "clearance_class vaut B1, B2 ou B4. La valeur par défaut est B2.",
            "copper fixe l'épaisseur de cuivre : outer_oz et inner_oz.",
          ],
        },
      ],
    },
    {
      id: "rules",
      title: "Règles",
      blocks: [
        {
          kind: "table",
          head: ["Règle", "Norme"],
          rows: [
            ["trace_current", "IPC-2152"],
            ["voltage_clearance", "IPC-2221B (classes B1, B2, B4)"],
            ["fab_profile", "JLCPCB 2 couches et 4 couches"],
            ["trace_clearance, shorted_nets, unconnected_net, pin_mux", "Contrôles de topologie et de puce"],
            ["bypass_cap", "Guide de placement, 3 mm"],
            ["body_interference", "Géométrie de la carte et des corps"],
            ["mech.wall_thickness, mech.standoff_height, mech.draft", "DFAM Academy, RTP Company"],
          ],
        },
        {
          kind: "p",
          text:
            "Vérifications de conception, pas des certifications. " +
            "Un produit soumis à des normes de sécurité passe encore par un laboratoire.",
        },
      ],
    },
    {
      id: "mechanical",
      title: "Pièces mécaniques",
      blocks: [
        { kind: "code", code: "fusible body new bracket --material petg --process fdm\nfusible body bracket" },
        {
          kind: "p",
          text:
            "body new écrit mech/bracket.ts, mech/fusible.d.ts et une entrée bodies dans fusible.yml. " +
            "Construire une pièce en code écrit du STEP, du STL et du GLB. " +
            "Les procédés sont fdm, sla, mjf, injection et cnc.",
        },
        {
          kind: "p",
          text:
            "Une pièce est une fonction TypeScript qui reçoit la carte et un objet Cad. " +
            "Cad propose box, cylinder, extrude, union, cut, intersect, move, rotate, shell, boss, " +
            "standoff, ventSlots, cutout et draft. Utilisez ces aides pour que les règles s'appliquent.",
        },
      ],
    },
    {
      id: "agents",
      title: "Agents",
      blocks: [
        {
          kind: "p",
          text:
            "fusible mcp expose ces outils sur stdio. " +
            "Chaque écriture renvoie le diff et le verdict du fusible.",
        },
        { kind: "table", head: ["Outil", "Rôle"], rows: frTools },
        {
          kind: "code",
          code: "fusible mcp --print-config bulle\nfusible mcp --print-config claude\nfusible mcp --print-config codex",
        },
        { kind: "code", code: BULLE_SNIPPET },
        { kind: "code", code: "claude mcp add fusible -- fusible mcp" },
        { kind: "code", code: CODEX_SNIPPET },
      ],
    },
    {
      id: "phone",
      title: "Association du téléphone",
      blocks: [
        { kind: "code", code: "fusible studio --share" },
        {
          kind: "p",
          text:
            "Appuyez sur Pair phone dans le studio et scannez le QR code avec l'application Android. " +
            "Les deux appareils doivent être sur le même réseau privé. La vue du téléphone est en lecture seule.",
        },
      ],
    },
    {
      id: "suite",
      title: "Suite Facile",
      blocks: [
        { kind: "code", code: "fusible check --opus\nfusible bom --opus" },
        {
          kind: "p",
          text:
            "Un fusible sauté ou une pièce non achetée devient une tâche Opus. " +
            "Le côté Opus ne change jamais le verdict.",
        },
        { kind: "code", code: "fusible share nuage --space <name> --preset fab --link --expires 30d" },
        {
          kind: "p",
          text: "L'export arrive dans un espace Nuage synchronisé. Rien n'est copié tant que le fusible est sauté.",
        },
      ],
    },
  ],
};
