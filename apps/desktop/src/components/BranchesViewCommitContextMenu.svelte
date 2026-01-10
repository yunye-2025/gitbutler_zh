<script lang="ts">
	import { ContextMenuItem, ContextMenuSection, KebabButton, TestId } from '@gitbutler/ui';

	type Props = {
		onCherryPick: () => void;
		rightClickTrigger: HTMLElement;
	};

	// 提交右键菜单回调
	const { onCherryPick: onclick, rightClickTrigger }: Props = $props();
</script>

<KebabButton showOnHover contextElement={rightClickTrigger} testId={TestId.KebabMenuButton}>
	{#snippet contextMenu({ close })}
		<ContextMenuSection>
			<ContextMenuItem
				label="挑拣提交"
				icon="cherry-pick"
				onclick={() => {
					close();
					onclick();
				}}
			/>
		</ContextMenuSection>
	{/snippet}
</KebabButton>
