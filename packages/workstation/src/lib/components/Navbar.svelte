<script lang="ts">
  import { Navbar, NavBrand, NavUl, NavLi, NavHamburger } from 'flowbite-svelte';
  import { fly } from 'svelte/transition';
  import { page } from '$app/state';
  import logo from '$lib/images/logo.png';
  import { navLinks } from '$lib/routes/nav';
  import {
    backendOnline,
    checkBackendStatus,
    isCheckingBackend,
  } from '../../routes/dashboard/status.svelte';
  import { fetchStatsRobots, StatsRobotsResponse } from '$lib/api/stats/robots';

  let activeUrl = $derived(page.url.pathname);
  let baseUrl = $derived.by(() => {
    const segments = activeUrl.split('/');
    return `/${segments.length > 1 ? segments[1] : ''}`;
  });
</script>

<Navbar class="fixed top-0 z-50 sm:mx-0 border-b border-gray-200">
  <NavBrand class="mx-10">
    <img src={logo} class="me-2.5 h-6 sm:h-8" alt="Alliance Logo" />
    <span
      class="ml-px self-center text-xl font-semibold whitespace-nowrap sm:text-2xl dark:text-white"
    >
      Alliance
    </span>
  </NavBrand>
  <NavHamburger />
  <NavUl activeUrl={baseUrl} transition={fly} transitionParams={{ y: -20, duration: 250 }}>
    {#each navLinks as { href, display }}
      <NavLi {href}>{display}</NavLi>
    {/each}
    <NavLi onclick={() => !$isCheckingBackend && checkBackendStatus()}>
      <span
        class="gap-1 font-semibold flex items-center cursor-pointer px-2 hover:bg-gray-100"
        color="alternative"
      >
        {#if $isCheckingBackend}
          <span class="text-yellow-500 flex items-center">
            <span class="inline-block text-xs leading-none mr-1" style="vertical-align: middle;"
              >●</span
            >
            Checking...
          </span>
        {:else if $backendOnline}
          <span class="text-green-500 flex items-center">
            <span class="inline-block text-xs leading-none mr-1" style="vertical-align: middle;"
              >●</span
            >
            Online
          </span>
        {:else}
          <span class="text-red-500 flex items-center">
            <span class="inline-block text-xs leading-none mr-1" style="vertical-align: middle;"
              >●</span
            >
            Offline
          </span>
        {/if}
      </span>
    </NavLi>
  </NavUl>
</Navbar>
