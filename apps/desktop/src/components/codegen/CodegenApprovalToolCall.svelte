<script lang="ts">
	import { type ToolCall } from '$lib/codegen/messages';
	import { formatToolCall, getToolIcon } from '$lib/utils/codegenTools';
	import { Codeblock } from '@gitbutler/ui';
	import {
		DropdownButton,
		ContextMenuItem,
		ContextMenuSection,
		ContextMenu,
		Button,
		Icon
	} from '@gitbutler/ui';
	import type { PermissionDecision } from '$lib/codegen/types';

	type Props = {
		projectId: string;

		toolCall: ToolCall;
		onPermissionDecision: (
			id: string,
			decision: PermissionDecision,
			useWildcard: boolean
		) => Promise<void>;
	};
	const { toolCall, onPermissionDecision }: Props = $props();

	let allowDropdownButton = $state<ReturnType<typeof DropdownButton>>();
	let denyDropdownButton = $state<ReturnType<typeof DropdownButton>>();
	let wildcardButton = $state<HTMLElement>();
	let wildcardContextMenu = $state<ReturnType<typeof ContextMenu>>();

	type AllowDecision = 'allowOnce' | 'allowSession' | 'allowProject' | 'allowAlways';
	type DenyDecision = 'denyOnce' | 'denySession' | 'denyProject' | 'denyAlways';
	type WildcardDecision = 'precise' | 'wild';

	let selectedAllowDecision = $state<AllowDecision>('allowOnce');
	let selectedDenyDecision = $state<DenyDecision>('denyOnce');
	let selectedWildcardDecision = $state<WildcardDecision>('precise');

	const helperText = $derived.by(() => {
		const isProject =
			selectedAllowDecision === 'allowProject' || selectedDenyDecision === 'denyProject';
		const isAlways =
			selectedAllowDecision === 'allowAlways' || selectedDenyDecision === 'denyAlways';

		if (isProject) {
			return '权限将保存到 .claude/settings.local.json';
		} else if (isAlways) {
			return '权限将保存到 ~/.claude/settings.json';
		}
		return null;
	});

	const allowLabels: Record<AllowDecision, string> = {
		allowOnce: '仅允许一次',
		allowProject: '允许此项目',
		allowSession: '允许在此会话',
		allowAlways: '始终允许'
	};

	const denyLabels: Record<DenyDecision, string> = {
		denyOnce: '仅拒绝一次',
		denySession: '拒绝此会话',
		denyProject: '拒绝此项目',
		denyAlways: '始终拒绝'
	};

	// The wildcard selector only shows up for certain tool calls
	const wildcardSelector = $derived.by<
		{ show: false } | { show: true; options: { label: string; value: WildcardDecision }[] }
	>(() => {
		if (toolCall.name === 'Edit' || toolCall.name === 'Write') {
			return {
				show: true,
				options: [
					{ value: 'precise', label: '此文件' },
					{ value: 'wild', label: '同一文件夹中的任意文件' }
				]
			};
		} else if (toolCall.name === 'Bash') {
			return {
				show: true,
				options: [
					{ value: 'precise', label: '此命令' },
					{ value: 'wild', label: '任意子命令或参数' }
				]
			};
		} else {
			return { show: false };
		}
	});
</script>

