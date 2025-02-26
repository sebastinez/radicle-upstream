<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import { fade } from "svelte/transition";
  import type { SvelteComponent } from "svelte";

  import Icon from "./Icon";
  import Overlay from "./Overlay.svelte";
  import Tooltip from "./Tooltip.svelte";
  import Copyable from "./Copyable.svelte";

  interface MenuItem {
    title: string;
    icon: typeof SvelteComponent;
    event: () => void;
    tooltip?: string;
    dataCy?: string;
    disabled?: boolean;
  }
  export let menuItems: MenuItem[];
  export let headerTitle: string | undefined = undefined;

  export let dataCy: string | undefined = undefined;
  export let style: string | undefined = undefined;

  let triggerEl: HTMLButtonElement;
  let expanded = false;

  const toggleModal = () => {
    expanded = !expanded;
  };

  const hideModal = () => {
    expanded = false;
  };

  const handleItemSelection = (item: MenuItem) => {
    hideModal();
    item.event();
  };
</script>

<style>
  .container {
    position: relative;
    height: 40px;
    width: 40px;
  }

  .additional-actions-dropdown-button {
    height: 100%;
    width: 100%;
    display: flex;
    justify-content: center;
    align-items: center;
    border-radius: 0.5rem;
    cursor: pointer;
    outline-style: none;
    border: 1px solid var(--color-foreground-level-3);
  }

  .additional-actions-dropdown-button :global(svg) {
    fill: var(--color-foreground-level-6);
  }

  .additional-actions-dropdown-button:active :global(svg) {
    fill: var(--color-foreground-level-5);
  }

  .additional-actions-dropdown-button:hover {
    background-color: var(--color-foreground-level-2);
  }

  .modal {
    position: absolute;
    top: 100%;
    right: 0;
    width: 240px;
    margin-top: 15px;
    background-color: var(--color-background);
    box-shadow: var(--elevation-medium);
    border-radius: 0.5rem;
    cursor: pointer;
    border: 1px solid var(--color-foreground-level-3);
    overflow: hidden; /* hack to make inner option rounded corners */
    z-index: 10;
    user-select: none;
  }

  .header {
    padding: 12px 16px;
    color: var(--color-foreground-level-5);
    display: flex;
    justify-content: space-between;
    border-bottom: solid 1px var(--color-foreground-level-3);
  }

  .header:hover {
    color: var(--color-foreground-level-6);
  }

  .menu {
    cursor: pointer;
  }

  .menu-item {
    display: flex;
    padding: 8px 12px;
    color: var(--color-foreground-level-6);
  }

  .menu-item:hover {
    background-color: var(--color-foreground-level-1);
  }

  .menu-item.disabled {
    color: var(--color-foreground-level-4);
    cursor: not-allowed;
  }

  .menu-item.disabled :global(svg) {
    fill: var(--color-foreground-level-4);
  }
</style>

<Overlay {expanded} on:hide={hideModal}>
  <div data-cy={dataCy} class="container" {style}>
    <button
      class="additional-actions-dropdown-button button-transition"
      bind:this={triggerEl}
      on:click|stopPropagation={toggleModal}>
      <svelte:component this={Icon.Ellipsis} />
    </button>
    {#if expanded}
      <div out:fade={{ duration: 100 }} class="modal" hidden={!expanded}>
        {#if headerTitle}
          <div class="header">
            <Copyable name={headerTitle}>
              {headerTitle}
            </Copyable>
          </div>
        {/if}

        {#if menuItems}
          <div class="menu" data-cy="dropdown-menu">
            {#each menuItems as item}
              {#if item !== undefined}
                <Tooltip value={item.tooltip} position="left">
                  <div
                    data-cy={item.dataCy}
                    class="menu-item"
                    class:disabled={item.disabled}
                    on:click={!item.disabled
                      ? () => handleItemSelection(item)
                      : undefined}>
                    <svelte:component
                      this={item.icon}
                      style="margin-right: 12px" />
                    <p>{item.title}</p>
                  </div>
                </Tooltip>
              {/if}
            {/each}
          </div>
        {/if}
      </div>
    {/if}
  </div>
</Overlay>
