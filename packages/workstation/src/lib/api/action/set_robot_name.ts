import * as z from 'zod';
import { getEndpoint } from '$lib/api/api';
import { STATS_ONLINE_ROBOTS_ENDPOINT } from '$lib/api/stats/online_robots';
import { STATS_ROBOTS_ENDPOINT } from '$lib/api/stats/robots';
import { STATS_ROBOT_ENDPOINT } from '$lib/api/stats/robot_uuid';

export const ACTION_SET_ROBOT_NAME_ENDPOINT = '/action/set_robot_name';

export const ActionSetRobotNameRequest = z.object({
  robot_uuid: z.uuidv4(),
  new_robot_name: z.string().min(1).max(100),
});
export type ActionSetRobotNameRequest = z.infer<typeof ActionSetRobotNameRequest>;

export async function actionSetRobotName(request: ActionSetRobotNameRequest): Promise<void> {
  const body = ActionSetRobotNameRequest.parse(request);

  const response = await fetch(getEndpoint(ACTION_SET_ROBOT_NAME_ENDPOINT), {
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
