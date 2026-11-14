import type { Content } from "./types";
import { enDocs } from "./en-docs";
import { esLanding } from "./es-landing";

export const es: Content = {
  locale: "es",
  htmlLang: "es",
  ogLocale: "es_ES",
  prefix: "/es",
  docsPrefix: "",
  ui: {
    skip: "Ir al contenido",
    themeToLight: "Cambiar al tema claro",
    themeToDark: "Cambiar al tema oscuro",
    openMenu: "Abrir el menú de navegación",
    closeMenu: "Cerrar el menú de navegación",
    language: "Idioma",
    copy: "Copiar",
    copied: "Copiado",
    code: "Código",
    menuTitle: "Menú",
    primaryNav: "Principal",
  },
  nav: { features: "Funciones", agents: "Agentes", docs: "Docs", download: "Descargar" },
  meta: {
    home: {
      title: "Fusible · Revisa tu placa antes de pedirla",
      description:
        "Describe tu placa electrónica en código. Fusible revisa cada regla y bloquea la exportación " +
        "mientras quede un error. Gratis y de código abierto (GPL-3.0).",
      imageAlt: "Fusible: revisa tu placa antes de pedirla",
    },
    download: {
      title: "Descargar Fusible",
      description:
        "Descarga Fusible para Windows, macOS, Linux y Android, o instala la herramienta de línea de comandos " +
        "con cargo. Gratis y de código abierto, bajo GPL-3.0.",
      imageAlt: "Descargar Fusible",
    },
    docs: {
      title: "Documentación de Fusible",
      description:
        "Instala Fusible, escribe el archivo de proyecto, lee las reglas y sus normas, crea piezas mecánicas, " +
        "conecta agentes por MCP y empareja tu móvil en lectura.",
      imageAlt: "Documentación de Fusible",
    },
  },
  ...esLanding,
  download: {
    title: "Descargar Fusible",
    intro: "El estudio de escritorio, el visor Android y la herramienta de línea de comandos.",
    heading: "Instalar Fusible",
    note:
      "Los instaladores no están firmados con certificados de Apple ni de Microsoft, así que tu sistema te avisará. " +
      "Las actualizaciones integradas se verifican con la clave de firma de Fusible.",
    cli: [
      { label: "Cargo", command: "cargo install fusible" },
      { label: "CLI de Facile", command: "facile install fusible" },
    ],
    actionLine: "GitHub Action:",
  },
  docs: enDocs,
};
