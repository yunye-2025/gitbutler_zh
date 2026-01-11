<script lang="ts">
	import SettingsModalLayout from '$components/SettingsModalLayout.svelte';

	import AiSettings from '$components/profileSettings/AiSettings.svelte';
	import ExperimentalSettings from '$components/profileSettings/ExperimentalSettings.svelte';
	import GeneralSettings from '$components/profileSettings/GeneralSettings.svelte';
	import GitSettings from '$components/profileSettings/GitSettings.svelte';
	import IntegrationsSettings from '$components/profileSettings/IntegrationsSettings.svelte';
	import LanesAndBranchesSettings from '$components/profileSettings/LanesAndBranchesSettings.svelte';
	import OrganisationSettings from '$components/profileSettings/OrganisationSettings.svelte';
	import TelemetrySettings from '$components/profileSettings/TelemetrySettings.svelte';
	import AppearanceSettings from '$components/projectSettings/AppearanceSettings.svelte';
	import { USER_SERVICE } from '$lib/user/userService';
	import { URL_SERVICE } from '$lib/utils/url';
	import { inject } from '@gitbutler/core/context';
	import { Icon } from '@gitbutler/ui';
	import iconsJson from '@gitbutler/ui/data/icons.json';
	import type { GeneralSettingsModalState } from '$lib/state/uiState.svelte';

	type Props = {
		data: GeneralSettingsModalState;
	};

	const { data }: Props = $props();

	const userService = inject(USER_SERVICE);
	const user = userService.user;
	const urlService = inject(URL_SERVICE);

	const pages = [
		{
			id: 'general',
			label: '常规',
			icon: 'settings' as keyof typeof iconsJson
		},
		{
			id: 'appearance',
			label: '外观',
			icon: 'appearance' as keyof typeof iconsJson
		},
		{
			id: 'lanes-and-branches',
			label: '泳道与分支',
			icon: 'lanes' as keyof typeof iconsJson
		},
		{
			id: 'git',
			label: 'Git 设置',
			icon: 'git' as keyof typeof iconsJson
		},
		{
			id: 'integrations',
			label: '集成',
			icon: 'integrations' as keyof typeof iconsJson
		},
		{
			id: 'ai',
			label: 'AI 选项',
			icon: 'ai' as keyof typeof iconsJson
		},
		{
			id: 'telemetry',
			label: '遥测',
			icon: 'stat' as keyof typeof iconsJson
		},
		{
			id: 'experimental',
			label: '实验功能',
			icon: 'idea' as keyof typeof iconsJson
		},
		{
			id: 'organizations',
			label: '组织',
			icon: 'idea' as keyof typeof iconsJson,
			adminOnly: true
		}
	];

	let currentSelectedId = $state(data.selectedId || pages[0]!.id);

	function selectPage(pageId: string) {
		currentSelectedId = pageId;
	}
</script>

<SettingsModalLayout
	title="全局设置"
	{pages}
	selectedId={data.selectedId}
	isAdmin={$user?.role === 'admin'}
	onSelectPage={selectPage}
>
	{#snippet content({ currentPage })}
		{#if currentPage}
			{#if currentPage.id === 'general'}
				<GeneralSettings />
			{:else if currentPage.id === 'appearance'}
				<AppearanceSettings />
			{:else if currentPage.id === 'lanes-and-branches'}
				<LanesAndBranchesSettings />
			{:else if currentPage.id === 'git'}
				<GitSettings />
			{:else if currentPage.id === 'integrations'}
				<IntegrationsSettings />
			{:else if currentPage.id === 'ai'}
				<AiSettings />
			{:else if currentPage.id === 'telemetry'}
				<TelemetrySettings />
			{:else if currentPage.id === 'experimental'}
				<ExperimentalSettings />
			{:else if currentPage.id === 'organizations'}
				<OrganisationSettings />
			{:else}
				未找到设置页面 {currentPage.id}。
			{/if}
		{:else}
			未找到设置页面 {currentSelectedId}。
		{/if}
	{/snippet}

	{#snippet footer()}
		<div class="social">
			<button
				type="button"
				class="social-btn"
				onclick={async () => await urlService.openExternalUrl('https://docs.gitbutler.com/')}
			>
				<Icon name="docs" />
				<span class="text-13 text-bold">文档</span>
				<div class="text-13 open-link-icon">↗</div>
			</button>
			<button
				type="button"
				class="social-btn"
				onclick={async () => await urlService.openExternalUrl('https://discord.gg/MmFkmaJ42D')}
			>
				<Icon name="discord" />
				<span class="text-13 text-bold">官方 Discord</span>
				<div class="text-13 open-link-icon">↗</div>
			</button>
		</div>
	{/snippet}
</SettingsModalLayout>

<style lang="postcss">
	/* BANNERS */
	.social {
		display: flex;
		flex-direction: column;
		gap: 2px;
	}

	.social-btn {
		display: flex;
		align-items: center;
		padding: 8px 12px;
		gap: 12px;
		border-radius: var(--radius-m);
		background-color: var(--clr-bg-1);
		color: var(--clr-text-2);
		text-align: left;
		transition: all var(--transition-fast);

		&:hover {
			background-color: var(--hover-bg-1);
		}
	}

	.open-link-icon {
		transform: translateY(-2px) translateX(-4px);
		color: var(--clr-text-3);
	}
</style>
