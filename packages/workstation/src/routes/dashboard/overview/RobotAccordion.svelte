<script lang="ts">
  import {
    fetchStatsRobotNetwork,
    type StatsRobotNetworkResponse
  } from '$lib/api/stats/robot_network';
  import { fetchStatsRobot, StatsRobotResponse } from '$lib/api/stats/robot_uuid';
  import { AccordionItem } from 'flowbite-svelte';
  import { backendOnline } from '$lib/stores/status';
  import { EditOutline } from 'flowbite-svelte-icons';
  import RobotNameEditDialog from './RobotNameEditDialog.svelte';

  function formatError(err: unknown): string {
    return err instanceof Error ? err.message : String(err ?? 'Unknown error');
  }

  interface Props {
    robotUuid: string;
  }

  const { robotUuid }: Props = $props();

  let robotNameEditDialogOpen = $state(false);

  let robotDataPromise = $state(Promise.resolve(null as StatsRobotResponse | null));
  let robotNetworkPromise = $state(
    Promise.resolve({
      data: null as StatsRobotNetworkResponse | null,
      error: null as string | null
    })
  );

  $effect(() => {
    robotDataPromise = $backendOnline ? fetchStatsRobot(robotUuid) : Promise.resolve(null);
  });

  $effect(() => {
    if (!$backendOnline) {
      robotNetworkPromise = Promise.resolve({ data: null, error: null });
      return;
    }

    robotNetworkPromise = fetchStatsRobotNetwork(robotUuid)
      .then(function (data) {
        return { data: data, error: null };
      })
      .catch(function (err) {
        return { data: null, error: formatError(err) };
      });
  });
</script>

{#snippet row(title: string, value?: string)}
  <div class="flex justify-between">
    <span class="font-medium">{title}</span>
    <span class="font-mono text-sm">{value === null || value?.length === 0 ? 'N/A' : value}</span>
  </div>
{/snippet}

{#snippet robotNetworks()}
  {#await robotNetworkPromise then result}
    {#if result?.error}
      <p class="text-red-500">Error loading network data: {result.error}</p>
    {:else if result?.data}
      {#each result.data.stats as network}
        <div class="border rounded p-2">
          <div class="font-semibold mb-1">Network {network.name}</div>
          {@render row('MTU', `${network.mtu ?? 'N/A'}`)}
          {@render row('Hardware Address', network.hardware_addr)}
          {@render row('Flags', network.flags.join(', '))}
          {@render row('Addresses', network.addrs.map((t) => t.addr).join(', '))}
        </div>
      {/each}
    {:else}
      <p>Loading network data...</p>
    {/if}
  {/await}
{/snippet}

{#await robotDataPromise then robotData}
  {#if robotData}
    <AccordionItem>
      {#snippet header()}
        <div class="flex flex-col">
          <span class="font-semibold">{robotData?.name}</span>
          <span class="text-sm text-gray-500">UUID: {robotData?.uuid}</span>
        </div>
      {/snippet}
      <div class="space-y-2">
        <div class="flex justify-between">
          <span class="font-medium">Name</span>
          <span class="font-mono text-sm">
            <EditOutline
              class="cursor-pointer inline hover:text-primary-700"
              onclick={() => robotNameEditDialogOpen = true}
            />
            {robotData?.name}
          </span>
        </div>
        {@render row('UUID', robotData?.uuid)}
        {@render robotNetworks()}
      </div>
    </AccordionItem>
    <!-- Edit dialog -->
    <RobotNameEditDialog robotUuid={robotData?.uuid} bind:open={robotNameEditDialogOpen} />
  {:else}
    <AccordionItem>
      {#snippet header()}
        <span class="font-semibold">Loading Robot {robotUuid}...</span>
      {/snippet}
      <div class="space-y-2">
        <p>Loading robot data...</p>
        {@render robotNetworks()}
      </div>
    </AccordionItem>
  {/if}
{/await}
