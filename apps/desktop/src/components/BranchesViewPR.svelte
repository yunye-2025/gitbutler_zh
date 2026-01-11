<script lang="ts">
	import { goto } from '$app/navigation';
	import ReduxResult from '$components/ReduxResult.svelte';
	import PRListCard from '$components/branchesPage/PRListCard.svelte';
	import { BASE_BRANCH_SERVICE } from '$lib/baseBranch/baseBranchService.svelte';
	import { DEFAULT_FORGE_FACTORY } from '$lib/forge/forgeFactory.svelte';
	import { showError } from '$lib/notifications/toasts';
	import { REMOTES_SERVICE } from '$lib/remotes/remotesService';
	import { workspacePath } from '$lib/routes/routes.svelte';
	import { handleCreateBranchFromBranchOutcome } from '$lib/stacks/stack';
	import { STACK_SERVICE } from '$lib/stacks/stackService.svelte';

	import { UI_STATE } from '$lib/state/uiState.svelte';
	import { inject } from '@gitbutler/core/context';
	import { Button, Modal, TestId, Textbox } from '@gitbutler/ui';
	import type { DetailedPullRequest } from '$lib/forge/interface/types';

	type Props = {
		projectId: string;
		prNumber: number;
		onerror?: (error: unknown) => void;
	};

	const { projectId, prNumber, onerror }: Props = $props();

	const forge = inject(DEFAULT_FORGE_FACTORY);
	const prService = $derived(forge.current.prService);
	const prQuery = $derived(prService?.get(prNumber, { forceRefetch: true }));
	const prUnit = $derived(prService?.unit);

	const uiState = inject(UI_STATE);
	const projectState = $derived(uiState.project(projectId));
	const branchesState = $derived(projectState.branchesSelection);

	const selected = $derived(branchesState.current.prNumber === prNumber);

	const baseBranchService = inject(BASE_BRANCH_SERVICE);
	const baseRepoQuery = $derived(baseBranchService.repo(projectId));
	const baseRepo = $derived(baseRepoQuery.response);

	const remotesService = inject(REMOTES_SERVICE);
	const stackService = inject(STACK_SERVICE);

	let createRemoteModal = $state<Modal>();
	let inputRemoteName = $state<string>();
	let loading = $state(false);

	function getRemoteUrl(pr: DetailedPullRequest) {
		if (!baseRepo) return;

		if (baseRepo.protocol?.startsWith('http')) {
			return pr.repositoryHttpsUrl;
		} else {
			return pr.repositorySshUrl;
		}
	}

	async function handleConfirmRemote(pr: DetailedPullRequest) {
		const remoteUrl = getRemoteUrl(pr);

		if (!remoteUrl) {
			throw new Error(`PR #${prNumber} 的远程 URL 不可用。`);
		}

		if (!inputRemoteName) {
			showError('无法创建远程', '请提供远程名称。');
			return;
		}

		loading = true;

		try {
			const remoteRef = 'refs/remotes/' + inputRemoteName + '/' + pr.sourceBranch;
			await remotesService.addRemote(projectId, inputRemoteName, remoteUrl);
			await baseBranchService.fetchFromRemotes(projectId);
			const outcome = await stackService.createVirtualBranchFromBranch({
				projectId,
				branch: remoteRef,
				remote: remoteRef,
				prNumber
			});

			handleCreateBranchFromBranchOutcome(outcome);
			goto(workspacePath(projectId));
		} catch (err: unknown) {
			showError('应用分叉分支失败', err);
		} finally {
			loading = false;
		}
	}

	export function applyPr() {
		createRemoteModal?.show();
	}
</script>

<ReduxResult result={prQuery?.result} {projectId} {onerror}>
	{#snippet children(pr)}
		<div class="pr-card">
			<PRListCard
				reviewUnit={prUnit}
				number={pr.number}
				title={pr.title}
				sourceBranch={pr.sourceBranch}
				isDraft={pr.draft ?? false}
				noRemote
				{selected}
			/>
		</div>

		<Modal
			testId={TestId.BranchesView_CreateRemoteModal}
			title="应用 {forge.reviewUnitName}"
			width="small"
			bind:this={createRemoteModal}
			onSubmit={async () => await handleConfirmRemote(pr)}
		>
			<p class="fork-notice">
				要应用来自分叉的分支，GitButler 必须先把分叉添加为远程。<span
					class="text-bold">选择一个远程名称：</span
				>
			</p>
			<Textbox bind:value={inputRemoteName} placeholder="远程名称示例" required />

			{#snippet controls(close)}
				<Button kind="outline" onclick={close}>取消</Button>
				<Button
					testId={TestId.BranchesView_CreateRemoteModalActionButton}
					style="pop"
					type="submit"
					{loading}
					disabled={!inputRemoteName}>确认并应用</Button
				>
			{/snippet}
		</Modal>
	{/snippet}
</ReduxResult>

<style lang="postcss">
	.pr-card {
		display: flex;
		flex-direction: column;
		overflow: hidden;
		border: 1px solid var(--clr-border-2);
		border-radius: var(--radius-m);
		pointer-events: none;
	}
	.fork-notice {
		margin-bottom: 14px;
	}
</style>
