<script lang="ts">
	import { Navbar, NavBrand, NavHamburger, NavUl, NavLi, Button } from 'flowbite-svelte';
</script>

<Navbar class="sticky h-10 w-full bg-red-300 dark:bg-gray-900">
	<span class="self-center text-xl font-semibold whitespace-nowrap dark:text-white">Flowbite</span>
	<div class="flex items-center lg:order-2">
		<Button href="/" color="dark">Log in</Button>
		<Button href="/" class="ml-2" color="red">Get started</Button>
		<NavHamburger />
	</div>
	<NavLi href="/">Home</NavLi>
	<NavLi href="/">Company</NavLi>
	<NavLi href="/">Marketplace</NavLi>
	<NavLi href="/">Features</NavLi>
	<NavLi href="/">Team</NavLi>
	<NavLi href="/">Contact</NavLi>
</Navbar>
