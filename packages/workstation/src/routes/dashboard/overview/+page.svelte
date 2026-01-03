<script lang="ts">
  import { onMount } from 'svelte';
  import { fetchStatsOnlineRobots, type StatsOnlineRobotsResponse } from '$lib/api/stats/online_robots';
  import { fetchStatsRobots, type StatsRobotsResponse } from '$lib/api/stats/robots';
  import { Accordion } from 'flowbite-svelte';
  import RobotAccordion from './RobotAccordion.svelte';

  let combinedPromise: Promise<[StatsRobotsResponse, StatsOnlineRobotsResponse]> =
    Promise.resolve([[] as StatsRobotsResponse, [] as StatsOnlineRobotsResponse]);

  onMount(() => {
    combinedPromise = (async () => {
      try {
        const res = await Promise.all([fetchStatsRobots(), fetchStatsOnlineRobots()]);
        return res as [StatsRobotsResponse, StatsOnlineRobotsResponse];
      } catch (err) {
        throw err;
      }
    })();
  });
</script>

{#await combinedPromise then result}
  {@const allRobots = result[0]}
  {@const onlineRobots = result[1]}
  {#if allRobots.length === 0}
    <p>Loading robots...</p>
  {:else}
    <Accordion flush>
      <!-- Online robots -->
      {#each onlineRobots as robotUuid}
        <RobotAccordion {robotUuid} />
      {/each}
      <!-- Offline robots -->
      {#each allRobots.filter((uuid) => !onlineRobots.includes(uuid)) as robotUuid}
        <RobotAccordion {robotUuid} />
      {/each}
    </Accordion>
  {/if}
{:catch error}
  <p class="text-red-600">Error loading robots: {error?.message ?? String(error)}</p>
{/await}
