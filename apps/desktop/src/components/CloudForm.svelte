<script lang="ts">
	import AiPromptSelect from '$components/AIPromptSelect.svelte';
	import SettingsSection from '$components/SettingsSection.svelte';
	import WelcomeSigninAction from '$components/WelcomeSigninAction.svelte';
	import { projectAiExperimentalFeaturesEnabled, projectAiGenEnabled } from '$lib/config/config';
	import { useSettingsModal } from '$lib/settings/settingsModal.svelte';
	import { USER_SERVICE } from '$lib/user/userService';
	import { inject } from '@gitbutler/core/context';
	import { Button, CardGroup, Spacer, Toggle } from '@gitbutler/ui';

	const { projectId }: { projectId: string } = $props();

	const userService = inject(USER_SERVICE);
	const user = userService.user;
	const { openGeneralSettings } = useSettingsModal();

	const aiGenEnabled = $derived(projectAiGenEnabled(projectId));
	const experimentalAiGenEnabled = $derived(projectAiExperimentalFeaturesEnabled(projectId));
</script>

<SettingsSection>
	{#snippet description()}
		GitButler 支持使用 OpenAI 和 Anthropic 来生成提交信息和分支名称。此功能可以通过 GitButler 的 API
		使用，也可以使用自带密钥的配置，并可在主偏好设置界面中进行配置。
	{/snippet}

	<Spacer />

	{#if !$user}
		<WelcomeSigninAction />
		<Spacer />
	{/if}

	<CardGroup>
		<CardGroup.Item labelFor="aiGenEnabled">
			{#snippet title()}
				启用分支名与提交信息生成
			{/snippet}
			{#snippet caption()}
				启用后，点击“生成提交信息”和“生成分支名”按钮时，会将差异发送到 OpenAI 或 Anthropic 的服务器。
			{/snippet}
			{#snippet actions()}
				<Toggle
					id="aiGenEnabled"
					checked={$aiGenEnabled}
					onclick={() => {
						$aiGenEnabled = !$aiGenEnabled;
					}}
				/>
			{/snippet}
		</CardGroup.Item>
	</CardGroup>

	{#if $aiGenEnabled}
		<CardGroup>
			<CardGroup.Item labelFor="aiExperimental">
				{#snippet title()}
					启用实验性 AI 功能
				{/snippet}
				{#snippet caption()}
					启用后，你将能够使用当前正在开发的 AI 功能。要使这些功能生效，还需要通过 GitButler
					使用 OpenAI。
				{/snippet}
				{#snippet actions()}
					<Toggle
						id="aiExperimental"
						checked={$experimentalAiGenEnabled}
						onclick={() => {
							$experimentalAiGenEnabled = !$experimentalAiGenEnabled;
						}}
					/>
				{/snippet}
			</CardGroup.Item>
		</CardGroup>
	{/if}

	<CardGroup>
		<CardGroup.Item>
			{#snippet title()}
				自定义提示词
			{/snippet}

			<AiPromptSelect {projectId} promptUse="commits" />
			<AiPromptSelect {projectId} promptUse="branches" />

			<Spacer margin={8} />

			<p class="text-12 text-body">
				你可以为项目应用自定义提示词。默认情况下项目使用 GitButler 提示词，但你可以在通用设置中创建
				自己的提示词。
			</p>
			<Button kind="outline" icon="edit" onclick={() => openGeneralSettings('ai')}
				>自定义提示词</Button
			>
		</CardGroup.Item>
	</CardGroup>
</SettingsSection>
