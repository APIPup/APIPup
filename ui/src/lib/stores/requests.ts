import { writable, derived } from 'svelte/store';
import type { HttpResponse } from '$lib/api/http';

export interface RequestItem {
  id: string;
  method: string;
  url: string;
  headers: Array<{ key: string; value: string; enabled: boolean }>;
  body: string;
}

function createId(): string {
  return crypto.randomUUID();
}

function createDefaultRequest(): RequestItem {
  return {
    id: createId(),
    method: 'GET',
    url: '',
    headers: [{ key: '', value: '', enabled: true }],
    body: ''
  };
}

export const requestList = writable<RequestItem[]>([createDefaultRequest()]);
export const activeRequestId = writable<string>('');
export const response = writable<HttpResponse | null>(null);
export const responseError = writable<string | null>(null);
export const loading = writable(false);

// Initialize activeRequestId to the first request
requestList.subscribe((list) => {
  if (list.length > 0) {
    activeRequestId.update((current) => {
      if (!current || !list.find((r) => r.id === current)) {
        return list[0].id;
      }
      return current;
    });
  }
});

export const activeRequest = derived(
  [requestList, activeRequestId],
  ([$list, $id]) => $list.find((r) => r.id === $id) ?? null
);

export function addRequest() {
  const newReq = createDefaultRequest();
  requestList.update((list) => [...list, newReq]);
  activeRequestId.set(newReq.id);
}

export function removeRequest(id: string) {
  requestList.update((list) => {
    const filtered = list.filter((r) => r.id !== id);
    return filtered.length === 0 ? [createDefaultRequest()] : filtered;
  });
}

export function updateRequest(id: string, updates: Partial<RequestItem>) {
  requestList.update((list) =>
    list.map((r) => (r.id === id ? { ...r, ...updates } : r))
  );
}

export function selectRequest(id: string) {
  activeRequestId.set(id);
  response.set(null);
  responseError.set(null);
}
