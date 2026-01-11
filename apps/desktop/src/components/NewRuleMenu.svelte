<script lang="ts">
	import { ContextMenu, ContextMenuItem, ContextMenuSection } from '@gitbutler/ui';
	import type { RuleFilterType } from '$lib/rules/rule';

	type Props = {
		addedFilterTypes: RuleFilterType[];
		addFromFilter: (type: RuleFilterType) => void;
		addEmpty?: () => void;
		trigger?: HTMLElement;
	};

	const { addFromFilter, trigger, addEmpty, addedFilterTypes }: Props = $props();

	let contextMenu = $state<ContextMenu>();

	function filterHasBeenAdded(type: RuleFilterType): boolean {
		return addedFilterTypes.includes(type);
	}

	function handleAddFilter(type: RuleFilterType) {
		addFromFilter(type);
		contextMenu?.close();
	}

	function handleAddEmpty() {
		addEmpty?.();
		contextMenu?.close();
	}

	export function toggle(e: MouseEvent) {
		contextMenu?.toggle(e);
	}
</script>

<ContextMenu bind:this={contextMenu} leftClickTrigger={trigger}>
	<ContextMenuSection>
		<ContextMenuItem
			icon="folder"
			label="文件或文件夹路径"
			disabled={filterHasBeenAdded('pathMatchesRegex')}
			onclick={() => {
				handleAddFilter('pathMatchesRegex');
			}}
		/>
		<ContextMenuItem
			icon="text-width"
			label="包含文本"
			disabled={filterHasBeenAdded('contentMatchesRegex')}
			onclick={() => {
				handleAddFilter('contentMatchesRegex');
			}}
		/>
		<ContextMenuItem
			icon="file-changes"
			label="更改类型（即将上线）"
			disabled={filterHasBeenAdded('fileChangeType') || true}
			onclick={() => {
				handleAddFilter('fileChangeType');
			}}
		/>
		<ContextMenuItem
			icon="tag"
			label="工作类别（即将上线）"
			disabled={filterHasBeenAdded('semanticType') || true}
			onclick={() => {
				handleAddFilter('semanticType');
			}}
		/>
	</ContextMenuSection>
	{#if addEmpty}
		<ContextMenuSection>
			<ContextMenuItem icon="arrow-right" label="全部分配到分支" onclick={handleAddEmpty} />
		</ContextMenuSection>
	{/if}
</ContextMenu>
