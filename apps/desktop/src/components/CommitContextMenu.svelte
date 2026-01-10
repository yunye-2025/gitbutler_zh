<script lang="ts" module>
	import type { CommitStatusType } from '$lib/commits/commit';
	interface BaseContextData {
		commitStatus: CommitStatusType;
		commitId: string;
		commitMessage: string;
		commitUrl?: string;
	}

	interface LocalCommitContextData extends BaseContextData {
		commitStatus: 'LocalOnly' | 'LocalAndRemote';
		stackId?: string;
		onUncommitClick: (event: MouseEvent) => void;
		onEditMessageClick: (event: MouseEvent) => void;
	}

	interface RemoteCommitContextData extends BaseContextData {
		commitStatus: 'Remote';
		stackId?: string;
	}

	interface IntegratedCommitContextData extends BaseContextData {
		commitStatus: 'Integrated';
		stackId?: string;
	}

	interface BaseCommitContextData extends BaseContextData {
		commitStatus: 'Base';
	}

	export type CommitContextData =
		| LocalCommitContextData
		| RemoteCommitContextData
		| IntegratedCommitContextData
		| BaseCommitContextData;

	export type CommitMenuContext = {
		position: { coords?: { x: number; y: number }; element?: HTMLElement };
		data: CommitContextData;
	};
</script>

<script lang="ts">
	import { CLIPBOARD_SERVICE } from '$lib/backend/clipboard';
	import { rewrapCommitMessage } from '$lib/config/uiFeatureFlags';
	import { editPatch } from '$lib/editMode/editPatchUtils';
	import { MODE_SERVICE } from '$lib/mode/modeService';
	import { STACK_SERVICE } from '$lib/stacks/stackService.svelte';
	import { URL_SERVICE } from '$lib/utils/url';
	import { ensureValue } from '$lib/utils/validation';
	import { inject, injectOptional } from '@gitbutler/core/context';
	import {
		ContextMenuItem,
		ContextMenuItemSubmenu,
		ContextMenuSection,
		KebabButton,
		TestId
	} from '@gitbutler/ui';
	import type { AnchorPosition } from '$lib/stacks/stack';

	type Props = {
		showOnHover?: boolean;
		projectId: string;
		openId?: string;
		rightClickTrigger?: HTMLElement;
		contextData: CommitContextData | undefined;
	};

	let {
		showOnHover,
		projectId,
		openId = $bindable(),
		rightClickTrigger,
		contextData
	}: Props = $props();

	const urlService = inject(URL_SERVICE);
	const stackService = inject(STACK_SERVICE);
	const clipboardService = inject(CLIPBOARD_SERVICE);
	const modeService = injectOptional(MODE_SERVICE, undefined);
	const [insertBlankCommitInBranch, commitInsertion] = stackService.insertBlankCommit;
	const [createRef, refCreation] = stackService.createReference;

	// 当 stackId 为空时，该组件为只读状态
	const isReadOnly = $derived(
		contextData?.commitStatus === 'LocalAndRemote' || contextData?.commitStatus === 'LocalOnly'
			? !contextData.stackId
			: false
	);

	async function insertBlankCommit(
		stackId: string,
		commitId: string,
		location: 'above' | 'below' = 'below'
	) {
		await insertBlankCommitInBranch({
			projectId,
			stackId,
			commitId: commitId,
			offset: location === 'above' ? -1 : 1
		});
	}

	async function handleCreateNewRef(stackId: string, commitId: string, position: AnchorPosition) {
		const newName = await stackService.fetchNewBranchName(projectId);
		await createRef({
			projectId,
			stackId,
			request: {
				newName,
				anchor: {
					type: 'atCommit',
					subject: {
						commit_id: commitId,
						position
					}
				}
			}
		});
	}

	async function handleEditPatch(commitId: string, stackId: string) {
		if (isReadOnly) return;
		await editPatch({
			modeService,
			commitId,
			stackId,
			projectId
		});
	}
</script>

