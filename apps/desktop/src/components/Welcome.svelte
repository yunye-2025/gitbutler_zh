<script lang="ts">
	import { goto } from '$app/navigation';
	import IconLink from '$components/IconLink.svelte';
	import WelcomeAction from '$components/WelcomeAction.svelte';
	import WelcomeSigninAction from '$components/WelcomeSigninAction.svelte';
	import { OnboardingEvent, POSTHOG_WRAPPER } from '$lib/analytics/posthog';
	import cloneRepoSvg from '$lib/assets/welcome/clone-repo.svg?raw';
	import newProjectSvg from '$lib/assets/welcome/new-local-project.svg?raw';
	import { handleAddProjectOutcome } from '$lib/project/project';
	import { PROJECTS_SERVICE } from '$lib/project/projectsService';
	import { inject } from '@gitbutler/core/context';
	import { TestId } from '@gitbutler/ui';

	const projectsService = inject(PROJECTS_SERVICE);
	const posthog = inject(POSTHOG_WRAPPER);

	let newProjectLoading = $state(false);
	let directoryInputElement = $state<HTMLInputElement | undefined>();

	async function onNewProject() {
		newProjectLoading = true;
		try {
			const testDirectoryPath = directoryInputElement?.value;
			const outcome = await projectsService.addProject(testDirectoryPath ?? '');

			posthog.captureOnboarding(OnboardingEvent.AddLocalProject);
			if (outcome) {
				handleAddProjectOutcome(outcome);
			}
		} catch (e: unknown) {
			posthog.captureOnboarding(OnboardingEvent.AddLocalProjectFailed, e);
		} finally {
			newProjectLoading = false;
		}
	}

	async function onCloneProject() {
		goto('/onboarding/clone');
	}
</script>

<div class="welcome" data-testid={TestId.WelcomePage}>
	<h1 class="welcome-title text-serif-42">欢迎使用 GitButler！</h1>
	<div class="welcome__actions">
		<div class="welcome__actions--repo">
			<input
				type="text"
				hidden
				bind:this={directoryInputElement}
				data-testid="test-directory-path"
			/>
			<WelcomeAction
				title="添加本地项目"
				loading={newProjectLoading}
				onclick={onNewProject}
				dimMessage
				testId={TestId.WelcomePageAddLocalProjectButton}
			>
				{#snippet icon()}
					{@html newProjectSvg}
				{/snippet}
				{#snippet message()}
					应为有效的 Git 仓库
				{/snippet}
			</WelcomeAction>
			<WelcomeAction title="克隆仓库" onclick={onCloneProject} dimMessage>
				{#snippet icon()}
					{@html cloneRepoSvg}
				{/snippet}
				{#snippet message()}
					使用 URL 克隆仓库
				{/snippet}
			</WelcomeAction>
		</div>
		<!-- Using instance of user here to not hide after login -->
		<WelcomeSigninAction />
	</div>

	<div class="links">
		<div class="links__section">
			<p class="links__title text-14 text-bold">快速开始</p>
			<div class="education-links">
				<IconLink
					icon="docs"
					href="https://docs.gitbutler.com/features/virtual-branches/branch-lanes"
				>
					GitButler 文档
				</IconLink>
				<IconLink icon="video" href="https://www.youtube.com/@gitbutlerapp">
					观看教程
				</IconLink>
			</div>
		</div>
		<div class="links__section">
			<p class="links__title text-14 text-bold">加入社区</p>
			<div class="community-links">
				<IconLink icon="discord" href="https://discord.gg/MmFkmaJ42D">Discord</IconLink>
				<IconLink icon="bluesky" href="https://bsky.app/profile/gitbutler.com">Bluesky</IconLink>
				<IconLink icon="instagram" href="https://www.instagram.com/gitbutler/">Instagram</IconLink>
				<IconLink icon="youtube" href="https://www.youtube.com/@gitbutlerapp">YouTube</IconLink>
			</div>
		</div>
	</div>
</div>

<style lang="postcss">
	.welcome {
		width: 100%;
	}

	.welcome-title {
		color: var(--clr-text-1);
		line-height: 1;
	}

	.welcome__actions {
		display: flex;
		flex-direction: column;
		margin-top: 32px;
		gap: 8px;
	}

	.welcome__actions--repo {
		display: flex;
		gap: 8px;
	}

	.links {
		display: flex;
		margin-top: 20px;
		padding: 28px;
		gap: 56px;
		border-radius: var(--radius-m);
		background: var(--clr-bg-muted);
	}

	.links__section {
		display: flex;
		flex-direction: column;
		gap: 20px;
	}

	.education-links {
		display: flex;
		flex-direction: column;
		align-items: flex-start;
		margin-left: -6px;
		gap: 6px;
	}

	.community-links {
		display: grid;
		grid-template-columns: repeat(2, 1fr);
		column-gap: 12px;
		row-gap: 4px;
		max-width: 192px;
		margin-left: -6px;
	}

	/* SMALL ILLUSTRATIONS */
</style>
