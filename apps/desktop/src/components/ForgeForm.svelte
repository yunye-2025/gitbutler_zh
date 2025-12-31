<script lang="ts">
	import GitHubAccountBadge from '$components/GitHubAccountBadge.svelte';
	import { DEFAULT_FORGE_FACTORY } from '$lib/forge/forgeFactory.svelte';
	import {
		githubAccountIdentifierToString,
		stringToGitHubAccountIdentifier
	} from '$lib/forge/github/githubUserService.svelte';
	import { usePreferredGitHubUsername } from '$lib/forge/github/hooks.svelte';
	import { GITLAB_STATE } from '$lib/forge/gitlab/gitlabState.svelte';
	import { PROJECTS_SERVICE } from '$lib/project/projectsService';
	import { inject } from '@gitbutler/core/context';
	import { reactive } from '@gitbutler/shared/reactiveUtils.svelte';
	import { CardGroup, InfoMessage, Link, Select, SelectItem, Spacer, Textbox } from '@gitbutler/ui';

	import type { ForgeName } from '$lib/forge/interface/forge';
	import type { Project } from '$lib/project/project';

	const { projectId }: { projectId: string } = $props();

	const forge = inject(DEFAULT_FORGE_FACTORY);
	const gitLabState = inject(GITLAB_STATE);
	const { preferredGitHubAccount, githubAccounts } = usePreferredGitHubUsername(
		reactive(() => projectId)
	);

	const token = gitLabState.token;
	const forkProjectId = gitLabState.forkProjectId;
	const upstreamProjectId = gitLabState.upstreamProjectId;
	const instanceUrl = gitLabState.instanceUrl;

	const projectsService = inject(PROJECTS_SERVICE);
	const projectQuery = $derived(projectsService.getProject(projectId));
	const project = $derived(projectQuery.response);

	const forgeOptions: { label: string; value: ForgeName }[] = [
		{
			label: '无',
			value: 'default'
		},
		{
			label: 'GitHub',
			value: 'github'
		},
		{
			label: 'GitLab',
			value: 'gitlab'
		},
		{
			label: 'Azure',
			value: 'azure'
		},
		{
			label: 'BitBucket',
			value: 'bitbucket'
		}
	];
	let selectedOption = $derived(project?.forge_override || 'default');

	function handleSelectionChange(selectedOption: ForgeName) {
		if (!project) return;

		const mutableProject: Project & { unset_forge_override?: boolean } = structuredClone(project);

		if (selectedOption === 'default') {
			mutableProject.unset_forge_override = true;
		} else {
			mutableProject.forge_override = selectedOption;
		}
		projectsService.updateProject(mutableProject);
	}
</script>

<CardGroup>
	<CardGroup.Item>
		{#snippet title()}
			Forge 覆盖设置
		{/snippet}

		{#snippet caption()}
			{#if forge.determinedForgeType === 'default'}
				无法检测你正在使用的 Forge。
				<br />
				要启用 Forge 集成，请从下拉列表中选择你的 Forge。
				<br />
				<span class="text-bold">注意:</span> 目前只有 GitHub 和 GitLab 支持创建 Pull Request。
			{:else}
				已检测到你正在使用 <span class="text-bold"
					>{forge.determinedForgeType.toUpperCase()}</span
				>.
				<br />
				目前无法手动覆盖已检测到的 Forge 类型。
			{/if}
		{/snippet}

		{#if forge.determinedForgeType === 'default'}
			<Select
				value={selectedOption}
				options={forgeOptions}
				wide
				onselect={(value) => {
					selectedOption = value as ForgeName;
					handleSelectionChange(selectedOption);
				}}
			>
				{#snippet itemSnippet({ item, highlighted })}
					<SelectItem selected={item.value === selectedOption} {highlighted}>
						{item.label}
					</SelectItem>
				{/snippet}
			</Select>
		{/if}
	</CardGroup.Item>

	{#if forge.current.name === 'gitlab'}
		<CardGroup.Item>
			{#snippet title()}
				配置 GitLab 集成
			{/snippet}

			{#snippet caption()}
				在我们的 <Link
					href="https://docs.gitbutler.com/features/forge-integration/gitlab-integration">文档</Link
				>中了解如何找到 GitLab Personal Token 和 Project ID。
				<br />
				Fork Project ID 是分支推送的目标；Upstream Project ID 是创建合并请求的目标。
			{/snippet}

			<Textbox
				label="个人访问令牌"
				type="password"
				value={$token}
				oninput={(value) => ($token = value)}
			/>
			<Textbox
				label="你的 fork 项目 ID"
				value={$forkProjectId}
				oninput={(value) => ($forkProjectId = value)}
			/>
			<Textbox
				label="上游项目 ID"
				value={$upstreamProjectId}
				oninput={(value) => ($upstreamProjectId = value)}
			/>
			<Textbox
				label="实例 URL"
				value={$instanceUrl}
				oninput={(value) => ($instanceUrl = value)}
			/>

			<Spacer margin={5} />

			<p class="text-12 text-body clr-text-2">
				对于自定义 GitLab 实例（非 gitlab.com），请将其添加为自定义 CSP 条目，以便 GitButler
				能够连接。更多信息请查看 <Link
					href="https://docs.gitbutler.com/troubleshooting/custom-csp">文档</Link
				>。
			</p>
		</CardGroup.Item>
	{/if}

	{#if forge.current.name === 'github'}
		<CardGroup.Item>
			{#snippet title()}
				配置 GitHub 集成
			{/snippet}

			{#snippet caption()}
				启用 Pull Request 创建。更多信息请查看 <Link
					href="https://docs.gitbutler.com/features/forge-integration/github-integration">文档</Link
				>。
			{/snippet}

			{#if githubAccounts.current.length === 0 || !preferredGitHubAccount.current}
				<!-- TODO: Link to the general settings -->
				<InfoMessage style="warning" filled outlined={false}>
					{#snippet title()}
						未找到 GitHub 账号
					{/snippet}
					{#snippet content()}
						在常规设置中添加 GitHub 账号以启用 GitHub 集成
					{/snippet}
				</InfoMessage>
			{:else}
				{@const account = preferredGitHubAccount.current}
				<Select
					label="此项目使用的 GitHub 账号"
					value={githubAccountIdentifierToString(account)}
					options={githubAccounts.current.map((account) => ({
						label: account.info.username,
						value: githubAccountIdentifierToString(account)
					}))}
					onselect={(value) => {
						const account = stringToGitHubAccountIdentifier(value);
						if (!account) return;
						projectsService.updatePreferredForgeUser(projectId, account);
					}}
					disabled={githubAccounts.current.length <= 1}
					wide
				>
					{#snippet itemSnippet({ item, highlighted })}
						{@const itemAccount = item.value && stringToGitHubAccountIdentifier(item.value)}
						<SelectItem
							selected={item.value === githubAccountIdentifierToString(account)}
							{highlighted}
						>
							{item.label}

							{#if itemAccount}
								<GitHubAccountBadge account={itemAccount} class="m-l-4" />
							{/if}
						</SelectItem>
					{/snippet}
				</Select>
			{/if}
		</CardGroup.Item>
	{/if}
</CardGroup>
