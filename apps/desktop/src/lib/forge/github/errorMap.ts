import type { Toast } from '$lib/notifications/toasts';

export function mapErrorToToast(err: any): Toast | undefined {
	// We expect an object to be thrown by octokit.
	if (typeof err !== 'object') return;

	const status = err?.status;
	const response = err?.response;
	const data = response?.data;
	const message = data?.message;
	const errors = data?.errors;

	// If this expectation isn't met we must be doing something wrong
	if (status === undefined || message === undefined) return;

	if (message.includes('Draft pull requests are not supported')) {
		return {
			title: '未启用草稿拉取请求',
			message: `
                看起来你的仓库未启用草稿拉取请求。

                请查看我们的[文档](https://docs.gitbutler.com/)
                以获得更多帮助。
            `,
			error: message,
			style: 'error'
		};
	}

	if (message.includes('enabled OAuth App access restrictions')) {
		return {
			title: 'OAuth 访问受限',
			message: `
                看起来你的组织限制了 OAuth 访问。

                请查看我们的[文档](https://docs.gitbutler.com/)
                以获得更多帮助。
            `,
			error: message,
			style: 'error'
		};
	}

	if (message.includes('Validation Failed')) {
		let errorStrings = '';
		if (errors instanceof Array) {
			errorStrings = errors
				.map((err) => {
					if (err.message) return err.message;
					if (err.field && err.code) return `${err.field} ${err.code}`;
					return '未知的验证错误';
				})
				.join('\n');
		}
		return {
			title: 'GitHub 验证失败',
			message: `
                验证请求时似乎出现问题。

                请查看我们的[文档](https://docs.gitbutler.com/)
                以获得更多帮助。
            `,
			error: errorStrings,
			style: 'error'
		};
	}
}
