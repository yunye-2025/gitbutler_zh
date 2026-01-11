<script lang="ts">
	import FullscreenUtilityCard from '$lib/components/service/FullscreenUtilityCard.svelte';
</script>

<svelte:head>
	<title>GitButler | 已登录</title>
</svelte:head>

<FullscreenUtilityCard
	title="登录成功 🎯"
	backlink={{
		label: '主页',
		href: '/'
	}}
>
	<p class="text-14 clr-text-2">你现在可以关闭此窗口并返回客户端。</p>
</FullscreenUtilityCard>
