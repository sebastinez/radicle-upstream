<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import { createEventDispatcher } from "svelte";

  import { PeerRole } from "ui/src/project";
  import type { User } from "ui/src/project";

  import { Icon, Overlay, Tooltip } from "ui/DesignSystem";

  import Peer from "./PeerSelector/Peer.svelte";

  export let expanded: boolean = false;
  // If `true`,  this component is used in a stand-alone context. This means it
  // has all corners rounded and it acts like a push button.
  // If `false`, this component is used together with the "Edit Remotes" button
  // on the right-hand side and doesn't animate when clicked.
  export let standalone: boolean = false;
  export let peers: User[];
  export let selected: User;
  export let showProfile: boolean = true;
  let dropdownHeight: number;

  const orderPeers = (peers: User[]): User[] => {
    return [selected].concat(
      peers.filter(peer => peer.peerId !== selected.peerId)
    );
  };

  const hide = () => {
    expanded = false;
  };
  const show = () => {
    expanded = true;
  };

  const dispatch = createEventDispatcher();
  const onOpen = (peer: User) => {
    hide();
    dispatch("open", peer);
  };
  const onSelect = (peer: User) => {
    if (peer.role === PeerRole.Tracker) {
      return;
    }
    hide();
    dispatch("select", peer);
  };
</script>

<style>
  .peer-selector {
    display: flex;
    border: 1px solid var(--color-foreground-level-3);
    border-right: none;
    border-top-left-radius: 0.5rem;
    border-bottom-left-radius: 0.5rem;
    padding: 0 0.5rem;
    align-items: center;
    height: 2.5rem;
    cursor: pointer;
    justify-content: space-between;
    background-color: var(--color-foreground-level-1);
    user-select: none;
  }

  .peer-selector:hover {
    background-color: var(--color-foreground-level-2);
  }

  .peer-selector[hidden] {
    visibility: hidden;
  }

  .selector-expand {
    margin-left: 0.5rem;
  }

  .peer-dropdown-container {
    display: flex;
    position: absolute;
    right: 0;
    top: -1px;
    user-select: none;
  }

  .peer-dropdown {
    border: 1px solid transparent;
    border-right: none;
    border-top-left-radius: 0.5rem;
    border-bottom-left-radius: 0.5rem;
    box-shadow: var(--elevation-medium);
    z-index: 8;
    max-width: 30rem;
    height: 100%;
    min-width: 100%;
    overflow: hidden;
  }

  .action {
    height: 1.5rem;
    margin-left: 0.5rem;
    width: 1.5rem;
  }

  .open-profile {
    cursor: pointer;
    display: flex;
    justify-content: center;
  }

  .rounded {
    border-top-right-radius: 0.5rem;
    border-bottom-right-radius: 0.5rem !important;
    border-top-left-radius: 0.5rem;
    border-bottom-left-radius: 0.5rem;
    border: 1px solid var(--color-foreground-level-3);
  }

  .entry {
    align-items: center;
    background-color: var(--color-background);
    color: var(--color-foreground-level-3);
    cursor: not-allowed;
    display: flex;
    height: 2.5rem;
    justify-content: space-between;
    padding: 0 0.5em;
  }

  .entry.enabled {
    color: var(--color-foreground-level-6);
  }
  .entry.enabled:hover {
    background-color: var(--color-foreground-level-2);
    cursor: pointer;
  }

  .entry.selected {
    background-color: var(--color-foreground-level-2);
  }
</style>

<Overlay
  {expanded}
  on:hide={hide}
  style="position: relative; user-select: none;">
  <div
    class="peer-selector typo-overflow-ellipsis"
    class:button-transition={standalone}
    class:rounded={standalone}
    data-cy="peer-selector"
    hidden={expanded}
    on:click|stopPropagation={show}>
    <Peer peer={selected} />
    <div class="selector-expand">
      <Icon.ChevronUpDown
        style="vertical-align: bottom; fill: var(--color-foreground-level-4)" />
    </div>
  </div>
  <div class="peer-dropdown-container" data-cy="peer-dropdown-container">
    <div
      bind:clientHeight={dropdownHeight}
      class="peer-dropdown"
      hidden={!expanded}
      class:rounded={standalone}
      style={`border-bottom-right-radius: ${
        dropdownHeight > 40 ? "0.5rem" : "0"
      }`}>
      {#each orderPeers(peers) as peer (peer.peerId)}
        <div
          data-cy="peer-dropdown-entry"
          class="entry"
          class:enabled={peer.role !== PeerRole.Tracker}
          class:selected={peer.identity.peerId === selected.identity.peerId}
          on:click|stopPropagation={() => onSelect(peer)}>
          {#if peer.role === PeerRole.Tracker}
            <Tooltip position="left" value="Remote has no changes">
              <Peer {peer} />
            </Tooltip>
          {:else}
            <Peer {peer} />
          {/if}

          <div class="action">
            {#if showProfile}
              <Tooltip value="Go to profile" position="top">
                <div
                  class="open-profile"
                  data-cy={`open-profile-${peer.identity.metadata.handle}`}
                  on:click|stopPropagation={() => onOpen(peer)}>
                  <Icon.ArrowBoxUpRight />
                </div>
              </Tooltip>
            {/if}
          </div>
        </div>
      {/each}
    </div>
  </div>
</Overlay>
