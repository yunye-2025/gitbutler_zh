<script lang="ts">
	import DecorativeSplitView from '$components/DecorativeSplitView.svelte';
	import ProjectSwitcher from '$components/ProjectSwitcher.svelte';
	import loadErrorSvg from '$lib/assets/illustrations/load-error.svg?raw';
	import { InfoMessage } from '@gitbutler/ui';

	interface Props {
		projectId?: string;
		error?: any;
	}

	const { projectId, error = undefined }: Props = $props();
</script>

<DecorativeSplitView img={loadErrorSvg}>
	<div class="problem__container">
		<h2 class="problem__title text-18 text-body text-bold">加载应用时发生问题</h2>

		<InfoMessage filled outlined={false} style="danger" icon="info">
			{#snippet content()}
				{error ? error : '发生了未知错误'}
			{/snippet}
		</InfoMessage>

		<ProjectSwitcher {projectId} />
	</div>
</DecorativeSplitView>

<style lang="postcss">
	.problem__container {
		display: flex;
		flex-direction: column;
		gap: 20px;
	}

	.problem__title {
		color: var(--clr-text-1);
	}
</style>
