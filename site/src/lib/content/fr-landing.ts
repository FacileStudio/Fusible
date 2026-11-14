import type { Content } from "./types";

type Landing = Pick<Content, "hero" | "demos" | "agents" | "closing" | "footer">;

export const frLanding: Landing = {
  hero: {
    title: "Un seul projet pour vos cartes, votre firmware et votre boîtier.",
    sub: "Quand une règle échoue, le fusible saute et rien ne part.",
    download: "Télécharger",
    copyLabel: "Copier la commande d'installation",
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
    check: {
      title: "Un fusible grillé arrête l'export",
      text: "Code de sortie 1, et aucun fichier Gerber, STEP ou firmware n'est écrit.",
    },
    firmware: {
      title: "Le firmware suit la carte",
      text: "Déplacez une broche dans le TSX, les constantes Rust suivent.",
    },
    cost: {
      title: "Des coûts à jour",
      text: "Composants, impressions et prix dans une seule nomenclature.",
      label: "Total",
    },
    loop: {
      title: "Repéré dans le schéma, pas sur le banc",
      short: {
        title: "Les courts-circuits n'atteignent pas la fab",
        text: "Deux nets qui se touchent font sauter le fusible avant tout routage.",
      },
      route: { title: "Les pistes se routent seules", text: "fusible route transforme le chevelu en cuivre." },
      sync: {
        title: "Les retouches KiCad reviennent",
        text: "Déplacez un composant dans KiCad, fusible sync le reporte.",
      },
    },
    board: {
      title: "Pastille, en 3D",
      text: "La vraie carte Pastille, conçue avec Fusible. Touchez Explorer en 3D pour la tourner.",
      presets: { iso: "Iso", top: "Dessus", side: "Côté" },
      auto: "Auto",
      explore: "Explorer en 3D",
      done: "Terminé",
      hint: "Glissez pour tourner, pincez ou défilez pour zoomer",
      loading: "Chargement de la carte 3D",
      failed: "Aperçu 3D indisponible",
      failedText: "Ce navigateur n'a pas pu démarrer WebGL.",
      status: "Fusible intact, 0 violation",
    },
  },
  agents: {
    title: "Des agents sur vos cartes",
    text: "Fusible parle MCP : Bulle, Claude Code et Codex reçoivent le verdict du fusible à chaque modification.",
  },
  closing: { title: "Vérifiez votre prochaine carte avant de la commander." },
  footer: { license: "Licence GPL-3.0", github: "GitHub", crates: "crates.io", studio: "Facile Studio" },
};
