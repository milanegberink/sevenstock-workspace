<script>
	let { children } = $props();
	import { VerticalSeparator } from '$lib/separators/index.js';
	import NavLink from './nav-link.svelte';
	import MenuSpace from './menu-space.svelte';
	import KeyBold from '~icons/ph/key-bold';
	import SettingsIcon from '~icons/ph/gear-six-bold';
	import QuestionBold from '~icons/ph/question-bold';
</script>

<div class="flex h-full w-56">
	<div class="flex h-full flex-1 flex-col">
		<MenuSpace name="Accessibility">
			<ul class="border-primary flex flex-1 flex-col gap-1">
				<NavLink href={'/settings'} text={'API keys'}>
					{#snippet icon()}
						<KeyBold />
					{/snippet}
				</NavLink>
				<NavLink href={'/help'} text={'Help'}>
					{#snippet icon()}
						<QuestionBold />
					{/snippet}
				</NavLink>
			</ul>
		</MenuSpace>
		<MenuSpace name="Security">
			<ul class="border-primary flex flex-1 flex-col gap-1">
				<NavLink href={'/settings/security'} text={'API keys'}>
					{#snippet icon()}
						<KeyBold />
					{/snippet}
				</NavLink>
				<NavLink href={'/help'} text={'Help'}>
					{#snippet icon()}
						<QuestionBold />
					{/snippet}
				</NavLink>
			</ul>
		</MenuSpace>
		<!-- {@render children()} -->
	</div>
	<VerticalSeparator />
</div>
