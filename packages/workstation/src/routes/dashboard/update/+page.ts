import type { PageLoad } from './$types';
import { fetchStatsOnlineRobots } from '$lib/api/stats/online_robots';

export const prerender = false;
export const ssr = false;

export const load: PageLoad = async ({ fetch }) => {
  const onlineRobots = await fetchStatsOnlineRobots(fetch);

  return {
    onlineRobots,
  };
};
