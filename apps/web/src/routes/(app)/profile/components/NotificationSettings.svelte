<script lang="ts">
	import { CardGroup, Spacer, Toggle } from '@gitbutler/ui';
	import type { NotificationSettingsService } from '@gitbutler/shared/settings/notificationSettingsService';
	import type { NotificationSettings as NotificationSettingsType } from '@gitbutler/shared/settings/types';

	interface Props {
		notificationSettings: NotificationSettingsType;
		notificationSettingsService: NotificationSettingsService;
	}

	let { notificationSettings, notificationSettingsService }: Props = $props();

	let updatingReceiveChatMentionEmails = $state(false);
	let updatingReceiveChatReplyEmails = $state(false);
	let updatingReceiveIssueCreationEmails = $state(false);
	let updatingReceiveIssueResolutionEmails = $state(false);
	let updatingReceiveReviewBranchEmails = $state(false);
	let updatingReceiveSignOffEmails = $state(false);

	async function updateReceiveChatMentionEmails(value: boolean) {
		updatingReceiveChatMentionEmails = true;
		await notificationSettingsService.updateNotificationSettings({
			receiveChatMentionEmails: value
		});
		updatingReceiveChatMentionEmails = false;
	}

	async function updateReceiveChatReplyEmails(value: boolean) {
		updatingReceiveChatReplyEmails = true;
		await notificationSettingsService.updateNotificationSettings({
			receiveChatReplyEmails: value
		});
		updatingReceiveChatReplyEmails = false;
	}

	async function updateReceiveIssueCreationEmails(value: boolean) {
		updatingReceiveIssueCreationEmails = true;
		await notificationSettingsService.updateNotificationSettings({
			receiveIssueCreationEmails: value
		});
		updatingReceiveIssueCreationEmails = false;
	}

	async function updateReceiveIssueResolutionEmails(value: boolean) {
		updatingReceiveIssueResolutionEmails = true;
		await notificationSettingsService.updateNotificationSettings({
			receiveIssueResolutionEmails: value
		});
		updatingReceiveIssueResolutionEmails = false;
	}

	async function updateReceiveReviewBranchEmails(value: boolean) {
		updatingReceiveReviewBranchEmails = true;
		await notificationSettingsService.updateNotificationSettings({
			receiveReviewBranchEmails: value
		});
		updatingReceiveReviewBranchEmails = false;
	}

	async function updateReceiveSignOffEmails(value: boolean) {
		updatingReceiveSignOffEmails = true;
		await notificationSettingsService.updateNotificationSettings({
			receiveSignOffEmails: value
		});
		updatingReceiveSignOffEmails = false;
	}
</script>

<Spacer />

<div class="stack-v gap-8">
	<h2 class="text-15 text-bold">通知设置</h2>
	<p class="text-12 text-body clr-text-2">
		管理你在 GitButler 中各种活动的邮件通知偏好。
	</p>
</div>

<CardGroup>
	<CardGroup.Item labelFor="receive-chat-mention-emails">
		{#snippet title()}
			聊天消息提及邮件
		{/snippet}
		{#snippet caption()}
			当你在消息中被提及时发送邮件。
		{/snippet}
		{#snippet actions()}
			<Toggle
				id="receive-chat-mention-emails"
				checked={notificationSettings.receiveChatMentionEmails}
				disabled={updatingReceiveChatMentionEmails}
				onclick={() =>
					updateReceiveChatMentionEmails(!notificationSettings.receiveChatMentionEmails)}
			/>
		{/snippet}
	</CardGroup.Item>

	<CardGroup.Item labelFor="receive-chat-reply-emails">
		{#snippet title()}
			聊天回复邮件
		{/snippet}
		{#snippet caption()}
			当你收到聊天消息回复时发送邮件。
		{/snippet}
		{#snippet actions()}
			<Toggle
				id="receive-chat-reply-emails"
				checked={notificationSettings.receiveChatReplyEmails}
				disabled={updatingReceiveChatReplyEmails}
				onclick={() => updateReceiveChatReplyEmails(!notificationSettings.receiveChatReplyEmails)}
			/>
		{/snippet}
	</CardGroup.Item>

	<CardGroup.Item labelFor="receive-issue-creation-emails">
		{#snippet title()}
			问题创建邮件
		{/snippet}
		{#snippet caption()}
			在你参与的变更中创建新问题时发送邮件。
		{/snippet}
		{#snippet actions()}
			<Toggle
				id="receive-issue-creation-emails"
				checked={notificationSettings.receiveIssueCreationEmails}
				disabled={updatingReceiveIssueCreationEmails}
				onclick={() =>
					updateReceiveIssueCreationEmails(!notificationSettings.receiveIssueCreationEmails)}
			/>
		{/snippet}
	</CardGroup.Item>

	<CardGroup.Item labelFor="receive-issue-resolution-emails">
		{#snippet title()}
			问题状态邮件
		{/snippet}
		{#snippet caption()}
			在你参与的变更中，问题状态更新时发送邮件。
		{/snippet}
		{#snippet actions()}
			<Toggle
				id="receive-issue-resolution-emails"
				checked={notificationSettings.receiveIssueResolutionEmails}
				disabled={updatingReceiveIssueResolutionEmails}
				onclick={() =>
					updateReceiveIssueResolutionEmails(!notificationSettings.receiveIssueResolutionEmails)}
			/>
		{/snippet}
	</CardGroup.Item>

	<CardGroup.Item labelFor="receive-review-branch-emails">
		{#snippet title()}
			分支版本更新邮件
		{/snippet}
		{#snippet caption()}
			创建新的评审分支版本时发送邮件。
		{/snippet}
		{#snippet actions()}
			<Toggle
				id="receive-review-branch-emails"
				checked={notificationSettings.receiveReviewBranchEmails}
				disabled={updatingReceiveReviewBranchEmails}
				onclick={() =>
					updateReceiveReviewBranchEmails(!notificationSettings.receiveReviewBranchEmails)}
			/>
		{/snippet}
	</CardGroup.Item>

	<CardGroup.Item labelFor="receive-sign-off-emails">
		{#snippet title()}
			变更状态更新邮件
		{/snippet}
		{#snippet caption()}
			在你参与的变更中，评审状态更新时发送邮件。
		{/snippet}
		{#snippet actions()}
			<Toggle
				id="receive-sign-off-emails"
				checked={notificationSettings.receiveSignOffEmails}
				disabled={updatingReceiveSignOffEmails}
				onclick={() => updateReceiveSignOffEmails(!notificationSettings.receiveSignOffEmails)}
			/>
		{/snippet}
	</CardGroup.Item>
</CardGroup>
