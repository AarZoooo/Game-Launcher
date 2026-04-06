import type { Action } from "svelte/action";

type PortalTarget = HTMLElement | string | null | undefined;

function resolveTarget(target: PortalTarget) {
	if (typeof document === "undefined") {
		return null;
	}

	if (target instanceof HTMLElement) {
		return target;
	}

	if (typeof target === "string" && target.trim() !== "") {
		return document.querySelector<HTMLElement>(target);
	}

	return document.body;
}

export const portal: Action<HTMLElement, PortalTarget> = (
	node,
	target = "#portal-root",
) => {
	let currentTarget = resolveTarget(target) || document.body;
	currentTarget.appendChild(node);

	return {
		update(nextTarget) {
			const nextResolved = resolveTarget(nextTarget) || document.body;
			if (nextResolved === currentTarget) {
				return;
			}

			currentTarget = nextResolved;
			currentTarget.appendChild(node);
		},
		destroy() {
			node.remove();
		},
	};
};
