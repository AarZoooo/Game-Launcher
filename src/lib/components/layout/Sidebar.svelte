<script lang="ts">
import { goto } from "$app/navigation";
import { page } from "$app/stores";
import { appIcons } from "$lib/assets";
import Icon from "$lib/components/common/Icon.svelte";
import { pageLabels } from "$lib/data/labels";
import { appBrand, navItems, sidebarProfile } from "$lib/data/navigation";
import { games } from "$lib/stores/libraryStore";
import { libraryBusy } from "$lib/stores/uiStore";

async function refreshInstalledMedia() {
	try {
		await games.refreshInstalledGameMedia();
	} catch (error) {
		console.error("Failed to refresh installed game media:", error);
	}
}
</script>

<aside class="sidebar">
  <div class="brand-row">
    <button type="button" class="brand" on:click={() => goto('/')}>
      <strong>{appBrand.name}</strong>
      <span>{appBrand.version}</span>
    </button>

    <button
      type="button"
      class="menu-trigger refresh-button"
      aria-label={pageLabels.common.refreshInstalledMedia}
      title={pageLabels.common.refreshInstalledMedia}
      disabled={$libraryBusy}
      on:click={refreshInstalledMedia}
    >
      <span class:spinning={$libraryBusy} class="refresh-icon-shell">
        <Icon src={appIcons.ui.refresh} size="0.95rem" />
      </span>
    </button>
  </div>

  <nav>
    {#each navItems as item}
      <button
        class:active={$page.url.pathname === item.path}
        on:click={() => goto(item.path)}
      >
        {item.label}
      </button>
    {/each}
  </nav>

  <div class="profile">
    <div class="avatar">{sidebarProfile.initial}</div>
    <div>
      <strong>{sidebarProfile.name}</strong>
      <span><i></i>{sidebarProfile.status}</span>
    </div>
  </div>
</aside>

<style>
  .sidebar {
    position: sticky;
    top: 0;
    z-index: 8;
    height: 100vh;
    padding: var(--space-6) 0 var(--space-4);
    display: flex;
    flex-direction: column;
    justify-content: space-between;
    background: var(--app-sidebar);
    border-right: 1px solid var(--surface-border-soft);
    backdrop-filter: blur(calc(var(--blur-lg) * 1.15)) saturate(140%);
    box-shadow:
      inset -1px 0 0 rgb(255 255 255 / 0.05),
      1rem 0 2.8rem rgb(0 0 0 / 0.18);
    overflow: hidden;
  }

  .sidebar::before {
    content: '';
    position: absolute;
    inset: -30% -20% auto;
    height: 60%;
    background: radial-gradient(circle, var(--surface-hover) 0%, transparent 60%);
    filter: blur(var(--blur-xl));
    opacity: 0.65;
    pointer-events: none;
  }

  .sidebar::after {
    content: '';
    position: absolute;
    inset: 0;
    background:
      linear-gradient(180deg, rgb(255 255 255 / 0.08), transparent 24%),
      linear-gradient(90deg, rgb(255 255 255 / 0.06), transparent 24%);
    opacity: 0.55;
    pointer-events: none;
  }

  .brand-row,
  .brand,
  nav,
  .profile {
    position: relative;
    z-index: 1;
  }

  .brand-row {
    display: flex;
    align-items: flex-start;
    justify-content: space-between;
    gap: var(--space-3);
    padding: 0 var(--space-6);
  }

  .brand {
    border: 0;
    background: transparent;
    padding: 0;
    text-align: left;
    cursor: pointer;
  }

  .brand strong {
    display: block;
    font: 700 2rem/1 var(--font-display);
  }

  .brand span {
    color: var(--text-muted);
    font-size: 0.72rem;
  }

  .refresh-button {
    margin-top: 0.1rem;
    opacity: 0.82;
    transform: scale(1);
    flex: 0 0 auto;
  }

  .refresh-button:disabled {
    cursor: default;
    opacity: 0.52;
    transform: scale(1);
  }

  .refresh-icon-shell {
    display: inline-grid;
    place-items: center;
  }

  .refresh-icon-shell.spinning {
    animation: spin 900ms linear infinite;
  }

  nav {
    display: grid;
    gap: var(--space-1);
  }

  nav button {
    width: 100%;
    border: 0;
    background: transparent;
    color: var(--text-secondary);
    font: inherit;
    text-align: left;
    padding: var(--space-4) calc(var(--space-6) + var(--space-1));
    cursor: pointer;
    transition:
      background-color var(--motion-fast) ease,
      color var(--motion-fast) ease,
      transform var(--motion-fast) ease;
  }

  nav button:hover {
    color: var(--text-primary);
    background: var(--surface-hover);
    transform: translateX(var(--space-1));
  }

  nav button.active {
    color: var(--interactive-primary-text);
    background:
      linear-gradient(90deg, rgb(var(--accent-rgb) / 0.78), rgb(var(--accent-rgb) / 0.58)),
      var(--surface-glass);
    font-weight: 700;
    box-shadow: var(--shadow-sm);
  }

  .profile {
    display: flex;
    align-items: center;
    gap: var(--space-3);
    padding: 0 var(--space-6);
  }

  .avatar {
    display: grid;
    place-items: center;
    width: 2rem;
    height: 2rem;
    border-radius: var(--radius-round);
    background: linear-gradient(
      135deg,
      rgb(var(--accent-rgb) / 0.9),
      rgb(var(--accent-rgb) / 0.52)
    );
    color: var(--accent-contrast);
    font-size: 0.85rem;
    font-weight: 800;
  }

  .profile strong {
    display: block;
    font-size: 0.95rem;
  }

  .profile span {
    display: flex;
    align-items: center;
    gap: var(--space-2);
    font-size: 0.72rem;
    color: var(--text-secondary);
  }

  .profile i {
    width: 0.42rem;
    height: 0.42rem;
    border-radius: var(--radius-round);
    background: var(--color-success-1);
    box-shadow: var(--shadow-status-glow-sm);
  }

  @media (max-width: 920px) {
    .sidebar {
      position: static;
      height: auto;
      display: grid;
      grid-template-columns: minmax(0, 1fr) auto;
      grid-template-areas:
        "brand profile"
        "nav nav";
      align-items: start;
      gap: var(--space-4);
      padding: var(--space-4) 0 1.25rem;
    }

    .brand {
      padding: 0;
    }

    .brand-row {
      grid-area: brand;
      padding: 0 var(--space-4);
    }

    nav {
      grid-area: nav;
      grid-template-columns: repeat(4, minmax(0, 1fr));
      padding: 0 var(--space-2);
    }

    nav button {
      text-align: center;
      padding: var(--space-3) var(--space-2);
      border-radius: var(--radius-control-sm);
    }

    .profile {
      grid-area: profile;
      justify-self: end;
      align-self: start;
      padding: 0 var(--space-4);
      gap: var(--space-2);
    }

    .profile strong,
    .profile span {
      white-space: nowrap;
    }
  }

  @keyframes spin {
    from {
      transform: rotate(0deg);
    }

    to {
      transform: rotate(360deg);
    }
  }
</style>
