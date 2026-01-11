<script lang="ts">
	import Factoid from '$lib/components/infoFlexRow/Factoid.svelte';
	import InfoFlexRow from '$lib/components/infoFlexRow/InfoFlexRow.svelte';
	import { inject } from '@gitbutler/core/context';
	import { getChatChannelParticipants } from '@gitbutler/shared/chat/chatChannelsPreview.svelte';
	import { CHAT_CHANNELS_SERVICE } from '@gitbutler/shared/chat/chatChannelsService';
	import {
		getUsersWithAvatars,
		getPatchApproversWithAvatars,
		getPatchContributorsWithAvatars,
		getPatchRejectorsWithAvatars
	} from '@gitbutler/shared/contributors';
	import ChangeStatus from '@gitbutler/shared/patches/ChangeStatus.svelte';
	import { type PatchCommit } from '@gitbutler/shared/patches/types';
	import { APP_STATE } from '@gitbutler/shared/redux/store.svelte';
	import { AvatarGroup, Icon } from '@gitbutler/ui';
	import { copyToClipboard } from '@gitbutler/ui/utils/clipboard';

	const NO_REVIEWERS = '尚未评审';
	const NO_CONTRIBUTORS = '暂无贡献者';
	const NO_COMMENTS = '暂无评论';

	interface Props {
		projectId: string;
		patchCommit: PatchCommit;
	}

	const { patchCommit, projectId }: Props = $props();
	const appState = inject(APP_STATE);
	const chatChannelService = inject(CHAT_CHANNELS_SERVICE);

	const chatParticipants = $derived(
		getChatChannelParticipants(appState, chatChannelService, projectId, patchCommit.changeId)
	);

	const commenters = $derived(
		chatParticipants.current === undefined
			? Promise.resolve([])
			: getUsersWithAvatars(chatParticipants.current)
	);
	const contributors = $derived(getPatchContributorsWithAvatars(patchCommit));
	const approvers = $derived(getPatchApproversWithAvatars(patchCommit));
	const rejectors = $derived(getPatchRejectorsWithAvatars(patchCommit));

	const commitShortSha = patchCommit.commitSha.substring(0, 7);
</script>

<InfoFlexRow>
	<Factoid label="状态">
		<ChangeStatus {patchCommit} />
	</Factoid>
	<Factoid label="评审人" placeholderText={NO_REVIEWERS}>
		{#await Promise.all([approvers, rejectors]) then [approvers, rejectors]}
			{#if approvers.length > 0 || rejectors.length > 0}
				<AvatarGroup avatars={rejectors} maxAvatars={2} icon="refresh-small" iconColor="warning" />
				<AvatarGroup avatars={approvers} maxAvatars={2} icon="tick-small" iconColor="safe" />
			{/if}
		{/await}
	</Factoid>
	<Factoid label="评论者" placeholderText={NO_COMMENTS}>
		{#await commenters then commentors}
			{#if commentors.length > 0}
				<AvatarGroup avatars={commentors} />
			{/if}
		{/await}
	</Factoid>
	<Factoid label="作者" placeholderText={NO_CONTRIBUTORS}>
		{#await contributors then contributors}
			{#if contributors.length > 0}
				<AvatarGroup avatars={contributors} />
			{/if}
		{/await}
	</Factoid>
	<Factoid label="版本">
		v{patchCommit.version}
	</Factoid>
	<Factoid label="提交 SHA">
		<button type="button" class="commit-sha" onclick={() => copyToClipboard(patchCommit.commitSha)}>
			<span>
				{commitShortSha}
			</span>
			<div class="factoid-icon">
				<Icon name="copy-small" />
			</div>
		</button>
	</Factoid>
</InfoFlexRow>

<style lang="postcss">
	.commit-sha {
		display: flex;
		align-items: center;
		justify-content: flex-start;
		gap: 2px;
		text-decoration-line: underline;
		text-decoration-style: dashed;
		text-underline-offset: 2px;
		cursor: pointer;

		&:hover {
			.factoid-icon {
				opacity: 1;
			}
		}
	}

	.factoid-icon {
		color: var(--clr-text-2);
		opacity: 0;
		transition: opacity var(--transition-fast);
	}
</style>
