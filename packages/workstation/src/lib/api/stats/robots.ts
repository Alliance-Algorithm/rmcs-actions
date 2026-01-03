import * as z from 'zod';
import { getEndpoint } from '../api';

export const STATS_ROBOTS_ENDPOINT = '/stats/robots';

export const StatsRobotsResponse = z.array(z.uuidv4());

export type StatsRobotsResponse = z.infer<typeof StatsRobotsResponse>;

export async function fetchStatsRobots(trackedFetch: typeof fetch): Promise<StatsRobotsResponse> {
  const response = await trackedFetch(getEndpoint(STATS_ROBOTS_ENDPOINT), {
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
  return StatsRobotsResponse.parse(data);
}
