import { writable } from 'svelte/store';
import type { HttpResponse } from '$lib/api/http';

export interface EditablePair {
  key: string;
  value: string;
  enabled: boolean;
}

export interface RequestItem {
  file: string;
  path: string;
  method: string;
  url: string;
  queryParams: EditablePair[];
  pathParams: EditablePair[];
  cookieParams: EditablePair[];
  headers: EditablePair[];
  body: string;
  operation: Record<string, unknown>;
}

function readParameterPairs(operation: Record<string, unknown>, location: string): EditablePair[] {
  const parametersValue = operation.parameters;
  if (!Array.isArray(parametersValue)) {
    return [{ key: '', value: '', enabled: true }];
  }

  const pairs = parametersValue
    .filter((value) => {
      if (!value || typeof value !== 'object') {
        return false;
      }

      const parameter = value as Record<string, unknown>;
      return parameter.in === location && typeof parameter.name === 'string';
    })
    .map((value) => {
      const parameter = value as Record<string, unknown>;

      const exampleValue = parameter.example;
      const pairValue = typeof exampleValue === 'string'
        ? exampleValue
        : exampleValue != null
          ? JSON.stringify(exampleValue)
          : '';

      return {
        key: String(parameter.name),
        value: pairValue,
        enabled: true
      };
    });

  return pairs.length > 0 ? pairs : [{ key: '', value: '', enabled: true }];
}

function readBody(operation: Record<string, unknown>): string {
  const requestBodyValue = operation.requestBody;
  if (!requestBodyValue || typeof requestBodyValue !== 'object') {
    return '';
  }

  const requestBody = requestBodyValue as Record<string, unknown>;
  const contentValue = requestBody.content;
  if (!contentValue || typeof contentValue !== 'object') {
    return '';
  }

  const content = contentValue as Record<string, unknown>;
  const jsonContent = content['application/json'];
  if (!jsonContent || typeof jsonContent !== 'object') {
    return '';
  }

  const jsonBody = jsonContent as Record<string, unknown>;
  const example = jsonBody.example;

  if (typeof example === 'string') {
    return example;
  }

  if (example && typeof example === 'object') {
    return JSON.stringify(example, null, 2);
  }

  return '';
}

export function createRequestFromEndpoint(
  file: string,
  path: string,
  method: string,
  operation: Record<string, unknown>
): RequestItem {
  return {
    file,
    path,
    method,
    url: path,
    queryParams: readParameterPairs(operation, 'query'),
    pathParams: readParameterPairs(operation, 'path'),
    cookieParams: readParameterPairs(operation, 'cookie'),
    headers: readParameterPairs(operation, 'header'),
    body: readBody(operation),
    operation
  };
}

export const activeRequest = writable<RequestItem | null>(null);
export const response = writable<HttpResponse | null>(null);
export const responseError = writable<string | null>(null);
export const loading = writable(false);

export function selectRequest(request: RequestItem): void {
  activeRequest.set(request);
  response.set(null);
  responseError.set(null);
}

export function updateActiveRequest(updates: Partial<RequestItem>): void {
  activeRequest.update((current) => {
    if (!current) {
      return current;
    }

    return {
      ...current,
      ...updates
    };
  });
}
