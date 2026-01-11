<script lang="ts">
	import BranchCommitsRow from '$lib/components/changes/BranchCommitsRow.svelte';
	import Table from '$lib/components/table/Table.svelte';
	import { type Branch } from '@gitbutler/shared/branches/types';
	import { type ProjectReviewParameters } from '@gitbutler/shared/routing/webRoutes.svelte';

	type Props = {
		data: ProjectReviewParameters;
		branch: Branch;
	};

	const { data, branch }: Props = $props();
</script>

<table class="commits-table">
	<Table
		headColumns={[
			{ key: 'position', value: '' },
			{ key: 'status', value: '状态' },
			{ key: 'version', value: '版本' },
			{ key: 'string', value: '名称' },
			{ key: 'changes', value: '变更' },
			{ key: 'date', value: '更新' },
			{ key: 'avatars', value: '作者' },
			{ key: 'reviewers', value: '评审人' },
			{ key: 'comments', value: '' }
		]}
	>
		{#snippet body()}
			{#each branch.patchCommitIds || [] as changeId, index}
				<BranchCommitsRow
					{changeId}
					params={data}
					branchUuid={branch.uuid}
					last={index === branch.patchCommitIds.length - 1}
				/>
			{/each}
		{/snippet}
	</Table>
</table>

<style lang="postcss">
</style>
