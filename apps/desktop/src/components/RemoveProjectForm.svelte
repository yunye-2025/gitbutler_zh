<script lang="ts">
	import { goto } from '$app/navigation';
	import ReduxResult from '$components/ReduxResult.svelte';
	import RemoveProjectButton from '$components/RemoveProjectButton.svelte';
	import { showError } from '$lib/notifications/toasts';
	import { PROJECTS_SERVICE } from '$lib/project/projectsService';
	import { useSettingsModal } from '$lib/settings/settingsModal.svelte';
	import { inject } from '@gitbutler/core/context';

	import { CardGroup, chipToasts } from '@gitbutler/ui';

	const { projectId }: { projectId: string } = $props();

	const projectsService = inject(PROJECTS_SERVICE);
	const projectQuery = $derived(projectsService.getProject(projectId));
	const { closeSettings } = useSettingsModal();

	let isDeleting = $state(false);

	async function onDeleteClicked() {
		isDeleting = true;
		try {
			await projectsService.deleteProject(projectId);
			closeSettings();
			goto('/');
			chipToasts.success('项目已删除');
		} catch (err: any) {
			console.error(err);
			showError('删除项目失败', err);
		} finally {
			isDeleting = false;
		}
	}
</script>

<ReduxResult {projectId} result={projectQuery.result}>
	{#snippet children(project)}
		<CardGroup.Item standalone>
			{#snippet title()}
				移除项目
			{/snippet}
			{#snippet caption()}
				移除项目只会清除配置，你的代码仍然安全。
			{/snippet}

			<div>
				<RemoveProjectButton projectTitle={project.title} {isDeleting} {onDeleteClicked} />
			</div>
		</CardGroup.Item>
	{/snippet}
</ReduxResult>
