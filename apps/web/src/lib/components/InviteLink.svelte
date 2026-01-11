<script lang="ts">
	import { browser } from '$app/environment';
	import { inject } from '@gitbutler/core/context';
	import {
		OrganizationService,
		ORGANIZATION_SERVICE
	} from '@gitbutler/shared/organizations/organizationService';
	import { Button, Textbox } from '@gitbutler/ui';

	interface Props {
		organizationSlug: string;
		inviteCode: string;
	}

	let { organizationSlug, inviteCode }: Props = $props();

	let inviteUrl = $state('');
	let copied = $state(false);
	let resetting = $state(false);
	let serviceError = $state(false);

	// Get the OrganizationService from context
	const organizationService = inject(ORGANIZATION_SERVICE);

	$effect(() => {
		if (browser) {
			updateInviteUrl();
		}
	});

	function updateInviteUrl() {
		// Create the invite URL with the origin of the current page
		const baseUrl = window.location.origin;
		inviteUrl = `${baseUrl}/organizations/invite/${organizationSlug}/${inviteCode}`;
	}

	function copyToClipboard() {
		if (!browser) return;

		// Use the Clipboard API to copy the invite URL
		navigator.clipboard
			.writeText(inviteUrl)
			.then(() => {
				copied = true;
				setTimeout(() => {
					copied = false;
				}, 2000);
			})
			.catch((err) => {
				console.error('复制失败：', err);
			});
	}

	async function resetInviteCode() {
		if (!browser) return;

		// Show confirmation dialog
		const confirmed = confirm(
			'确定要重置邀请码吗？这将使所有现有邀请链接失效。'
		);

		if (confirmed) {
			try {
				resetting = true;
				const updatedOrg = await (organizationService as OrganizationService).resetInviteCode(
					organizationSlug
				);

				// Update the invite code from the result
				inviteCode = updatedOrg.inviteCode || '';

				// Update the invite URL with the new code
				updateInviteUrl();
			} catch (error) {
				console.error('重置邀请码失败：', error);
				alert('重置邀请码失败，请重试。');
			} finally {
				resetting = false;
			}
		}
	}
</script>

{#if inviteCode}
	<div class="invite-link-container">
		<p>分享此链接邀请他人加入该组织：</p>

		<div class="invite-url-container">
			<Textbox readonly value={inviteCode} />
			<Button onclick={copyToClipboard} style={copied ? 'safe' : 'pop'}>复制链接</Button>
		</div>

		<p class="info-text">
			任何获得此链接的人都可以通过接受邀请加入你的组织。
		</p>

		<div class="reset-container">
			<Button onclick={resetInviteCode} style="warning" disabled={resetting || serviceError}>
				{#if serviceError}
					服务不可用
				{:else if resetting}
					重置中...
				{:else}
					重置邀请码
				{/if}
			</Button>
			{#if serviceError}
				<p class="error-text">
					当前无法重置。未找到组织服务。
				</p>
			{/if}
		</div>
	</div>
{/if}

<style>
	.invite-link-container {
		padding: 1rem;
	}

	p {
		margin-bottom: 1rem;
		font-size: 0.9rem;
	}

	.invite-url-container {
		display: flex;
		margin-bottom: 0.75rem;
	}

	.info-text {
		margin-top: 0.5rem;
		color: #718096;
		font-style: italic;
		font-size: 0.8rem;
	}

	.reset-container {
		margin-top: 1.5rem;
		padding-top: 1rem;
		border-top: 1px solid #e2e8f0;
	}

	.error-text {
		margin-top: 0.5rem;
		color: #e53e3e;
		font-weight: bold;
		font-size: 0.8rem;
	}
</style>
