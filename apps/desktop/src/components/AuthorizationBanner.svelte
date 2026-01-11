<script lang="ts">
	import LoginButtons from '$components/LoginButtons.svelte';
	import { CardGroup, Icon } from '@gitbutler/ui';

	interface Props {
		title?: string;
		message?: string;
	}

	const {
		title: titleLabel = '需要授权',
		message = '请授权 GitButler 使用此服务'
	}: Props = $props();
</script>

<CardGroup>
	<CardGroup.Item>
		{#snippet iconSide()}
			<Icon name="warning" color="warning" />
		{/snippet}
		{#snippet title()}
			{titleLabel}
		{/snippet}
		{#snippet caption()}
			{message}
		{/snippet}
		{#snippet actions()}
			<LoginButtons />
		{/snippet}
	</CardGroup.Item>
</CardGroup>
