<script lang="ts">
import { goto } from "$app/navigation";
import { page } from "$app/stores";
import { appBrand, navItems, sidebarProfile } from "$lib/data/navigation";
</script>

<aside class="sidebar">
  <button type="button" class="brand" on:click={() => goto('/')}>
    <strong>{appBrand.name}</strong>
    <span>{appBrand.version}</span>
  </button>

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
    height: 100vh;
    padding: var(--space-6) 0 var(--space-4);
    display: flex;
    flex-direction: column;
    justify-content: space-between;
    background: var(--app-sidebar);
    border-right: 1px solid var(--surface-border-soft);
    backdrop-filter: blur(var(--blur-lg));
    overflow: hidden;
  }

  .sidebar::before {
    content: '';
    position: absolute;
    inset: -30% -20% auto;
    height: 60%;
    background: radial-gradient(circle, var(--surface-hover) 0%, transparent 60%);
    filter: blur(38px);
    opacity: 0.65;
    pointer-events: none;
  }

  .brand,
  nav,
  .profile {
    position: relative;
    z-index: 1;
  }

  .brand {
    border: 0;
    background: transparent;
    padding: 0 var(--space-6);
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
    background: var(--interactive-primary-bg);
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
    border-radius: var(--radius-pill);
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
    border-radius: var(--radius-pill);
    background: var(--color-success-1);
    box-shadow: 0 0 0.5rem var(--color-success-1);
  }

  @media (max-width: 920px) {
    .sidebar {
      position: static;
      height: auto;
      gap: 1rem;
      padding-bottom: 1.25rem;
    }

    nav {
      grid-template-columns: repeat(4, minmax(0, 1fr));
      padding: 0 var(--space-2);
    }

    nav button {
      text-align: center;
      padding: var(--space-3) var(--space-2);
      border-radius: var(--radius-sm);
    }
  }
</style>
