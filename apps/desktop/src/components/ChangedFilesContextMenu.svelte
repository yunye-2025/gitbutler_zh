<!-- 这是 `FileContextMenu.svelte` 的 V3 替代实现 -->
<script lang="ts">
	import BranchNameTextbox from '$components/BranchNameTextbox.svelte';
	import ReduxResult from '$components/ReduxResult.svelte';
	import { ACTION_SERVICE } from '$lib/actions/actionService.svelte';
	import { AI_SERVICE } from '$lib/ai/service';
	import { BACKEND } from '$lib/backend';
	import { CLIPBOARD_SERVICE } from '$lib/backend/clipboard';
	import { changesToDiffSpec } from '$lib/commits/utils';
	import { projectAiExperimentalFeaturesEnabled, projectAiGenEnabled } from '$lib/config/config';
	import { autoSelectBranchCreationFeature } from '$lib/config/uiFeatureFlags';
	import { FILE_SERVICE } from '$lib/files/fileService';
	import { isTreeChange, type TreeChange } from '$lib/hunks/change';
	import { vscodePath } from '$lib/project/project';
	import { PROJECTS_SERVICE } from '$lib/project/projectsService';
	import { FILE_SELECTION_MANAGER } from '$lib/selection/fileSelectionManager.svelte';
	import { SETTINGS } from '$lib/settings/userSettings';
	import { STACK_SERVICE } from '$lib/stacks/stackService.svelte';
	import { UI_STATE } from '$lib/state/uiState.svelte';
	import { computeChangeStatus } from '$lib/utils/fileStatus';
	import { getEditorUri, URL_SERVICE } from '$lib/utils/url';
	import { inject } from '@gitbutler/core/context';
	import {
		AsyncButton,
		Button,
		ContextMenu,
		ContextMenuItem,
		ContextMenuItemSubmenu,
		ContextMenuSection,
		FileListItem,
		Modal,
		chipToasts
	} from '@gitbutler/ui';

	import type { SelectionId } from '$lib/selection/key';

	type Props = {
		projectId: string;
		stackId: string | undefined;
		selectionId: SelectionId;
		trigger?: HTMLElement;
		editMode?: boolean;
	};

	type ChangedFilesItem = {
		changes: TreeChange[];
	};

	function isChangedFilesItem(item: unknown): item is ChangedFilesItem {
		return (
			typeof item === 'object' &&
			item !== null &&
			'changes' in item &&
			Array.isArray(item.changes) &&
			item.changes.every(isTreeChange)
		);
	}

	type ChangedFolderItem = ChangedFilesItem & {
		path: string;
	};

	function isChangedFolderItem(item: ChangedFilesItem): item is ChangedFolderItem {
		return 'path' in item && typeof item.path === 'string';
	}

	const { trigger, selectionId, stackId, projectId, editMode = false }: Props = $props();
	const stackService = inject(STACK_SERVICE);
	const uiState = inject(UI_STATE);
	const idSelection = inject(FILE_SELECTION_MANAGER);
	const aiService = inject(AI_SERVICE);
	const actionService = inject(ACTION_SERVICE);
	const fileService = inject(FILE_SERVICE);
	const urlService = inject(URL_SERVICE);
	const clipboardService = inject(CLIPBOARD_SERVICE);
	const backend = inject(BACKEND);
	const [autoCommit, autoCommitting] = actionService.autoCommit;
	const [branchChanges, branchingChanges] = actionService.branchChanges;
	const [absorbChanges, absorbingChanges] = actionService.absorb;
	const [splitOffChanges] = stackService.splitBranch;
	const [splitBranchIntoDependentBranch] = stackService.splitBrancIntoDependentBranch;

	const projectService = inject(PROJECTS_SERVICE);

	const userSettings = inject(SETTINGS);
	const isUncommitted = $derived(selectionId.type === 'worktree');
	const isBranchFiles = $derived(selectionId.type === 'branch');
	const selectionBranchName = $derived(
		selectionId.type === 'branch' ? selectionId.branchName : undefined
	);

	// 平台相关的“在 Finder/资源管理器/文件管理器中显示”文案
	const showInFolderLabel = (() => {
		switch (backend.platformName) {
			case 'macos':
				return '在 Finder 中显示';
			case 'windows':
				return '在资源管理器中显示';
			default:
				return '在文件管理器中显示';
		}
	})();

	let confirmationModal: ReturnType<typeof Modal> | undefined;
	let stashConfirmationModal: ReturnType<typeof Modal> | undefined;
	let contextMenu: ReturnType<typeof ContextMenu>;
	let aiConfigurationValid = $state(false);

	const aiGenEnabled = $derived(projectAiGenEnabled(projectId));
	const experimentalFeaturesEnabled = $derived(projectAiExperimentalFeaturesEnabled(projectId));

	const canUseGBAI = $derived(
		$aiGenEnabled && aiConfigurationValid && $experimentalFeaturesEnabled
	);

	function isDeleted(item: ChangedFilesItem): boolean {
		if (isChangedFolderItem(item)) {
			return false;
		}
		return item.changes.some((change) => {
			return change.status.type === 'Deletion';
		});
	}

	function getItemPath(item: ChangedFilesItem): string | null {
		if (isChangedFolderItem(item)) {
			return item.path;
		}
		if (item.changes.length === 1) {
			return item.changes[0]!.path;
		}
		return null;
	}

	async function confirmDiscard(item: ChangedFilesItem) {
		await stackService.discardChanges({
			projectId,
			worktreeChanges: changesToDiffSpec(item.changes)
		});

		const selectedFiles = item.changes.map((change) => ({ ...selectionId, path: change.path }));

		// 取消选择已丢弃的文件
		idSelection.removeMany(selectedFiles);

		confirmationModal?.close();
	}

	let stashBranchName = $state<string>();
	let slugifiedRefName: string | undefined = $state();
	let stashBranchNameInput = $state<ReturnType<typeof BranchNameTextbox>>();

	async function confirmStashIntoBranch(item: ChangedFilesItem, branchName: string | undefined) {
		if (!branchName) {
			return;
		}

		await stackService.stashIntoBranch({
			projectId,
			branchName,
			worktreeChanges: changesToDiffSpec(item.changes)
		});

		stashConfirmationModal?.close();
	}

	export function open(e: MouseEvent, item: ChangedFilesItem) {
		contextMenu.open(e, item);
		aiService.validateGitButlerAPIConfiguration().then((value) => {
			aiConfigurationValid = value;
		});
	}

	async function uncommitChanges(stackId: string, commitId: string, changes: TreeChange[]) {
		const { replacedCommits } = await stackService.uncommitChanges({
			projectId,
			stackId,
			commitId,
			changes: changesToDiffSpec(changes)
		});
		const newCommitId = replacedCommits.find(([before]) => before === commitId)?.[1];
		const branchName = uiState.lane(stackId).selection.current?.branchName;
		const selectedFiles = changes.map((change) => ({ ...selectionId, path: change.path }));

		// 取消选择已撤销提交的文件
		idSelection.removeMany(selectedFiles);

		if (newCommitId && branchName) {
			const previewOpen = uiState.lane(stackId).selection.current?.previewOpen ?? false;
			// 将选择更新为新的提交
			uiState.lane(stackId).selection.set({ branchName, commitId: newCommitId, previewOpen });
		}
		contextMenu.close();
	}

	async function triggerAutoCommit(changes: TreeChange[]) {
		if (!canUseGBAI) {
			chipToasts.error('此项目未配置或未启用 GitButler AI。');
			return;
		}

		try {
			await chipToasts.promise(autoCommit({ projectId, changes }), {
				loading: '已开始自动提交',
				success: '自动提交成功',
				error: '自动提交失败'
			});
		} catch (error) {
			console.error('自动提交失败:', error);
		}
	}

	async function triggerBranchChanges(changes: TreeChange[]) {
		if (!canUseGBAI) {
			chipToasts.error('此项目未配置或未启用 GitButler AI。');
			return;
		}

		try {
			await chipToasts.promise(branchChanges({ projectId, changes }), {
				loading: '正在创建分支并提交更改',
				success: '分支创建并提交成功',
				error: '分支创建并提交失败'
			});
		} catch (error) {
			console.error('分支创建并提交失败:', error);
		}
	}

	async function triggerAbsorbChanges(changes: TreeChange[]) {
		if (!canUseGBAI) {
			chipToasts.error('此项目未配置或未启用 GitButler AI。');
			return;
		}

		try {
			await chipToasts.promise(absorbChanges({ projectId, changes }), {
				loading: '正在寻找最佳位置以吸收更改',
				success: '吸收更改成功',
				error: '吸收更改失败'
			});
		} catch (error) {
			console.error('吸收更改失败:', error);
		}
	}

	async function split(changes: TreeChange[]) {
		if (!stackId) {
			chipToasts.error('未选择堆栈，无法拆分更改。');
			return;
		}

		if (selectionId.type !== 'branch') {
			chipToasts.error('请选择一个分支以拆分更改。');
			return;
		}

		const branchName = selectionId.branchName;

		const fileNames = changes.map((change) => change.path);

		try {
			await chipToasts.promise(
				(async () => {
					const newBranchName = await stackService.fetchNewBranchName(projectId);

					if (!newBranchName) {
						throw new Error('生成新分支名失败。');
					}

					await splitOffChanges({
						projectId,
						sourceStackId: stackId,
						sourceBranchName: branchName,
						fileChangesToSplitOff: fileNames,
						newBranchName: newBranchName
					});
				})(),
				{
					loading: '正在拆分更改',
					success: '更改已拆分到新分支',
					error: '拆分更改失败'
				}
			);
		} catch (error) {
			console.error('拆分更改失败:', error);
		}
	}

	async function splitIntoDependentBranch(changes: TreeChange[]) {
		if (!stackId) {
			chipToasts.error('未选择堆栈，无法拆分更改。');
			return;
		}

		if (selectionId.type !== 'branch') {
			chipToasts.error('请选择一个分支以拆分更改。');
			return;
		}

		const branchName = selectionId.branchName;
		const fileNames = changes.map((change) => change.path);

		try {
			await chipToasts.promise(
				(async () => {
					const newBranchName = await stackService.fetchNewBranchName(projectId);

					if (!newBranchName) {
						throw new Error('生成新分支名失败。');
					}

					await splitBranchIntoDependentBranch({
						projectId,
						sourceStackId: stackId,
						sourceBranchName: branchName,
						fileChangesToSplitOff: fileNames,
						newBranchName: newBranchName
					});
				})(),
				{
					loading: '正在拆分到依赖分支',
					success: '更改已拆分到依赖分支',
					error: '拆分到依赖分支失败'
				}
			);
		} catch (error) {
			console.error('拆分到依赖分支失败:', error);
		}
	}
