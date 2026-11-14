export type Locale = "en" | "fr" | "es" | "de";

export interface Link {
  label: string;
  href: string;
}

export interface Meta {
  title: string;
  description: string;
  imageAlt: string;
}

export type Block =
  | { kind: "p"; text: string }
  | { kind: "code"; code: string }
  | { kind: "list"; items: string[] }
  | { kind: "table"; head: string[]; rows: string[][] };

export interface DocSection {
  id: string;
  title: string;
  blocks: Block[];
}

export interface Ui {
  skip: string;
  themeToLight: string;
  themeToDark: string;
  openMenu: string;
  closeMenu: string;
  language: string;
  copy: string;
  copied: string;
  code: string;
  menuTitle: string;
  primaryNav: string;
}

export interface NavContent {
  features: string;
  agents: string;
  docs: string;
  download: string;
}

export interface HeroContent {
  title: string;
  sub: string;
  note: string;
  download: string;
  copyLabel: string;
}

export interface SuiteContent {
  description: string;
  nuage: string;
  bulle: string;
  pastille: string;
  suite: string;
  suiteLink: string;
}

export interface DevContent {
  description: string;
  mcp: string;
  actionTitle: string;
  action: string;
  source: string;
}

export interface AgentsContent {
  title: string;
  text: string;
}

export interface ClosingContent {
  title: string;
}

export interface FooterContent {
  tagline: string;
  license: string;
  github: string;
  crates: string;
  studio: string;
}

export interface DownloadContent {
  title: string;
  intro: string;
  heading: string;
  note: string;
  cli: { label: string; command: string }[];
  actionLine: string;
}

export interface DocsContent {
  title: string;
  intro: string;
  toc: string;
  sections: DocSection[];
}

export interface HeroDemoCopy {
  label: string;
  intact: string;
  blown: string;
  check: string;
  export: string;
  checks: string;
  violation: string;
  refused: string;
  views: string[];
}

export interface AgentDemoCopy {
  prompt: string;
  placeholder: string;
  answer: string;
  blown: string;
  intact: string;
}

export interface DemoTile {
  title: string;
  text: string;
  label?: string;
}

export interface DemosContent {
  hero: HeroDemoCopy;
  agent: AgentDemoCopy;
  title: string;
  check: DemoTile;
  firmware: DemoTile;
  cost: DemoTile;
  loop: LoopContent;
  board: BoardContent;
}

export interface LoopContent {
  title: string;
  short: DemoTile;
  route: DemoTile;
  sync: DemoTile;
}

export interface BoardContent {
  title: string;
  text: string;
  link: string;
  pcb: string;
  enclosure: string;
  presets: { iso: string; top: string; side: string };
  auto: string;
  explore: string;
  done: string;
  hint: string;
  loading: string;
  failed: string;
  failedText: string;
  status: string;
}

export interface Content {
  locale: Locale;
  demos: DemosContent;
  htmlLang: string;
  ogLocale: string;
  prefix: string;
  docsPrefix: string;
  ui: Ui;
  nav: NavContent;
  meta: { home: Meta; download: Meta; docs: Meta };
  hero: HeroContent;
  agents: AgentsContent;
  suite: SuiteContent;
  dev: DevContent;
  closing: ClosingContent;
  footer: FooterContent;
  download: DownloadContent;
  docs: DocsContent;
}
