export type NavLink = {
  display: string;
  href: string;
};

export const navLinks: NavLink[] = [
  {
    display: 'Home',
    href: '/app',
  },
  {
    display: 'Dashboard',
    href: '/dashboard',
  },
  {
    display: 'About',
    href: '/app/about',
  },
] as const;
