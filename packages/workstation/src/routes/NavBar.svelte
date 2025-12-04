<script lang="ts">
  import Link from '$lib/components/link.svelte';
  let { title = 'RMCS', children }: { title?: string; children: () => any } = $props();
</script>

<header class="navbar">
  <div class="nav-left">
    <button class="hamburger" aria-label="Toggle sidebar">☰</button>
    <Link href="/">{title}</Link>
  </div>
  <div class="nav-center">
    <input class="search" type="search" placeholder="搜索…" aria-label="Search" />
  </div>
  <div class="nav-right">
    {@render children?.()}
  </div>
  </header>

<style>
  .navbar {
    position: sticky;
    top: 0;
    z-index: 10;
    display: grid;
    grid-template-columns: auto 1fr auto;
    align-items: center;
    gap: 12px;
    padding: 10px 16px;
    border-bottom: 1px solid #e5e7eb;
    background: #ffffffcc;
    backdrop-filter: blur(8px);
  }

  .nav-left { display: flex; align-items: center; gap: 8px; }
  .hamburger { border: 1px solid #e5e7eb; border-radius: 6px; background: #fafafa; padding: 4px 8px; cursor: pointer; }
  .hamburger:hover { background: #f1f5f9; }

  .search { width: 100%; border: 1px solid #e5e7eb; border-radius: 8px; padding: 8px 10px; }

  .nav-right { display: flex; align-items: center; gap: 12px; }

  @media (max-width: 640px) {
    .search { display: none; }
  }
</style>