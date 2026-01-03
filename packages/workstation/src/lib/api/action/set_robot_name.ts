import * as z from 'zod';
import { getEndpoint } from '$lib/api/api';
import { STATS_ROBOT_ENDPOINT } from '$lib/api/stats/robot_uuid';
import { STATS_ROBOT_NETWORK_ENDPOINT } from '$lib/api/stats/robot_network';
import { STATS_ONLINE_ROBOTS_ENDPOINT } from '$lib/api/stats/online_robots';

export const ACTION_SET_ROBOT_NAME_ENDPOINT = '/action/set_robot_name';

export const ActionSetRobotNameRequest = z.object({
  robot_uuid: z.uuidv4(),
  new_robot_name: z.string().min(1),
});
export type ActionSetRobotNameRequest = z.infer<typeof ActionSetRobotNameRequest>;

export async function actionSetRobotName(
  trackedFetch: typeof fetch,
  request: ActionSetRobotNameRequest,
): Promise<void> {
  const body = ActionSetRobotNameRequest.parse(request);

  const response = await trackedFetch(getEndpoint(ACTION_SET_ROBOT_NAME_ENDPOINT), {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(body),
    signal: AbortSignal.timeout(5000),
  });

  if (!response.ok) {
    throw new Error(`Error setting robot name: ${response.status} ${response.statusText}`);
  }
}

export const ACTION_SET_ROBOT_NAME_INVALIDATE_KEYS = (robotUuid: string) => [
  STATS_ONLINE_ROBOTS_ENDPOINT,
  STATS_ROBOT_ENDPOINT(robotUuid),
  STATS_ROBOT_NETWORK_ENDPOINT(robotUuid),
];
