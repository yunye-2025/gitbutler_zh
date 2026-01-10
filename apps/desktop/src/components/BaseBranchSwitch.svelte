<script lang="ts">
	import { BASE_BRANCH_SERVICE } from '$lib/baseBranch/baseBranchService.svelte';
	import { STACK_SERVICE } from '$lib/stacks/stackService.svelte';
	import { inject } from '@gitbutler/core/context';
	import { Button, CardGroup, InfoMessage, Select, SelectItem } from '@gitbutler/ui';

	const { projectId }: { projectId: string } = $props();

	const stackService = inject(STACK_SERVICE);
	const baseBranchService = inject(BASE_BRANCH_SERVICE);
	const baseBranchQuery = $derived(baseBranchService.baseBranch(projectId));
	const baseBranch = $derived(baseBranchQuery.response);
	const remoteBranchesQuery = $derived(baseBranchService.remoteBranches(projectId));
	const [setBaseBranchTarget, targetBranchSwitch] = baseBranchService.setTarget;

	let selectedBranch = $derived(baseBranch?.branchName);
	let selectedRemote = $derived(baseBranch?.actualPushRemoteName());

	const stacksQuery = $derived(stackService.stacks(projectId));
	const stackCount = $derived(stacksQuery.response?.length);
	const targetChangeDisabled = $derived(!!(stackCount && stackCount > 0));

	function uniqueRemotes(remoteBranches: { name: string }[]) {
		return Array.from(new Set(remoteBranches.map((b) => b.name.split('/')[0]))).map((r) => ({
			name: r
		}));
	}

	async function switchTarget(branch: string, pushRemote?: string) {
		await setBaseBranchTarget({ projectId, branch, pushRemote });
	}

	async function onSetBaseBranchClick() {
		if (!selectedBranch) return;

		if (selectedRemote) {
			await switchTarget(selectedBranch, selectedRemote);
		} else {
			await switchTarget(selectedBranch);
		}
	}
</script>

{#if remoteBranchesQuery.result.isLoading}
	<InfoMessage filled outlined={false} icon="info">
		{#snippet content()}
			正在加载远端分支...
		{/snippet}
	</InfoMessage>
{:else if remoteBranchesQuery.result.isSuccess}
	{@const remoteBranches = remoteBranchesQuery.response}
	{#if remoteBranches && remoteBranches.length > 0}
		<CardGroup>
			<CardGroup.Item>
				{#snippet title()}
					远端配置
				{/snippet}
				{#snippet caption()}
					用于选择推送位置并设置贡献的目标分支。目标分支通常是“生产”分支，例如
					'origin/master' 或 'upstream/main'。此处帮助确保代码集成到正确的远端与分支。
				{/snippet}

				<Select
					value={selectedBranch}
					options={remoteBranches.map((b) => ({ label: b.name, value: b.name }))}
					wide
					onselect={(value) => {
						selectedBranch = value;
					}}
					disabled={targetChangeDisabled}
					label="当前目标分支"
					searchable
				>
					{#snippet itemSnippet({ item, highlighted })}
						<SelectItem selected={item.value === selectedBranch} {highlighted}>
							{item.label}
						</SelectItem>
					{/snippet}
				</Select>

				{#if uniqueRemotes(remoteBranches).length > 1}
					<Select
						value={selectedRemote}
						options={uniqueRemotes(remoteBranches).map((r) => ({ label: r.name!, value: r.name! }))}
						wide
						onselect={(value) => {
							selectedRemote = value;
						}}
						disabled={targetChangeDisabled}
						label="创建分支所用远端"
					>
						{#snippet itemSnippet({ item, highlighted })}
							<SelectItem selected={item.value === selectedRemote} {highlighted}>
								{item.label}
							</SelectItem>
						{/snippet}
					</Select>
				{/if}

				{#if targetChangeDisabled}
					<InfoMessage filled outlined={false} icon="info">
						{#snippet content()}
							你的工作区中有
							{stackCount === 1 ? '1 个活跃分支' : `${stackCount} 个活跃分支`}
							，请先清理工作区再切换基分支。
						{/snippet}
					</InfoMessage>
				{:else}
					<Button
						kind="outline"
						onclick={onSetBaseBranchClick}
						id="set-base-branch"
						loading={targetBranchSwitch.current.isLoading}
						disabled={(selectedBranch === baseBranch?.branchName &&
							selectedRemote === baseBranch?.actualPushRemoteName()) ||
							targetChangeDisabled}
					>
						{targetBranchSwitch.current.isLoading ? '正在切换分支...' : '更新配置'}
					</Button>
				{/if}
			</CardGroup.Item>
		</CardGroup>
	{/if}
{:else if remoteBranchesQuery.result.isError}
	<InfoMessage filled outlined={true} style="danger">
		{#snippet title()}
			列出远端分支时出错
		{/snippet}
	</InfoMessage>
{/if}
