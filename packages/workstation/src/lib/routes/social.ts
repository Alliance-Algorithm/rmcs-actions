import type { Component } from 'svelte';
import { GithubSolid } from 'flowbite-svelte-icons';

export type Social = {
  name: string;
  href: string;
  icon: Component;
};

export const socials: Social[] = [
  {
    name: 'GitHub',
    href: 'https://github.com/Alliance-Algorithm/rmcs-actions',
    icon: GithubSolid,
  },
] as const;
