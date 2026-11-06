import { createTranslator, getI18n, provideLocale } from "@facile/muse";
import { en, type Catalog } from "./i18n-en";
import { fr } from "./i18n-fr";

const catalogs = { en, fr, es: en, de: en };

export type Key = keyof Catalog;

/** Sets the locale from the phone language (`fr*` is French, anything else English). Call once in the root layout. */
export function initLocale() {
  provideLocale(() => (navigator.language.toLowerCase().startsWith("fr") ? "fr" : "en"));
}

/** A translator bound to the current locale. Call during component setup. */
export function useT() {
  const i18n = getI18n();
  const t = createTranslator(catalogs, () => i18n.locale);
  return (key: Key, params?: Record<string, string | number>) => t(key, params);
}
