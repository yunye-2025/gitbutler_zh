<script lang="ts">
	import AIPromptEdit from '$components/AIPromptEdit.svelte';
	import AiCredentialCheck from '$components/AiCredentialCheck.svelte';
	import AuthorizationBanner from '$components/AuthorizationBanner.svelte';
	import SettingsSection from '$components/SettingsSection.svelte';
	import { AISecretHandle, AI_SERVICE, GitAIConfigKey, KeyOption } from '$lib/ai/service';
	import { OpenAIModelName, AnthropicModelName, ModelKind } from '$lib/ai/types';
	import { GIT_CONFIG_SERVICE } from '$lib/config/gitConfigService';
	import { SECRET_SERVICE } from '$lib/secrets/secretsService';
	import { USER_SERVICE } from '$lib/user/userService';
	import { inject } from '@gitbutler/core/context';
	import {
		CardGroup,
		Icon,
		InfoMessage,
		Link,
		RadioButton,
		Select,
		SelectItem,
		Spacer,
		Textbox
	} from '@gitbutler/ui';

	import { onMount, tick } from 'svelte';
	import { run } from 'svelte/legacy';

	const gitConfigService = inject(GIT_CONFIG_SERVICE);
	const secretsService = inject(SECRET_SERVICE);
	const aiService = inject(AI_SERVICE);
	const userService = inject(USER_SERVICE);
	const user = userService.user;
	let initialized = false;

	let modelKind: ModelKind | undefined = $state();
	let openAIKeyOption: KeyOption | undefined = $state();
	let anthropicKeyOption: KeyOption | undefined = $state();
	let openAIKey: string | undefined = $state();
	let openAICustomEndpoint: string | undefined = $state();
	let openAIModelName: OpenAIModelName | undefined = $state();
	let anthropicKey: string | undefined = $state();
	let anthropicModelName: AnthropicModelName | undefined = $state();
	let diffLengthLimit: number | undefined = $state();
	let ollamaEndpoint: string | undefined = $state();
	let ollamaModel: string | undefined = $state();
	let lmStudioEndpoint: string | undefined = $state();
	let lmStudioModel: string | undefined = $state();

	async function setConfiguration(key: GitAIConfigKey, value: string | undefined) {
		if (!initialized) return;
		gitConfigService.set(key, value || '');
	}

	async function setSecret(handle: AISecretHandle, secret: string | undefined) {
		if (!initialized) return;
		await secretsService.set(handle, secret || '');
	}

	onMount(async () => {
		modelKind = await aiService.getModelKind();

		openAIKeyOption = await aiService.getOpenAIKeyOption();
		openAIModelName = await aiService.getOpenAIModleName();
		openAIKey = await aiService.getOpenAIKey();
		openAICustomEndpoint = await aiService.getOpenAICustomEndpoint();

		anthropicKeyOption = await aiService.getAnthropicKeyOption();
		anthropicModelName = await aiService.getAnthropicModelName();
		anthropicKey = await aiService.getAnthropicKey();

		diffLengthLimit = await aiService.getDiffLengthLimit();

		ollamaEndpoint = await aiService.getOllamaEndpoint();
		ollamaModel = await aiService.getOllamaModelName();

		lmStudioEndpoint = await aiService.getLMStudioEndpoint();
		lmStudioModel = await aiService.getLMStudioModelName();

		// Ensure reactive declarations have finished running before we set initialized to true
		await tick();

		initialized = true;
	});

	const keyOptions = [
		{
			label: '使用 GitButler API',
			value: KeyOption.ButlerAPI
		},
		{
			label: '自带密钥',
			value: KeyOption.BringYourOwn
		}
	];

	const openAIModelOptions = [
		{
			label: 'GPT 5',
			value: OpenAIModelName.GPT5
		},
		{
			label: 'GPT 5 Mini',
			value: OpenAIModelName.GPT5Mini
		},
		{
			label: 'o3 Mini',
			value: OpenAIModelName.O3mini
		},
		{
			label: 'o1 Mini',
			value: OpenAIModelName.O1mini
		},
		{
			label: 'GPT 4o mini',
			value: OpenAIModelName.GPT4oMini
		},
		{
			label: 'GPT 4.1',
			value: OpenAIModelName.GPT4_1
		},
		{
			label: 'GPT 4.1 mini (recommended)',
			value: OpenAIModelName.GPT4_1Mini
		}
	];

	const anthropicModelOptions = [
		{
			label: 'Haiku',
			value: AnthropicModelName.Haiku
		},
		{
			label: 'Sonnet 3.5',
			value: AnthropicModelName.Sonnet35
		},
		{
			label: 'Sonnet 3.7 (recommended)',
			value: AnthropicModelName.Sonnet37
		},
		{
			label: 'Sonnet 4',
			value: AnthropicModelName.Sonnet4
		},
		{
			label: 'Opus 4',
			value: AnthropicModelName.Opus4
		}
	];

	let form = $state<HTMLFormElement>();

	function onFormChange(form: HTMLFormElement) {
		const formData = new FormData(form);
		modelKind = formData.get('modelKind') as ModelKind;
	}
	run(() => {
		setConfiguration(GitAIConfigKey.ModelProvider, modelKind);
	});
	run(() => {
		setConfiguration(GitAIConfigKey.OpenAIKeyOption, openAIKeyOption);
	});
	run(() => {
		setConfiguration(GitAIConfigKey.OpenAIModelName, openAIModelName);
	});
	run(() => {
		setConfiguration(GitAIConfigKey.OpenAICustomEndpoint, openAICustomEndpoint);
	});
	run(() => {
		setSecret(AISecretHandle.OpenAIKey, openAIKey);
	});
	run(() => {
		setConfiguration(GitAIConfigKey.AnthropicKeyOption, anthropicKeyOption);
	});
	run(() => {
		setConfiguration(GitAIConfigKey.AnthropicModelName, anthropicModelName);
	});
	run(() => {
		setConfiguration(GitAIConfigKey.DiffLengthLimit, diffLengthLimit?.toString());
	});
	run(() => {
		setSecret(AISecretHandle.AnthropicKey, anthropicKey);
	});
	run(() => {
		setConfiguration(GitAIConfigKey.OllamaEndpoint, ollamaEndpoint);
	});
	run(() => {
		setConfiguration(GitAIConfigKey.OllamaModelName, ollamaModel);
	});
	run(() => {
		setConfiguration(GitAIConfigKey.LMStudioEndpoint, lmStudioEndpoint);
	});
	run(() => {
		setConfiguration(GitAIConfigKey.LMStudioModelName, lmStudioModel);
	});
	run(() => {
		if (form) form.modelKind.value = modelKind;
	});
