<script lang="ts">
	import LoginModal from '$lib/components/LoginModal.svelte';
	import { USER_SERVICE } from '$lib/user/userService';
	import { inject } from '@gitbutler/core/context';
	import { PATCH_COMMIT_SERVICE } from '@gitbutler/shared/patches/patchCommitService';
	import { type PatchCommit } from '@gitbutler/shared/patches/types';
	import {
		CommitStatusBadge,
		ContextMenuItem,
		ContextMenuSection,
		DropdownButton
	} from '@gitbutler/ui';

	interface Props {
		branchUuid: string;
		patch: PatchCommit;
		isUserLoggedIn: boolean;
	}

	const actionLabels = {
		approve: '批准提交',
		requestChanges: '请求修改'
	} as const;

	type Action = keyof typeof actionLabels;
	type UserActionType = 'requested-changes' | 'approved' | 'not-reviewed';

	const { patch, branchUuid, isUserLoggedIn }: Props = $props();

	const patchService = inject(PATCH_COMMIT_SERVICE);
	const userService = inject(USER_SERVICE);
	const user = userService.user;

	const userAction = $derived.by<UserActionType>(() => {
		if (!$user) return 'not-reviewed';
		if (patch.reviewAll.rejected.some((rejector) => rejector.id === $user.id)) {
			return 'requested-changes';
		} else if (patch.reviewAll.signedOff.some((approver) => approver.id === $user.id)) {
			return 'approved';
		}

		return 'not-reviewed';
	});

	let loginModal = $state<LoginModal>();
	let action = $state<Action>('approve');
	let isExecuting = $state(false);
	let dropDownButton = $state<ReturnType<typeof DropdownButton>>();

	const buttonColor = $derived.by(() => {
		switch (action) {
			case 'approve':
				return 'pop';
			case 'requestChanges':
				return 'warning';
		}
	});

	const icon = $derived.by(() => {
		switch (action) {
			case 'approve':
				return 'success';
			case 'requestChanges':
				return 'refresh-in-circle';
		}
	});

	async function approve() {
		await patchService.updatePatch(branchUuid, patch.changeId, { signOff: true });
	}

	async function requestChanges() {
		await patchService.updatePatch(branchUuid, patch.changeId, { signOff: false });
	}

	async function handleClick(action: Action) {
		if (!isUserLoggedIn) {
			loginModal?.show();
			return;
		}

		if (isExecuting) return;
		isExecuting = true;

		try {
			switch (action) {
				case 'approve':
					await approve();
					break;
				case 'requestChanges':
					await requestChanges();
					break;
			}
		} finally {
			isExecuting = false;
		}
	}

	function confirmStatusChange(action: Action): boolean {
		const message =
			action === 'requestChanges'
				? '你已批准此提交。是否改为请求修改？'
				: '你已对该提交请求修改。是否改为批准？';

		return confirm(message);
	}

	function handleChangeStatus(action: Action) {
		if (!confirmStatusChange(action)) {
			return;
		}
		handleClick(action);
	}
</script>

{#if userAction === 'approved' || userAction === 'requested-changes'}
	<div class="my-status">
		<div class="text-12 my-status-text">
			{#if userAction === 'approved'}
				<CommitStatusBadge status="approved" kind="icon" />
				<span>你已批准此提交</span>
			{:else}
				<CommitStatusBadge status="changes-requested" kind="icon" />
				<span>你已请求修改</span>
			{/if}
		</div>

		<button
			class="text-12 change-status-btn"
			type="button"
			onclick={() => handleChangeStatus(userAction === 'approved' ? 'requestChanges' : 'approve')}
		>
			更改状态
		</button>
	</div>
{:else}
	<DropdownButton
		bind:this={dropDownButton}
		loading={isExecuting}
		{icon}
		menuSide="top"
		style={buttonColor}
		onclick={() => handleClick(action)}
	>
		{actionLabels[action]}
		{#snippet contextMenuSlot()}
			<ContextMenuSection>
				<ContextMenuItem
					label={actionLabels.approve}
					onclick={() => {
						action = 'approve';
						dropDownButton?.close();
					}}
				/>
				<ContextMenuItem
					label={actionLabels.requestChanges}
					onclick={() => {
						action = 'requestChanges';
						dropDownButton?.close();
					}}
				/>
			</ContextMenuSection>
		{/snippet}
	</DropdownButton>
{/if}

<LoginModal bind:this={loginModal}>
	要批准此提交或请求修改，请先登录。
</LoginModal>

<style lang="postcss">
	.my-status {
		display: flex;
		align-items: center;
		gap: 8px;
	}

	.my-status-text {
		display: flex;
		align-items: center;
		gap: 6px;
		font-style: italic;
	}

	.change-status-btn {
		text-decoration: underline;
		text-decoration-style: dotted;
	}
</style>
