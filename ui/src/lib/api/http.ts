import { invoke } from '@tauri-apps/api/core';

export interface HttpRequest {
  method: string;
  url: string;
  headers: Record<string, string>;
  body: string | null;
}

export interface HttpResponse {
  status: number;
  status_text: string;
  headers: Record<string, string>;
  body: string;
  elapsed_ms: number;
}

export async function sendRequest(request: HttpRequest): Promise<HttpResponse> {
  return invoke<HttpResponse>('send_request', { request });
}
