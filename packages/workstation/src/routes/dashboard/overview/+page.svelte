<script lang="ts">
  import { AccordionItem } from 'flowbite-svelte';
  import { Accordion } from 'flowbite-svelte';
  import type { PageData } from './$types';
  import RobotAccordion from './RobotAccordion.svelte';

  const { data }: { data: PageData } = $props();
</script>

{#snippet offlineEntry(uuid: string)}
  <AccordionItem>
    {#snippet header()}
      <div class="flex flex-col">
        <span class="font-semibold">{uuid}</span>
      </div>
    {/snippet}
    <div class="space-y-2">
      <p class="text-red-700">Offline</p>
    </div>
  </AccordionItem>
{/snippet}

<Accordion flush>
  {#each data.robots.entries() as [uuid, robot]}
    {#if robot.isOnline}
      <RobotAccordion stats={robot.robotStats} network={robot.network} />
    {:else}
      {@render offlineEntry(uuid)}
    {/if}
  {/each}
</Accordion>
