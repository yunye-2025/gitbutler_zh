<script lang="ts">
	import { SETTINGS_SERVICE } from '$lib/config/appSettingsV2';
	import { GIT_CONFIG_SERVICE } from '$lib/config/gitConfigService';
	import { inject } from '@gitbutler/core/context';
	import { CardGroup, Link, Select, SelectItem, Toggle } from '@gitbutler/ui';
	import { onMount } from 'svelte';

	const gitConfig = inject(GIT_CONFIG_SERVICE);
	const settingsService = inject(SETTINGS_SERVICE);
	const settings = settingsService.appSettings;

	let annotateCommits = $state(true);
	let fetchFrequency = $state<number>(-1);

	const fetchFrequencyOptions = [
		{ label: '1 分钟', value: '1', minutes: 1 },
		{ label: '5 分钟', value: '5', minutes: 5 },
		{ label: '10 分钟', value: '10', minutes: 10 },
		{ label: '15 分钟', value: '15', minutes: 15 },
		{ label: '不自动', value: 'none', minutes: -1 }
	] as const;

	function toggleCommitterSigning() {
		annotateCommits = !annotateCommits;
		gitConfig.set('gitbutler.gitbutlerCommitter', annotateCommits ? '1' : '0');
	}

	async function updateFetchFrequency(value: string) {
		const option = fetchFrequencyOptions.find((opt) => opt.value === value);
		if (option) {
			fetchFrequency = option.minutes;
			await settingsService.updateFetch({ autoFetchIntervalMinutes: option.minutes });
		}
	}

	const selectedValue = $derived(
		fetchFrequencyOptions.find((opt) => opt.minutes === fetchFrequency)?.value ?? 'none'
	);

	onMount(async () => {
		annotateCommits = (await gitConfig.get('gitbutler.gitbutlerCommitter')) === '1';
	});

	$effect(() => {
		if ($settings?.fetch) {
			fetchFrequency = $settings.fetch.autoFetchIntervalMinutes;
		}
	});
</script>

<CardGroup.Item standalone labelFor="committerSigning">
	{#snippet title()}
		将 GitButler 记为提交者
	{/snippet}
	{#snippet caption()}
		默认情况下，GitButler 客户端功能免费使用。你可以选择在虚拟分支提交中将提交者标记为
		GitButler，以帮助我们传播。
		<Link
			href="https://github.com/gitbutlerapp/gitbutler-docs/blob/d81a23779302c55f8b20c75bf7842082815b4702/content/docs/features/virtual-branches/committer-mark.mdx"
		>
			了解更多
		</Link>
	{/snippet}
	{#snippet actions()}
		<Toggle id="committerSigning" checked={annotateCommits} onclick={toggleCommitterSigning} />
	{/snippet}
</CardGroup.Item>

<CardGroup.Item standalone labelFor="fetchFrequency" alignment="center">
	{#snippet title()}
		自动拉取频率
	{/snippet}
	{#snippet actions()}
		<Select
			id="fetchFrequency"
			options={fetchFrequencyOptions}
			value={selectedValue}
			onselect={updateFetchFrequency}
		>
			{#snippet itemSnippet({ item, highlighted })}
				<SelectItem selected={item.value === selectedValue} {highlighted}>
					{item.label}
				</SelectItem>
			{/snippet}
		</Select>
	{/snippet}
</CardGroup.Item>
