<script lang="ts">
	import { Button, TestId } from '@gitbutler/ui';

	interface Props {
		showFrame?: boolean;
		handleShow: () => void;
	}

	const { handleShow, showFrame = false }: Props = $props();

	function show() {
		handleShow();
	}
</script>

<div data-testid={TestId.LargeDiffMessage} class="large-diff-message" class:frame-box={showFrame}>
	<p class="text-13">为避免大型差异拖慢界面，已隐藏更改</p>
	<Button testId={TestId.LargeDiffMessageButton} kind="outline" onclick={show}>仍然显示</Button>
</div>

<style>
	.large-diff-message {
		display: flex;
		flex-direction: column;
		padding: 12px;
		gap: 12px;
		border: 1px solid var(--clr-border-2);
		border-radius: var(--radius-m);
	}

	.frame-box {
		border: 1px solid var(--clr-border-2);
		border-radius: var(--radius-m);
	}
</style>