</script>

<ContextMenu bind:this={contextMenu} rightClickTrigger={trigger}>
	{#snippet children(item: unknown)}
		{#if isChangedFilesItem(item)}
			{@const deletion = isDeleted(item)}
			{@const itemPath = getItemPath(item)}
			{#if item.changes.length > 0 && !editMode}
				<ContextMenuSection>
					{@const changes = item.changes}
					{#if isUncommitted}
						<ContextMenuItem
							label="丢弃更改…"
							icon="bin"
							onclick={() => {
								confirmationModal?.show(item);
								contextMenu.close();
							}}
						/>
					{/if}
					{#if isUncommitted}
						<ContextMenuItem
							label="暂存到分支…"
							icon="stash"
							onclick={async () => {
								stashConfirmationModal?.show(item);
								stashBranchName = await stackService.fetchNewBranchName(projectId);
								// 异步值加载并更新 DOM 后选中文本
								if ($autoSelectBranchCreationFeature) {
									await stashBranchNameInput?.selectAll();
								}
								contextMenu.close();
							}}
						/>
					{/if}
					{#if selectionId.type === 'commit' && stackId && !editMode}
						{@const commitId = selectionId.commitId}
						<ContextMenuItem
							label="撤销提交更改"
							icon="undo-small"
							onclick={async () => uncommitChanges(stackId, commitId, changes)}
						/>
					{/if}

					{#if isBranchFiles && stackId && selectionBranchName}
						{@const branchIsConflicted = stackService.isBranchConflicted(
							projectId,
							stackId,
							selectionBranchName
						)}
						<ReduxResult {projectId} result={branchIsConflicted?.result}>
							{#snippet children(isConflicted)}
								{#if isConflicted === false}
									<ContextMenuItem
										label="拆分更改"
										icon="split"
										onclick={() => {
											split(changes);
											contextMenu.close();
										}}
									/>
									<ContextMenuItem
										label="拆分到依赖分支"
										icon="new-dep-branch"
										onclick={() => {
											splitIntoDependentBranch(changes);
											contextMenu.close();
										}}
									/>
								{/if}
							{/snippet}
						</ReduxResult>
					{/if}
				</ContextMenuSection>
			{/if}

			{#if itemPath}
				<ContextMenuSection>
					<ContextMenuItemSubmenu label="复制路径" icon="copy">
						{#snippet submenu({ close: closeSubmenu })}
							<ContextMenuSection>
								<ContextMenuItem
									label="复制路径"
									onclick={async () => {
										const project = await projectService.fetchProject(projectId);
										const projectPath = project?.path;
										if (projectPath) {
											const absPath = await backend.joinPath(projectPath, itemPath);

											await clipboardService.write(absPath, {
												message: '已复制绝对路径',
												errorMessage: '复制绝对路径失败'
											});
										}
										closeSubmenu();
										contextMenu.close();
									}}
								/>
								<ContextMenuItem
									label="复制相对路径"
									onclick={async () => {
										await clipboardService.write(itemPath, {
											message: '已复制相对路径',
											errorMessage: '复制相对路径失败'
										});
										closeSubmenu();
										contextMenu.close();
									}}
								/>
							</ContextMenuSection>
						{/snippet}
					</ContextMenuItemSubmenu>
				</ContextMenuSection>
			{/if}

			<ContextMenuSection>
				{#if !isChangedFolderItem(item)}
				<ContextMenuItem
					label="在 {$userSettings.defaultCodeEditor.displayName} 中打开"
					icon="open-editor"
					disabled={deletion}
					onclick={async () => {
							try {
								const project = await projectService.fetchProject(projectId);
								const projectPath = project?.path;
								if (projectPath) {
									for (let change of item.changes) {
										const path = getEditorUri({
											schemeId: $userSettings.defaultCodeEditor.schemeIdentifer,
											path: [vscodePath(projectPath), change.path]
										});
										urlService.openExternalUrl(path);
									}
								}
								contextMenu.close();
							} catch {
								chipToasts.error('在编辑器中打开失败');
								console.error('在编辑器中打开失败');
							}
						}}
					/>
				{/if}
				{#if itemPath}
					<ContextMenuItem
						label={showInFolderLabel}
						icon="open-folder"
						onclick={async () => {
							const project = await projectService.fetchProject(projectId);
							const projectPath = project?.path;
							if (projectPath) {
								const absPath = await backend.joinPath(projectPath, itemPath);
								await fileService.showFileInFolder(absPath);
							}
							contextMenu.close();
						}}
					/>
				{/if}
			</ContextMenuSection>

			{#if canUseGBAI && isUncommitted}
				<ContextMenuSection>
					<ContextMenuItemSubmenu label="实验性 AI" icon="lab">
						{#snippet submenu({ close: closeSubmenu })}
							<ContextMenuSection>
								<ContextMenuItem
									label="自动提交"
									tooltip="尝试判断更改应提交到哪里，也可能会创建新分支。"
									onclick={async () => {
										closeSubmenu();
										contextMenu.close();
										triggerAutoCommit(item.changes);
									}}
									disabled={autoCommitting.current.isLoading}
								/>
								<ContextMenuItem
									label="分支更改"
									tooltip="创建新分支并将更改提交到该分支。"
									onclick={() => {
										closeSubmenu();
										contextMenu.close();
										triggerBranchChanges(item.changes);
									}}
									disabled={branchingChanges.current.isLoading}
								/>
								<ContextMenuItem
									label="吸收更改"
									tooltip="尝试找到吸收更改的最佳位置。"
									onclick={() => {
										closeSubmenu();
										contextMenu.close();
										triggerAbsorbChanges(item.changes);
									}}
									disabled={absorbingChanges.current.isLoading}
								/>
							</ContextMenuSection>
						{/snippet}
					</ContextMenuItemSubmenu>
				</ContextMenuSection>
			{/if}
		{:else}
			<ContextMenuSection>
				<p class="text-13">哎呀！数据异常 :(</p>
			</ContextMenuSection>
		{/if}
	{/snippet}
</ContextMenu>

<Modal
	width="small"
	type="warning"
	title="丢弃更改"
	bind:this={confirmationModal}
	onSubmit={(_, item) => isChangedFilesItem(item) && confirmDiscard(item)}
>
	{#snippet children(item)}
		{#if isChangedFilesItem(item)}
			{#if isChangedFolderItem(item)}
				<p class="discard-caption">
					确定要丢弃以下目录中的所有更改吗：
					<span class="text-bold">{item.path}</span>?
				</p>
			{:else}
				{@const changes = item.changes}
				{#if changes.length < 10}
					<p class="discard-caption">
						确定要丢弃以下文件的更改吗：
					</p>
					<ul class="file-list">
						{#each changes as change, i}
							<FileListItem
								filePath={change.path}
								fileStatus={computeChangeStatus(change)}
								clickable={false}
								listMode="list"
								hideBorder={i === changes.length - 1}
							/>
						{/each}
					</ul>
				{:else}
					<p>
						确定要丢弃
						<span class="text-bold">{changes.length} 个文件</span>？
					</p>
				{/if}
			{/if}
		{:else}
			<p class="text-13">哎呀！数据异常 :(</p>
		{/if}
	{/snippet}
	{#snippet controls(close, item)}
		<Button kind="outline" onclick={close}>取消</Button>
		<AsyncButton style="danger" type="submit" action={async () => await confirmDiscard(item)}>
			确认
		</AsyncButton>
	{/snippet}
</Modal>

<Modal
	width={434}
	type="info"
	title="将更改暂存到新分支"
	bind:this={stashConfirmationModal}
	onSubmit={(_, item) => isChangedFilesItem(item) && confirmStashIntoBranch(item, slugifiedRefName)}
>
	{#snippet children(item)}
		<div class="content-wrap">
			<BranchNameTextbox
				bind:this={stashBranchNameInput}
				id="stashBranchName"
				placeholder="请输入分支名..."
				bind:value={stashBranchName}
				autofocus
				onslugifiedvalue={(value) => (slugifiedRefName = value)}
			/>
			<div class="explanation">
				<p class="primary-text">
					{#if isChangedFolderItem(item)}
						此文件夹中的所有更改
					{:else}
						你选择的更改
					{/if}
					将被移动到新分支，并从当前工作区移除。若要找回这些更改，请切换到新分支并撤销提交该暂存。
				</p>
			</div>

			<div class="technical-note">
				<p class="text-12 text-body clr-text-2">
					💡 这会创建一个新分支、提交更改，然后取消应用该分支。后续版本会提供更简洁的暂存管理。
				</p>
			</div>
		</div>
	{/snippet}
	{#snippet controls(close, item)}
		<Button kind="outline" type="reset" onclick={close}>取消</Button>
		<AsyncButton
			style="pop"
			disabled={!slugifiedRefName}
			type="submit"
			action={async () => await confirmStashIntoBranch(item, slugifiedRefName)}
		>
			暂存到分支
		</AsyncButton>
	{/snippet}
</Modal>

<style lang="postcss">
	.discard-caption {
		color: var(--clr-text-2);
	}
	.file-list {
		display: flex;
		flex-direction: column;
		margin-top: 12px;
		overflow: hidden;
		border: 1px solid var(--clr-border-2);
		border-radius: var(--radius-m);
		background-color: var(--clr-bg-1);
	}
	/* 模态窗口 */
	.content-wrap {
		display: flex;
		flex-direction: column;
		gap: 16px;
	}
</style>
