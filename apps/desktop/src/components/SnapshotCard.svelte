<script lang="ts">
	import FileList from '$components/FileList.svelte';
	import ReduxResult from '$components/ReduxResult.svelte';
	import SnapshotAttachment from '$components/SnapshotAttachment.svelte';
	import { createdOnDay, HISTORY_SERVICE } from '$lib/history/history';
	import { MODE_SERVICE } from '$lib/mode/modeService';
	import { toHumanReadableTime } from '$lib/utils/time';
	import { inject } from '@gitbutler/core/context';
	import { Button, Icon, ScrollableContainer } from '@gitbutler/ui';
	import { focusable } from '@gitbutler/ui/focus/focusable';
	import type { Snapshot, SnapshotDetails } from '$lib/history/types';
	import type iconsJson from '@gitbutler/ui/data/icons.json';

	interface Props {
		entry: Snapshot;
		isWithinRestore?: boolean;
		onRestoreClick: () => void;
		onDiffClick: (filePath: string) => void;
		projectId: string;
	}

	const { projectId, entry, isWithinRestore = true, onRestoreClick, onDiffClick }: Props = $props();

	function getShortSha(sha: string | undefined) {
		if (!sha) return '';

		return `${sha.slice(0, 7)}`;
	}

	function createdOnDayAndTime(epoch: number) {
		const date = new Date(epoch);
		return `${createdOnDay(date)}, ${toHumanReadableTime(date)}`;
	}

	function camelToTitleCase(str: string | undefined) {
		if (!str) return '';
		const lowerCaseStr = str.replace(/([a-z])([A-Z])/g, '$1 $2').toLowerCase();
		return lowerCaseStr.charAt(0).toUpperCase() + lowerCaseStr.slice(1);
	}

	function getOperationLabel(operation: string | undefined) {
		if (!operation) return '';

		const labels: Record<string, string> = {
			DeleteBranch: '删除分支',
			ApplyBranch: '应用分支',
			UnapplyBranch: '取消应用分支',
			UpdateBranchName: '重命名分支',
			CreateBranch: '创建分支',
			ReorderBranches: '重排分支',
			SelectDefaultVirtualBranch: '选择默认虚拟分支',
			UpdateBranchRemoteName: '更新分支远程名称',
			SetBaseBranch: '设置基线分支',
			GenericBranchUpdate: '分支常规更新',
			CreateCommit: '创建提交',
			UndoCommit: '撤销提交',
			AmendCommit: '修订提交',
			SquashCommit: '压缩提交',
			UpdateCommitMessage: '更新提交信息',
			MoveCommit: '移动提交',
			MoveBranch: '移动分支',
			TearOffBranch: '拆出分支',
			ReorderCommit: '调整提交顺序',
			InsertBlankCommit: '插入空提交',
			MoveCommitFile: '移动提交文件',
			MoveHunk: '移动补丁块',
			DiscardLines: '丢弃行',
			DiscardHunk: '丢弃更改',
			DiscardFile: '丢弃文件',
			FileChanges: '文件更改',
			MergeUpstream: '合并上游',
			UpdateWorkspaceBase: '更新工作区基线',
			EnterEditMode: '进入编辑模式',
			RestoreFromSnapshot: '还原快照',
			SplitBranch: '拆分分支',
			OnDemandSnapshot: '手动快照'
		};

		return labels[operation] ?? camelToTitleCase(operation);
	}

	function mapOperation(snapshotDetails: SnapshotDetails | undefined): {
		text: string;
		icon?: keyof typeof iconsJson;
		commitMessage?: string;
	} {
		if (!snapshotDetails) return { text: '', icon: 'commit' };

		switch (snapshotDetails.operation) {
			// BRANCH OPERATIONS
			case 'DeleteBranch':
				return {
					text: `删除分支 "${entry.details?.trailers.find((t) => t.key === 'name')?.value}"`,
					icon: 'item-cross'
				};
			case 'ApplyBranch':
				return {
					text: `应用分支 "${entry.details?.trailers.find((t) => t.key === 'name')?.value}"`,
					icon: 'item-tick'
				};
			case 'UnapplyBranch':
				return {
					text: `取消应用分支 "${snapshotDetails.trailers.find((t) => t.key === 'name')?.value}"`,
					icon: 'item-dashed'
				};
			case 'UpdateBranchName':
				return {
					text: `将分支 "${snapshotDetails.trailers.find((t) => t.key === 'before')?.value}" 重命名为 "${snapshotDetails.trailers.find((t) => t.key === 'after')?.value}"`,
					icon: 'item-slash'
				};
			case 'CreateBranch':
				return {
					text: `创建分支 "${snapshotDetails.trailers.find((t) => t.key === 'name')?.value}"`,
					icon: 'item-plus'
				};
			case 'ReorderBranches':
				return {
					text: `重排分支 "${snapshotDetails.trailers.find((t) => t.key === 'before')?.value}" 和 "${snapshotDetails.trailers.find((t) => t.key === 'after')?.value}"`,
					icon: 'item-link'
				};
			case 'SelectDefaultVirtualBranch':
				return {
					text: `选择默认虚拟分支 "${snapshotDetails.trailers.find((t) => t.key === 'after')?.value}"`,
					icon: 'item-dot'
				};
			case 'UpdateBranchRemoteName':
				return {
					text: `将分支远程名称从 "${snapshotDetails.trailers.find((t) => t.key === 'before')?.value}" 更新为 "${snapshotDetails.trailers.find((t) => t.key === 'after')?.value}"`,
					icon: 'item-slash'
				};
			case 'SetBaseBranch':
				return { text: '设置基线分支', icon: 'item-slash' };
			case 'GenericBranchUpdate':
				return { text: '分支常规更新', icon: 'item-slash' };

			// COMMIT OPERATIONS
			case 'CreateCommit':
				return {
					text: `创建提交 ${getShortSha(entry.details?.trailers.find((t) => t.key === 'sha')?.value)}`,
					icon: 'new-commit',
					commitMessage: entry.details?.trailers.find((t) => t.key === 'message')?.value
				};
			case 'UndoCommit':
				return {
					text: `撤销提交 ${getShortSha(entry.details?.trailers.find((t) => t.key === 'sha')?.value)}`,
					icon: 'undo-commit',
					commitMessage: entry.details?.trailers.find((t) => t.key === 'message')?.value
				};
			case 'AmendCommit':
				return { text: '修订提交', icon: 'amend-commit' };
			case 'SquashCommit':
				return { text: '压缩提交', icon: 'squash-commit' };
			case 'UpdateCommitMessage':
				return { text: '更新提交信息', icon: 'edit' };
			case 'MoveCommit':
				return { text: '移动提交', icon: 'move-commit' };
			case 'MoveBranch':
				return { text: '移动分支', icon: 'move-commit' };
			case 'TearOffBranch':
				return { text: '拆出分支', icon: 'move-commit' };
			case 'ReorderCommit':
				return { text: '调整提交顺序', icon: 'move-commit' };
			case 'InsertBlankCommit':
				return { text: '插入空提交', icon: 'blank-commit' };
			case 'MoveCommitFile':
				return { text: '移动提交文件', icon: 'move-commit-file-small' };

			// FILE OPERATIONS
			case 'MoveHunk':
				return {
					text: `移动补丁块到 "${entry.details?.trailers.find((t) => t.key === 'name')?.value}"`,
					icon: 'item-move'
				};
			case 'DiscardLines':
				return { text: '丢弃行', icon: 'item-cross' };
			case 'DiscardHunk':
				return { text: '丢弃更改', icon: 'item-cross' };
			case 'DiscardFile':
				return { text: '丢弃文件', icon: 'discard-file-small' };
			case 'FileChanges':
				return { text: '文件更改', icon: 'file-changes-small' };

			// OTHER OPERATIONS
			case 'MergeUpstream':
				return { text: '合并上游', icon: 'merged-pr-small' };
			case 'UpdateWorkspaceBase':
				return { text: '更新工作区基线', icon: 'rebase' };
			case 'EnterEditMode':
				return { text: '进入编辑模式', icon: 'edit' };
			case 'RestoreFromSnapshot':
				return { text: '还原快照' };
			case 'SplitBranch':
				return { text: '拆分分支', icon: 'branch-local' };
			case 'OnDemandSnapshot':
				return {
					text: snapshotDetails.body
						? `手动快照：${snapshotDetails.body}`
						: '手动快照',
					icon: 'camera'
				};
			default:
				return { text: snapshotDetails.operation, icon: 'commit' };
		}
	}

	const isRestoreSnapshot = entry.details?.operation === 'RestoreFromSnapshot';
	const error = entry.details?.trailers.find((t) => t.key === 'error')?.value;

	const operation = mapOperation(entry.details);

	const modeService = inject(MODE_SERVICE);
	const mode = $derived(modeService.mode(projectId));

	const historyService = inject(HISTORY_SERVICE);
	const snapshotDiff = $derived(historyService.snapshotDiff({ projectId, snapshotId: entry.id }));
