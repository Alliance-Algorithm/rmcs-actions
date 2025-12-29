<script lang="ts">
  import { fetchStatsRobot, StatsRobotResponse } from '$lib/api/stats/robot_uuid';
  import { AccordionItem } from 'flowbite-svelte';

  interface Props {
    robotUuid: string;
  }

  const { robotUuid }: Props = $props();

  let robotData: StatsRobotResponse | null = $state(null);
  $effect(() => {
    fetchStatsRobot(robotUuid).then((data) => {
      robotData = data;
    });
  });
</script>

{#if robotData}
  <AccordionItem>
    {#snippet header()}
      <div class="flex flex-col">
        <span class="font-semibold">{robotData.name}</span>
        <span class="text-sm text-gray-500">UUID: {robotData.uuid}</span>
      </div>
    {/snippet}
    <div class="space-y-2">
      <div class="flex justify-between">
        <span class="font-medium">MAC Address:</span>
        <span class="font-mono text-sm">{robotData.mac}</span>
      </div>
    </div>
  </AccordionItem>
{:else}
  <AccordionItem>
    {#snippet header()}
      <span class="font-semibold">Loading Robot {robotUuid}...</span>
    {/snippet}
    <p>{robotUuid}</p>
  </AccordionItem>
{/if}
