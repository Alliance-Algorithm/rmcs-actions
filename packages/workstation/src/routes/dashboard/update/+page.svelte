<script lang="ts">
  import { Card, Input, Label, Button, Select, Alert, Spinner } from 'flowbite-svelte';
  import {
    actionUpdateBinary,
    actionUpdateBinaryAll,
    type ActionUpdateBinaryResponse,
    type ActionUpdateBinaryAllResponse,
  } from '$lib/api/action/update_binary';
  import type { PageData } from './$types';

  const { data }: { data: PageData } = $props();

  let artifactUrl = $state('');
  let selectedRobotId = $state('');
  let loading = $state(false);
  let result: ActionUpdateBinaryResponse | null = $state(null);
  let allResult: ActionUpdateBinaryAllResponse | null = $state(null);
  let error = $state('');

  const robotOptions = $derived(data.onlineRobots.map((id) => ({ value: id, name: id })));

  async function handleUpdateSelected() {
    if (!artifactUrl || !selectedRobotId) return;
    loading = true;
    result = null;
    allResult = null;
    error = '';
    try {
      result = await actionUpdateBinary(fetch, {
        robot_id: selectedRobotId,
        artifact_url: artifactUrl,
      });
    } catch (err) {
      error = err instanceof Error ? err.message : 'Failed to update binary';
    } finally {
      loading = false;
    }
  }

  async function handleUpdateAll() {
    if (!artifactUrl) return;
    loading = true;
    result = null;
    allResult = null;
    error = '';
    try {
      allResult = await actionUpdateBinaryAll(fetch, {
        artifact_url: artifactUrl,
      });
    } catch (err) {
      error = err instanceof Error ? err.message : 'Failed to update binaries';
    } finally {
      loading = false;
    }
  }
</script>

<div class="mx-auto max-w-2xl space-y-6 p-4">
  <h1 class="text-2xl font-bold text-gray-900 dark:text-white">Binary Update</h1>

  <Card>
    <div class="space-y-4">
      <Label class="space-y-2">
        <span>Artifact URL</span>
        <Input
          type="url"
          bind:value={artifactUrl}
          placeholder="https://example.com/bot-binary"
          required
        />
      </Label>

      <Label class="space-y-2">
        <span>Target Robot</span>
        <Select bind:value={selectedRobotId} items={robotOptions} placeholder="Select a robot" />
      </Label>

      <div class="flex space-x-2">
        <Button
          color="primary"
          disabled={loading || !artifactUrl || !selectedRobotId}
          onclick={handleUpdateSelected}
        >
          {#if loading}
            <Spinner size="4" class="me-2" />
          {/if}
          Update Selected Bot
        </Button>
        <Button color="alternative" disabled={loading || !artifactUrl} onclick={handleUpdateAll}>
          {#if loading}
            <Spinner size="4" class="me-2" />
          {/if}
          Update All Bots
        </Button>
      </div>
    </div>
  </Card>

  {#if error}
    <Alert color="red">
      <span class="font-medium">Error:</span>
      {error}
    </Alert>
  {/if}

  {#if result}
    <Alert color={result.status === 'error' ? 'red' : 'green'}>
      <span class="font-medium">Status: {result.status}</span>
      <p>{result.message}</p>
    </Alert>
  {/if}

  {#if allResult}
    <Alert color={allResult.status === 'partial_failure' ? 'yellow' : 'green'}>
      <span class="font-medium">Overall: {allResult.status}</span>
    </Alert>
    {#each allResult.results as r}
      <Alert color={r.status === 'error' ? 'red' : 'green'}>
        <span class="font-medium">{r.robot_id}: {r.status}</span>
        <p>{r.message}</p>
      </Alert>
    {/each}
  {/if}
</div>
