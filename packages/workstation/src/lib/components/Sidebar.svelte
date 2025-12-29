<script lang="ts">
  import { Sidebar, SidebarButton, SidebarWrapper, uiHelpers } from 'flowbite-svelte';

  const { children }: { children: () => any } = $props();

  const sidebarUi = uiHelpers();
  let isOpen = $state(false);
  $effect(() => {
    isOpen = sidebarUi.isOpen;
  });

  export function toggleSidebar() {
    sidebarUi.toggle();
  }
</script>

<SidebarButton breakpoint="lg" onclick={sidebarUi.toggle}
               class="inline-flex items-center text-sm rounded-lg hover:bg-gray-100 focus:outline-hidden focus:ring-2 focus:ring-gray-200 dark:hover:bg-gray-700 dark:focus:ring-gray-600 lg:hidden fixed top-2 z-40 mb-2 md:top-4" />
<Sidebar
  breakpoint="lg"
  backdrop={false}
  {isOpen}
  closeSidebar={sidebarUi.close}
  params={{ x: -50, duration: 50 }}
  class="top-0 left-0 mt-[61px] lg:mt-[94px] h-screen w-64 bg-gray-50 transition-transform lg:block dark:bg-gray-800 border-r border-gray-200"
  classes={{ div: "h-full overflow-y-auto bg-gray-50 dark:bg-gray-800 p-0" }}
>
  <h4 class="sr-only">Main Menu</h4>
  <SidebarWrapper
    class="scrolling-touch h-full max-w-2xs overflow-y-auto bg-white px-4 pt-20 lg:sticky lg:me-0 lg:block lg:h-[calc(100vh-4rem)] lg:pt-5 dark:bg-gray-800">
    {@render children()}
  </SidebarWrapper>
</Sidebar>
