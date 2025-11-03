/**
 * Toast Notification Store
 * Simple toast notification system for user feedback
 */

import { writable } from 'svelte/store';

export type ToastType = 'success' | 'error' | 'info' | 'warning';

export interface Toast {
	id: string;
	type: ToastType;
	message: string;
	duration?: number;
}

function createToastStore() {
	const { subscribe, update } = writable<Toast[]>([]);

	return {
		subscribe,
		show: (type: ToastType, message: string, duration: number = 4000) => {
			const id = Math.random().toString(36).substring(7);
			const toast: Toast = { id, type, message, duration };

			update(toasts => [...toasts, toast]);

			// Auto-remove after duration
			if (duration > 0) {
				setTimeout(() => {
					update(toasts => toasts.filter(t => t.id !== id));
				}, duration);
			}

			return id;
		},
		remove: (id: string) => {
			update(toasts => toasts.filter(t => t.id !== id));
		},
		clear: () => {
			update(() => []);
		}
	};
}

export const toast = createToastStore();
