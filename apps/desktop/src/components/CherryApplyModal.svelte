<script lang="ts">
	import { goto } from '$app/navigation';
	import ReduxResult from '$components/ReduxResult.svelte';
	import { CHERRY_APPLY_SERVICE } from '$lib/cherryApply/cherryApplyService';
	import { workspacePath } from '$lib/routes/routes.svelte';
	import { getStackName } from '$lib/stacks/stack';
	import { STACK_SERVICE } from '$lib/stacks/stackService.svelte';
	import { combineResults } from '$lib/state/helpers';
	import { inject } from '@gitbutler/core/context';
	import { Button, CardGroup, InfoMessage, Modal, RadioButton } from '@gitbutler/ui';

	type Props = {
		projectId: string;
		/** The commit hash to cherry-apply */
		subject?: string;
	};

	let { projectId, subject }: Props = $props();

	const cherryApplyService = inject(CHERRY_APPLY_SERVICE);
	const stackService = inject(STACK_SERVICE);

	let modalRef = $state<Modal>();

	const statusResult = $derived(
		subject ? cherryApplyService.status({ projectId, subject }) : undefined
	);
	const stacksResult = $derived(stackService.stacks(projectId));
	const status = $derived(statusResult?.response);

	let selectedStackId = $state<string | undefined>(undefined);
	const [applyCommit, applyResult] = cherryApplyService.apply();

	$effect(() => {
		if (status?.type === 'lockedToStack') {
			selectedStackId = status.subject;
		}
	});

	export function close() {
		modalRef?.close();
	}

	export function open() {
		modalRef?.show();
	}

	async function handleApply() {
		if (!selectedStackId || !subject) return;

		await applyCommit({
			projectId,
			subject,
			target: selectedStackId
		});

		goto(workspacePath(projectId));

		close();
	}

	function getStatusMessage(): string {
		if (!status) return '';

		switch (status.type) {
			case 'applicableToAnyStack':
				return '此提交可以应用到任何堆叠。请在下方选择一个堆叠。';
			case 'lockedToStack':
				return '此提交在应用到所选堆叠时会产生冲突，因此必须应用到该堆叠以避免工作区冲突。';
			case 'causesWorkspaceConflict':
				return '此提交无法应用，因为会导致工作区冲突。';
			case 'noStacks':
				return '当前没有堆叠被应用到工作区。';
		}
	}

	const canApply = $derived(
		status?.type === 'applicableToAnyStack' || status?.type === 'lockedToStack'
	);
	const canSelectStack = $derived(status?.type === 'applicableToAnyStack');
	const isApplying = $derived(applyResult.current.isLoading);

	function handleStackSelectionChange(form: HTMLFormElement) {
		const formData = new FormData(form);
		const selected = formData.get('stackSelection') as string | null;
		if (selected) {
			selectedStackId = selected;
		}
	}

	const messageStyle = $derived(status?.type === 'causesWorkspaceConflict' ? 'warning' : 'info');
</script>

<Modal bind:this={modalRef} title="拣选提交" width={500}>
	{#if statusResult}
		<ReduxResult {projectId} result={combineResults(statusResult?.result, stacksResult.result)}>
			{#snippet children([_status, stacks], { projectId: _projectId })}
				<div class="cherry-apply-modal">
					<InfoMessage style={messageStyle} outlined>
						{#snippet content()}
							{getStatusMessage()}
						{/snippet}
					</InfoMessage>

					{#if canApply && stacks.length > 0}
						<CardGroup>
							<form onchange={(e) => handleStackSelectionChange(e.currentTarget)}>
								{#each stacks as stack}
									{@const isDisabled = !canSelectStack && selectedStackId !== stack.id}

									<CardGroup.Item labelFor="stack-{stack.id}" disabled={isDisabled}>
										{#snippet title()}
											{getStackName(stack)}
										{/snippet}
										{#snippet caption()}
											{stack.heads.length}
											个分支
										{/snippet}
										{#snippet actions()}
											<RadioButton
												name="stackSelection"
												value={stack.id ?? undefined}
												id="stack-{stack.id}"
												checked={selectedStackId === stack.id}
												disabled={isDisabled}
											/>
										{/snippet}
									</CardGroup.Item>
								{/each}
							</form>
						</CardGroup>
					{/if}
				</div>
			{/snippet}
		</ReduxResult>
	{/if}
	{#snippet controls()}
		<Button kind="outline" onclick={close} disabled={isApplying}>取消</Button>
		<Button
			style="pop"
			onclick={handleApply}
			disabled={!canApply || !selectedStackId || isApplying}
			loading={isApplying}
		>
			应用提交
		</Button>
	{/snippet}
</Modal>

<style lang="postcss">
	.cherry-apply-modal {
		display: flex;
		flex-direction: column;
		gap: 16px;
	}
</style>
