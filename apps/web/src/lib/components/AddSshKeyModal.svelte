<script lang="ts">
	import { SSH_KEY_SERVICE } from '$lib/sshKeyService';
	import { inject } from '@gitbutler/core/context';
	import { AsyncButton, Button, Modal, Textarea, Textbox } from '@gitbutler/ui';

	const sshKeyService = inject(SSH_KEY_SERVICE);
	let name = $state('');
	let publicKey = $state('');
	let error = $state<string | null>(null);

	const { onClose } = $props<{
		onClose: () => void;
	}>();

	async function handleSubmit() {
		if (!name.trim() || !publicKey.trim()) {
			error = '请填写所有字段';
			return;
		}

		error = null;

		try {
			await sshKeyService.addSshKey(name.trim(), publicKey.trim());
			// Close modal and reset form
			name = '';
			publicKey = '';
			modal?.close();
			onClose();
		} catch (err) {
			console.error('添加 SSH 密钥失败：', err);
			error = '添加 SSH 密钥失败，请重试。';
		}
	}

	let modal = $state<Modal>();

	export function show() {
		modal?.show();
	}
</script>

<Modal bind:this={modal} {onClose} title="添加 SSH 密钥">
	<div class="container">
		<p class="description">
			为你的账号添加新的 SSH 密钥。你可以在 SSH 密钥文件中找到公钥（通常以 .pub 结尾）。
		</p>

		<Textbox label="密钥名称" placeholder="例如：MacBook Pro" bind:value={name} required={false} />

		<Textarea
			label="公钥"
			placeholder="ssh-rsa AAAAB3NzaC1yc2EAAAADA..."
			bind:value={publicKey}
			minRows={6}
			required={false}
		/>

		{#if error}
			<div class="error-key">{error}</div>
		{/if}
	</div>

	{#snippet controls()}
		<Button onclick={() => modal?.close()} kind="outline">取消</Button>
		<AsyncButton action={handleSubmit} style="pop">添加密钥</AsyncButton>
	{/snippet}
</Modal>

<style lang="postcss">
	.error-key {
		color: red;
	}

	.container {
		display: flex;
		flex-direction: column;
		gap: 20px;
	}

	.description {
		color: var(--clr-text-2);
		font-size: 14px;
		line-height: 1.5;
	}
</style>