</script>

<div
	class="snapshot-card show-restore-on-hover"
	class:restored-snapshot={isRestoreSnapshot || isWithinRestore}
	use:focusable={{ focusable: true }}
>
	<div class="snapshot-right-container">
		<div class="restore-btn">
			<Button
				size="tag"
				kind="outline"
				tooltip="将 GitButler 和你的文件恢复到此操作之前的状态。还原操作也可以撤销。"
				onclick={() => {
					onRestoreClick();
				}}
				disabled={mode.response?.type !== 'OpenWorkspace'}>还原</Button
			>
		</div>
		<span class="snapshot-time text-11">
			{toHumanReadableTime(entry.createdAt)}
		</span>
	</div>

	<div class="snapshot-line">
		{#if isRestoreSnapshot}
			<img src="/images/history/restore-icon.svg" alt="" />
		{:else if operation.icon}
			<Icon name={operation.icon} />
		{/if}
	</div>

	<div class="snapshot-content">
		<div class="snapshot-details">
			<h4 class="snapshot-title text-13 text-body text-semibold">
				<span>{operation.text}</span>
				<span class="snapshot-sha text-12 text-body"> • {getShortSha(entry.id)}</span>
			</h4>

			{#if operation.commitMessage}
				<p class="text-12 text-body snapshot-commit-message">
					<span>消息：</span>
					{operation.commitMessage}
				</p>
			{/if}
		</div>

		<ReduxResult result={snapshotDiff.result} {projectId}>
			{#snippet children(files)}
				{#if files.length > 0 && !isRestoreSnapshot}
					<SnapshotAttachment
						foldable={files.length > 2}
						foldedAmount={files.length}
						foldedHeight="7rem"
					>
						<ScrollableContainer>
							<FileList
								selectionId={{ type: 'snapshot', snapshotId: entry.id }}
								{projectId}
								stackId={undefined}
								changes={files}
								listMode="list"
								onselect={(change) => onDiffClick(change.path)}
								allowUnselect={false}
							/>
						</ScrollableContainer>
					</SnapshotAttachment>
				{/if}
			{/snippet}
		</ReduxResult>

		{#if isRestoreSnapshot}
			<SnapshotAttachment>
				<div class="restored-attacment">
					<Icon name="commit" />
					<div class="restored-attacment__content">
						<h4 class="text-13 text-semibold">
							{getOperationLabel(
								entry.details?.trailers.find((t) => t.key === 'restored_operation')?.value
							)}
						</h4>
						<span class="restored-attacment__details text-12">
							{getShortSha(entry.details?.trailers.find((t) => t.key === 'restored_from')?.value)} •
							{createdOnDayAndTime(
								parseInt(
									entry.details?.trailers.find((t) => t.key === 'restored_date')?.value || ''
								)
							)}
						</span>
					</div>
				</div>
			</SnapshotAttachment>
		{/if}
		{#if error}
			<div class="error-text text-12 text-body">
				{error}
			</div>
		{/if}
	</div>
</div>

<style lang="postcss">
	/* SNAPSHOT CARD */
	.snapshot-card {
		display: flex;
		position: relative;
		padding: 10px 14px 8px 14px;
		overflow: hidden;
		gap: 12px;
		background-color: var(--clr-bg-1);
		transition: padding 0.2s;
	}

	.show-restore-on-hover {
		&:hover {
			background-color: var(--hover-bg-1);
			& .restore-btn {
				display: flex;
			}
			& .snapshot-time {
				display: none;
			}
		}
	}

	.show-restore-on-hover:global(.focused) {
		background-color: var(--hover-bg-1);
		& .restore-btn {
			display: flex;
		}
		& .snapshot-time {
			display: none;
		}
	}

	.snapshot-right-container {
		display: flex;
		justify-content: flex-end;
		width: 60px;
	}

	.restore-btn {
		display: none;
	}

	.snapshot-time {
		margin-top: 2px;
		color: var(--clr-text-2);
		line-height: 1.8;
		text-align: right;
	}

	.snapshot-line {
		display: flex;
		position: relative;
		flex-direction: column;
		align-items: center;
		margin-top: 3px;

		&::after {
			position: absolute;
			top: 24px;
			width: 1px;
			height: calc(100% - 14px);
			min-height: 8px;
			background-color: var(--clr-border-2);
			content: '';
		}
	}

	/* CARD CONTENT */
	.snapshot-content {
		display: flex;
		flex: 1;
		flex-direction: column;
		align-items: flex-start;
		min-height: var(--size-tag);
		overflow: hidden;
		gap: 6px;
	}

	.snapshot-details {
		display: flex;
		flex-direction: column;
		align-items: flex-start;
		width: 100%;
		margin-top: 2px;
		margin-bottom: 4px;
		gap: 6px;
	}

	.snapshot-title {
		flex: 1;
	}

	.snapshot-commit-message {
		margin-bottom: 2px;
		color: var(--clr-text-2);

		& span {
			color: var(--clr-text-3);
		}
	}

	.snapshot-sha {
		color: var(--clr-text-3);
		white-space: nowrap;
	}

	/* ATTACHMENT RESTORE */
	.restored-attacment {
		display: flex;
		padding: 12px;
		gap: 8px;
	}

	.restored-attacment__content {
		display: flex;
		flex-direction: column;
		gap: 6px;
	}

	.restored-attacment__details {
		color: var(--clr-text-2);
	}

	/* RESTORED  */
	.restored-snapshot {
		background-color: var(--clr-bg-2);
	}

	/* --- */
	.error-text {
		display: flex;
		width: 100%;
		padding: 6px 10px;
		border-radius: var(--radius-m);
		background-color: var(--clr-theme-danger-bg);
		color: var(--clr-theme-danger-element);
	}
</style>
