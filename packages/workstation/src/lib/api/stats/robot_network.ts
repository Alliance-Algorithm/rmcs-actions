import * as z from 'zod';
import { getEndpoint } from '../api';

export const STATS_ROBOT_NETWORK_ENDPOINT = (robotUuid: string) =>
  `/stats/robot/${robotUuid}/network`;

export const StatsRobotNetworkResponse = z.object({
  last_updated: z.string().refine((date) => !isNaN(Date.parse(date)), {
    message: 'Invalid date format',
  }),
  stats: z.array(
    z.object({
      index: z.number(),
      mtu: z.number(),
      name: z.string(),
      hardware_addr: z.string(),
      flags: z.array(z.string()),
      addrs: z.array(
        z.object({
          addr: z.string(),
        }),
      ),
    }),
  ),
});

export type StatsRobotNetworkResponse = z.infer<typeof StatsRobotNetworkResponse>;

export async function fetchStatsRobotNetwork(
  robotUuid: string,
): Promise<StatsRobotNetworkResponse> {
  const response = await fetch(getEndpoint(STATS_ROBOT_NETWORK_ENDPOINT(robotUuid)), {
    method: 'GET',
    headers: {
      'Content-Type': 'application/json',
    },
    signal: AbortSignal.timeout(5000),
  });

  if (!response.ok) {
    throw new Error(
      `Error fetching stats robot network ${robotUuid}: ${response.status} ${response.statusText}`,
    );
  }

  const data = await response.json();
  return StatsRobotNetworkResponse.parse(data);
}
