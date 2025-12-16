<script lang="ts">
  import "./layout.css";

  const { children }: { children: () => any } = $props();
</script>

{@render children()}
