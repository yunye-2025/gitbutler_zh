<script lang="ts">
	import { SETTINGS_SERVICE } from '$lib/config/appSettingsV2';
	import {
		ircEnabled,
		ircServer,
		fModeEnabled,
		useNewRebaseEngine
	} from '$lib/config/uiFeatureFlags';
	import { USER } from '$lib/user/user';
	import { inject } from '@gitbutler/core/context';
	import { CardGroup, Textbox, Toggle } from '@gitbutler/ui';

	const settingsService = inject(SETTINGS_SERVICE);
	const settingsStore = settingsService.appSettings;

	const user = inject(USER);
</script>

<p class="text-12 text-body experimental-settings__text">
	开发中或测试中的功能开关，可能无法完全工作。
	<br />
	使用风险自担。
</p>

<CardGroup>
	<CardGroup.Item labelFor="apply3">
		{#snippet title()}
			新工作区应用方式
		{/snippet}
		{#snippet caption()}
			对工作区更改使用 V3 版的应用/取消应用操作。
		{/snippet}
		{#snippet actions()}
			<Toggle
				id="apply3"
				checked={$settingsStore?.featureFlags.apply3}
				onclick={() =>
					settingsService.updateFeatureFlags({ apply3: !$settingsStore?.featureFlags.apply3 })}
			/>
		{/snippet}
	</CardGroup.Item>
	<CardGroup.Item labelFor="f-mode">
		{#snippet title()}
			F 模式导航
		{/snippet}
		{#snippet caption()}
			启用 F 模式，通过双字符快捷键快速定位按钮。
		{/snippet}
		{#snippet actions()}
			<Toggle
				id="f-mode"
				checked={$fModeEnabled}
				onclick={() => fModeEnabled.set(!$fModeEnabled)}
			/>
		{/snippet}
	</CardGroup.Item>
	<CardGroup.Item labelFor="new-rebase-engine">
		{#snippet title()}
			新版变基引擎
		{/snippet}
		{#snippet caption()}
			在堆叠操作中使用基于图的新变基引擎。
		{/snippet}
		{#snippet actions()}
			<Toggle
				id="new-rebase-engine"
				checked={$useNewRebaseEngine}
				onclick={() => useNewRebaseEngine.set(!$useNewRebaseEngine)}
			/>
		{/snippet}
	</CardGroup.Item>

	{#if $user?.role === 'admin'}
		<CardGroup.Item labelFor="single-branch">
			{#snippet title()}
				单分支模式
			{/snippet}
			{#snippet caption()}
				离开 gitbutler/workspace 分支时仍停留在工作区视图。
			{/snippet}
			{#snippet actions()}
				<Toggle
					id="single-branch"
					checked={$settingsStore?.featureFlags.singleBranch}
					onclick={() =>
						settingsService.updateFeatureFlags({
							singleBranch: !$settingsStore?.featureFlags.singleBranch
						})}
				/>
			{/snippet}
		</CardGroup.Item>

		<CardGroup.Item labelFor="irc">
			{#snippet title()}
				IRC
			{/snippet}
			{#snippet caption()}
				启用实验性的应用内聊天。
			{/snippet}
			{#snippet actions()}
				<Toggle id="irc" checked={$ircEnabled} onclick={() => ($ircEnabled = !$ircEnabled)} />
			{/snippet}
		</CardGroup.Item>
		{#if $ircEnabled}
			<CardGroup.Item>
				<Textbox
					value={$ircServer}
					size="large"
					label="服务器"
					placeholder="wss://irc.gitbutler.com:443"
					onchange={(value) => ($ircServer = value)}
				/>
			</CardGroup.Item>
		{/if}
	{/if}
</CardGroup>

<style>
	.experimental-settings__text {
		margin-bottom: 10px;
		color: var(--clr-text-2);
	}
</style>
