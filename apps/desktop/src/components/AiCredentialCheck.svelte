<script lang="ts">
	import { AI_SERVICE, type DiffInput } from '$lib/ai/service';
	import { ModelKind } from '$lib/ai/types';
	import { USER_SERVICE } from '$lib/user/userService';
	import { inject } from '@gitbutler/core/context';
	import { Button, InfoMessage, Link } from '@gitbutler/ui';
	import { slide } from 'svelte/transition';

	const aiService = inject(AI_SERVICE);
	const userService = inject(USER_SERVICE);
	const user = userService.user;

	let testing = $state(false);
	let isStreaming = $state(false);
	let result = $state<string | null>(null);
	let streamingResult = $state<string>('');
	let error = $state<string | null>(null);
	let modelKind = $state<ModelKind | undefined>();
	let isUsingButlerAPI = $state(false);
	let debugInfo = $state<string | null>(null);
	let showDebug = $state(false);
	let showSampleDiff = $state(false);
	let testTimeout: NodeJS.Timeout | null = null;
	let abortController: AbortController | null = null;

	// Simple test diff for commit message generation
	const testDiff: DiffInput[] = [
		{
			filePath: 'example.js',
			diff: `@@ -1,3 +1,5 @@
 function hello() {
  -  return "Hello World";
  +  // Add a greeting with the current time
  +  const now = new Date();
  +  return \`Hello World! The time is \${now.toLocaleTimeString()}\`;
 }`
		}
	];

	async function testAiCredentials() {
		testing = true;
		isStreaming = false;
		result = null;
		streamingResult = '';
		error = null;
		debugInfo = null;

		// Clear any existing timeout
		if (testTimeout) {
			clearTimeout(testTimeout);
			testTimeout = null;
		}

		// Abort any pending request
		if (abortController) {
			abortController.abort();
		}

		// Create a new abort controller for this request
		abortController = new AbortController();

		try {
			// Get current model kind
			modelKind = await aiService.getModelKind();
			debugInfo = `模型类型：${modelKind}`;

			// Check if using GitButler API
			isUsingButlerAPI = await aiService.usingGitButlerAPI();
			debugInfo += `，使用 GB API：${isUsingButlerAPI}`;

			// Check if configuration is valid
			const isConfigValid = await aiService.validateConfiguration();
			debugInfo += `，配置有效：${isConfigValid}`;

			if (!isConfigValid) {
				if (modelKind === ModelKind.OpenAI || modelKind === ModelKind.Anthropic) {
					if (isUsingButlerAPI && !$user) {
						throw new Error('请登录以使用 GitButler 的 AI API');
					} else {
						throw new Error('请为所选 AI 服务提供有效的 API 密钥');
					}
				} else if (modelKind === ModelKind.Ollama) {
					// Get Ollama configuration for more detailed error
					const endpoint = await aiService.getOllamaEndpoint();
					const model = await aiService.getOllamaModelName();
					throw new Error(`请检查 Ollama 配置：endpoint=${endpoint}, model=${model}`);
				} else if (modelKind === ModelKind.LMStudio) {
					// Get LM Studio configuration for more detailed error
					const endpoint = await aiService.getLMStudioEndpoint();
					throw new Error(`请检查 LM Studio 配置：endpoint=${endpoint}`);
				}
			}

			debugInfo += `，正在测试提交信息生成`;

			// Set a timeout to fail if the streaming doesn't start or complete
			testTimeout = setTimeout(() => {
				if (testing) {
					console.error('AI 响应在 20 秒后超时');
					error =
						'AI 响应在 20 秒后超时。请检查你的 AI 服务是否正常运行。';
					testing = false;
					isStreaming = false; // Make sure streaming state is reset on timeout
					debugInfo += `，20 秒超时`;

					// Abort the request if possible
					if (abortController) {
						try {
							abortController.abort();
						} catch (err) {
							console.error('终止请求时出错：', err);
						}
					}

					// Force a UI update (this ensures the reactive system recognizes the state changes)
					testing = false;
					isStreaming = false;
				}
			}, 20000);

			// Start streaming mode
			isStreaming = true;

			// Use the summarizeCommit method with the onToken callback for streaming
			const aiResult = await aiService.summarizeCommit({
				diffInput: testDiff,
				useEmojiStyle: false,
				useBriefStyle: false,
				onToken: (token) => {
					// Append each token as it comes in
					streamingResult += token;
				}
			});

			// Clear the timeout since we got a result
			if (testTimeout) {
				clearTimeout(testTimeout);
				testTimeout = null;
			}

			// Set the final result (handling undefined case)
			result = aiResult || streamingResult || null;

			debugInfo += `，已收到提交信息：${result?.substring(0, 30)}${result && result.length > 30 ? '...' : ''}`;

			// If result is empty or undefined, show an error
			if (!result || result.trim() === '') {
				throw new Error('从 AI 服务收到空响应');
			}
		} catch (e) {
			console.error('AI 凭据检查出错：', e);

			// Don't show abort errors as they're expected when we cancel the request
			if (e instanceof Error && e.name === 'AbortError') {
				error = 'AI 请求已取消';
			} else {
				error = e instanceof Error ? e.message : '发生未知错误';
			}

			debugInfo += `，错误：${error}`;

			// Clear the timeout if there was an error
			if (testTimeout) {
				clearTimeout(testTimeout);
				testTimeout = null;
			}

			// Ensure streaming and testing states are reset on error
			isStreaming = false;
			testing = false;
		} finally {
			testing = false;
			isStreaming = false;
			abortController = null;
		}
	}

	function toggleDebug() {
		showDebug = !showDebug;
	}

	function toggleSampleMessage() {
		showSampleDiff = !showSampleDiff;
	}
