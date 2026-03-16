import * as z from 'zod';
import { getEndpoint } from '$lib/api/api';

export const ACTION_UPDATE_BINARY_ENDPOINT = '/action/update_binary';
export const ACTION_UPDATE_BINARY_ALL_ENDPOINT = '/action/update_binary_all';

export const ActionUpdateBinaryRequest = z.object({
  robot_id: z.string().min(1),
  artifact_url: z.string().url(),
});
export type ActionUpdateBinaryRequest = z.infer<typeof ActionUpdateBinaryRequest>;

export const ActionUpdateBinaryAllRequest = z.object({
  artifact_url: z.string().url(),
});
export type ActionUpdateBinaryAllRequest = z.infer<typeof ActionUpdateBinaryAllRequest>;

export const ActionUpdateBinaryResponse = z.object({
  status: z.string(),
  message: z.string(),
});
export type ActionUpdateBinaryResponse = z.infer<typeof ActionUpdateBinaryResponse>;

export async function actionUpdateBinary(
  trackedFetch: typeof fetch,
  request: ActionUpdateBinaryRequest,
): Promise<ActionUpdateBinaryResponse> {
  const body = ActionUpdateBinaryRequest.parse(request);

  const response = await trackedFetch(getEndpoint(ACTION_UPDATE_BINARY_ENDPOINT), {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(body),
    signal: AbortSignal.timeout(30000),
  });

  if (!response.ok) {
    throw new Error(`Error updating binary: ${response.status} ${response.statusText}`);
  }

  const data = await response.json();
  return ActionUpdateBinaryResponse.parse(data);
}

export async function actionUpdateBinaryAll(
  trackedFetch: typeof fetch,
  request: ActionUpdateBinaryAllRequest,
): Promise<ActionUpdateBinaryResponse> {
  const body = ActionUpdateBinaryAllRequest.parse(request);

  const response = await trackedFetch(getEndpoint(ACTION_UPDATE_BINARY_ALL_ENDPOINT), {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(body),
    signal: AbortSignal.timeout(60000),
  });

  if (!response.ok) {
    throw new Error(`Error updating all binaries: ${response.status} ${response.statusText}`);
  }

  const data = await response.json();
  return ActionUpdateBinaryResponse.parse(data);
}
