<script lang="ts">
  import Navbar from '../dashboard/Navbar.svelte';
  import Footer from '$lib/components/Footer.svelte';

  const { children }: { children: () => any } = $props();
</script>

<div class="min-h-screen flex flex-col">
  <header
    class="fixed top-0 z-40 mx-auto w-full flex-none border-b border-gray-200 bg-white dark:border-gray-600 dark:bg-gray-800">
    <Navbar />
  </header>

  <main class="flex-1 overflow-hidden pt-[70px] lg:pt-[89px]">
    <div class="overflow-hidden lg:flex h-full">
      <div class="relative flex-1 min-h-0 w-full lg:min-h-[calc(100vh-89px)] flex flex-col">
        <div class="overflow-y-auto flex-1">
          <div class="max-w-7xl mx-auto px-8 pt-6 pb-20">
            {@render children()}
          </div>
        </div>

        <div class="flex-none">
          <Footer />
        </div>
      </div>
    </div>
  </main>
</div>
