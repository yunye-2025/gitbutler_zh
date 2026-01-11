<script lang="ts">
	import { Textbox } from '@gitbutler/ui';

	interface Props {
		// Username-specific props
		customValidationMessage?: string;
		minLength?: number;
		maxLength?: number;
		value?: string;
		label?: string;
		placeholder?: string;
		// All other props are forwarded to Textbox
		[key: string]: any;
	}

	let {
		customValidationMessage = '请输入有效的用户名。',
		minLength = 3,
		maxLength = 30,
		value = $bindable(),
		label = '用户名',
		...restProps
	}: Props = $props();

	let usernameError = $state<string | undefined>(undefined);
	let usernameTouched = $state(false);

	function validateUsername(val: string): { isValid: boolean; message?: string } {
		if (!val) return { isValid: true }; // Empty is valid (unless required)

		// Check length
		if (val.length < minLength) {
			return {
				isValid: false,
				message: `用户名至少需要 ${minLength} 个字符。`
			};
		}

		if (val.length > maxLength) {
			return {
				isValid: false,
				message: `用户名不能超过 ${maxLength} 个字符。`
			};
		}

		// Check for valid characters: alphanumeric, underscores, hyphens
		// Must start with alphanumeric character
		if (!/^[a-zA-Z0-9][a-zA-Z0-9_-]*$/.test(val)) {
			return {
				isValid: false,
				message: '用户名必须以字母或数字开头，只能包含字母、数字、下划线和连字符。'
			};
		}

		// Cannot end with hyphen or underscore
		if (/[-_]$/.test(val)) {
			return {
				isValid: false,
				message: '用户名不能以连字符或下划线结尾。'
			};
		}

		// Cannot have consecutive special characters
		if (/[-_]{2,}/.test(val)) {
			return {
				isValid: false,
				message: '用户名不能包含连续的连字符或下划线。'
			};
		}

		return { isValid: true };
	}

	function handleInput(val: string) {
		value = val;

		// Only show validation errors after the field has been touched (blurred once)
		if (usernameTouched) {
			const validation = validateUsername(val);
			usernameError =
				val && !validation.isValid ? validation.message || customValidationMessage : undefined;
		}
	}

	function handleChange() {
		// Mark as touched when user leaves the field
		usernameTouched = true;

		// Validate on blur
		if (value) {
			const validation = validateUsername(value);
			if (!validation.isValid) {
				usernameError = validation.message || customValidationMessage;
			} else {
				usernameError = undefined;
			}
		} else {
			usernameError = undefined;
		}
	}

	// Export validation state for parent components
	export function isValid(): boolean {
		if (!value) return true;
		return validateUsername(value).isValid;
	}

	export function validate(): boolean {
		usernameTouched = true;
		if (value) {
			const validation = validateUsername(value);
			if (!validation.isValid) {
				usernameError = validation.message || customValidationMessage;
				return false;
			} else {
				usernameError = undefined;
				return true;
			}
		} else {
			usernameError = undefined;
			return true;
		}
	}
</script>

<Textbox
	{...restProps}
	{label}
	type="text"
	bind:value
	error={usernameError}
	oninput={handleInput}
	onchange={handleChange}
/>
