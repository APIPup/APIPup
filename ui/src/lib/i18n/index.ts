import { writable, derived } from 'svelte/store';
import en from './en.json';
import zh from './zh.json';
import ja from './ja.json';
import fr from './fr.json';
import de from './de.json';

type Locale = 'en' | 'zh' | 'ja' | 'fr' | 'de';
type Messages = Record<string, string>;

const messages: Record<Locale, Messages> = { en, zh, ja, fr, de };

export const locale = writable<Locale>('zh');

export const t = derived(locale, ($locale) => {
  const dict = messages[$locale];
  return (key: string): string => dict[key] ?? key;
});
