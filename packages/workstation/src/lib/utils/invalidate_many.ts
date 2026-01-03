import { invalidate } from '$app/navigation';

export async function invalidateMany(keys: string[]): Promise<void> {
  await invalidate((url) => keys.some((t) => url.pathname.includes(t)));
}
