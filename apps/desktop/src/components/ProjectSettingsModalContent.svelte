<script lang="ts">
	import CloudForm from '$components/CloudForm.svelte';
	import GitForm from '$components/GitForm.svelte';
	import PreferencesForm from '$components/PreferencesForm.svelte';
	import SettingsModalLayout from '$components/SettingsModalLayout.svelte';
	import AgentSettings from '$components/projectSettings/AgentSettings.svelte';
	import GeneralSettings from '$components/projectSettings/GeneralSettings.svelte';
	import { projectDisableCodegen } from '$lib/config/config';
	import iconsJson from '@gitbutler/ui/data/icons.json';
	import type { ProjectSettingsModalState } from '$lib/state/uiState.svelte';

	type Props = {
		data: ProjectSettingsModalState;
	};

	const { data }: Props = $props();

	const codegenDisabled = $derived(projectDisableCodegen(data.projectId));

	const allPages = [
		{
			id: 'project',
			label: '项目',
			icon: 'profile' as keyof typeof iconsJson
		},
		{
			id: 'git',
			label: 'Git 设置',
			icon: 'git' as keyof typeof iconsJson
		},
		{
			id: 'ai',
			label: 'AI 选项',
			icon: 'ai' as keyof typeof iconsJson
		},
		{
			id: 'agent',
			label: 'Agent',
			icon: 'ai-agent' as keyof typeof iconsJson,
			requireCodegen: true
		},
		{
			id: 'experimental',
			label: '实验性',
			icon: 'idea' as keyof typeof iconsJson
		}
	];

	const pages = $derived(allPages.filter((page) => !page.requireCodegen || !$codegenDisabled));

	let currentSelectedId = $derived(data.selectedId || pages.at(0)?.id);

	function selectPage(pageId: string) {
		currentSelectedId = pageId;
	}
</script>

<SettingsModalLayout
	title="项目设置"
	{pages}
	selectedId={data.selectedId}
	onSelectPage={selectPage}
>
	{#snippet content({ currentPage })}
		{#if currentPage}
			{#if currentPage.id === 'project'}
				<GeneralSettings projectId={data.projectId} />
			{:else if currentPage.id === 'git'}
				<GitForm projectId={data.projectId} />
			{:else if currentPage.id === 'ai'}
				<CloudForm projectId={data.projectId} />
			{:else if currentPage.id === 'agent'}
				<AgentSettings />
			{:else if currentPage.id === 'experimental'}
				<PreferencesForm projectId={data.projectId} />
			{:else}
				未找到设置页面：{currentPage.id}。
			{/if}
		{:else}
			未找到设置页面：{currentSelectedId}。
		{/if}
	{/snippet}
</SettingsModalLayout>
