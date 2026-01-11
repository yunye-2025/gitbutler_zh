<script lang="ts">
	import newProjectSvg from '$lib/assets/splash-illustrations/new-project.svg?raw';
	import RedirectIfLoggedIn from '$lib/auth/RedirectIfLoggedIn.svelte';
	import OAuthButtons from '$lib/components/auth/OAuthButtons.svelte';
	import PasswordConfirmation from '$lib/components/auth/PasswordConfirmation.svelte';
	import UsernameTextbox from '$lib/components/auth/UsernameTextbox.svelte';
	import FullscreenIllustrationCard from '$lib/components/service/FullscreenIllustrationCard.svelte';
	import { inject } from '@gitbutler/core/context';
	import { LOGIN_SERVICE } from '@gitbutler/shared/login/loginService';
	import { WEB_ROUTES_SERVICE } from '@gitbutler/shared/routing/webRoutes.svelte';
	import { Button, EmailTextbox, InfoMessage } from '@gitbutler/ui';

	let username = $state<string>();
	let email = $state<string>();
	let password = $state<string>();
	let passwordConfirmation = $state<string>();
	let error = $state<string>();
	let successMessage = $state<string>();

	let emailTextbox: any = $state();
	let usernameTextbox: any = $state();
	let passwordComponent: PasswordConfirmation | undefined = $state();

	const isFormValid = $derived(
		username?.trim() &&
			email?.trim() &&
			emailTextbox?.isValid() &&
			usernameTextbox?.isValid() &&
			passwordComponent?.isValid?.()
	);

	const loginService = inject(LOGIN_SERVICE);
	const routesService = inject(WEB_ROUTES_SERVICE);

	async function handleSubmit(event: Event) {
		event.preventDefault();
		if (!username || !email || !password || !passwordConfirmation) {
			error = '用户名、邮箱和密码不能为空';
			return;
		}

		if (!passwordComponent?.isValid()) {
			error = '请检查密码及确认密码';
			return;
		}

		if (!usernameTextbox?.isValid()) {
			error = '请检查用户名';
			return;
		}

		const response = await loginService.createAccountWithEmail(
			username,
			email,
			password,
			passwordConfirmation
		);

		if (response.type === 'error') {
			error = response.errorMessage;
			console.error('注册失败：', response.raw ?? response.errorMessage);
		} else {
			error = undefined;
			successMessage = response.data.message;
		}
	}
</script>

<svelte:head>
	<title>GitButler | 注册</title>
</svelte:head>

<RedirectIfLoggedIn />

<FullscreenIllustrationCard illustration={successMessage ? newProjectSvg : undefined}>
	{#snippet title()}
		{#if !successMessage}
			<i>注册</i>
			GitButler
		{:else}
			🚀 请查收<i>邮件</i>获取确认说明
		{/if}
	{/snippet}

	{#if !successMessage}
		<form id="signup-form" class="stack-v" onsubmit={handleSubmit}>
			<div class="auth-form__inputs">
				<UsernameTextbox bind:this={usernameTextbox} bind:value={username} />
				<EmailTextbox
					bind:this={emailTextbox}
					label="邮箱"
					placeholder=" "
					bind:value={email}
					autocomplete={false}
					autocorrect={false}
					spellcheck
				/>
				<PasswordConfirmation
					bind:this={passwordComponent}
					bind:password
					bind:passwordConfirmation
				/>
			</div>

			{#if error}
				<InfoMessage filled outlined={false} style="danger" class="m-b-16">
					{#snippet content()}
						{error}
					{/snippet}
				</InfoMessage>
			{/if}

			<Button type="submit" style="pop" disabled={!isFormValid}>创建账号</Button>

			<OAuthButtons mode="signup" />
		</form>
	{/if}

	{#snippet footer()}
		<div class="auth-form__footer">
			{#if !successMessage}
				<p>
					*注册即表示你同意我们的
					<a
						href="https://app.termly.io/document/terms-and-conditions/7924c71d-80bb-4444-9381-947237b9fc0f"
						>条款</a
					>
					和
					<a
						href="https://app.termly.io/document/privacy-policy/a001c8b7-505b-4eab-81e3-fcd1c72bdd79"
						>隐私政策</a
					>
				</p>
				<p>
					已有账号？<a href={routesService.loginPath()}>立即登录</a>
				</p>
			{:else}
				<p>
					需要帮助？<a
						href="https://github.com/gitbutlerapp/gitbutler/issues/new?template=BLANK_ISSUE"
						target="_blank"
						rel="noopener noreferrer"
					>
						提交支持请求
					</a>
				</p>
			{/if}
		</div>
	{/snippet}
</FullscreenIllustrationCard>

<style lang="postcss">
	.auth-form__inputs {
		display: flex;
		flex-direction: column;
		margin-bottom: 24px;
		gap: 14px;
	}

	.auth-form__footer {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 8px;
	}
</style>
