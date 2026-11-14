import type { Content } from "./types";

type Landing = Pick<Content, "hero" | "demos" | "agents" | "suite" | "dev" | "closing" | "footer">;

export const esLanding: Landing = {
  hero: {
    title: "Revisa tu placa antes de pedirla.",
    sub:
      "Describe tu placa electrónica en código. Fusible revisa cada regla y bloquea la exportación " +
      "mientras quede un error.",
    note: "Gratis y de código abierto (GPL-3.0). No necesitas cuenta de Facile.",
    download: "Descargar",
    copyLabel: "Copiar el comando de instalación",
  },
  demos: {
    hero: {
      label: "El estudio Fusible bloquea una exportación hasta corregir una pista demasiado fina",
      intact: "Fusible intacto",
      blown: "Fusible fundido",
      check: "Revisar todo",
      export: "Exportar",
      checks: "Controles",
      violation: "VBUS lleva 2 A. Una pista de 0,2 mm se calienta, necesita 0,8 mm.",
      refused: "Exportación bloqueada: el fusible está fundido.",
      views: ["Esquema", "PCB", "3D", "Carcasas", "Piezas"],
    },
    agent: {
      prompt: "Añade la alimentación USB a la placa.",
      placeholder: "Escribe al agente",
      answer: "Hecho. La primera pista VBUS era demasiado fina, la he ensanchado a 0,8 mm. El fusible está intacto.",
      blown: "Fusible fundido",
      intact: "Fusible intacto",
    },
    title: "Nada sale hasta que el fusible aguanta",
    check: {
      title: "Un fusible fundido detiene la exportación",
      text: "Código de salida 1, y no se escribe ningún archivo Gerber, STEP ni firmware.",
    },
    firmware: {
      title: "El firmware sigue a la placa",
      text: "Mueve un pin en el TSX y las constantes Rust lo siguen.",
    },
    cost: {
      title: "Costes siempre al día",
      text: "Componentes, impresiones y precios en una sola lista de materiales.",
      label: "Total",
    },
    loop: {
      title: "Detectado en el diseño, no en el banco",
      short: {
        title: "Los cortocircuitos no llegan a fábrica",
        text: "Dos redes que se tocan funden el fusible antes de enrutar nada.",
      },
      route: { title: "Las pistas se enrutan solas", text: "fusible route convierte el ratsnest en cobre." },
      sync: {
        title: "Los cambios de KiCad vuelven",
        text: "Mueve un componente en KiCad y fusible sync lo trae de vuelta.",
      },
    },
    board: {
      title: "Pastille, en 3D",
      text: "La placa real de Pastille, diseñada con Fusible. Toca Explorar en 3D y arrastra para girarla.",
      link: "Ver Pastille",
      pcb: "PCB 3D",
      enclosure: "Carcasa",
      presets: { iso: "iso", top: "arriba", side: "lado" },
      auto: "Auto",
      explore: "Explorar en 3D",
      done: "Listo",
      hint: "Arrastra para girar · Pellizca o desplaza para hacer zoom",
      loading: "Cargando la geometría 3D",
      failed: "Vista 3D no disponible",
      failedText: "No se pudo cargar el modelo 3D. Recarga la página para intentarlo de nuevo.",
      status: "Fusible intacto · 0 errores DRC",
    },
  },
  agents: {
    title: "Agentes que trabajan en tus placas",
    text: "Fusible habla MCP, así que Bulle, Claude Code y Codex reciben el veredicto del fusible en cada cambio.",
  },
  suite: {
    description: "Fusible se conecta con tres apps de Facile.",
    nuage: "Envía los archivos de fabricación al equipo con un comando.",
    bulle: "Un asistente que edita y revisa tu placa.",
    pastille: "Una placa real diseñada con Fusible.",
    suite: "Fusible forma parte de la suite Facile.",
    suiteLink: "Ver todas las apps",
  },
  dev: {
    description: "Instala la app de escritorio o usa cargo, y conecta Bulle o tus agentes por MCP.",
    mcp: "Bulle, Claude Code y Codex leen el veredicto del fusible y editan la placa con 15 herramientas.",
    actionTitle: "GitHub Action",
    action: "El fusible salta en la CI en cada pull request.",
    source: "GPL-3.0. Lee el código, abre una incidencia, envía un arreglo.",
  },
  closing: { title: "Tu próxima placa, revisada antes de fábrica." },
  footer: {
    tagline: "Revisa tu placa electrónica antes de pedirla.",
    license: "Licencia GPL-3.0",
    github: "GitHub",
    crates: "crates.io",
    studio: "Facile Studio",
  },
};
