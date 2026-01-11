import { createKeybindingsHandler } from 'tinykeys';

interface KeybindDefinitions {
	[combo: string]: (event: KeyboardEvent) => void;
}

export const shortcuts = {
	global: {
		open_repository: {
			title: '添加本地仓库…',
			keys: '$mod+O',
			description: null
		},
		clone_repository: {
			title: '克隆仓库',
			keys: '$mod+Shift+O',
			description: null
		},
		create_branch: {
			title: '创建分支',
			keys: '$mod+B',
			description: null
		},
		create_dependent_branch: {
			title: '创建依赖分支',
			keys: '$mod+Shift+B',
			description: null
		}
	},
	view: {
		switch_theme: {
			title: '切换主题',
			keys: '$mod+T',
			description: null
		},
		toggle_sidebar: {
			title: '切换未分配区',
			keys: '$mod+\\',
			description: null
		},
		zoom_in: {
			title: '放大',
			keys: '$mod+=',
			description: null
		},
		zoom_out: {
			title: '缩小',
			keys: '$mod+-',
			description: null
		},
		reset_zoom: {
			title: '重置缩放',
			keys: '$mod+0',
			description: null
		},
		reload_view: {
			title: '重新加载视图',
			keys: '$mod+R',
			description: null
		}
	},
	project: {
		project_history: {
			title: '项目历史',
			description: '打开项目历史视图。可还原更改、查看提交等。',
			keys: '$mod+Shift+H'
		}
	}
};

export function createKeybind(keybinds: KeybindDefinitions) {
	const keys: KeybindDefinitions = {
		// Ignore backspace keydown events always
		Backspace: () => {}
	};

	for (const [combo, callback] of Object.entries(keybinds)) {
		keys[combo] = (event: KeyboardEvent) => {
			if (
				event.repeat ||
				event.target instanceof HTMLInputElement ||
				event.target instanceof HTMLTextAreaElement ||
				('_lexicalHandled' in event && event._lexicalHandled)
			) {
				return;
			}

			event.preventDefault();

			callback(event);
		};
	}

	return createKeybindingsHandler(keys);
}