</script>

<div class="ai-credential-check">
	{#if isStreaming || result || error}
		<div transition:slide={{ duration: 250 }}>
			<InfoMessage
				style={error ? 'danger' : 'success'}
				icon={error ? 'error' : isStreaming ? 'robot' : 'success'}
				filled
				outlined={false}
			>
				{#snippet title()}
					{#if error}
						AI 凭据检查失败
					{:else if result}
						AI 凭据检查通过
					{:else if isStreaming}
						AI 正在响应…
					{/if}
				{/snippet}

				{#snippet content()}
					<div class="result-content" transition:slide={{ duration: 250 }}>
						{#if error}
							{#if (modelKind === ModelKind.OpenAI || modelKind === ModelKind.Anthropic) && isUsingButlerAPI && !$user}
								<span> 请登录以使用 GitButler 的 AI API。 </span>
							{:else if modelKind === ModelKind.OpenAI || modelKind === ModelKind.Anthropic}
								<span> 请检查你的 API 密钥，或尝试 GitButler 的 API。 </span>
							{:else if modelKind === ModelKind.Ollama}
								<span>
									请检查你的 Ollama 端点和模型配置。
									<br />
									请确保 Ollama 在本机运行且可访问。

									<Link href="https://ollama.ai">了解更多</Link>
								</span>
							{:else if modelKind === ModelKind.LMStudio}
								<span>
									请检查你的 LM Studio 配置。
									<br />
									请确保 LM Studio 在本机运行且可访问。

									<Link href="https://lmstudio.ai">了解更多</Link>
								</span>
							{/if}
						{:else}
							<div class="text-12 text-body ai-response">
								<pre class:streaming={isStreaming}>{isStreaming
										? streamingResult
											? streamingResult.trim()
											: '加载中...'
										: `响应：\n\n${result?.trim()}`}
								</pre>
							</div>
						{/if}
					</div>
				{/snippet}
			</InfoMessage>
		</div>
	{/if}
	<Button
		style="pop"
		wide
		icon="ai-small"
		disabled={testing || isStreaming}
		onclick={testAiCredentials}
	>
		{#if testing || isStreaming}
			{isStreaming ? 'AI 正在响应…' : '正在测试 AI 连接…'}
		{:else if error}
			重试
		{:else if result}
			再次测试
		{:else}
			测试 AI 连接
		{/if}
	</Button>

	{#if showDebug && debugInfo}
		<div class="debug-info text-12 text-body">
			<p><span class="text-bold">调试信息</span>：</p>
			<p>{debugInfo}</p>
		</div>
	{/if}

	{#if showSampleDiff}
		<div class="debug-info text-12 text-body">
			<p class="text-bold">示例差异：</p>
			<pre class="debug-info__code">{testDiff[0]?.diff}</pre>
		</div>
	{/if}

	<div class="debug-info-buttons">
		<button type="button" class="text-12 debug-button" onclick={toggleSampleMessage}>
			{showSampleDiff ? '隐藏' : '显示'}差异示例
		</button>
		<button
			type="button"
			class="text-12 debug-button"
			class:debug-button_disabled={!debugInfo}
			onclick={toggleDebug}
		>
			{showDebug ? '隐藏' : '显示'}调试信息
		</button>
	</div>
</div>

<style>
	.ai-credential-check {
		display: flex;
		flex-direction: column;
		gap: 16px;
	}

	.result-content {
		display: flex;
		flex-direction: column;
		margin-top: 4px;
		overflow-x: auto;
		gap: 4px;
	}

	.ai-response {
		display: flex;
		flex-direction: column;
		gap: 10px;
		border-radius: var(--radius-m);
		background-color: var(--clr-bg-1);

		pre {
			max-width: 100%;
			padding: 14px;
			overflow-x: auto;
			white-space: pre;
		}
	}

	/* DEBUG SECTION */
	.debug-button {
		border: none;
		background: none;
		color: var(--clr-text-2);
		font-size: 11px;
		text-decoration: underline dotted;
		cursor: pointer;
	}

	.debug-button_disabled {
		color: var(--clr-text-3);
		cursor: not-allowed;
	}

	.debug-info-buttons {
		display: flex;
		align-items: center;
		justify-content: flex-end;
		width: auto;
		margin-top: 4px;
		gap: 14px;
	}

	.debug-info {
		display: flex;
		flex-direction: column;
		margin-bottom: -8px;
		padding: 14px;
		gap: 4px;
		border-radius: var(--radius-m);
		background-color: var(--clr-bg-2);
	}

	.debug-info__code {
		white-space: pre-wrap;
	}
</style>
