<script lang="ts">
	import { Button, Modal, TestId } from '@gitbutler/ui';

	interface Props {
		projectTitle?: string;
		isDeleting?: boolean;
		noModal?: boolean;
		outlineStyle?: boolean;
		onDeleteClicked: () => Promise<void>;
	}

	const {
		projectTitle = '#',
		isDeleting,
		noModal,
		outlineStyle,
		onDeleteClicked
	}: Props = $props();

	export function show() {
		modal?.show();
	}
	export function close() {
		modal?.close();
	}

	function handleClick() {
		if (noModal) {
			onDeleteClicked();
		} else {
			modal?.show();
		}
	}

	let modal = $state<Modal>();
</script>

<Button
	testId={TestId.ProjectDeleteButton}
	style="danger"
	kind={outlineStyle ? 'outline' : 'solid'}
	icon="bin-small"
	reversedDirection
	onclick={handleClick}
>
	移除项目…
</Button>

<Modal
	bind:this={modal}
	width="small"
	onSubmit={(close) => {
		onDeleteClicked().then(close);
	}}
>
	<div class="remove-project-description">
		<p class="text-14 text-body">
			确定要从 GitButler 中移除
			<span class="text-bold">{projectTitle}</span> 吗？
		</p>

		<p class="text-12 text-body details-text">
			从 GitButler 删除项目不会删除你的仓库，只会将项目从列表中移除，仓库仍然安全可用。
		</p>
	</div>

	{#snippet controls()}
		<Button
			testId={TestId.ProjectDeleteModalConfirm}
			style="danger"
			kind="outline"
			reversedDirection
			loading={isDeleting}
			icon="bin-small"
			type="submit"
		>
			移除
		</Button>
		<Button style="pop" onclick={close}>取消</Button>
	{/snippet}
</Modal>

<style lang="postcss">
	.remove-project-description {
		display: flex;
		flex-direction: column;
		gap: 8px;
	}

	.details-text {
		opacity: 0.5;
	}
</style>
