<script lang="ts">
	import { featureShowOrganizations, featureShowProjectPage } from '$lib/featureFlags';
	import { CardGroup, Spacer, Toggle } from '@gitbutler/ui';
</script>

<Spacer />

<div class="stack-v gap-8">
	<h2 class="text-15 text-bold">实验功能</h2>
	<p class="text-12 text-body clr-text-2">
		这些设置用于启用仍在开发中的实验功能。
		<br />
		它们可能不稳定或不完整，请自行承担风险。
	</p>
</div>

<CardGroup>
	<CardGroup.Item labelFor="showOrganizations">
		{#snippet title()}组织{/snippet}
		{#snippet caption()}
			组织用于将项目关联在一起。
		{/snippet}
		{#snippet actions()}
			<Toggle
				id="showOrganizations"
				checked={$featureShowOrganizations}
				onclick={() => ($featureShowOrganizations = !$featureShowOrganizations)}
			/>
		{/snippet}
	</CardGroup.Item>

	<CardGroup.Item labelFor="showProjectPage">
		{#snippet title()}用户 / 组织 / 项目页面{/snippet}
		{#snippet caption()}
			这将显示组织、用户和项目的占位落地页。
		{/snippet}
		{#snippet actions()}
			<Toggle
				id="showProjectPage"
				checked={$featureShowProjectPage}
				onclick={() => ($featureShowProjectPage = !$featureShowProjectPage)}
			/>
		{/snippet}
	</CardGroup.Item>
</CardGroup>
