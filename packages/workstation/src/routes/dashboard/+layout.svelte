<script lang="ts">
  import Sidebar from '$lib/components/Sidebar.svelte';
  import Footer from '$lib/components/Footer.svelte';
  import SidebarContent from './SidebarContent.svelte';

  const { children }: { children: () => any } = $props();
</script>

<Sidebar>
  <SidebarContent />
</Sidebar>

<!-- Content region: make it a flex column item that can scroll internally."
           min-h-0 is important so the flex child can shrink and allow overflow-y-auto to work. -->
<div class="relative flex-1 min-h-0 w-full lg:ml-64 lg:min-h-[calc(100vh-89px)] flex flex-col">
  <!-- Scrollable content area (grows/shrinks) -->
  <div class="overflow-y-auto flex-1">
    <!-- Inner container gives a clear whitespace / padding area before the footer -->
    <div class="max-w-7xl mx-auto px-8 pt-6 pb-20">
      {@render children()}
    </div>
  </div>

  <!-- Footer is a non-growing element and will sit at the bottom of this column.
             On lg+ the parent has a min-height that accounts for the fixed header, so
             when content is short the footer will be pushed to the bottom (aligned
             with the Sidebar). On smaller screens it simply follows the content. -->
  <div class="flex-none">
    <Footer />
  </div>
</div>
