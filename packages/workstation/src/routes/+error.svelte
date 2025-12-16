<script lang="ts">
  import { Button } from 'flowbite-svelte';
  import Link from '$lib/components/Link.svelte';
  import { page } from '$app/state';
</script>

<div class="max-w-xl w-full bg-white dark:bg-gray-800 shadow-md rounded-lg p-8 mx-auto my-16 self-center">
  <div class="flex items-start gap-4">
    <div class="flex-1">
      <h1 class="text-2xl font-semibold text-gray-900 dark:text-gray-100">Something went wrong</h1>
      <p class="mt-2 text-sm text-gray-600 dark:text-gray-300">Sorry — we ran into an unexpected problem while loading
        this page.</p>

      <div class="mt-4" role="alert" aria-live="assertive">
        <div
          class="inline-flex items-center gap-2 text-sm font-medium text-red-700 bg-red-50 dark:bg-red-900/20 dark:text-red-300 px-3 py-1 rounded">
          <span class="font-semibold">Error</span>
          <span class="px-2 py-0.5 text-xs bg-red-100 dark:bg-transparent text-red-800 rounded">{page.status}</span>
        </div>

        <p
          class="mt-3 text-sm text-gray-700 dark:text-gray-200">{page.error?.message ?? 'An unknown error occurred.'}</p>
      </div>

      <div class="mt-6 flex gap-3">
        <Button href="/">Go to homepage</Button>

        <Button type="button" color="alternative" onclick={() => location.reload()}>Retry</Button>

        <span class="pl-8">
          <Link href="/" class="ml-auto text-xs text-gray-500 dark:text-gray-400">
            Report this issue
          </Link>
        </span>
      </div>
    </div>
  </div>
</div>
