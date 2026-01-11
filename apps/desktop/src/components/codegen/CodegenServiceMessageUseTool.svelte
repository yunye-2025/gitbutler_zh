<script lang="ts">
	import CodegenServiceMessage from '$components/codegen/CodegenServiceMessage.svelte';
	import type { ToolCall } from '$lib/codegen/messages';

	type Props = { toolCall: ToolCall };

	const { toolCall }: Props = $props();

	const wantsToDo = $derived.by(() => {
		switch (toolCall.name) {
			case 'Task':
				return '启动子代理';
			case 'Bash':
				return '运行命令';
			case 'Glob':
				return '按模式匹配一些文件';
			case 'Grep':
				return '进行内容搜索';
			case 'Read':
				return '读取文件';
			case 'Edit':
				return '编辑文件';
			case 'MultiEdit':
				return '编辑文件';
			case 'Write':
				return '创建文件';
			case 'WebFetch':
				return '搜索互联网';
			case 'WebSearch':
				return '搜索互联网';
			default:
				return '执行需要你批准的操作';
		}
	});

	const actionName = $derived.by(() => {
		switch (toolCall.name) {
			case 'Task':
				return '子代理请求';
			case 'Bash':
				return '命令';
			case 'Glob':
				return '模式匹配';
			case 'Grep':
				return '内容搜索';
			case 'Read':
				return '读取请求';
			case 'Edit':
				return '编辑请求';
			case 'MultiEdit':
				return '编辑请求';
			case 'Write':
				return '创建请求';
			case 'WebFetch':
				return '网络搜索';
			case 'WebSearch':
				return '网络搜索';
			default:
				return '操作';
		}
	});
</script>

<CodegenServiceMessage style="pop" face="waiting">
	<div class="flex flex-col gap-2">
		<p class="text-13 text-semibold text-body">Claude Code 想要 {wantsToDo} 👆</p>
		<p class="text-13 text-italic text-body opacity-60">
			请查看上方的{actionName}，然后批准或拒绝。
		</p>
	</div>
</CodegenServiceMessage>
