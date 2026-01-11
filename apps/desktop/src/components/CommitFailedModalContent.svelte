<script lang="ts">
	import CommitFailedFileEntry from '$components/CommitFailedFileEntry.svelte';
	import ConfigurableScrollableContainer from '$components/ConfigurableScrollableContainer.svelte';
	import { REJECTTION_REASONS, type RejectionReason } from '$lib/stacks/stackService.svelte';
	import { Icon, ModalHeader, TestId, Tooltip } from '@gitbutler/ui';
	import type { CommitFailedModalState } from '$lib/state/uiState.svelte';

	type Props = {
		data: CommitFailedModalState;
		oncloseclick?: () => void;
	};

	const { data, oncloseclick }: Props = $props();

	type ReasonGroup = {
		reason: RejectionReason;
		reasonReadable: string;
		paths: string[];
	};

	function getReadableRejectionReason(reason: RejectionReason): string {
		switch (reason) {
			case 'cherryPickMergeConflict':
				return '拣选合并冲突';
			case 'noEffectiveChanges':
				return '无有效更改';
			case 'workspaceMergeConflict':
				return '工作区合并冲突';
			case 'worktreeFileMissingForObjectConversion':
				return '对象转换缺少工作树文件';
			case 'fileToLargeOrBinary':
				return '文件过大或为二进制';
			case 'pathNotFoundInBaseTree':
				return '基础树中未找到路径';
			case 'unsupportedDirectoryEntry':
				return '不支持的目录条目';
			case 'unsupportedTreeEntry':
				return '不支持的树条目';
			case 'missingDiffSpecAssociation':
				return '缺少差异规格关联';
		}
	}

	function groupByReason(data: CommitFailedModalState): ReasonGroup[] {
		const grouped: Partial<Record<RejectionReason, string[]>> = {};

		for (const [path, reason] of Object.entries(data.pathsToRejectedChanges)) {
			if (!grouped[reason]) {
				grouped[reason] = [];
			}
			grouped[reason].push(path);
		}

		const result: ReasonGroup[] = [];
		for (const reason of REJECTTION_REASONS) {
			const paths = grouped[reason];
			if (!paths) continue;
			result.push({ reason, reasonReadable: getReadableRejectionReason(reason), paths });
		}

		return result;
	}

	const groupedData = groupByReason(data);

	let isScrollTopVisible = $state(true);
</script>

<div class="commit-failed__wrapper">
	<ModalHeader
		sticky={!isScrollTopVisible}
		type={data.newCommitId ? 'warning' : 'danger'}
		closeButton
		{oncloseclick}
		closeButtonTestId={TestId.GlobalModalActionButton}
		>{data.newCommitId ? '部分更改未提交' : '创建提交失败'}</ModalHeader
	>
	<ConfigurableScrollableContainer
		onscrollTop={(visible) => {
			isScrollTopVisible = visible;
		}}
	>
		<div class="commit-failed__content">
			<div class="text-13 commit-failed__description">
				{#if data.newCommitId}
					提交 <i class="commit-failed__text-icon"><Icon name="commit" /></i>
					<Tooltip text={data.commitTitle ? data.commitTitle : '未提供提交标题'}
						><span class="h-dotted-underline text-semibold">{data.newCommitId.substring(0, 7)}</span
						></Tooltip
					> 已创建，但部分更改未完整提交：
				{:else}
					由于以下原因无法创建提交：
				{/if}
			</div>

			<div class="commit-failed__reasons">
				{#each groupedData as { reason, paths, reasonReadable } (reason)}
					<hr class="commit-failed__reasons-divider" />

					<p class="text-13">
						原因：<span class="text-bold">{reasonReadable}</span>
					</p>

					<div class="commit-failed__reason-file-list">
						{#each paths as path (path)}
							<CommitFailedFileEntry {path} {reason} projectId={data.projectId} />
						{/each}
					</div>
				{/each}
			</div>
		</div>
	</ConfigurableScrollableContainer>
</div>

<style lang="postcss">
	.commit-failed__wrapper {
		display: flex;
		flex-direction: column;
		overflow: hidden;
	}

	.commit-failed__content {
		display: flex;
		flex-direction: column;
		padding: 0 16px 16px 16px;
		gap: 16px;
	}

	.commit-failed__text-icon {
		display: inline-flex;
		align-items: center;
		gap: 4px;
		transform: translateY(4px);
		color: var(--clr-text-2);
	}

	/* Groups */
	.commit-failed__reasons {
		display: flex;
		flex-direction: column;
		gap: 16px;
	}

	.commit-failed__reasons-divider {
		margin: 0 -16px;
		border: 0;
		border-top: 1px solid var(--clr-border-2);
	}

	.commit-failed__reason-file-list {
		display: flex;
		flex-direction: column;
		gap: 8px;
	}
</style>
