<script lang="ts">
	import { APP_SETTINGS } from '$lib/config/appSettings';
	import { inject } from '@gitbutler/core/context';
	import { CardGroup, Link, TestId, Toggle } from '@gitbutler/ui';

	const appSettings = inject(APP_SETTINGS);
	const errorReportingEnabled = appSettings.appErrorReportingEnabled;
	const metricsEnabled = appSettings.appMetricsEnabled;
	const nonAnonMetricsEnabled = appSettings.appNonAnonMetricsEnabled;
</script>

<div class="analytics-settings__content">
	<p class="text-13 text-body analytics-settings__text">
		GitButler 的遥测数据仅用于帮助我们改进客户端。除非你在下方明确允许，否则我们不会收集任何个人
		信息。<Link href="https://gitbutler.com/privacy">
			隐私政策
		</Link>
	</p>
	<p class="text-13 text-body analytics-settings__text">
		我们建议你考虑保持这些设置开启，这有助于我们更快发现问题。如果你选择关闭，也欢迎在我们的 <Link
			href="https://discord.gg/MmFkmaJ42D"
		>
			Discord
		</Link>.
	</p>
</div>

<CardGroup testId={TestId.OnboardingPageAnalyticsSettings}>
	<CardGroup.Item labelFor="errorReportingToggle">
		{#snippet title()}
			错误报告
		{/snippet}
		{#snippet caption()}
			开关应用崩溃与错误的上报。
		{/snippet}
		{#snippet actions()}
			<Toggle
				id="errorReportingToggle"
				testId={TestId.OnboardingPageAnalyticsSettingsErrorReportingToggle}
				checked={$errorReportingEnabled}
				onclick={() => ($errorReportingEnabled = !$errorReportingEnabled)}
			/>
		{/snippet}
	</CardGroup.Item>

	<CardGroup.Item labelFor="metricsEnabledToggle">
		{#snippet title()}
			使用情况指标
		{/snippet}
		{#snippet caption()}
			开关共享使用统计。
		{/snippet}
		{#snippet actions()}
			<Toggle
				id="metricsEnabledToggle"
				testId={TestId.OnboardingPageAnalyticsSettingsTelemetryToggle}
				checked={$metricsEnabled}
				onclick={() => ($metricsEnabled = !$metricsEnabled)}
			/>
		{/snippet}
	</CardGroup.Item>

	<CardGroup.Item labelFor="nonAnonMetricsEnabledToggle">
		{#snippet title()}
			非匿名使用指标
		{/snippet}
		{#snippet caption()}
			开关共享可识别的使用统计。
		{/snippet}
		{#snippet actions()}
			<Toggle
				id="nonAnonMetricsEnabledToggle"
				testId={TestId.OnboardingPageAnalyticsSettingsNonAnonymousToggle}
				checked={$nonAnonMetricsEnabled}
				onclick={() => ($nonAnonMetricsEnabled = !$nonAnonMetricsEnabled)}
			/>
		{/snippet}
	</CardGroup.Item>
</CardGroup>

<style lang="postcss">
	.analytics-settings__content {
		display: flex;
		flex-direction: column;
		gap: 16px;
	}

	.analytics-settings__text {
		margin-bottom: 10px;
		color: var(--clr-text-2);
	}
</style>
