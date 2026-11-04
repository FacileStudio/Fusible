import type { Content } from "./types";
import { frDocs } from "./fr-docs";
import { frLanding } from "./fr-landing";

export const fr: Content = {
  locale: "fr",
  htmlLang: "fr",
  ogLocale: "fr_FR",
  prefix: "/fr",
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
  nav: { features: "Fonctions", agents: "Agents", mobile: "Mobile", docs: "Docs", download: "Télécharger" },
  meta: {
    home: {
      title: "Fusible : cartes, firmware et boîtiers dans un seul projet vérifié",
      description:
        "Fusible vérifie cartes, firmware et boîtiers selon des règles électriques, physiques et mécaniques. " +
        "Quand le fusible saute, rien n'est exporté ni flashé.",
      imageAlt: "Fusible : un seul projet pour vos cartes, votre firmware et votre boîtier",
    },
    download: {
      title: "Télécharger Fusible",
      description:
        "Téléchargez Fusible pour Windows, macOS, Linux et Android, ou installez l'outil en ligne de commande " +
        "avec cargo. Gratuit et open source sous licence MIT.",
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
    intro:
      "Choisissez votre système. Chaque bouton fonctionne aussi sans données de version : " +
      "il ouvre la dernière version sur GitHub.",
    platforms: [
      { key: "windows", title: "Windows", hint: "Installeur (.msi ou .exe)" },
      { key: "macos", title: "macOS", hint: "Image disque universelle (.dmg)" },
      { key: "linux", title: "Linux", hint: "AppImage, .deb ou .rpm" },
      { key: "android", title: "Android", hint: "Application compagnon en lecture seule (.apk)" },
    ],
    button: "Télécharger",
    checksum: "SHA-256",
    allReleases: "Toutes les versions sur GitHub",
    versionLabel: "Dernière version",
    cliTitle: "Ligne de commande",
    cliLines: ["cargo install fusible", "facile install fusible"],
    actionLine: "GitHub Action : uses: FacileStudio/Fusible@v1",
    androidNote: "Autorisez les installations depuis cette source quand Android le demande.",
    desktopNote: "Les installeurs ne sont pas encore signés ; votre système demandera une confirmation.",
  },
  docs: frDocs,
};
