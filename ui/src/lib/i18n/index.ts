import { writable, derived } from 'svelte/store';
import en from './en.json';
import zh from './zh.json';

type Locale = 'en' | 'zh';
type Messages = Record<string, string>;

const messages: Record<Locale, Messages> = { en, zh };

export const locale = writable<Locale>('zh');

export const t = derived(locale, ($locale) => {
  const dict = messages[$locale];
  return (key: string): string => dict[key] ?? key;
});
