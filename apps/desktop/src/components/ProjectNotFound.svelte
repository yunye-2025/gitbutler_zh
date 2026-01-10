<script lang="ts">
	import { goto } from '$app/navigation';
	import DecorativeSplitView from '$components/DecorativeSplitView.svelte';
	import ProjectSwitcher from '$components/ProjectSwitcher.svelte';
	import ReduxResult from '$components/ReduxResult.svelte';
	import RemoveProjectButton from '$components/RemoveProjectButton.svelte';
	import notFoundSvg from '$lib/assets/illustrations/not-found.svg?raw';
	import { PROJECTS_SERVICE } from '$lib/project/projectsService';
	import { inject } from '@gitbutler/core/context';
	import { Button, InfoMessage, type MessageStyle, Spacer, TestId } from '@gitbutler/ui';

	interface Props {
		projectId: string;
	}
	const { projectId }: Props = $props();

	const projectsService = inject(PROJECTS_SERVICE);
	const projectQuery = $derived(projectsService.getProject(projectId, true));

	let deleteSucceeded: boolean | undefined = $state(undefined);
	let isDeleting = $state(false);

	async function stopTracking(id: string) {
		isDeleting = true;
		deleteProject: {
			try {
				await projectsService.deleteProject(id);
			} catch {
				deleteSucceeded = false;
				break deleteProject;
			}
			deleteSucceeded = true;
		}
		isDeleting = false;
		goto('/');
	}

	async function locate(id: string) {
		await projectsService.relocateProject(id);
	}

	interface DeletionStatus {
		message: string;
		style: MessageStyle;
	}

	function getDeletionStatus(repoName: string, deleteSucceeded: boolean): DeletionStatus {
		return deleteSucceeded
			? { message: `已成功删除项目“${repoName}”`, style: 'success' }
			: { message: `删除项目“${repoName}”失败`, style: 'danger' };
	}
</script>

<DecorativeSplitView testId={TestId.ProjectNotFoundPage} img={notFoundSvg}>
	<div class="container">
		<ReduxResult {projectId} result={projectQuery.result}>
			{#snippet children(project)}
				{#if deleteSucceeded === undefined}
					<div class="text-content">
						<h2 class="title-text text-18 text-body text-bold">
							找不到“{project.title}”
						</h2>

						<p class="description-text text-13 text-body">
							抱歉，未找到你要的项目。
							<br />
							它可能已被移除或不存在。
							<button type="button" class="check-again-btn" onclick={() => location.reload()}
								>点击这里</button
							>
							再次检查。
							<br />
							当前项目路径：<span class="code-string">{project.path}</span>
						</p>
					</div>

					<div class="button-container">
						<Button type="button" style="pop" onclick={async () => await locate(projectId)}
							>定位项目…</Button
						>
						<RemoveProjectButton
							noModal
							{isDeleting}
							onDeleteClicked={async () => await stopTracking(project.id)}
						/>
					</div>
				{/if}

				{#if deleteSucceeded !== undefined}
					{@const deletionStatus = getDeletionStatus(project.title, deleteSucceeded)}
					<InfoMessage filled outlined={false} style={deletionStatus.style} icon="info">
						{#snippet content()}
							{deletionStatus.message}
						{/snippet}
					</InfoMessage>
				{/if}
			{/snippet}
		</ReduxResult>

		<Spacer dotted margin={0} />
		<ProjectSwitcher {projectId} />
	</div>
</DecorativeSplitView>

<style lang="postcss">
	.container {
		display: flex;
		flex-direction: column;
		gap: 20px;
	}

	.button-container {
		display: flex;
		gap: 8px;
	}

	.text-content {
		display: flex;
		flex-direction: column;
		gap: 12px;
	}

	.title-text {
		color: var(--clr-text-1);
	}

	.description-text {
		color: var(--clr-text-2);
		line-height: 1.6;
	}

	.check-again-btn {
		text-decoration: underline;
	}
</style>
