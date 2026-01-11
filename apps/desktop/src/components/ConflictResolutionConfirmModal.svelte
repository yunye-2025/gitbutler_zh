<script lang="ts">
	import { AsyncButton, Button, Modal } from '@gitbutler/ui';

	interface Props {
		onSubmit: () => void;
	}

	const { onSubmit }: Props = $props();

	let modalEl = $state<ReturnType<typeof Modal>>();

	export function show() {
		modalEl?.show();
	}
	export function close() {
		modalEl?.close();
	}
</script>

<Modal bind:this={modalEl} width="small">
	<div>
		<p>通常从最下方开始解决冲突会更好。</p>
		<br />
		<p>确定要解决此提交的冲突吗？</p>
	</div>
	{#snippet controls(close)}
		<Button kind="outline" type="reset" onclick={close}>取消</Button>
		<AsyncButton
			style="pop"
			action={async () => {
				await onSubmit();
				close();
			}}>是</AsyncButton
		>
	{/snippet}
</Modal>
