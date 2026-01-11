<script lang="ts">
	import { Modal, Button } from '@gitbutler/ui';

	type Props = {
		fileName: string;
		onConfirm: () => void;
		onCancel: () => void;
	};

	const { fileName, onConfirm, onCancel }: Props = $props();

	let modal: Modal | undefined = $state();

	export function show() {
		modal?.show();
	}

	export function hide() {
		modal?.close();
	}
</script>

<Modal bind:this={modal} width="small" type="warning" title="先解决冲突再预览">
	<p class="text-base-body-13 text-light">
		文件 <span class="text-bold">{fileName}</span> 存在未解决的合并冲突，需先处理后才能预览。
	</p>

	{#snippet controls()}
		<Button kind="outline" onclick={onCancel}>取消</Button>
		<Button style="pop" onclick={onConfirm}>解决冲突</Button>
	{/snippet}
</Modal>
