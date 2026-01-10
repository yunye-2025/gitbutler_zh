<script lang="ts">
	import { goto } from '$app/navigation';
	import { handleAddProjectOutcome } from '$lib/project/project';
	import { PROJECTS_SERVICE } from '$lib/project/projectsService';
	import { projectPath } from '$lib/routes/routes.svelte';
	import { inject } from '@gitbutler/core/context';
	import { Button, OptionsGroup, Select, SelectItem } from '@gitbutler/ui';

	const { projectId }: { projectId?: string } = $props();

	const projectsService = inject(PROJECTS_SERVICE);
	const projectsQuery = $derived(projectsService.projects());

	let selectedId = $state<string | undefined>(projectId);

	const mappedProjects = $derived(
		projectsQuery.response?.map((project) => ({
			value: project.id,
			label: project.title
		})) || []
	);

	let newProjectLoading = $state(false);
	let cloneProjectLoading = $state(false);
</script>

<div class="project-switcher">
	<Select
		value={selectedId}
		options={mappedProjects}
		label="切换到其他项目"
		wide
		onselect={(value) => {
			selectedId = value;
		}}
		searchable
	>
		{#snippet itemSnippet({ item, highlighted })}
			<SelectItem selected={item.value === selectedId} {highlighted}>
				{item.label}
			</SelectItem>
		{/snippet}

		<OptionsGroup>
			<SelectItem
				icon="plus"
				loading={newProjectLoading}
				onClick={async () => {
					newProjectLoading = true;
					try {
						const outcome = await projectsService.addProject();
						if (!outcome) {
							// 用户已取消创建项目
							newProjectLoading = false;
							return;
						}
						handleAddProjectOutcome(outcome, (project) => goto(projectPath(project.id)));
					} finally {
						newProjectLoading = false;
					}
				}}
			>
				添加本地仓库
			</SelectItem>
			<SelectItem
				icon="clone"
				loading={cloneProjectLoading}
				onClick={async () => {
					cloneProjectLoading = true;
					try {
						goto('/onboarding/clone');
					} finally {
						cloneProjectLoading = false;
					}
				}}
			>
				克隆仓库
			</SelectItem>
		</OptionsGroup>
	</Select>

	<Button
		style="pop"
		icon="chevron-right-small"
		disabled={selectedId === projectId}
		onclick={() => {
			if (selectedId) goto(projectPath(selectedId));
		}}
	>
		打开项目
	</Button>
</div>

<style lang="postcss">
	.project-switcher {
		display: flex;
		flex-direction: column;
		align-items: flex-end;
		gap: 10px;
	}
</style>