<div class="tool-call">
	<div class="tool-call__details">
		<div class="tool-call__header">
			<Icon name={getToolIcon(toolCall.name)} color="var(--clr-text-3)" />
			<span class="text-13 tool-name">{toolCall.name}</span>
		</div>

		<Codeblock content={formatToolCall(toolCall)} />
	</div>

	{#if helperText}
		<div class="tool-call__helper-text">
			<Icon name="info-small-outline" />
			<p class="text-12">
				{helperText}
			</p>
		</div>
	{/if}

	<div class="tool-call__actions">
		{#if wildcardSelector.show}
			<Button
				bind:el={wildcardButton}
				kind="outline"
				icon="select-chevron"
				shrinkable
				onclick={() => {
					wildcardContextMenu?.toggle();
				}}
			>
				{wildcardSelector.options.find((opt) => opt.value === selectedWildcardDecision)?.label ||
					'选择范围'}
			</Button>

			<ContextMenu bind:this={wildcardContextMenu} leftClickTrigger={wildcardButton}>
				<ContextMenuSection>
					{#each wildcardSelector.options as option}
						<ContextMenuItem
							label={option.label}
							selected={option.value === selectedWildcardDecision}
							onclick={() => {
								selectedWildcardDecision = option.value;
								wildcardContextMenu?.close();
							}}
						/>
					{/each}
				</ContextMenuSection>
			</ContextMenu>
		{/if}

		<DropdownButton
			bind:this={denyDropdownButton}
			style="danger"
			kind="outline"
			onclick={async () => {
				await onPermissionDecision(
					toolCall.id,
					selectedDenyDecision,
					selectedWildcardDecision === 'wild'
				);
				denyDropdownButton?.close();
			}}
		>
			{denyLabels[selectedDenyDecision]}
			{#snippet contextMenuSlot()}
				<ContextMenuSection>
					<ContextMenuItem
						label="仅拒绝一次"
						onclick={() => {
							selectedDenyDecision = 'denyOnce';
							denyDropdownButton?.close();
						}}
					/>
					<ContextMenuItem
						label="拒绝此会话"
						onclick={() => {
							selectedDenyDecision = 'denySession';
							denyDropdownButton?.close();
						}}
					/>
					<ContextMenuItem
						label="拒绝此项目"
						onclick={() => {
							selectedDenyDecision = 'denyProject';
							denyDropdownButton?.close();
						}}
					/>
					<ContextMenuItem
						label="始终拒绝"
						onclick={() => {
							selectedDenyDecision = 'denyAlways';
							denyDropdownButton?.close();
						}}
					/>
				</ContextMenuSection>
			{/snippet}
		</DropdownButton>

		<DropdownButton
			bind:this={allowDropdownButton}
			style="pop"
			onclick={async () => {
				await onPermissionDecision(
					toolCall.id,
					selectedAllowDecision,
					selectedWildcardDecision === 'wild'
				);
				allowDropdownButton?.close();
			}}
		>
			{allowLabels[selectedAllowDecision]}
			{#snippet contextMenuSlot()}
				<ContextMenuSection>
					<ContextMenuItem
						label="仅允许一次"
						onclick={() => {
							selectedAllowDecision = 'allowOnce';
							allowDropdownButton?.close();
						}}
					/>
					<ContextMenuItem
						label="允许在此会话"
						onclick={() => {
							selectedAllowDecision = 'allowSession';
							allowDropdownButton?.close();
						}}
					/>
					<ContextMenuItem
						label="允许此项目"
						onclick={() => {
							selectedAllowDecision = 'allowProject';
							allowDropdownButton?.close();
						}}
					/>
					<ContextMenuItem
						label="始终允许"
						onclick={() => {
							selectedAllowDecision = 'allowAlways';
							allowDropdownButton?.close();
						}}
					/>
				</ContextMenuSection>
			{/snippet}
		</DropdownButton>
	</div>
</div>

<style lang="postcss">
	.tool-call {
		display: flex;
		flex-direction: column;
		max-width: 100%;
		max-width: var(--message-max-width);
		margin-bottom: 10px;
		overflow: hidden;
		border: 1px solid var(--clr-border-2);
		border-radius: var(--radius-m);
		user-select: text;
	}

	.tool-call__details {
		display: flex;
		flex-direction: column;
		padding: 12px;
		gap: 10px;
	}

	.tool-call__header {
		display: flex;
		align-items: center;
		gap: 8px;
	}

	.tool-call__helper-text {
		display: flex;
		align-items: center;
		padding: 10px 12px;
		gap: 8px;
		border-top: 1px solid var(--clr-border-2);
		background-color: var(--clr-bg-1);
		color: var(--clr-text-2);
	}

	.tool-call__actions {
		display: flex;
		align-items: center;
		justify-content: flex-end;
		padding: 12px;
		gap: 6px;
		border-top: 1px solid var(--clr-border-2);
	}
</style>
