<script lang="ts">
	import ClaudeCheck from '$components/codegen/ClaudeCheck.svelte';
	import { SETTINGS_SERVICE } from '$lib/config/appSettingsV2';
	import { newlineOnEnter } from '$lib/config/uiFeatureFlags';
	import { inject } from '@gitbutler/core/context';
	import { CardGroup, Link, Spacer, Toggle } from '@gitbutler/ui';

	const settingsService = inject(SETTINGS_SERVICE);
	const settingsStore = settingsService.appSettings;

	// Agent settings state
	let notifyOnCompletion = $state(false);
	let notifyOnPermissionRequest = $state(false);
	let dangerouslyAllowAllPermissions = $state(false);
	let autoCommitAfterCompletion = $state(true);
	let useConfiguredModel = $state(false);

	// Initialize Claude settings from store
	$effect(() => {
		if ($settingsStore?.claude) {
			notifyOnCompletion = $settingsStore.claude.notifyOnCompletion;
			notifyOnPermissionRequest = $settingsStore.claude.notifyOnPermissionRequest;
			dangerouslyAllowAllPermissions = $settingsStore.claude.dangerouslyAllowAllPermissions;
			autoCommitAfterCompletion = $settingsStore.claude.autoCommitAfterCompletion;
			useConfiguredModel = $settingsStore.claude.useConfiguredModel;
		}
	});

	async function updateNotifyOnCompletion(value: boolean) {
		notifyOnCompletion = value;
		await settingsService.updateClaude({ notifyOnCompletion: value });
	}

	async function updateNotifyOnPermissionRequest(value: boolean) {
		notifyOnPermissionRequest = value;
		await settingsService.updateClaude({ notifyOnPermissionRequest: value });
	}

	async function updateDangerouslyAllowAllPermissions(value: boolean) {
		dangerouslyAllowAllPermissions = value;
		await settingsService.updateClaude({ dangerouslyAllowAllPermissions: value });
	}

	async function updateAutoCommitAfterCompletion(value: boolean) {
		autoCommitAfterCompletion = value;
		await settingsService.updateClaude({ autoCommitAfterCompletion: value });
	}

	async function updateUseConfiguredModel(value: boolean) {
		useConfiguredModel = value;
		await settingsService.updateClaude({ useConfiguredModel: value });
	}
</script>

<CardGroup.Item standalone>
	<ClaudeCheck showTitle />
</CardGroup.Item>

<p class="text-13 text-body clr-text-2">
	在 <Link href="https://docs.gitbutler.com/features/agents-tab#installing-claude-code">文档</Link>
	中查看 GitButler 智能体完整指南。
</p>

<Spacer margin={10} dotted />

<CardGroup.Item standalone labelFor="autoCommitAfterCompletion">
	{#snippet title()}
		完成后自动提交
	{/snippet}
	{#snippet caption()}
		Claude Code 完成后自动提交并重命名分支。关闭后可在提交前手动检查。
	{/snippet}
	{#snippet actions()}
		<Toggle
			id="autoCommitAfterCompletion"
			checked={autoCommitAfterCompletion}
			onchange={updateAutoCommitAfterCompletion}
		/>
	{/snippet}
</CardGroup.Item>

<CardGroup.Item standalone labelFor="useConfiguredModel">
	{#snippet title()}
		使用已配置的模型
	{/snippet}
	{#snippet caption()}
		使用 .claude/settings.json 中配置的模型。
	{/snippet}
	{#snippet actions()}
		<Toggle
			id="useConfiguredModel"
			checked={useConfiguredModel}
			onchange={updateUseConfiguredModel}
		/>
	{/snippet}
</CardGroup.Item>

<CardGroup.Item standalone labelFor="newlineOnEnter">
	{#snippet title()}
		按 Enter 换行
	{/snippet}
	{#snippet caption()}
		使用 Enter 换行，Cmd+Enter 提交。
	{/snippet}
	{#snippet actions()}
		<Toggle
			id="newlineOnEnter"
			checked={$newlineOnEnter}
			onchange={() => newlineOnEnter.set(!$newlineOnEnter)}
		/>
	{/snippet}
</CardGroup.Item>

<CardGroup>
	<CardGroup.Item labelFor="notifyOnCompletion">
		{#snippet title()}
			完成时通知
		{/snippet}
		{#snippet actions()}
			<Toggle
				id="notifyOnCompletion"
				checked={notifyOnCompletion}
				onchange={updateNotifyOnCompletion}
			/>
		{/snippet}
	</CardGroup.Item>
	<CardGroup.Item labelFor="notifyOnPermissionRequest">
		{#snippet title()}
			需要权限时通知
		{/snippet}
		{#snippet actions()}
			<Toggle
				id="notifyOnPermissionRequest"
				checked={notifyOnPermissionRequest}
				onchange={updateNotifyOnPermissionRequest}
			/>
		{/snippet}
	</CardGroup.Item>
</CardGroup>

<Spacer margin={10} dotted />

<CardGroup.Item standalone labelFor="dangerouslyAllowAllPermissions">
	{#snippet title()}
		⚠ 危险：允许所有权限
	{/snippet}
	{#snippet caption()}
		跳过所有权限提示，允许 Claude Code 不受限制地访问。请谨慎使用。
	{/snippet}
	{#snippet actions()}
		<Toggle
			id="dangerouslyAllowAllPermissions"
			checked={dangerouslyAllowAllPermissions}
			onchange={updateDangerouslyAllowAllPermissions}
		/>
	{/snippet}
</CardGroup.Item>
