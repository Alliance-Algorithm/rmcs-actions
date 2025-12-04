<script lang="ts">
  import Link from '$lib/components/link.svelte';
  import svelteLogo from '$lib/images/svelte-logo.svg';
  import './layout.css';
  import SideBar from './SideBar.svelte';
  import NavBar from './NavBar.svelte';

  let { children } = $props();
</script>

<div class="app">
  <NavBar>
    <div class="footer-widget">
      <span>
        Powered by <Link href="https://svelte.dev"><img id="svelte-logo" src={svelteLogo} alt="Svelte Logo" /> Svelte</Link>
      </span>
      <span>
        Created by <Link href="https://github.com/Embers-of-the-Fire">Embers-of-the-Fire</Link>
      </span>
      <span>&copy; 2025 the RMCS developers</span>
    </div>
  </NavBar>

  <div class="layout">
    <SideBar />
    <main>
      {@render children()}
    </main>
  </div>
</div>

<style>
  .app {
    display: flex;
    flex-direction: column;
    min-height: 100vh;
  }

  .layout {
    display: grid;
    grid-template-columns: 280px 1fr;
    gap: 0;
    flex: 1;
    min-height: 0;
  }

  main {
    display: flex;
    flex-direction: column;
    padding: 1rem;
    width: 100%;
    max-width: 64rem;
    margin: 0 auto;
    box-sizing: border-box;
  }

  .footer-widget {
    display: flex;
    align-items: center;
    gap: 1rem;
    flex-wrap: wrap;
  }

  #svelte-logo {
    height: 1em;
    vertical-align: middle;
    margin-bottom: 0.2em;
    display: inline-block;
  }

  @media (max-width: 768px) {
    .layout {
      grid-template-columns: 1fr;
    }
  }

  @media (min-width: 769px) {
    .footer-widget { gap: 1rem; }
  }
</style>
