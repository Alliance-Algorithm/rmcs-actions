<script lang="ts">
  import { AccordionItem } from 'flowbite-svelte';
  import { EditOutline } from 'flowbite-svelte-icons';
  import RobotNameEditDialog from './RobotNameEditDialog.svelte';
  import type { StatsRobotResponse } from '$lib/api/stats/robot_uuid';
  import type { StatsRobotNetworkResponse } from '$lib/api/stats/robot_network';

  interface Props {
    stats: StatsRobotResponse;
    network: StatsRobotNetworkResponse;
  }

  const { stats, network: networkStats }: Props = $props();

  let robotNameEditDialogOpen = $state(false);
</script>

{#snippet row(title: string, value?: string)}
  <div class="flex justify-between">
    <span class="font-medium">{title}</span>
    <span class="font-mono text-sm">{value === null || value?.length === 0 ? 'N/A' : value}</span>
  </div>
{/snippet}

{#snippet robotNetworks()}
  {#each networkStats.stats as network}
    <div class="border rounded p-2">
      <div class="font-semibold mb-1">Network {network.name}</div>
      {@render row('MTU', `${network.mtu ?? 'N/A'}`)}
      {@render row('Hardware Address', network.hardware_addr)}
      {@render row('Flags', network.flags.join(', '))}
      {@render row('Addresses', network.addrs.map((t) => t.addr).join(', '))}
    </div>
  {/each}
{/snippet}

<AccordionItem>
  {#snippet header()}
    <div class="flex flex-col">
      <span class="font-semibold">{stats.name}</span>
      <span class="text-sm text-gray-500">UUID: {stats.uuid}</span>
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
        {stats.name}
      </span>
    </div>
    {@render row('UUID', stats.uuid)}
    {@render robotNetworks()}
  </div>
</AccordionItem>
<!-- Edit dialog -->
<RobotNameEditDialog robotUuid={stats.uuid} initialName={stats.name} bind:open={robotNameEditDialogOpen} />
