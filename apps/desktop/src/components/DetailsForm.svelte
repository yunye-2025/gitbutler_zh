<script lang="ts">
	import ReduxResult from '$components/ReduxResult.svelte';
	import { projectRunCommitHooks } from '$lib/config/config';
	import { PROJECTS_SERVICE } from '$lib/project/projectsService';
	import { inject } from '@gitbutler/core/context';
	import { CardGroup, Spacer, Textarea, Textbox, Toggle } from '@gitbutler/ui';

	const { projectId }: { projectId: string } = $props();

	const projectsService = inject(PROJECTS_SERVICE);
	const projectQuery = $derived(projectsService.getProject(projectId));

	const runCommitHooks = $derived(projectRunCommitHooks(projectId));
</script>

<CardGroup>
	<ReduxResult {projectId} result={projectQuery.result}>
		{#snippet children(project)}
			<div class="fields-wrapper">
				<Textbox label="项目路径" readonly id="path" value={project?.path} />
				<div class="description-wrapper">
					<Textbox
						label="项目名称"
						id="name"
						placeholder="项目名称不能为空"
						value={project.title}
						required
						onchange={(value: string) => {
							projectsService.updateProject({ ...project, title: value });
						}}
					/>
					<Textarea
						id="description"
						minRows={3}
						maxRows={6}
						placeholder="项目描述"
						value={project.description}
						oninput={(e: Event) => {
							const target = e.currentTarget as HTMLTextAreaElement;
							projectsService.updateProject({ ...project, description: target.value });
						}}
					/>
				</div>
			</div>
		{/snippet}
	</ReduxResult>
</CardGroup>

<Spacer />

<CardGroup>
	<CardGroup.Item labelFor="runHooks">
		{#snippet title()}
			运行 Git 钩子
		{/snippet}
		{#snippet caption()}
			启用后将运行你在仓库中配置的 git pre-push、pre-commit、post-commit 和 commit-msg 钩子。
		{/snippet}
		{#snippet actions()}
			<Toggle id="runHooks" bind:checked={$runCommitHooks} />
		{/snippet}
	</CardGroup.Item>
</CardGroup>

<Spacer />

<style>
	.fields-wrapper {
		display: flex;
		flex-direction: column;
		padding: 16px;
		gap: 16px;
	}

	.description-wrapper {
		display: flex;
		flex-direction: column;
		gap: 8px;
	}
</style>
