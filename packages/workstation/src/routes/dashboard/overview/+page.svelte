<script lang="ts">
  import {
    fetchStatsOnlineRobots,
    type StatsOnlineRobotsResponse,
  } from '$lib/api/stats/online_robots';
  import { fetchStatsRobots, type StatsRobotsResponse } from '$lib/api/stats/robots';
  import { Accordion } from 'flowbite-svelte';
  import RobotAccordion from './RobotAccordion.svelte';

  let allRobots: StatsRobotsResponse = $state([]);
  let onlineRobots: StatsOnlineRobotsResponse = $state([]);

  $effect(() => {
    fetchStatsRobots().then((data) => {
      allRobots = data;
    });
    fetchStatsOnlineRobots().then((data) => {
      onlineRobots = data;
    });
  });
</script>

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
