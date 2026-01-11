<script lang="ts">
	import RedirectIfLoggedIn from '$lib/auth/RedirectIfLoggedIn.svelte';
	import FullscreenUtilityCard from '$lib/components/service/FullscreenUtilityCard.svelte';
	import { inject } from '@gitbutler/core/context';
	import { LOGIN_SERVICE } from '@gitbutler/shared/login/loginService';
	import { WEB_ROUTES_SERVICE } from '@gitbutler/shared/routing/webRoutes.svelte';
	import { Button, EmailTextbox, InfoMessage } from '@gitbutler/ui';

	const loginService = inject(LOGIN_SERVICE);
	const routesService = inject(WEB_ROUTES_SERVICE);

	let email = $state<string>();
	let emailTextbox: any = $state();
	let error = $state<string>();
	let isLinkSent = $state<boolean>(false);
	let sentToEmail = $state<string>();

	const canSubmit = $derived(!!email && emailTextbox?.isValid());

	async function handleSubmit() {
		if (!email) {
			error = '邮箱不能为空';
			return;
		}

		const response = await loginService.resetPassword(email);
		if (response.type === 'error') {
			error = response.errorMessage;
			console.error('重置密码失败：', response.raw ?? response.errorMessage);
		} else {
			error = undefined;
			sentToEmail = email;
			isLinkSent = true;
		}
	}
</script>

<svelte:head>
	<title>GitButler | 忘记密码</title>
</svelte:head>

<RedirectIfLoggedIn />

<FullscreenUtilityCard
	title={isLinkSent ? '已发送链接！' : '忘记密码？'}
	backlink={{ label: '登录', href: routesService.loginPath() }}
>
	{#if isLinkSent}
		<p class="text-13 text-body">
			我们已将密码重置链接发送至：<i class="clr-text-2">{sentToEmail}</i>
			<br />
			请点击邮件中的链接重置密码。
		</p>
	{:else}
		<div class="service-form__inputs">
			<EmailTextbox bind:this={emailTextbox} bind:value={email} label="邮箱" />

			{#if error}
				<InfoMessage filled outlined={false} style="danger">
					{#snippet content()}
						{error}
					{/snippet}
				</InfoMessage>
			{/if}

			<Button style="pop" disabled={!canSubmit} onclick={handleSubmit}>发送重置链接</Button>
		</div>
	{/if}
</FullscreenUtilityCard>

<style lang="postcss">
	.service-form__inputs {
		display: flex;
		flex-direction: column;
		gap: 16px;
	}
</style>
