<script lang="ts">
import { goto } from "$app/navigation";
import { page } from "$app/stores";
import type { AccentTone } from "$lib/stores/libraryStore";
import { navItems } from "$lib/utils/constants";

export let accent: AccentTone = "silver";
</script>

<aside class={`sidebar ${accent}`}>
  <div class="brand">
    <strong>Scoped</strong>
    <span>v 0.1.1</span>
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
    <div class="avatar">N</div>
    <div>
      <strong>NezukoChan</strong>
      <span><i></i>Online</span>
    </div>
  </div>
</aside>

<style>
  .sidebar {
    position: sticky;
    top: 0;
    height: 100vh;
    padding: 1.35rem 0 1rem;
    display: flex;
    flex-direction: column;
    justify-content: space-between;
    background:
      linear-gradient(180deg, rgba(255, 255, 255, 0.1), rgba(255, 255, 255, 0.02)),
      rgba(85, 86, 94, 0.72);
    border-right: 1px solid rgba(255, 255, 255, 0.06);
    backdrop-filter: blur(18px);
    overflow: hidden;
  }

  .sidebar::before {
    content: '';
    position: absolute;
    inset: -30% -20% auto;
    height: 60%;
    background: radial-gradient(circle, rgba(255, 255, 255, 0.12) 0%, transparent 60%);
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
    padding: 0 1.5rem;
  }

  .brand strong {
    display: block;
    font: 700 2rem/1 'Bahnschrift', 'Segoe UI Variable Text', sans-serif;
  }

  .brand span {
    color: rgba(234, 231, 239, 0.42);
    font-size: 0.72rem;
  }

  nav {
    display: grid;
    gap: 0.3rem;
  }

  nav button {
    width: 100%;
    border: 0;
    background: transparent;
    color: rgba(238, 236, 242, 0.56);
    font: inherit;
    text-align: left;
    padding: 0.9rem 1.7rem;
    cursor: pointer;
    transition: 180ms ease;
  }

  nav button:hover {
    color: #fbf9fe;
    background: rgba(255, 255, 255, 0.04);
  }

  nav button.active {
    color: #282a30;
    background: var(--page-accent);
    font-weight: 700;
  }

  .profile {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 0 1.5rem;
  }

  .avatar {
    display: grid;
    place-items: center;
    width: 2rem;
    height: 2rem;
    border-radius: 999px;
    background: linear-gradient(135deg, #f0b9c1, #f3d899);
    color: #292b31;
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
    gap: 0.35rem;
    font-size: 0.72rem;
    color: rgba(230, 226, 236, 0.56);
  }

  .profile i {
    width: 0.42rem;
    height: 0.42rem;
    border-radius: 999px;
    background: #80d967;
    box-shadow: 0 0 0.5rem rgba(128, 217, 103, 0.8);
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
      padding: 0 0.5rem;
    }

    nav button {
      text-align: center;
      padding: 0.75rem 0.5rem;
      border-radius: 0.7rem;
    }
  }
</style>
