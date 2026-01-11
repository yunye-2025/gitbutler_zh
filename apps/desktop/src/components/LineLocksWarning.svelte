<script lang="ts">
	import ReduxResult from '$components/ReduxResult.svelte';
	import { getStackName } from '$lib/stacks/stack';
	import { STACK_SERVICE } from '$lib/stacks/stackService.svelte';
	import { inject } from '@gitbutler/core/context';
	import { TestId } from '@gitbutler/ui';
	import type { DependencyLock } from '@gitbutler/ui/utils/diffParsing';

	type Props = {
		projectId: string;
		locks: DependencyLock[];
	};

	const { projectId, locks }: Props = $props();

	const stackService = inject(STACK_SERVICE);

	const lockedToStackIds = $derived(locks.map((lock) => lock.stackId));
	const stacksQuery = $derived(stackService.stacks(projectId));
</script>

<ReduxResult result={stacksQuery.result} {projectId}>
	{#snippet children(stacks)}
		{@const lockedToStacks = stacks.filter(
			(stack) => stack.id && lockedToStackIds.includes(stack.id)
		)}
		{@const stackNames = lockedToStacks.map(getStackName)}
		<div data-testid={TestId.UnifiedDiffViewLockWarning}>
			{#if stackNames.length > 1}
				<p>此行依赖以下堆叠中的更改</p>
				<br />
				<p>{stackNames.join(', ')}</p>
			{:else if stackNames.length === 1}
				<p>此行依赖 <b>'{stackNames[0]}'</b> 中的更改</p>
			{:else}
				<p>这很奇怪，不应发生</p>
			{/if}
		</div>
	{/snippet}
</ReduxResult>
