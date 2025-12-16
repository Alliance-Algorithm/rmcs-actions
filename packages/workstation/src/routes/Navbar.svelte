<script lang="ts">
  import { Navbar, NavBrand, NavUl, NavLi, SidebarButton, NavHamburger } from 'flowbite-svelte';
  import { Dropdown, DropdownItem } from 'flowbite-svelte';
  import { ChevronDownOutline } from 'flowbite-svelte-icons';
</script>

<Navbar class="fixed top-0 z-50 mx-10 w-full flex-none sm:mx-0 border-b border-gray-200">
  <NavBrand class="mx-10">
    <img
      src="https://avatars.githubusercontent.com/u/115159719?s=512&v=4"
      class="me-2.5 h-6 sm:h-8"
      alt="Alliance Logo"
    />
    <span
      class="ml-px self-center text-xl font-semibold whitespace-nowrap sm:text-2xl dark:text-white"
    >Alliance</span
    >
  </NavBrand>
  <NavHamburger />
  <NavUl class="ml-2" activeUrl="/" activeClass="text-primary-600 dark:text-primary-500">
    <NavLi href="/">Home</NavLi>
    <NavLi href="/">Messages</NavLi>
    <NavLi class="cursor-pointer">
      Dropdown
      <ChevronDownOutline class="ms-0 inline" />
    </NavLi>
    <Dropdown class="z-20 w-44">
      <DropdownItem href="#top">Item 1</DropdownItem>
      <DropdownItem href="#top">Item 2</DropdownItem>
      <DropdownItem href="#top">Item 3</DropdownItem>
    </Dropdown>
  </NavUl>
</Navbar>
