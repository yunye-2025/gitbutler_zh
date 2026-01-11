<script lang="ts">
	import PrStatusBadge from '$components/PrStatusBadge.svelte';
	import PullRequestPolling from '$components/PullRequestPolling.svelte';
	import ReduxResult from '$components/ReduxResult.svelte';
	import { CLIPBOARD_SERVICE } from '$lib/backend/clipboard';
	import { DEFAULT_FORGE_FACTORY } from '$lib/forge/forgeFactory.svelte';
	import { URL_SERVICE } from '$lib/utils/url';
	import { inject } from '@gitbutler/core/context';
	import {
		Button,
		ContextMenu,
		ContextMenuItem,
		ContextMenuSection,
		Icon,
		AvatarGroup,
		TestId
	} from '@gitbutler/ui';
	import { getForgeLogo } from '@gitbutler/ui/utils/getForgeLogo';
	import type { DetailedPullRequest } from '$lib/forge/interface/types';
	import type { Snippet } from 'svelte';

	type ButtonStatus = {
		disabled: boolean;
		tooltip?: string;
	};

	interface Props {
		testId?: string;
		branchName: string;
		poll?: boolean;
		prNumber: number;
		isPushed?: boolean;
		hasParent?: boolean;
		baseIsTargetBranch?: boolean;
		parentIsPushed?: boolean;
		button?: Snippet<
			[{ pr: DetailedPullRequest; mergeStatus: ButtonStatus; reopenStatus: ButtonStatus }]
		>;
	}

	const {
		testId,
		poll,
		prNumber,
		isPushed,
		hasParent,
		baseIsTargetBranch,
		parentIsPushed,
		button
	}: Props = $props();

	let contextMenuEl = $state<ReturnType<typeof ContextMenu>>();
	let container = $state<HTMLElement>();
	let hasChecks = $state(false);

	const forge = inject(DEFAULT_FORGE_FACTORY);
	const forgeName = $derived(forge.current.name);
	const prService = $derived(forge.current.prService);
	const checksService = $derived(forge.current.checks);
	const urlService = inject(URL_SERVICE);
	const clipboardService = inject(CLIPBOARD_SERVICE);

	const prQuery = $derived(prService?.get(prNumber, { forceRefetch: true }));
	const pr = $derived(prQuery?.response);

	const { name, abbr, symbol } = $derived(prService!.unit);

	const prLoading = $state(false);

	const mergeStatus = $derived.by(() => {
		let disabled = true;
		let tooltip = undefined;
		if (isPushed && hasParent && !parentIsPushed) {
			tooltip = '远程父分支似乎已被删除';
		} else if (!baseIsTargetBranch) {
			tooltip = name + ' 不是堆叠中的下一个';
		} else if (prLoading) {
			tooltip = '正在重新加载拉取请求数据';
		} else if (!pr?.permissions?.canMerge) {
			tooltip = name + ' 需要推送权限';
		} else if (pr?.draft) {
			tooltip = name + ' 是草稿';
		} else if (pr?.mergeableState === 'blocked') {
			tooltip = name + ' 需要批准';
		} else if (pr?.mergeableState === 'unknown') {
			tooltip = name + ' 的可合并性未知';
		} else if (pr?.mergeableState === 'behind') {
			tooltip = name + ' 的基分支落后过多';
		} else if (pr?.mergeableState === 'dirty') {
			tooltip = name + ' 存在冲突';
		} else if (!pr?.mergeable) {
			tooltip = name + ' 无法合并';
		} else {
			disabled = false;
		}
		return { disabled, tooltip };
	});

	const reopenStatus = $derived.by(() => {
		let disabled = true;
		let tooltip = undefined;
		if (isPushed && hasParent && !parentIsPushed) {
			tooltip = '远程父分支似乎已被删除';
		} else {
			disabled = false;
		}
		return { disabled, tooltip };
	});
</script>

