<script lang="ts">
  import Navbar from './Navbar.svelte';
  import Sidebar from './Sidebar.svelte';

  import './layout.css';

  const { children }: { children: () => any } = $props();
</script>

<header
  class="fixed top-0 z-40 mx-auto w-full flex-none border-b border-gray-200 bg-white dark:border-gray-600 dark:bg-gray-800">
  <Navbar />
</header>
<div class="overflow-hidden lg:flex">
  <Sidebar>1234</Sidebar>
  <div class="relative h-full w-full overflow-y-auto pt-[70px] lg:ml-64">
    {@render children()}
  </div>
</div>
