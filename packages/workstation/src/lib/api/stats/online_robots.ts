import * as z from 'zod';
import { getEndpoint } from '../api';

export const STATS_ONLINE_ROBOTS_ENDPOINT = '/stats/online_robots';

export const StatsOnlineRobotsResponse = z.array(z.uuidv4());

export type StatsOnlineRobotsResponse = z.infer<typeof StatsOnlineRobotsResponse>;

export async function fetchStatsOnlineRobots(): Promise<StatsOnlineRobotsResponse> {
  const response = await fetch(getEndpoint(STATS_ONLINE_ROBOTS_ENDPOINT), {
    method: 'GET',
    headers: {
      'Content-Type': 'application/json',
    },
    signal: AbortSignal.timeout(5000),
  });

  if (!response.ok) {
    throw new Error(`Error fetching stats robots: ${response.status} ${response.statusText}`);
  }

  const data = await response.json();
  return StatsOnlineRobotsResponse.parse(data);
}
