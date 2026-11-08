export type Locale = "en" | "fr";

export interface Link {
  label: string;
  href: string;
}

export interface Meta {
  title: string;
  description: string;
  imageAlt: string;
}

export interface Card {
  icon: string;
  title: string;
  text: string;
}

export interface RuleItem {
  id: string;
  summary: string;
  standard?: Link;
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
  mobile: string;
  docs: string;
  download: string;
}

export interface HeroContent {
  title: string;
  sub: string;
  download: string;
  copyLabel: string;
}

export interface LoopContent {
  title: string;
  steps: Card[];
}

export interface RulesContent {
  title: string;
  intro: string;
  items: RuleItem[];
  disclaimer: string;
}

export interface AgentsContent {
  title: string;
  text: string;
  evidence: string;
  sources: Link[];
  snippetLabel: string;
  snippet: string;
}

export interface ShotContent {
  title: string;
  text: string;
  alt: string;
  cta?: Link;
}

export interface SuiteContent {
  title: string;
  text: string;
  links: Link[];
}

export interface FooterContent {
  license: string;
  github: string;
  crates: string;
  studio: string;
}

export interface PlatformCard {
  key: "windows" | "macos" | "linux" | "android";
  title: string;
  hint: string;
}

export interface DownloadContent {
  title: string;
  intro: string;
  platforms: PlatformCard[];
  button: string;
  checksum: string;
  allReleases: string;
  versionLabel: string;
  cliTitle: string;
  cliLines: string[];
  actionLine: string;
  androidNote: string;
  desktopNote: string;
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
  intro: string;
  check: DemoTile;
  firmware: DemoTile;
  cost: DemoTile;
}

export interface Content {
  locale: Locale;
  demos: DemosContent;
  htmlLang: string;
  ogLocale: string;
  prefix: string;
  ui: Ui;
  nav: NavContent;
  meta: { home: Meta; download: Meta; docs: Meta };
  hero: HeroContent;
  loop: LoopContent;
  rules: RulesContent;
  agents: AgentsContent;
  board3d: ShotContent;
  bodies: ShotContent;
  mobile: ShotContent;
  desktop: ShotContent;
  suite: SuiteContent;
  footer: FooterContent;
  download: DownloadContent;
  docs: DocsContent;
}
