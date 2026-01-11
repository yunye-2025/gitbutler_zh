<script lang="ts">
	import { Textbox } from '@gitbutler/ui';

	type Props = {
		value: string | undefined;
	};

	let { value = $bindable() }: Props = $props();
</script>

<Textbox {value} onchange={(v) => (value = v)} placeholder="路径：例如 src/components" />
