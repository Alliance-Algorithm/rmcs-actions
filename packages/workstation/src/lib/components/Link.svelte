<script lang="ts">
  let { href, children, class: className = '' }: { href: string; children: () => any; class?: string } = $props();
</script>

<a {href} class={className} target="_blank" rel="noopener noreferrer">
  {@render children()}
</a>

<style>
  a {
    font-weight: bold;
    background:
      linear-gradient(
        to right,
        rgba(100, 200, 200, 1),
        rgba(100, 200, 200, 1)
      ),
      linear-gradient(
        to right,
        rgba(255, 0, 0, 1),
        rgba(255, 0, 180, 1),
        rgba(0, 100, 200, 1)
      );
    background-size:
      100% 0.1em,
      0 0.1em;
    background-position:
      100% 100%,
      0 100%;
    background-repeat: no-repeat;
    transition: background-size 400ms;
  }

  a:hover,
  a:focus {
    background-size:
      0 0.1em,
      100% 0.1em;
  }
</style>
