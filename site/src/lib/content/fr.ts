import type { Content } from "./types";
import { frDocs } from "./fr-docs";
import { frLanding } from "./fr-landing";

export const fr: Content = {
  locale: "fr",
  htmlLang: "fr",
  ogLocale: "fr_FR",
  prefix: "/fr",
  docsPrefix: "/fr",
  ui: {
    skip: "Aller au contenu",
    themeToLight: "Passer au thème clair",
    themeToDark: "Passer au thème sombre",
    openMenu: "Ouvrir le menu de navigation",
    closeMenu: "Fermer le menu de navigation",
    language: "Langue",
    copy: "Copier",
    copied: "Copié",
    code: "Code",
    menuTitle: "Menu",
    primaryNav: "Principale",
  },
  nav: { features: "Fonctions", agents: "Agents", docs: "Docs", download: "Télécharger" },
  meta: {
    home: {
      title: "Fusible : vérifiez votre carte avant de la commander",
      description:
        "Décrivez votre carte électronique en code. Fusible vérifie chaque règle et bloque l'export tant " +
        "qu'une erreur reste. Gratuit et open source, sous GPL-3.0.",
      imageAlt: "Fusible : vérifiez votre carte avant de la commander",
    },
    download: {
      title: "Télécharger Fusible",
      description:
        "Téléchargez Fusible pour Windows, macOS, Linux et Android, ou installez l'outil en ligne de commande " +
        "avec cargo. Gratuit et open source sous licence GPL-3.0.",
      imageAlt: "Télécharger Fusible",
    },
    docs: {
      title: "Documentation Fusible",
      description:
        "Installez Fusible, écrivez le fichier de projet, lisez les règles et normes, créez des pièces " +
        "mécaniques, branchez des agents MCP et associez votre téléphone.",
      imageAlt: "Documentation Fusible",
    },
  },
  ...frLanding,
  download: {
    title: "Télécharger Fusible",
    intro: "Le studio de bureau, l'application Android en lecture seule et l'outil en ligne de commande.",
    heading: "Installer Fusible",
    note:
      "Les installateurs ne sont pas signés avec des certificats Apple ou Microsoft, " +
      "donc votre système vous avertira. " +
      "Les mises à jour intégrées sont vérifiées avec la clé de signature de Fusible.",
    cli: [
      { label: "Cargo", command: "cargo install fusible" },
      { label: "CLI Facile", command: "facile install fusible" },
    ],
    actionLine: "GitHub Action :",
  },
  docs: frDocs,
};
