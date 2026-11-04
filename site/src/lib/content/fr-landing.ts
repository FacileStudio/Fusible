import type { Content } from "./types";
import { ARXIV, BULLE_SNIPPET, SOURCES, SUITE_LINKS } from "./shared";

type Landing = Pick<
  Content,
  "hero" | "loop" | "pillars" | "rules" | "agents" | "mobile" | "desktop" | "suite" | "footer"
>;

export const frLanding: Landing = {
  hero: {
    title: "Un seul projet pour vos cartes, votre firmware et votre boîtier.",
    sub:
      "Fusible vérifie toute la conception selon des règles électriques, physiques et mécaniques. " +
      "Quand une règle échoue, le fusible saute : rien n'est exporté, généré ni flashé.",
    download: "Télécharger",
    copyLabel: "Copier la commande d'installation",
    shotAlt: "Le studio Fusible affichant un circuit imprimé et ses vérifications",
  },
  loop: {
    title: "La boucle du fusible",
    steps: [
      {
        icon: "solar:pen-new-square-linear",
        title: "Concevoir",
        text: "Écrivez vos cartes en TSX et vos pièces en code.",
      },
      {
        icon: "solar:shield-check-linear",
        title: "Vérifier",
        text: "Le fusible applique chaque règle à chaque carte.",
      },
      {
        icon: "solar:danger-triangle-linear",
        title: "Sauté",
        text: "Chaque violation nomme sa règle, sa norme et une correction.",
      },
      {
        icon: "solar:box-minimalistic-linear",
        title: "Livrer",
        text: "Gerbers, STEP, nomenclature et broches du firmware, seulement quand le fusible est intact.",
      },
    ],
  },
  pillars: {
    title: "Un seul espace, de la carte au boîtier",
    items: [
      {
        icon: "solar:cpu-bolt-bold-duotone",
        title: "Cartes",
        text: "TSX tscircuit, avec vues PCB, schéma et 3D en direct.",
      },
      {
        icon: "solar:bolt-circle-bold-duotone",
        title: "Règles physiques",
        text: "Courant IPC-2152, isolement IPC-2221B et capacités de fabrication.",
      },
      {
        icon: "solar:code-square-bold-duotone",
        title: "Firmware",
        text: "Broches Rust typées pour Embassy et ESP-IDF, bloquées quand la carte est fausse.",
      },
      {
        icon: "solar:box-bold-duotone",
        title: "Mécanique",
        text: "Pièces écrites en TypeScript sur OpenCascade, vrai STEP et vérifications selon le procédé.",
      },
      {
        icon: "solar:bill-list-bold-duotone",
        title: "Nomenclature",
        text: "Prix, choix de pièces, matières, masse et coût d'impression.",
      },
      {
        icon: "solar:layers-linear",
        title: "Fabrication et KiCad",
        text: "Gerbers, fichiers de perçage, STEP, GLB et aller-retour avec KiCad.",
      },
    ],
  },
  rules: {
    title: "Des règles qui citent leur source",
    intro: "Chaque violation porte l'identifiant de la règle, la norme d'origine et un lien vers la source.",
    items: [
      {
        id: "trace_current",
        summary: "Une piste supporte son courant déclaré sans dépasser l'échauffement admis.",
        standard: { label: "IPC-2152", href: SOURCES.ipc2152 },
      },
      {
        id: "voltage_clearance",
        summary: "Le cuivre de réseaux différents respecte l'isolement que sa tension exige.",
        standard: { label: "IPC-2221B", href: SOURCES.ipc2221b },
      },
      {
        id: "fab_profile",
        summary: "Pistes, perçages, vias et taille de carte respectent les limites du fabricant.",
        standard: { label: "JLCPCB 2/4 couches", href: SOURCES.jlcpcb },
      },
      { id: "trace_clearance", summary: "Le cuivre de réseaux différents garde un espacement minimal." },
      { id: "shorted_nets", summary: "Aucune piste ne relie des réseaux de noms différents, comme GND et 3V3." },
      { id: "unconnected_net", summary: "Chaque réseau relie au moins deux pastilles ou ports." },
      { id: "pin_mux", summary: "Chaque broche gère la fonction de périphérique demandée sur la puce choisie." },
      {
        id: "bypass_cap",
        summary:
          "Chaque broche d'alimentation d'un circuit intégré a un condensateur de découplage, " +
          "à 3 mm au plus quand le placement est connu.",
        standard: { label: "guide de placement", href: SOURCES.bypass },
      },
      { id: "body_interference", summary: "Un corps ne chevauche ni la carte, ni un composant, ni un autre corps." },
      {
        id: "mech.wall_thickness",
        summary: "Les parois respectent le minimum du procédé d'impression ou de moulage.",
        standard: { label: "DFAM Academy", href: SOURCES.dfam },
      },
      {
        id: "mech.standoff_height",
        summary: "Les entretoises dépassent d'au moins 0,5 mm la pièce la plus haute sous la carte.",
        standard: { label: "DFAM Academy", href: SOURCES.dfam },
      },
      {
        id: "mech.draft",
        summary: "Les pièces moulées par injection déclarent une dépouille d'au moins 0,5 degré.",
        standard: { label: "RTP Company", href: SOURCES.rtp },
      },
    ],
    disclaimer:
      "Vérifications de conception, pas des certifications. " +
      "Un produit soumis à des normes de sécurité passe encore par un laboratoire.",
  },
  agents: {
    title: "Laissez des agents travailler sur vos cartes.",
    text:
      "Fusible parle MCP. Bulle, Claude Code et Codex peuvent vérifier, modifier et construire, " +
      "et chaque modification renvoie le verdict du fusible.",
    evidence:
      "Les recherches sur les agents PCB montrent que les agents en boucle fermée, contrôlés par des règles, " +
      "font mieux qu'une génération en un seul coup.",
    sources: ARXIV,
    snippetLabel: "Configuration de Bulle, ~/.config/bulle/config.yml",
    snippet: BULLE_SNIPPET,
  },
  mobile: {
    title: "Votre projet dans la poche",
    text:
      "Une vue en lecture seule des cartes, des vérifications et de la liste de pièces. " +
      "Associez-la avec un QR code sur votre réseau local.",
    alt: "L'application Android Fusible affichant le résumé d'une carte",
    cta: { label: "Télécharger pour Android", href: "/download#android" },
  },
  desktop: {
    title: "Une application de bureau",
    text:
      "Projets, outils et mises à jour dans une seule fenêtre. " +
      "Les mises à jour sont signées et s'installent en arrière-plan.",
    alt: "L'application de bureau Fusible listant des projets",
    cta: { label: "Télécharger l'application de bureau", href: "/download" },
  },
  suite: {
    title: "Fait partie de Facile Studio",
    text: "Fusible travaille avec les autres applications Facile.",
    links: [
      { label: "Facile Studio", href: SUITE_LINKS.facile },
      { label: "Bulle, agents", href: SUITE_LINKS.bulle },
      { label: "Nuage, partager les fichiers de fabrication", href: SUITE_LINKS.nuage },
      { label: "Opus, tâches issues des fusibles sautés", href: SUITE_LINKS.opus },
    ],
  },
  footer: { license: "Licence MIT", github: "GitHub", crates: "crates.io", studio: "Facile Studio" },
};
