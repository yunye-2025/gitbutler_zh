<script lang="ts">
	import { GIT_SERVICE } from '$lib/git/gitService';
	import { inject } from '@gitbutler/core/context';
	import { TestId, ModalHeader, ModalFooter, Textbox, EmailTextbox, Button } from '@gitbutler/ui';
	import type { AuthorMissingModalState } from '$lib/state/uiState.svelte';

	type Props = {
		data: AuthorMissingModalState;
		close: () => void;
	};

	const { data, close }: Props = $props();

	const gitService = inject(GIT_SERVICE);
	const [setAuthorInfo, settingInfo] = gitService.setAuthorInfo;

	let name = $derived(data.authorName);
	let email = $derived(data.authorEmail);
	let emailTextbox: any;

	async function handleSubmit() {
		if (!name || !email) {
			return;
		}
		if (!emailTextbox.isValid()) {
			emailTextbox.validate();
			return;
		}
		await setAuthorInfo({
			projectId: data.projectId,
			name,
			email
		});
		close();
	}
</script>

<ModalHeader type="warning">设置你的 Git 作者信息</ModalHeader>
<div class="author-missing__content">
	你的提交需要作者信息来标识是谁做了更改。该信息会保存到全局 Git 配置中，并用于之后的所有提交。

	<Textbox
		disabled={settingInfo.current.isLoading}
		placeholder="你的全名"
		label="姓名"
		testId={TestId.GlobalModal_AuthorMissing_NameInput}
		bind:value={name}
		autofocus
	/>

	<EmailTextbox
		disabled={settingInfo.current.isLoading}
		placeholder="你的邮箱@example.com"
		label="邮箱地址"
		testId={TestId.GlobalModal_AuthorMissing_EmailInput}
		bind:value={email}
		bind:this={emailTextbox}
	/>
</div>
<ModalFooter>
	<Button kind="outline" onclick={close} disabled={settingInfo.current.isLoading}>取消</Button>
	<Button
		testId={TestId.GlobalModal_AuthorMissing_ActionButton}
		style="pop"
		onclick={handleSubmit}
		loading={settingInfo.current.isLoading}
		disabled={!name || !email}
	>
		{settingInfo.current.isLoading ? '正在保存...' : '保存并继续'}
	</Button>
</ModalFooter>

<style lang="postcss">
	.author-missing__content {
		display: flex;
		flex-direction: column;
		padding: 0 16px 16px 16px;
		gap: 16px;
	}
</style>
