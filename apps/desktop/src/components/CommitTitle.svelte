<script lang="ts">
	import { splitMessage } from '$lib/utils/commitMessage';
	import { TestId, Tooltip } from '@gitbutler/ui';

	type Props = {
		truncate?: boolean;
		commitMessage: string;
		className?: string;
		editable?: boolean;
	};

	const { commitMessage, truncate, className, editable }: Props = $props();

	const title = $derived(splitMessage(commitMessage).title);

	function getTitle() {
		if (title) {
			return title;
		}
		return editable ? '空提交。将更改拖到这里' : '空提交';
	}
</script>

<Tooltip text={getTitle()} delay={1200}>
	<h3
		data-testid={TestId.CommitDrawerTitle}
		class="{className} commit-title"
		class:truncate
		class:clr-text-3={!title}
	>
		{getTitle()}
	</h3>
</Tooltip>
