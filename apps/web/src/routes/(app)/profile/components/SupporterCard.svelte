<script lang="ts">
	import { Button } from '@gitbutler/ui';
	import { env } from '$env/dynamic/public';
</script>

<div class="supporter-card">
	<h3 class="text-15 text-bold">🎉 感谢你的支持！</h3>
	<p class="text-12 text-body clr-text-2">
		你的支持帮助我们打造更好的 GitButler。<br />感谢你的贡献。
	</p>
	<Button
		style="pop"
		onclick={() => (window.location.href = `${env.PUBLIC_APP_HOST}supporter/portal`)}
	>
		管理订阅
	</Button>
</div>

<style lang="postcss">
	.supporter-card {
		display: flex;
		flex-direction: column;
		width: 100%;
		padding: 16px;
		gap: 12px;
		border: 1px solid var(--clr-border-2);
		border-radius: var(--radius-m);
		background-color: var(--clr-bg-1);
	}
</style>
