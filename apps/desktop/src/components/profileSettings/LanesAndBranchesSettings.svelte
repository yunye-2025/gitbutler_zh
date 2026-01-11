<script lang="ts">
	import {
		autoSelectBranchNameFeature,
		autoSelectBranchCreationFeature
	} from '$lib/config/uiFeatureFlags';
	import { persisted } from '@gitbutler/shared/persisted';
	import { CardGroup, Toggle } from '@gitbutler/ui';

	const addToLeftmost = persisted<boolean>(false, 'branch-placement-leftmost');
</script>

<CardGroup.Item standalone labelFor="add-leftmost">
	{#snippet title()}
		新泳道放在左侧
	{/snippet}
	{#snippet caption()}
		默认将新泳道添加到最右侧。启用后改为添加到最左侧。
	{/snippet}
	{#snippet actions()}
		<Toggle
			id="add-leftmost"
			checked={$addToLeftmost}
			onclick={() => ($addToLeftmost = !$addToLeftmost)}
		/>
	{/snippet}
</CardGroup.Item>

<CardGroup>
	<CardGroup.Item labelFor="auto-select-creation">
		{#snippet title()}
			创建分支时自动选中文本
		{/snippet}
		{#snippet caption()}
			创建分支时自动选中分支名输入框中的预填文本，方便直接输入新名称。
		{/snippet}
		{#snippet actions()}
			<Toggle
				id="auto-select-creation"
				checked={$autoSelectBranchCreationFeature}
				onclick={() => ($autoSelectBranchCreationFeature = !$autoSelectBranchCreationFeature)}
			/>
		{/snippet}
	</CardGroup.Item>
	<CardGroup.Item labelFor="auto-select-rename">
		{#snippet title()}
			重命名分支时自动选中文本
		{/snippet}
		{#snippet caption()}
			重命名分支或泳道时自动选中文本，便于整体替换名称。
		{/snippet}
		{#snippet actions()}
			<Toggle
				id="auto-select-rename"
				checked={$autoSelectBranchNameFeature}
				onclick={() => ($autoSelectBranchNameFeature = !$autoSelectBranchNameFeature)}
			/>
		{/snippet}
	</CardGroup.Item>
</CardGroup>
