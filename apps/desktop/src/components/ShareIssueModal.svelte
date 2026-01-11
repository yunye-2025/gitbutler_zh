<script lang="ts">
	import { page } from '$app/stores';
	import { BACKEND } from '$lib/backend';
	import { FILE_SERVICE } from '$lib/files/fileService';
	import { GIT_SERVICE } from '$lib/git/gitService';
	import { SHORTCUT_SERVICE } from '$lib/shortcuts/shortcutService';
	import { DATA_SHARING_SERVICE } from '$lib/support/dataSharing';
	import { USER } from '$lib/user/user';
	import { inject } from '@gitbutler/core/context';
	import { HTTP_CLIENT } from '@gitbutler/shared/network/httpClient';

	import { Button, Checkbox, Modal, Textarea, EmailTextbox, chipToasts } from '@gitbutler/ui';

	type Feedback = {
		id: number;
		user_id: number;
		feedback: string;
		context: string;
		created_at: string;
		updated_at: string;
	};

	const shortcutService = inject(SHORTCUT_SERVICE);
	const httpClient = inject(HTTP_CLIENT);
	const fileService = inject(FILE_SERVICE);
	const dataSharingService = inject(DATA_SHARING_SERVICE);
	const gitService = inject(GIT_SERVICE);
	const user = inject(USER);
	const backend = inject(BACKEND);

	export function show() {
		modal?.show();
	}

	async function gitIndexLength() {
		return await gitService.indexSize(projectId);
	}

	let modal: ReturnType<typeof Modal> | undefined = $state();

	let messageInputValue = $state('');
	let emailInputValue = $state('');
	let sendLogs = $state(false);
	let sendProjectRepository = $state(false);
	let sendGraph = $state(false);

	const projectId = $derived($page.params.projectId!);

	function reset() {
		messageInputValue = '';
		sendLogs = false;
		sendProjectRepository = false;
		sendGraph = false;
	}

	async function readZipFile(path: string, filename?: string): Promise<File | Blob> {
		// Using `new Uint8Array` to get an `ArrayBuffer` rather than `ArrayBufferLike`.
		const file = new Uint8Array(await fileService.readFile(path));
		const fileName = filename ?? path.split('/').pop();
		return fileName
			? new File([file], fileName, { type: 'application/zip' })
			: new Blob([file], { type: 'application/zip' });
	}

	async function submit() {
		const message = messageInputValue;
		const email = $user?.email ?? emailInputValue;

		// put together context information to send with the feedback
		let context = '';
		const appInfo = await backend.getAppInfo();
		const appVersion = appInfo.version;
		const indexLength = await gitIndexLength();
		context += 'GitButler 版本：' + appVersion + '\n';
		context += '浏览器：' + navigator.userAgent + '\n';
		context += 'URL：' + window.location.href + '\n';
		context += '索引长度：' + indexLength + '\n';

		chipToasts.promise(
			Promise.all([
				sendLogs
					? dataSharingService.logs().then(async (path) => await readZipFile(path, 'logs.zip'))
					: undefined,
				sendProjectRepository
					? dataSharingService
							.projectData(projectId)
							.then(async (path) => await readZipFile(path, 'project.zip'))
					: undefined,
				sendGraph
					? dataSharingService
							.graphFile(projectId)
							.then(async (path) => await readZipFile(path, 'graph.zip'))
					: undefined
			]).then(
				async ([logs, repo, graph]) =>
					await createFeedback($user?.access_token, {
						email,
						message,
						context,
						logs,
						repo,
						graph
					})
			),
			{
				loading:
					!sendLogs && !sendProjectRepository && !sendGraph
						? '正在发送反馈...'
						: '正在上传数据...',
				success: '反馈发送成功',
				error: '反馈发送失败'
			}
		);
		close();
	}

	async function createFeedback(
		token: string | undefined,
		params: {
			email?: string;
			message: string;
			context?: string;
			logs?: Blob | File;
			repo?: Blob | File;
			graph?: Blob | File;
		}
	): Promise<Feedback> {
		const formData = new FormData();
		formData.append('message', params.message);
		if (params.email) formData.append('email', params.email);
		if (params.context) formData.append('context', params.context);
		if (params.logs) formData.append('logs', params.logs);
		if (params.repo) formData.append('repo', params.repo);
		if (params.graph) formData.append('graph', params.graph);

		// Content Type must be unset for the right form-data border to be set automatically
		return await httpClient.put('feedback', {
			body: formData,
			headers: { 'Content-Type': undefined }
		});
	}

	function close() {
		reset();
		modal?.close();
	}

	$effect(() => shortcutService.on('share-debug-info', () => show()));
</script>

<Modal
	bind:this={modal}
	title="与 GitButler 团队分享调试数据以便审查"
	onSubmit={async () => await submit()}
>
	<div class="content-wrapper">
		<p class="content-wrapper__help-text text-13 text-body">
			如果你遇到问题，请与 GitButler 团队分享你的项目和日志。我们会进行审查并帮助定位解决方式。
		</p>

		{#if !$user}
			<EmailTextbox
				label="邮箱"
				placeholder="提供邮箱以便我们联系你"
				bind:value={emailInputValue}
				required
				autocomplete={false}
				autocorrect={false}
				spellcheck
				autofocus
			/>
		{/if}

		<Textarea
			label="说明"
			placeholder="请提供复现问题所需的步骤。"
			spellcheck
			id="comments"
			minRows={6}
			maxRows={10}
			bind:value={messageInputValue}
		/>

		<div class="content-wrapper__section">
			<span class="text-16 text-semibold"> 分享日志 </span>
			<span class="content-wrapper__help-text text-13 text-body">
				我们承诺你分享的所有信息仅用于内部审查，并在问题解决后删除
			</span>
		</div>

		<div class="content-wrapper__checkbox-group">
			<div class="content-wrapper__checkbox">
				<Checkbox name="logs" bind:checked={sendLogs} />
				<label class="text-13" for="logs">分享日志</label>
			</div>

			{#if projectId}
				<div class="content-wrapper__checkbox">
					<Checkbox name="project-repository" bind:checked={sendProjectRepository} />
					<label class="text-13" for="project-repository">分享项目仓库</label>
				</div>
				<div class="content-wrapper__checkbox">
					<Checkbox name="graph" bind:checked={sendGraph} />
					<label class="text-13" for="graph">分享匿名提交图</label>
				</div>
			{/if}
		</div>
	</div>

	<!-- Use our own close function -->
	{#snippet controls()}
		<Button kind="outline" type="reset" onclick={close}>关闭</Button>
		<Button disabled={!sendLogs && !sendProjectRepository && !sendGraph} style="pop" type="submit"
			>与 GitButler 分享</Button
		>
	{/snippet}
</Modal>

<style>
	.content-wrapper {
		display: flex;
		flex-direction: column;
		gap: 16px;
	}

	.content-wrapper__section {
		display: flex;
		flex-direction: column;
		gap: 8px;
	}

	.content-wrapper__help-text {
		opacity: 0.6;
	}

	.content-wrapper__checkbox-group {
		display: flex;
		flex-direction: column;
		gap: 10px;
	}

	.content-wrapper__checkbox {
		display: flex;
		align-items: center;
		gap: 10px;
	}
</style>
