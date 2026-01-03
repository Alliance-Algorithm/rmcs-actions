import type { PageLoad } from './$types';
import { fetchStatsRobots } from '$lib/api/stats/robots';
import { fetchStatsOnlineRobots } from '$lib/api/stats/online_robots';
import { fetchStatsRobot, type StatsRobotResponse } from '$lib/api/stats/robot_uuid';
import {
  fetchStatsRobotNetwork,
  type StatsRobotNetworkResponse,
} from '$lib/api/stats/robot_network';

export const prerender = false;
export const ssr = false;

export const load: PageLoad = async ({ params, fetch }) => {
  const robots = new Set(await fetchStatsRobots(fetch));
  const onlineRobots = new Set(await fetchStatsOnlineRobots(fetch));

  let robotsCollection: Map<
    string,
    | {
        name: string;
        isOnline: true;
        robotStats: StatsRobotResponse;
        network: StatsRobotNetworkResponse;
      }
    | {
        name: string;
        isOnline: false;
      }
  > = new Map();
  for (const robot of robots) {
    const isOnline = onlineRobots.has(robot);

    if (!isOnline) {
      robotsCollection.set(robot, {
        name: robot,
        isOnline: false,
      });
      continue;
    }

    const robotStats = await fetchStatsRobot(fetch, robot);
    const network = await fetchStatsRobotNetwork(fetch, robot);

    robotsCollection.set(robot, {
      name: robot,
      isOnline: true,
      robotStats,
      network,
    });
  }

  return {
    robots: robotsCollection,
  };
};
