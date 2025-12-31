<script lang="ts" module>
	export type AddDependentBranchModalProps = {
		projectId: string;
		stackId: string;
	};
</script>

<script lang="ts">
	import BranchNameTextbox from '$components/BranchNameTextbox.svelte';
	import { STACK_SERVICE } from '$lib/stacks/stackService.svelte';
	import { inject } from '@gitbutler/core/context';
	import { Button, Modal, TestId } from '@gitbutler/ui';

	const { projectId, stackId }: AddDependentBranchModalProps = $props();

	const stackService = inject(STACK_SERVICE);
	const [createNewBranch, branchCreation] = stackService.newBranch;

	let modal = $state<Modal>();
	let branchName = $state<string>();
	let slugifiedRefName: string | undefined = $state();

	async function handleAddDependentBranch(close: () => void) {
		if (!slugifiedRefName) return;

		await createNewBranch({
			projectId,
			stackId,
			request: {
				targetPatch: undefined,
				name: slugifiedRefName
			}
		});

		close();
	}

	export function show() {
		modal?.show();
	}
</script>

<Modal
	testId={TestId.BranchHeaderAddDependanttBranchModal}
	bind:this={modal}
	width="small"
	title="添加依赖分支"
	onSubmit={handleAddDependentBranch}
>
	<div class="content-wrap">
		<BranchNameTextbox
			placeholder="分支名称"
			bind:value={branchName}
			autofocus
			onslugifiedvalue={(value) => (slugifiedRefName = value)}
		/>
	</div>
	{#snippet controls(close)}
		<Button kind="outline" type="reset" onclick={close}>取消</Button>
		<Button
			testId={TestId.BranchHeaderAddDependanttBranchModal_ActionButton}
			style="pop"
			type="submit"
			disabled={!slugifiedRefName}
			loading={branchCreation.current.isLoading}>添加分支</Button
		>
	{/snippet}
</Modal>