<ReduxResult result={prQuery?.result} projectId="dummy">
	{#snippet children(pr)}
		{#if poll}
			<PullRequestPolling number={pr.number} />
		{/if}

		<ContextMenu bind:this={contextMenuEl} rightClickTrigger={container}>
			<ContextMenuSection>
				<ContextMenuItem
					label="在浏览器中打开"
					onclick={() => {
						urlService.openExternalUrl(pr.htmlUrl);
						contextMenuEl?.close();
					}}
				/>
				<ContextMenuItem
					label="复制链接"
					onclick={() => {
						clipboardService.write(pr.htmlUrl, { message: `已复制 ${abbr} 链接` });
						contextMenuEl?.close();
					}}
				/>
				<ContextMenuItem
					label="重新获取状态"
					onclick={() => {
						prService?.fetch(pr.number, { forceRefetch: true });
						contextMenuEl?.close();
						if (hasChecks) {
							checksService?.fetch(pr.sourceBranch, { forceRefetch: true });
						}
					}}
				/>
			</ContextMenuSection>
			{#if hasChecks}
				<ContextMenuSection>
					<ContextMenuItem
						label="打开检查"
						onclick={() => {
							urlService.openExternalUrl(`${pr.htmlUrl}/checks`);
							contextMenuEl?.close();
						}}
					/>
					<ContextMenuItem
						label="复制检查链接"
						onclick={() => {
							clipboardService.write(`${pr.htmlUrl}/checks`, { message: '已复制检查链接' });
							contextMenuEl?.close();
						}}
					/>
				</ContextMenuSection>
			{/if}
		</ContextMenu>

		<div
			data-testid={testId}
			bind:this={container}
			role="article"
			class="review-card pr-card"
			oncontextmenu={(e: MouseEvent) => {
				e.preventDefault();
				e.stopPropagation();
				contextMenuEl?.open(e);
			}}
		>
			<div class="pr-actions">
				<Button
					kind="outline"
					size="tag"
					icon="copy-small"
					tooltip="复制 {abbr} 链接"
					onclick={() => {
						clipboardService.write(pr.htmlUrl, { message: `已复制 ${abbr} 链接` });
					}}
				/>
				<Button
					kind="outline"
					size="tag"
					icon="open-link"
					tooltip="在浏览器中打开 {abbr}"
					onclick={() => {
						urlService.openExternalUrl(pr.htmlUrl);
					}}
				/>
			</div>

			<div class="text-13 text-semibold pr-row">
				<Icon name={getForgeLogo(forgeName)} />
				<h4 class="text-14 text-semibold">
					{`${abbr} ${symbol}${pr.number}`}
				</h4>

				<PrStatusBadge testId={TestId.PRStatusBadge} {pr} />
			</div>
			<div class="text-12 pr-row">
				<div class="factoid">
					{#if pr.reviewers.length > 0}
						<span class="label">审查者：</span>
						<div class="avatar-group-container">
							<AvatarGroup avatars={pr.reviewers} />
						</div>
					{:else}
						<span class="label italic">无审查者</span>
					{/if}
				</div>
				<span class="separator">•</span>
				<div class="factoid">
					<span class="label">
						<Icon name="chat-small" />
					</span>
					<span>{pr.commentsCount}</span>
				</div>
			</div>

			{#if button}
				<div class="pr-row">
					{@render button({ pr, mergeStatus, reopenStatus })}
				</div>
			{/if}
		</div>
	{/snippet}
</ReduxResult>

<style lang="postcss">
	.pr-row {
		display: flex;
		flex-wrap: wrap;
		align-items: center;
		gap: 6px;

		&:empty {
			display: none;
		}
	}

	.factoid {
		display: flex;
		align-items: center;
		gap: 4px;

		> .label {
			display: flex;
			color: var(--clr-text-2);

			&.italic {
				font-style: italic;
			}
		}
	}

	.separator {
		transform: translateY(-1.5px);
		color: var(--clr-text-3);
	}

	.pr-actions {
		display: flex;
		position: absolute;
		top: 8px;
		right: 8px;
		gap: 4px;
	}
</style>
