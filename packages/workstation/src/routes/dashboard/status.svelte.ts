import { wait } from '$lib/utils/wait';
import { writable, type Writable } from 'svelte/store';

export const backendOnline: Writable<boolean> = writable(false);
export const isCheckingBackend: Writable<boolean> = writable(false);

export async function checkBackendStatus(): Promise<void> {
  try {
    const response: Response = await wait(
      () =>
        fetch('http://localhost:3000/api/ping', {
          method: 'GET',
          signal: AbortSignal.timeout(3000),
        }),
      500,
      () => {
        console.log('Checking backend status...');
        isCheckingBackend.set(true);
      },
    )();

    if (response.ok) {
      backendOnline.set(true);
      return;
    } else {
      backendOnline.set(false);
      return;
    }
  } catch (error) {
    backendOnline.set(false);
  } finally {
    isCheckingBackend.set(false);
  }
}
