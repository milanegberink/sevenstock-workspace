<script lang="ts">
	import QuestionBold from '~icons/ph/question';
	import KeyBold from '~icons/ph/key';
	import UserIcon from '~icons/ph/user';
	import AppWindow from '~icons/ph/translate';
	import Building from '~icons/ph/building';
	import { userStore } from '@lib/core/stores';

	import AccessibilityIcon from '~icons/ph/person-arms-spread';
	import { CloseButton, Header, VerticalSeparator, Input } from '$lib/index.js';
	import Link from './link.svelte';
	import { fly } from 'svelte/transition';

	let { close } = $props();

	let currentPage = $state(account);

	const pages = [
		{
			name: 'Account',
			icon: UserIcon,
			page: account
		},
		{
			name: 'Appearance',
			icon: AppWindow,
			page: appearance
		},
		{
			name: 'Organisation',
			icon: Building,
			page: organisation
		},
		{
			name: 'Developer',
			icon: KeyBold,
			page: developer
		},
		{
			name: 'Accessibility',
			icon: AccessibilityIcon,
			page: accessibility
		},
		{
			name: 'Help',
			icon: QuestionBold,
			page: help
		}
	];
</script>

<div class="h-[70vh] w-[70vw] overflow-hidden rounded-2xl bg-white">
	<Header name="Settings">
		<CloseButton onclick={close} />
	</Header>

	<div class="flex h-full">
		<ul class="flex w-52 flex-col gap-1 p-2">
			{#each pages as tab}
				{@const isActive = tab.page === currentPage}
				<Link {isActive} onclick={() => (currentPage = tab.page)} text={tab.name}>
					{#snippet icon()}
						<tab.icon />
					{/snippet}
				</Link>
			{/each}
		</ul>
		<VerticalSeparator />

		<!-- Settings page content -->
		<div class="flex-1">
			{#key currentPage}
				<div in:fly={{ y: 10, duration: 100 }} class="flex flex-1 flex-col p-4">
					{@render currentPage()}
				</div>
			{/key}
		</div>
	</div>
</div>

{#snippet account()}
	<!-- {@render setting_info('Account', 'Change account information')}
	<Input bind:value={userStore.user.email} readonly></Input>
	<Input bind:value={userStore.user.ident} readonly></Input> -->
{/snippet}

{#snippet setting_info(title: string, description: string)}
	<span class="text-lg font-medium">{title}</span>
	<span class="text-gray-500">{description}</span>
{/snippet}

{#snippet appearance()}
	{@render setting_info('Account', 'Change account information')}
{/snippet}

{#snippet organisation()}
	<h2>First page</h2>
{/snippet}
{#snippet developer()}
	{@render setting_info('API keys', 'Generate a new API key')}
	{@render setting_info('Web hooks', 'Coming soon...')}
{/snippet}

{#snippet accessibility()}
	{@render setting_info('API key', 'Generate a new API key')}
{/snippet}

{#snippet help()}
	<h2>second page</h2>
{/snippet}
