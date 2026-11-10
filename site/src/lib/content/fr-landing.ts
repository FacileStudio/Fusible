import type { Content } from "./types";
import { ARXIV, BULLE_SNIPPET, SOURCES, SUITE_LINKS } from "./shared";

type Landing = Pick<
  Content,
  | "hero"
  | "loop"
  | "demos"
  | "rules"
  | "agents"
  | "board3d"
  | "bodies"
  | "mobile"
  | "desktop"
  | "suite"
  | "footer"
>;

export const frLanding: Landing = {
  hero: {
    title: "Un seul projet pour vos cartes, votre firmware et votre boîtier.",
    sub:
      "Fusible vérifie toute la conception selon des règles électriques, physiques et mécaniques. " +
      "Quand une règle échoue, le fusible saute : rien n'est exporté, généré ni flashé.",
    download: "Télécharger",
    copyLabel: "Copier la commande d'installation",
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
  demos: {
    hero: {
      label: "Le studio Fusible bloque un export jusqu'à ce qu'une piste trop fine soit corrigée",
      intact: "Fusible intact",
      blown: "Fusible grillé",
      check: "Tout vérifier",
      export: "Exporter",
      checks: "Contrôles",
      violation: "VBUS transporte 2 A. Une piste de 0,2 mm chauffe trop, il lui faut 0,8 mm.",
      refused: "Export bloqué : le fusible est grillé.",
      views: ["Schéma", "PCB", "3D", "Boîtiers", "Pièces"],
    },
    agent: {
      prompt: "Ajoute l'alimentation USB à la carte.",
      placeholder: "Écrire à l'agent",
      answer: "C'est fait. La première piste VBUS était trop fine, je l'ai élargie à 0,8 mm. Le fusible est intact.",
      blown: "Fusible grillé",
      intact: "Fusible intact",
    },
    title: "Rien ne part tant que le fusible ne tient pas",
    intro: "Le même verdict dans le terminal, dans votre firmware et dans vos coûts.",
    check: {
      title: "Un fusible grillé arrête l'export",
      text: "Code de sortie 1, et aucun fichier Gerber, STEP ou firmware n'est écrit.",
    },
    firmware: {
      title: "Le firmware suit la carte",
      text: "Déplacez une broche dans le TSX, les constantes Rust typées suivent à la prochaine compilation.",
    },
    cost: {
      title: "Des coûts toujours à jour",
      text: "Composants, impressions et prix se retrouvent dans une seule nomenclature.",
      label: "Total",
    },
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
      {
        id: "decoupling_capacitor",
        summary:
          "Chaque broche d'alimentation d'un circuit intégré a un condensateur de découplage, " +
          "à 3 mm au plus quand le placement est connu.",
        standard: { label: "guide de placement", href: SOURCES.bypass },
      },
      {
        id: "mech.wall_thickness",
        summary: "Les parois respectent le minimum du procédé d'impression ou de moulage.",
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
  board3d: {
    title: "Voyez la carte telle qu'elle sera fabriquée",
    text:
      "Le schéma, le PCB et la 3D se mettent à jour à chaque enregistrement, " +
      "avec des pièces inspectables pastille par pastille.",
    alt: "Le studio Fusible affichant un PCB rond en 3D",
  },
  bodies: {
    title: "Boîtiers et pièces en code",
    text:
      "Écrivez vos pièces mécaniques en TypeScript, choisissez matière et procédé, " +
      "et obtenez un vrai fichier STEP, la masse et le coût.",
    alt: "Le studio Fusible affichant un boîtier généré en 3D",
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
      "Les mises à jour sont vérifiées et s'installent en arrière-plan.",
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
  footer: { license: "Licence GPL-3.0", github: "GitHub", crates: "crates.io", studio: "Facile Studio" },
};
