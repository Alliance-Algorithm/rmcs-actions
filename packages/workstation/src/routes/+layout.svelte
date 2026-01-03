<script lang="ts">
  import Navbar from '$lib/components/Navbar.svelte';
  import { Banner } from 'flowbite-svelte';
  import './layout.css';
  import { BullhornSolid } from 'flowbite-svelte-icons';
  import Link from '$lib/components/Link.svelte';
  import { onMount } from 'svelte';
  import { afterNavigate } from '$app/navigation';
  import { checkBackendStatus } from '$lib/stores/status';

  onMount(() => {
    checkBackendStatus();
  });

  afterNavigate(() => {
    checkBackendStatus();
  });

  const { children }: { children: () => any } = $props();
</script>

<Banner class="absolute">
  <p class="me-8 flex items-center text-sm font-normal text-gray-500 md:me-0 dark:text-gray-400">
    <span class="me-3 inline-flex rounded-full bg-gray-200 p-1 dark:bg-gray-600">
      <BullhornSolid class="h-3 w-3 text-gray-500 dark:text-gray-400" />
      <span class="sr-only">Light bulb</span>
    </span>
    <span> KFC Crazy Thursday, <Link href="https://afdian.com/a/embersofthefire">V me 50!</Link></span>
  </p>
</Banner>

<div class="min-h-screen flex flex-col">
  <header
    class="fixed top-0 z-40 mx-auto w-full flex-none border-b border-gray-200 bg-white dark:border-gray-600 dark:bg-gray-800"
  >
    <Navbar />
  </header>

  <main class="flex-1 overflow-hidden pt-[70px] lg:pt-[89px]">
    <div class="overflow-hidden lg:flex h-full">
      {@render children()}
    </div>
  </main>
</div>
