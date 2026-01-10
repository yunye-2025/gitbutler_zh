<script lang="ts" module>
	export type CreateSnapshotModalProps = {
		projectId: string;
	};
</script>

<script lang="ts">
	import { HISTORY_SERVICE } from '$lib/history/history';
	import { inject } from '@gitbutler/core/context';
	import { Button, ElementId, Modal, TestId, Textbox } from '@gitbutler/ui';

	const { projectId }: CreateSnapshotModalProps = $props();

	const historyService = inject(HISTORY_SERVICE);

	let message: string = $state('');
	let modal: Modal | undefined = $state();
	let isCreating = $state(false);

	export function show() {
		message = '';
		modal?.show();
	}

	async function createSnapshot(close: () => void) {
		if (isCreating) return;

		try {
			isCreating = true;
			await historyService.createSnapshot(projectId, message || undefined);
			close();
		} catch (error) {
			console.error('创建快照失败:', error);
		} finally {
			isCreating = false;
		}
	}
</script>

<Modal
	testId={TestId.CreateSnapshotModal}
	width="small"
	title="创建快照"
	type="info"
	bind:this={modal}
	onSubmit={createSnapshot}
>
	<Textbox
		placeholder="快照描述（可选）"
		id={ElementId.SnapshotDescriptionInput}
		bind:value={message}
		autofocus
		helperText="描述保存内容，便于日后查找"
	/>

	{#snippet controls(close)}
		<Button kind="outline" type="reset" onclick={close}>取消</Button>
		<Button
			testId={TestId.CreateSnapshotModal_ActionButton}
			style="pop"
			type="submit"
			loading={isCreating}
		>
			创建快照
		</Button>
	{/snippet}
</Modal>
