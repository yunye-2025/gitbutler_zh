<script lang="ts" module>
	export type DeleteBranchModalProps = {
		projectId: string;
		stackId?: string;
		branchName: string;
	};
</script>

<script lang="ts">
	import { STACK_SERVICE } from '$lib/stacks/stackService.svelte';
	import { inject } from '@gitbutler/core/context';
	import { Button, Modal, TestId } from '@gitbutler/ui';

	const { projectId, stackId, branchName }: DeleteBranchModalProps = $props();
	const stackService = inject(STACK_SERVICE);
	const [removeBranch, branchRemovalOp] = stackService.removeBranch;

	let modal = $state<Modal>();

	export function show() {
		modal?.show();
	}
</script>

<Modal
	testId={TestId.BranchHeaderDeleteModal}
	bind:this={modal}
	width="small"
	title="删除分支"
	onSubmit={async (close) => {
		await removeBranch({
			projectId,
			stackId,
			branchName
		});
		close();
	}}
>
	<p class="text-13 text-body">
		确定要删除 <code class="code-string">{branchName}</code> 吗？
	</p>
	{#snippet controls(close)}
		<Button kind="outline" onclick={close} autofocus>取消</Button>
		<Button
			testId={TestId.BranchHeaderDeleteModal_ActionButton}
			style="danger"
			type="submit"
			loading={branchRemovalOp.current.isLoading}>删除</Button
		>
	{/snippet}
</Modal>
