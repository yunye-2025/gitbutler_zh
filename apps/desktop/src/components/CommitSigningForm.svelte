<script lang="ts">
	import SectionCardDisclaimer from '$components/SectionCardDisclaimer.svelte';
	import SettingsSection from '$components/SettingsSection.svelte';
	import { GIT_CONFIG_SERVICE } from '$lib/config/gitConfigService';
	import { GIT_SERVICE } from '$lib/git/gitService';
	import { inject } from '@gitbutler/core/context';
	import {
		Button,
		CardGroup,
		InfoMessage,
		Link,
		Select,
		SelectItem,
		Textbox,
		Toggle
	} from '@gitbutler/ui';

	const { projectId }: { projectId: string } = $props();

	const gitConfig = inject(GIT_CONFIG_SERVICE);
	const gitService = inject(GIT_SERVICE);

	async function setSignCommits(targetState: boolean) {
		signCommits = targetState;
		await gitConfig.setGbConfig(projectId, { signCommits: targetState });
	}

	const signingFormatOptions = [
		{
			label: 'GPG',
			value: 'openpgp',
			keyPlaceholder: 'ex: 723CCA3AC13CF28D',
			programPlaceholder: 'ex: /usr/local/bin/gpg'
		},
		{
			label: 'SSH',
			value: 'ssh',
			keyPlaceholder: 'ex: /Users/bob/.ssh/id_rsa.pub',
			programPlaceholder: 'ex: /Applications/1Password.app/Contents/MacOS/op-ssh-sign'
		}
	] as const;

	const selectedOption = $derived(
		signingFormatOptions.find((option) => option.value === signingFormat)
	);
	const keyPlaceholder = $derived(selectedOption?.keyPlaceholder);
	const programPlaceholder = $derived(selectedOption?.programPlaceholder);

	let checked = $state(false);
	let loading = $state(true);
	let signCheckResult = $state(false);
	let errorMessage = $state('');

	async function checkSigning() {
		errorMessage = '';
		checked = true;
		loading = true;
		await gitService
			.checkSigningSettings(projectId)
			.then(() => {
				signCheckResult = true;
			})
			.catch((err) => {
				console.error('Error checking signing:', err);
				errorMessage = err.message;
				signCheckResult = false;
			});
		loading = false;
	}

	async function updateSigningInfo() {
		let signUpdate = {
			signingFormat: signingFormat,
			signingKey: signingKey,
			gpgProgram: signingFormat === 'openpgp' ? signingProgram : '',
			gpgSshProgram: signingFormat === 'ssh' ? signingProgram : ''
		};
		await gitConfig.setGbConfig(projectId, signUpdate);
	}

	const gbConfig = $derived(gitConfig.gbConfig(projectId));
	let signCommits = $derived(gbConfig.response?.signCommits ?? false);
	let signingFormat = $derived(gbConfig.response?.signingFormat ?? 'openpgp');
	let signingKey = $derived(gbConfig.response?.signingKey ?? '');
	let signingProgram = $derived(
		gbConfig.response
			? signingFormat === 'openpgp'
				? (gbConfig.response.gpgProgram ?? '')
				: (gbConfig.response.gpgSshProgram ?? '')
			: ''
	);

	async function handleSignCommitsClick(event: MouseEvent) {
		await setSignCommits((event.target as HTMLInputElement)?.checked);
	}
</script>

<SettingsSection>
	<CardGroup>
		<CardGroup.Item labelFor="signCommits">
			{#snippet title()}
				提交签名
			{/snippet}
			{#snippet caption()}
				使用 GPG 或 SSH 为提交签名，以便验证其真实性。
				<br />
				GitButler 会按你的 Git 配置进行签名，但会优先使用
				<code class="code-string">gitbutler.signCommits</code>。
			{/snippet}
			{#snippet actions()}
				<Toggle id="signCommits" checked={signCommits} onclick={handleSignCommitsClick} />
			{/snippet}
		</CardGroup.Item>
	</CardGroup>
	{#if signCommits}
		<CardGroup>
			<CardGroup.Item>
				<Select
					value={signingFormat}
					options={signingFormatOptions}
					wide
					label="签名格式"
					onselect={(value: string) => {
						signingFormat = value;
						updateSigningInfo();
					}}
				>
					{#snippet itemSnippet({ item, highlighted })}
						<SelectItem selected={item.value === signingFormat} {highlighted}>
							{item.label}
						</SelectItem>
					{/snippet}
				</Select>

				<Textbox
					label="签名密钥"
					bind:value={signingKey}
					required
					onchange={updateSigningInfo}
					placeholder={keyPlaceholder}
				/>

				<Textbox
					label="签名程序（可选）"
					bind:value={signingProgram}
					onchange={updateSigningInfo}
					placeholder={programPlaceholder}
				/>

				{#if checked}
					<InfoMessage
						style={loading ? 'info' : signCheckResult ? 'success' : 'error'}
						filled
						outlined={false}
						error={errorMessage}
					>
						{#snippet title()}
							{#if loading}
								<p>正在检查签名</p>
							{:else if signCheckResult}
								<p>签名工作正常</p>
							{:else}
								<p>签名未正常工作</p>
							{/if}
						{/snippet}
					</InfoMessage>
				{/if}

				<Button style="pop" wide icon="item-tick" onclick={checkSigning}>
					{#if !checked}
						测试签名
					{:else}
						重新测试签名
					{/if}
				</Button>
				<SectionCardDisclaimer>
					若你公开签名密钥的公钥，提交签名可让其他人验证你的提交。
					<Link href="https://docs.gitbutler.com/features/virtual-branches/signing-commits"
						>了解更多</Link
					>关于提交签名与验证。
				</SectionCardDisclaimer>
			</CardGroup.Item>
		</CardGroup>
	{/if}
</SettingsSection>
