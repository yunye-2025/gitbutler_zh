<script lang="ts">
	import GerritPushModal from '$components/GerritPushModal.svelte';
	import ReduxResult from '$components/ReduxResult.svelte';
	import { CLIPBOARD_SERVICE } from '$lib/backend/clipboard';
	import { commitCreatedAtDate } from '$lib/branches/v3';
	import { projectRunCommitHooks } from '$lib/config/config';
	import { DEFAULT_FORGE_FACTORY } from '$lib/forge/forgeFactory.svelte';
	import { PROJECTS_SERVICE } from '$lib/project/projectsService';
	import {
		branchHasConflicts,
		branchHasUnpushedCommits,
		partialStackRequestsForcePush
	} from '$lib/stacks/stack';
	import { STACK_SERVICE } from '$lib/stacks/stackService.svelte';
	import { combineResults } from '$lib/state/helpers';
	import { UI_STATE } from '$lib/state/uiState.svelte';
	import { getBranchNameFromRef } from '$lib/utils/branch';
	import { splitMessage } from '$lib/utils/commitMessage';
	import { URL_SERVICE } from '$lib/utils/url';
	import { inject } from '@gitbutler/core/context';
	import { persisted } from '@gitbutler/shared/persisted';
	import {
		Button,
		Checkbox,
		Modal,
		TestId,
		SimpleCommitRow,
		ScrollableContainer,
		chipToasts
	} from '@gitbutler/ui';
	import { isDefined } from '@gitbutler/ui/utils/typeguards';
	import type { GerritPushFlag } from '$lib/stacks/stack';

	type Props = {
		projectId: string;
		stackId?: string;
		branchName: string;
		multipleBranches: boolean;
		isLastBranchInStack?: boolean;
		isFirstBranchInStack?: boolean;
	};

	const {
		projectId,
		branchName,
		stackId,
		multipleBranches,
		isFirstBranchInStack,
		isLastBranchInStack
	}: Props = $props();

	const stackService = inject(STACK_SERVICE);
	const projectsService = inject(PROJECTS_SERVICE);
	const uiState = inject(UI_STATE);
	const forge = inject(DEFAULT_FORGE_FACTORY);
	const urlService = inject(URL_SERVICE);
	const clipboardService = inject(CLIPBOARD_SERVICE);

	// Get current project to check gerrit_mode
	const projectResponse = $derived(projectsService.getProject(projectId));
	const isGerritMode = $derived(projectResponse.response?.gerrit_mode ?? false);

	// Component is read-only when stackId is undefined
	const isReadOnly = $derived(!stackId);

	const branchDetails = $derived(stackService.branchDetails(projectId, stackId, branchName));
	const branchesQuery = $derived(stackService.branches(projectId, stackId));
	const [pushStack, pushQuery] = stackService.pushStack;
	const upstreamCommitsQuery = $derived(
		stackService.upstreamCommits(projectId, stackId, branchName)
	);
	const upstreamCommits = $derived(upstreamCommitsQuery.response);
	const runHooks = $derived(projectRunCommitHooks(projectId));

	function handleClick(args: {
		withForce: boolean;
		skipForcePushProtection: boolean;
		gerritFlags: GerritPushFlag[];
	}) {
		if (isGerritMode) {
			gerritModal?.show();
			return;
		}

		if (multipleBranches && !isLastBranchInStack && !$doNotShowPushBelowWarning) {
			confirmationModal?.show();
			return;
		}

		push(args);
	}

	async function push(args: {
		withForce: boolean;
		skipForcePushProtection: boolean;
		gerritFlags: GerritPushFlag[];
	}) {
		const { withForce, skipForcePushProtection, gerritFlags } = args;
		try {
			const pushResult = await pushStack({
				projectId,
				stackId,
				withForce,
				skipForcePushProtection,
				branch: branchName,
				runHooks: $runHooks,
				pushOpts: gerritFlags
			});

			const upstreamBranchNames = pushResult.branchToRemote
				.map(([_, refname]) => getBranchNameFromRef(refname, pushResult.remote))
				.filter(isDefined);
			if (upstreamBranchNames.length === 0) return;
			uiState.project(projectId).branchesToPoll.add(...upstreamBranchNames);

			// Show success notification
			const branchText =
				multipleBranches && !isLastBranchInStack
					? `${branchName} 及其下方的所有分支`
					: branchName;
			chipToasts.success(`已成功推送 ${branchText}`);
		} catch (error: any) {
			if (error?.code === 'errors.git.force_push_protection') {
				forcePushProtectionModal?.show();
				return;
			}
			throw error;
		}
	}

	const loading = $derived(pushQuery.current.isLoading);

	function getButtonTooltip(
		hasThingsToPush: boolean,
		hasConflicts: boolean,
		withForce: boolean,
		remoteTrackingBranch: string | null
	): string | undefined {
		if (isReadOnly) {
			return '只读模式';
		}

		if (!hasThingsToPush) {
			return '没有可推送的提交';
		}

		if (hasConflicts) {
			return '要推送，请先解决所有冲突的提交。';
		}

		if (multipleBranches && !isLastBranchInStack) {
			return '推送该分支及其下方的所有分支';
		}

		if (withForce) {
			return remoteTrackingBranch
				? '强制推送此分支'
				: `将此分支强制推送到 ${remoteTrackingBranch}`;
		}

		return remoteTrackingBranch
			? `将此分支推送到 ${remoteTrackingBranch}`
			: '推送此分支';
	}

	const doNotShowPushBelowWarning = persisted<boolean>(false, 'doNotShowPushBelowWarning');
	let confirmationModal = $state<ReturnType<typeof Modal>>();
	let forcePushProtectionModal = $state<ReturnType<typeof Modal>>();
	let gerritModal = $state<GerritPushModal>();
	let pendingGerritFlags = $state<GerritPushFlag[]>([]);