{#if rightClickTrigger && contextData}
	<KebabButton
		{showOnHover}
		contextElement={rightClickTrigger}
		testId={TestId.KebabMenuButton}
		contextMenuTestId={TestId.CommitRowContextMenu}
	>
		{#snippet contextMenu({ close })}
			{@const { commitId, commitUrl, commitMessage } = contextData}
			{#if contextData.commitStatus === 'LocalAndRemote' || contextData.commitStatus === 'LocalOnly'}
				{@const { onUncommitClick, onEditMessageClick } = contextData}
				<ContextMenuSection>
					<ContextMenuItem
						label="撤销提交"
						icon="undo-small"
						testId={TestId.CommitRowContextMenu_UncommitMenuButton}
						disabled={isReadOnly}
						onclick={(e: MouseEvent) => {
							if (!isReadOnly) {
								onUncommitClick?.(e);
								close();
							}
						}}
					/>
					<ContextMenuItem
						label="编辑提交信息"
						icon="edit"
						testId={TestId.CommitRowContextMenu_EditMessageMenuButton}
						disabled={isReadOnly}
						onclick={(e: MouseEvent) => {
							if (!isReadOnly) {
								onEditMessageClick?.(e);
								close();
							}
						}}
					/>
					<ContextMenuItem
						label="编辑提交"
						icon="edit-commit"
						testId={TestId.CommitRowContextMenu_EditCommit}
						disabled={isReadOnly}
						onclick={async () => {
							if (!isReadOnly && contextData.stackId) {
								await handleEditPatch(commitId, contextData.stackId);
								close();
							}
						}}
					/>
				</ContextMenuSection>
			{/if}
			<ContextMenuSection>
				{#if commitUrl}
					<ContextMenuItem
						label="在浏览器中打开"
						icon="open-link"
						onclick={async () => {
							await urlService.openExternalUrl(commitUrl);
							close();
						}}
					/>
				{/if}
				<ContextMenuItemSubmenu label="复制" icon="copy">
					{#snippet submenu({ close: closeSubmenu })}
						<ContextMenuSection>
							{#if commitUrl}
								<ContextMenuItem
									label="复制提交链接"
									onclick={() => {
										clipboardService.write(commitUrl, { message: '提交链接已复制' });
										closeSubmenu();
										close();
									}}
								/>
							{/if}
							<ContextMenuItem
								label="复制提交哈希"
								onclick={() => {
									clipboardService.write(commitId, { message: '提交哈希已复制' });
									closeSubmenu();
									close();
								}}
							/>
							<ContextMenuItem
								label="复制提交信息"
								onclick={() => {
									clipboardService.write(commitMessage, { message: '提交信息已复制' });
									closeSubmenu();
									close();
								}}
							/>
						</ContextMenuSection>
					{/snippet}
				</ContextMenuItemSubmenu>
				{#if contextData.commitStatus === 'LocalAndRemote' || contextData.commitStatus === 'LocalOnly'}
					{@const stackId = contextData.stackId}

					<ContextMenuItemSubmenu label="添加空提交" icon="new-empty-commit">
						{#snippet submenu({ close: closeSubmenu })}
							<ContextMenuSection>
								<ContextMenuItem
									label="在上方添加空提交"
									disabled={isReadOnly || commitInsertion.current.isLoading}
									onclick={() => {
										insertBlankCommit(ensureValue(stackId), commitId, 'above');
										closeSubmenu();
										close();
									}}
								/>
								<ContextMenuItem
									label="在下方添加空提交"
									disabled={isReadOnly || commitInsertion.current.isLoading}
									onclick={() => {
										insertBlankCommit(ensureValue(stackId), commitId, 'below');
										closeSubmenu();
										close();
									}}
								/>
							</ContextMenuSection>
						{/snippet}
					</ContextMenuItemSubmenu>
					<ContextMenuItemSubmenu label="创建分支" icon="branch-remote">
						{#snippet submenu({ close: closeSubmenu })}
							<ContextMenuSection>
								<ContextMenuItem
									label="从此提交创建分支"
									disabled={isReadOnly || refCreation.current.isLoading}
									onclick={async () => {
										if (!isReadOnly) {
											await handleCreateNewRef(ensureValue(stackId), commitId, 'Above');
											closeSubmenu();
											close();
										}
									}}
								/>
								<ContextMenuItem
									label="在此提交之后创建分支"
									disabled={isReadOnly || refCreation.current.isLoading}
									onclick={async () => {
										if (!isReadOnly) {
											await handleCreateNewRef(ensureValue(stackId), commitId, 'Below');
											closeSubmenu();
											close();
										}
									}}
								/>
							</ContextMenuSection>
						{/snippet}
					</ContextMenuItemSubmenu>
				{/if}
			</ContextMenuSection>

			<ContextMenuSection>
				<ContextMenuItem
					label={$rewrapCommitMessage ? '显示原始换行' : '重新换行'}
					icon="text-wrap"
					disabled={commitInsertion.current.isLoading}
					onclick={() => {
						rewrapCommitMessage.set(!$rewrapCommitMessage);
						close();
					}}
				/>
			</ContextMenuSection>
		{/snippet}
	</KebabButton>
{/if}
