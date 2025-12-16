<script lang="ts">
  import Frame from './Frame.svelte';
  import Link from '$lib/components/Link.svelte';
  import { socials } from '$lib/routes/social';
</script>

<Frame tag="footer" rounded shadow
       class="rounded-lg bg-white p-4 shadow md:p-6 xl:p-8 dark:bg-gray-800 border-t border-gray-200">
  <div class="flex flex-col items-center justify-between gap-4 md:flex-row">
    <p class="text-sm text-gray-500 dark:text-gray-300">
      © 2025
      <Link href="https://github.com/Alliance-Algorithm">Alliance Algorithm Team</Link>
    </p>

    <div class="flex items-center space-x-6">
      {#each socials as { name, href, icon }}
        <a {href} target="_blank" class="text-gray-500 hover:text-gray-900 dark:text-gray-300 dark:hover:text-white"
           aria-label={name}>
          <span class="sr-only">{name}</span>
          <svelte:component this={icon} size="md" />
        </a>
      {/each}
    </div>
  </div>
</Frame>
