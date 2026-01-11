import {
	SHORT_DEFAULT_BRANCH_TEMPLATE,
	SHORT_DEFAULT_COMMIT_TEMPLATE,
	SHORT_DEFAULT_PR_TEMPLATE
} from '$lib/ai/prompts';
import type { Prompt, AIClient, AIEvalOptions } from '$lib/ai/types';

export const LM_STUDIO_DEFAULT_ENDPOINT = 'http://127.0.0.1:1234';
export const LM_STUDIO_DEFAULT_MODEL_NAME = 'default';

const DEFAULT_MAX_TOKENS = -1; // -1 means no limit
const DEFAULT_TEMPERATURE = 0.7;

/**
 * LMStudioClient implements the AIClient interface for LM Studio servers.
 * LM Studio provides an OpenAI-compatible API at the /v1/chat/completions endpoint.
 */
export class LMStudioClient implements AIClient {
	defaultCommitTemplate = SHORT_DEFAULT_COMMIT_TEMPLATE;
	defaultBranchTemplate = SHORT_DEFAULT_BRANCH_TEMPLATE;
	defaultPRTemplate = SHORT_DEFAULT_PR_TEMPLATE;

	private baseUrl: string;
	private modelName: string;

	constructor(endpoint: string, modelName: string) {
		this.modelName = modelName;
		// Format the base URL to ensure it ends with /v1
		this.baseUrl = endpoint.endsWith('/v1')
			? endpoint
			: endpoint.endsWith('/')
				? `${endpoint}v1`
				: `${endpoint}/v1`;
	}

	async evaluate(prompt: Prompt, options?: AIEvalOptions): Promise<string> {
		try {
			// Validate input
			if (!prompt || !Array.isArray(prompt) || prompt.length === 0) {
				throw new Error('无效的提示词：必须是非空数组');
			}

			// Format messages for the API
			const messages = prompt.map((msg) => ({
				role: msg.role.toLowerCase(),
				content: msg.content
			}));

			// Determine if we should stream the response
			const shouldStream = options?.onToken !== undefined;

			// Make request to the LM Studio API
			const response = await fetch(`${this.baseUrl}/chat/completions`, {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json'
				},
				body: JSON.stringify({
					model: this.modelName,
					messages: messages,
					temperature: DEFAULT_TEMPERATURE,
					max_tokens: options?.maxTokens ?? DEFAULT_MAX_TOKENS,
					stream: shouldStream
				})
			});

			if (!response.ok) {
				const errorText = await response.text();
				throw new Error(`LM Studio API 错误 (${response.status}): ${errorText}`);
			}

			// Handle streaming response
			if (shouldStream) {
				const reader = response.body?.getReader();
				if (!reader) {
					throw new Error('无法从响应中获取读取器');
				}

				let result = '';
				const decoder = new TextDecoder();

				while (true) {
					const { done, value } = await reader.read();
					if (done) break;

					const chunk = decoder.decode(value, { stream: true });
					const lines = chunk
						.split('\n')
						.filter((line) => line.trim() !== '' && line.trim() !== 'data: [DONE]');

					for (const line of lines) {
						if (!line.startsWith('data:')) continue;

						try {
							const jsonStr = line.slice(5).trim();
							if (jsonStr) {
								const json = JSON.parse(jsonStr);
								const token = json.choices[0]?.delta?.content || '';
								if (token) {
									options?.onToken?.(token);
									result += token;
								}
							}
						} catch (e) {
							console.warn('解析流式 JSON 出错', e);
						}
					}
				}

				return result;
			}
			// Handle non-streaming response
			else {
				const json = await response.json();
				return json.choices[0]?.message?.content || '';
			}
		} catch (error) {
			console.error('调用 LM Studio API 时出错：', error);
			throw new Error(
				`与 LM Studio 服务器通信失败：${error instanceof Error ? error.message : String(error)}`
			);
		}
	}
}
