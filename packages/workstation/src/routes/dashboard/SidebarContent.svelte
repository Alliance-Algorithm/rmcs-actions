<script lang="ts">
  import { SidebarGroup, SidebarItem, SidebarDropdownWrapper } from 'flowbite-svelte';
  import {
    DribbbleSolid,
    ShoppingBagSolid,
    GridSolid,
    MailBoxSolid,
    UserSolid,
    ArrowRightToBracketOutline,
    AdjustmentsHorizontalSolid
  } from 'flowbite-svelte-icons';
  import { page } from '$app/state';

  let activeUrl = $state(page.url.pathname);

  const sidebarMatch: string | string[] = 'docs/components/sidebar';
  const matchesRoute = $derived.by(() => {
    const list = Array.isArray(sidebarMatch) ? sidebarMatch : [sidebarMatch];
    return list.some(p => activeUrl.startsWith(`/${p}`));
  });

  $effect(() => {
    activeUrl = page.url.pathname;
  });

  const spanClass = 'flex-1 ms-3 whitespace-nowrap';
  const iconClass = 'h-5 w-5 text-gray-500 transition duration-75 group-hover:text-gray-900 dark:text-gray-400 dark:group-hover:text-white';
</script>

<SidebarGroup>
  <SidebarItem label="Overview" href="/dashboard/overview">
    {#snippet icon()}
      <DribbbleSolid class={iconClass} />
    {/snippet}
  </SidebarItem>
  <SidebarDropdownWrapper label="E-commerce" classes={{ btn: "p-2" }} isOpen={matchesRoute}>
    {#snippet icon()}
      <ShoppingBagSolid
        class={iconClass} />
    {/snippet}
    <SidebarItem label="Sidebar" />
    <SidebarItem label="Billing" />
    <SidebarItem label="Invoice" />
  </SidebarDropdownWrapper>
  <SidebarItem label="Kanban" {spanClass}>
    {#snippet icon()}
      <GridSolid
        class={iconClass} />
    {/snippet}
    {#snippet subtext()}
      <span
        class="ms-3 inline-flex items-center justify-center rounded-full bg-gray-200 px-2 text-sm font-medium text-gray-800 dark:bg-gray-700 dark:text-gray-300">Pro</span>
    {/snippet}
  </SidebarItem>
  <SidebarItem label="Inbox" {spanClass}>
    {#snippet icon()}
      <MailBoxSolid
        class={iconClass} />
    {/snippet}
    {#snippet subtext()}
      <span
        class="bg-primary-200 text-primary-600 dark:bg-primary-900 dark:text-primary-200 ms-3 inline-flex h-3 w-3 items-center justify-center rounded-full p-3 text-sm font-medium">3</span>
    {/snippet}
  </SidebarItem>
  <SidebarItem label="Users">
    {#snippet icon()}
      <UserSolid
        class={iconClass} />
    {/snippet}
  </SidebarItem>
  <SidebarItem label="Sign In">
    {#snippet icon()}
      <ArrowRightToBracketOutline
        class={iconClass} />
    {/snippet}
  </SidebarItem>
  <SidebarItem label="Configuration" href="/dashboard/config">
    {#snippet icon()}
      <AdjustmentsHorizontalSolid class={iconClass} />
    {/snippet}
  </SidebarItem>
</SidebarGroup>
