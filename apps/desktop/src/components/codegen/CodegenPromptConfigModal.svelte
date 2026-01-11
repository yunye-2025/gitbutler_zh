<script lang="ts">
	import { Codeblock } from '@gitbutler/ui';
	import { Modal, SegmentControl, Button } from '@gitbutler/ui';
	import type { PromptDir } from '$lib/codegen/types';

	type Props = {
		promptDirs: PromptDir[];
		openPromptConfigDir: (path: string) => void;
	};

	let modal = $state<Modal>();

	export function show() {
		modal?.show();
	}

	const { promptDirs, openPromptConfigDir }: Props = $props();

	let selectedSegment = $state<string>(promptDirs[0]?.label || '');
</script>

{#snippet pathContent({ path, caption }: { path: string; caption: string })}
	<Codeblock content={path} label="位置：" />
	<p class="text-13 text-body clr-text-2">{caption}</p>
{/snippet}

<Modal bind:this={modal} width={420} title="配置提示词模板">
	<div class="stack-v gap-16">
		<p class="text-13 text-body clr-text-2">
			提示词会从两个位置查找。项目提示词会覆盖全局提示词。以
			<code class="code-string">.local.md</code> 结尾的文件会覆盖常规项目提示词。
		</p>

		<SegmentControl selected={selectedSegment} onselect={(id) => (selectedSegment = id)}>
			{#each promptDirs as dir}
				<SegmentControl.Item
					id={dir.label}
					icon={dir.label === 'Global' ? 'global-small' : 'folder'}
				>
					{dir.label === 'Global' ? '全局' : dir.label === 'Project' ? '项目' : dir.label}
				</SegmentControl.Item>
			{/each}
		</SegmentControl>
		{#if selectedSegment === 'Global'}
			{@render pathContent({
				path: promptDirs.find((d) => d.label === 'Global')?.path || '',
				caption: '包含所有项目可用的全局提示词模板。'
			})}
		{:else if selectedSegment === 'Project'}
			{@render pathContent({
				path: promptDirs.find((d) => d.label === 'Project')?.path || '',
				caption: '包含项目专用提示词模板，会覆盖全局提示词。'
			})}
		{/if}

		<Button
			icon="open-editor-small"
			onclick={() => {
				const dir = promptDirs.find((d) => d.label === selectedSegment);
				if (dir) {
					openPromptConfigDir(dir.path);
				}
			}}>在编辑器中打开</Button
		>
	</div>
</Modal>
