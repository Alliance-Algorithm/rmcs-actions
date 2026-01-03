<script lang="ts">
  import { Modal, Label, Input, Button } from 'flowbite-svelte';
  import { actionSetRobotName } from '$lib/api/action/set_robot_name';

  interface Props {
    robotUuid: string;
    initialName: string;
    open?: boolean;
  }

  let { robotUuid, initialName, open = $bindable(false) }: Props = $props();

  let error = $state('');

  async function handleSubmit(event: SubmitEvent) {
    event.preventDefault();
    const form = event.currentTarget as HTMLFormElement;
    const data = new FormData(form);
    const name = (data.get('robot_name') as string) || '';

    // simple validation
    if (!name.trim()) {
      error = 'Please enter a robot name';
      return;
    }

    try {
      await actionSetRobotName({ robot_uuid: robotUuid, new_robot_name: name.trim() });
    } catch (err) {
      error = err instanceof Error ? err.message : 'Failed to set robot name';
      return;
    }
    error = '';
    open = false;
  }
</script>

<Modal bind:open={open} size="sm">
  <form method="dialog" name="robot_name_form" onsubmit={handleSubmit}>
    <div class="flex flex-col space-y-6">
      <h3 class="mb-4 text-xl font-medium text-gray-900">Edit Robot Name</h3>
      {#if error && error.length > 0}
        <Label color="red">
          {error}
        </Label>
      {/if}
      <Label class="space-y-2">
        <span>New Name</span>
        <Input type="text" name="robot_name" placeholder="Enter new robot name" defaultValue={initialName} required />
      </Label>
      <div class="flex space-x-2">
        <Button type="submit" color="primary">
          Save Changes
        </Button>
        <Button type="button" onclick={() => (open = false)} color="alternative">
          Cancel
        </Button>
      </div>
    </div>
  </form>
</Modal>
