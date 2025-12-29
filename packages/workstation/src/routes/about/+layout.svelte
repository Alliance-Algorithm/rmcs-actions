<script lang="ts">
  import Footer from '$lib/components/Footer.svelte';

  const { children }: { children: () => any } = $props();
</script>

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
