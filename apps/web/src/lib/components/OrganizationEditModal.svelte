<script lang="ts">
	import { inject } from '@gitbutler/core/context';
	import { ORGANIZATION_SERVICE } from '@gitbutler/shared/organizations/organizationService';

	import { Button, Modal, Textarea, Textbox, chipToasts } from '@gitbutler/ui';
	import { slugify } from '@gitbutler/ui/utils/string';

	interface Props {
		organizationSlug: string;
		onUpdate?: (newSlug: string) => void;
	}

	let { organizationSlug, onUpdate = () => {} }: Props = $props();

	// Get organization service from context
	const organizationService = inject(ORGANIZATION_SERVICE);

	// Form state
	let name = $state('');
	let slug = $state('');
	let description = $state('');
	let originalSlug = $state('');

	// Derived slugified value
	const sluggifiedSlug = $derived(slugify(slug).toLocaleLowerCase());

	// Form state
	let isLoading = $state(false);
	let submitAttempted = $state(false);
	const requiredFieldsFilled = $derived(!!(name && sluggifiedSlug));

	// The modal component reference
	let modal = $state<Modal>();

	// Function to fetch organization details
	async function fetchOrganizationDetails() {
		try {
			isLoading = true;

			const organization = await organizationService.getOrganizationBySlug(organizationSlug);

			if (organization) {
				name = organization.name || '';
				slug = organization.slug;
				originalSlug = organization.slug;
				description = organization.description || '';
			}
		} catch (error) {
			chipToasts.error(
				`获取组织详情失败：${error instanceof Error ? error.message : '未知错误'}`
			);
		} finally {
			isLoading = false;
		}
	}

	// Function to update organization details
	async function updateOrganization(close: () => void) {
		submitAttempted = true;

		if (!requiredFieldsFilled) return;

		try {
			isLoading = true;

			const hasSlugChanged = originalSlug !== sluggifiedSlug;

			await organizationService.updateOrganization(originalSlug, {
				name,
				new_slug: hasSlugChanged ? sluggifiedSlug : undefined, // Only send new_slug if it changed
				description
			});

			chipToasts.success('组织已更新');

			// Notify parent component about the update
			onUpdate(sluggifiedSlug);

			close();

			// If the slug changed, we might need to redirect or refresh
			if (hasSlugChanged) {
				// Handle slug change - might require page refresh or redirect
				window.location.href = window.location.href.replace(originalSlug, sluggifiedSlug);
			}
		} catch (error) {
			chipToasts.error(
				`更新组织失败：${error instanceof Error ? error.message : '未知错误'}`
			);
		} finally {
			isLoading = false;
		}
	}

	// Reset form on modal close
	function onModalClose() {
		submitAttempted = false;
	}

	// Public function to show the modal
	export function show() {
		fetchOrganizationDetails();
		modal?.show();
	}
</script>

<Modal bind:this={modal} title="编辑组织" onClose={onModalClose} width="medium">
	<div class="form-container">
		<Textbox bind:value={name} label="名称" required={submitAttempted} disabled={isLoading} />

		<Textbox bind:value={slug} label="标识" required={submitAttempted} disabled={isLoading} />

		{#if slug !== sluggifiedSlug}
			<p class="slug-note">标识将保存为：{sluggifiedSlug}</p>
		{/if}

		<Textarea bind:value={description} label="描述" disabled={isLoading} />
	</div>

	{#snippet controls(close)}
		<Button kind="outline" onclick={close} disabled={isLoading}>取消</Button>

		<Button
			style="pop"
			disabled={!requiredFieldsFilled || isLoading}
			loading={isLoading}
			onclick={() => updateOrganization(close)}
		>
			保存更改
		</Button>
	{/snippet}
</Modal>

<style lang="postcss">
	.form-container {
		display: flex;
		flex-direction: column;
		gap: 16px;
	}

	.slug-note {
		margin-top: -12px;
		color: var(--clr-text-2);
		font-size: 13px;
	}
</style>