</script>

{#snippet shortNote(text: string)}
	<div class="ai-settings__short-note">
		<Icon name="info-small" />
		<p class="text-12 text-body">{text}</p>
	</div>
{/snippet}

<p class="text-13 text-body ai-settings__about-text">
	GitButler 支持多个 AI 提供商：OpenAI 和 Anthropic（通过 API 或自带密钥），以及通过 Ollama 和
	LM Studio 的本地模型。
</p>

<CardGroup>
	<form class="git-radio" bind:this={form} onchange={(e) => onFormChange(e.currentTarget)}>
		<CardGroup.Item labelFor="open-ai">
			{#snippet title()}
				Open AI
			{/snippet}
			{#snippet actions()}
				<RadioButton name="modelKind" id="open-ai" value={ModelKind.OpenAI} />
			{/snippet}
		</CardGroup.Item>
		{#if modelKind === ModelKind.OpenAI}
			<CardGroup.Item>
				<Select
					value={openAIKeyOption}
					options={keyOptions}
					wide
					label="是否使用自带密钥？"
					onselect={(value) => {
						openAIKeyOption = value as KeyOption;
					}}
				>
					{#snippet itemSnippet({ item, highlighted })}
						<SelectItem selected={item.value === openAIKeyOption} {highlighted}>
							{item.label}
						</SelectItem>
					{/snippet}
				</Select>

				{#if openAIKeyOption === KeyOption.ButlerAPI}
					{#if !$user}
						<AuthorizationBanner message="请登录以使用 GitButler API。" />
					{:else}
						{@render shortNote('GitButler 使用 OpenAI API 生成提交信息与分支名称。')}
					{/if}
				{/if}

				{#if openAIKeyOption === KeyOption.BringYourOwn}
					<Textbox
						label="API 密钥"
						type="password"
						bind:value={openAIKey}
						required
						placeholder="sk-..."
					/>

					<Select
						value={openAIModelName}
						options={openAIModelOptions}
						label="模型版本"
						wide
						onselect={(value) => {
							openAIModelName = value as OpenAIModelName;
						}}
					>
						{#snippet itemSnippet({ item, highlighted })}
							<SelectItem selected={item.value === openAIModelName} {highlighted}>
								{item.label}
							</SelectItem>
						{/snippet}
					</Select>

					<Textbox
						label="自定义端点"
						bind:value={openAICustomEndpoint}
						placeholder="https://api.openai.com/v1"
					/>
				{/if}
			</CardGroup.Item>
		{/if}

		<CardGroup.Item labelFor="anthropic">
			{#snippet title()}
				Anthropic
			{/snippet}
			{#snippet actions()}
				<RadioButton name="modelKind" id="anthropic" value={ModelKind.Anthropic} />
			{/snippet}
		</CardGroup.Item>
		{#if modelKind === ModelKind.Anthropic}
			<CardGroup.Item>
				<Select
					value={anthropicKeyOption}
					options={keyOptions}
					wide
					label="是否使用自带密钥？"
					onselect={(value) => {
						anthropicKeyOption = value as KeyOption;
					}}
				>
					{#snippet itemSnippet({ item, highlighted })}
						<SelectItem selected={item.value === anthropicKeyOption} {highlighted}>
							{item.label}
						</SelectItem>
					{/snippet}
				</Select>

				{#if anthropicKeyOption === KeyOption.ButlerAPI}
					{#if !$user}
						<AuthorizationBanner message="请登录以使用 GitButler API。" />
					{:else}
						{@render shortNote(
							'GitButler 使用 Anthropic API 生成提交信息与分支名称。'
						)}
					{/if}
				{/if}

				{#if anthropicKeyOption === KeyOption.BringYourOwn}
					<Textbox
						label="API 密钥"
						type="password"
						bind:value={anthropicKey}
						required
						placeholder="sk-ant-api03-..."
					/>

					<Select
						value={anthropicModelName}
						options={anthropicModelOptions}
						label="模型版本"
						onselect={(value) => {
							anthropicModelName = value as AnthropicModelName;
						}}
					>
						{#snippet itemSnippet({ item, highlighted })}
							<SelectItem selected={item.value === anthropicModelName} {highlighted}>
								{item.label}
							</SelectItem>
						{/snippet}
					</Select>
				{/if}
			</CardGroup.Item>
		{/if}

		<CardGroup.Item labelFor="ollama">
			{#snippet title()}
				Ollama 🦙
			{/snippet}
			{#snippet actions()}
				<RadioButton name="modelKind" id="ollama" value={ModelKind.Ollama} />
			{/snippet}
		</CardGroup.Item>
		{#if modelKind === ModelKind.Ollama}
			<CardGroup.Item>
				<Textbox
					label="端点"
					bind:value={ollamaEndpoint}
					placeholder="http://127.0.0.1:11434"
				/>
				<Textbox label="模型" bind:value={ollamaModel} placeholder="llama3" />
				<InfoMessage filled outlined={false}>
					{#snippet title()}
						配置 Ollama
					{/snippet}
					{#snippet content()}
						要连接你的 Ollama 端点，请在应用的 CSP 设置中<b>将其加入允许列表</b>。
						<br />
						查看 <Link href="https://docs.gitbutler.com/troubleshooting/custom-csp"
							>文档了解详情</Link
						>
					{/snippet}
				</InfoMessage>
			</CardGroup.Item>
		{/if}

		<CardGroup.Item labelFor="lmstudio">
			{#snippet title()}
				LM Studio
			{/snippet}
			{#snippet actions()}
				<RadioButton name="modelKind" id="lmstudio" value={ModelKind.LMStudio} />
			{/snippet}
		</CardGroup.Item>
		{#if modelKind === ModelKind.LMStudio}
			<CardGroup.Item>
				<Textbox
					label="端点"
					bind:value={lmStudioEndpoint}
					placeholder="http://127.0.0.1:1234"
				/>
				<Textbox label="模型" bind:value={lmStudioModel} placeholder="default" />
				<InfoMessage filled outlined={false}>
					{#snippet title()}
						配置 LM Studio
					{/snippet}
					{#snippet content()}
						<div class="ai-settings__section-text-block">
							<p>连接你的 LM Studio 端点需要完成以下两项:</p>

							<p>
								1. <span class="text-bold">在应用的 CSP 设置中将其加入允许列表</span>。更多细节可查看
								<Link
									href="https://docs.gitbutler.com/troubleshooting/custom-csp">GitButler 文档</Link
								>。
							</p>

							<p>
								2. <span class="text-bold">在 LM Studio 中启用 CORS 支持</span>。更多细节可查看
								<Link
									href="https://lmstudio.ai/docs/cli/server-start#enable-cors-support"
									>LM Studio 文档</Link
								>。
							</p>
						</div>
					{/snippet}
				</InfoMessage>
			</CardGroup.Item>
		{/if}

		<CardGroup.Item>
			<AiCredentialCheck />
		</CardGroup.Item>
	</form>
</CardGroup>

<Spacer />

<CardGroup.Item standalone>
	{#snippet title()}
		提供的上下文长度
	{/snippet}
	{#snippet caption()}
		要提供给 AI 的 Git diff 字符数
	{/snippet}
	{#snippet actions()}
		<Textbox
			type="number"
			width={80}
			textAlign="center"
			value={diffLengthLimit?.toString()}
			minVal={100}
			oninput={(value: string) => {
				diffLengthLimit = parseInt(value);
			}}
			placeholder="5000"
		/>
	{/snippet}
</CardGroup.Item>

<Spacer />

<SettingsSection>
	{#snippet title()}
		自定义 AI 提示词
	{/snippet}
	{#snippet description()}
		GitButler 的 AI 助手会生成提交信息与分支名称。你可以使用默认提示词或自定义，并在项目设置中分配提示词。
	{/snippet}

	<div class="prompt-groups">
		<AIPromptEdit promptUse="commits" />
		<Spacer margin={12} />
		<AIPromptEdit promptUse="branches" />
	</div>
</SettingsSection>

<style>
	.ai-settings__about-text {
		margin-bottom: 12px;
		color: var(--clr-text-2);
	}

	.prompt-groups {
		display: flex;
		flex-direction: column;
		margin-top: 16px;
		gap: 12px;
	}

	.ai-settings__short-note {
		display: flex;
		align-items: center;
		padding: 6px 10px;
		gap: 8px;
		border-radius: var(--radius-m);
		background-color: var(--clr-bg-2);
		color: var(--clr-text-2);
	}

	.ai-settings__section-text-block {
		display: flex;
		flex-direction: column;
	}
</style>
