import * as z from 'zod';
import { getEndpoint } from '../api';

export const STATS_ROBOT_ENDPOINT = (robotUuid: string) => `/stats/robot/${robotUuid}`;

export const StatsRobotResponse = z.object({
  uuid: z.uuidv4(),
  mac: z.string(),
  name: z.string(),
});

export type StatsRobotResponse = z.infer<typeof StatsRobotResponse>;

export async function fetchStatsRobot(robotUuid: string): Promise<StatsRobotResponse> {
  const response = await fetch(getEndpoint(STATS_ROBOT_ENDPOINT(robotUuid)), {
    method: 'GET',
    headers: {
      'Content-Type': 'application/json',
    },
    signal: AbortSignal.timeout(5000),
  });

  if (!response.ok) {
    throw new Error(
      `Error fetching stats robot ${robotUuid}: ${response.status} ${response.statusText}`,
    );
  }

  const data = await response.json();
  return StatsRobotResponse.parse(data);
}