</script>

<ReduxResult {projectId} result={combineResults(branchDetails.result, branchesQuery.result)}>
	{#snippet children([branchDetails, branches])}
		{@const withForce = partialStackRequestsForcePush(branchName, branches)}
		{@const hasThingsToPush = branchHasUnpushedCommits(branchDetails)}
		{@const hasConflicts = branchHasConflicts(branchDetails)}
		{@const buttonDisabled = isReadOnly || !hasThingsToPush || hasConflicts}

		<Button
			testId={TestId.StackPushButton}
			kind={isFirstBranchInStack ? 'solid' : 'outline'}
			size="tag"
			style="gray"
			{loading}
			disabled={buttonDisabled}
			tooltip={getButtonTooltip(
				hasThingsToPush,
				hasConflicts,
				withForce,
				branchDetails.remoteTrackingBranch
			)}
			onclick={() => handleClick({ withForce, skipForcePushProtection: false, gerritFlags: [] })}
			icon={multipleBranches && !isLastBranchInStack ? 'push-below' : 'push'}
		>
			{isGerritMode ? '推送' : withForce ? '强制推送' : '推送'}
		</Button>

		<Modal
			title="推送依赖分支"
			width="small"
			bind:this={confirmationModal}
			onSubmit={async (close) => {
				close();
				push({
					withForce,
					skipForcePushProtection: false,
					gerritFlags: pendingGerritFlags
				});
				pendingGerritFlags = [];
			}}
		>
			<p>
				你即将推送 <span class="text-bold">{branchName}</span>。为保持正确的历史记录，GitButler
				还会推送该堆叠中位于此分支下方的所有分支。
			</p>

			{#snippet controls(close)}
				<div class="modal-footer">
					<div class="flex flex-1">
						<label for="dont-show-again" class="modal-footer__checkbox">
							<Checkbox name="dont-show-again" small bind:checked={$doNotShowPushBelowWarning} />
							<span class="text-12"> 不再提示</span>
						</label>
					</div>
					<Button
						kind="outline"
						onclick={() => {
							$doNotShowPushBelowWarning = false;
							close();
						}}
					>
						取消
					</Button>
					<Button testId={TestId.StackConfirmPushModalButton} style="pop" type="submit" width={90}>
						推送
					</Button>
				</div>
			{/snippet}
		</Modal>

		<Modal
			title="受保护的强制推送"
			width={480}
			type="warning"
			bind:this={forcePushProtectionModal}
			onSubmit={async (close) => {
				close();
				push({
					withForce,
					skipForcePushProtection: true,
					gerritFlags: pendingGerritFlags
				});
				pendingGerritFlags = [];
			}}
		>
			<p class="description">
				强制推送被阻止，因为远程分支包含你的本地分支没有的
				<span class="text-bold text-nowrap"
					>{upstreamCommits?.length === 1 ? '1 个提交' : `${upstreamCommits?.length} 个提交`}</span
				>。为避免覆盖历史，请 <span class="text-bold">取消并拉取并集成</span> 这些更改。
			</p>
			{#if upstreamCommits}
				<div class="scroll-wrap">
					<ScrollableContainer maxHeight="16.5rem">
						{#each upstreamCommits as commit}
							{@const commitUrl = forge.current.commitUrl(commit.id)}
							<SimpleCommitRow
								title={splitMessage(commit.message).title ?? ''}
								sha={commit.id}
								date={commitCreatedAtDate(commit)}
								author={commit.author.name}
								url={commitUrl}
								onOpen={(url) => urlService.openExternalUrl(url)}
								onCopy={() => clipboardService.write(commit.id, { message: '已复制提交哈希' })}
							/>
						{/each}
					</ScrollableContainer>
				</div>
			{/if}

			{#snippet controls(close)}
				<div class="controls">
					<Button kind="outline" type="submit">仍然强制推送</Button>
					<Button wide style="pop" onclick={close}>取消</Button>
				</div>
			{/snippet}
		</Modal>

		<GerritPushModal
			bind:this={gerritModal}
			{projectId}
			{stackId}
			{branchName}
			{multipleBranches}
			{isFirstBranchInStack}
			{isLastBranchInStack}
			onPush={(gerritFlags) => {
				if (multipleBranches && !isLastBranchInStack && !$doNotShowPushBelowWarning) {
					// Store all gerrit flags for later use when confirmation modal completes
					pendingGerritFlags = gerritFlags;
					confirmationModal?.show();
				} else {
					push({ withForce, skipForcePushProtection: false, gerritFlags });
				}
			}}
		/>
	{/snippet}
</ReduxResult>

<style>
	/* MODAL */
	.modal-footer {
		display: flex;
		width: 100%;
		gap: 6px;
	}

	/* CONTROLS */
	.controls {
		display: flex;
		width: 100%;
		gap: 6px;
	}

	.modal-footer__checkbox {
		display: flex;
		align-items: center;
		gap: 8px;
	}

	/* COMMITS SCROLL CONTAINER */
	.description {
		margin: 0 0 16px;
	}
	.scroll-wrap {
		overflow: hidden;
		border: 1px solid var(--clr-border-2);
		border-radius: var(--radius-m);
	}
</style>
