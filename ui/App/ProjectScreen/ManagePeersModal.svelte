<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import type { PeerId } from "ui/src/identity";
  import { PeerType, PeerRole } from "ui/src/project";
  import type { User } from "ui/src/project";
  import {
    addPeer,
    pendingPeers,
    peerValidation,
    removePeer,
    store,
  } from "ui/src/screen/project";

  import * as userProfile from "ui/src/userProfile";
  import * as modal from "ui/src/modal";

  import { Button, List, TextInput } from "ui/DesignSystem";

  import Modal from "ui/App/ModalLayout/Modal.svelte";
  import Remote from "ui/App/Remote.svelte";
  import Peer from "./ManagePeers/Peer.svelte";
  import PeerFollowRequest from "./ManagePeers/PeerFollowRequest.svelte";

  let newPeer: PeerId;

  $: if (newPeer === "") {
    peerValidation.reset();
  }

  const submitPeer = async (projectUrn: string) => {
    if (await addPeer(projectUrn, newPeer)) {
      newPeer = "";
    }
  };

  const cancelFollowRequest = (projectUrn: string, peerId: PeerId) => {
    removePeer(projectUrn, peerId);
    peerValidation.reset();
  };

  const unfollowPeer = (projectUrn: string, peerId: PeerId) => {
    removePeer(projectUrn, peerId);
    peerValidation.reset();
  };

  // Don't show our own peer in the list unless we have published something.
  const filteredPeers = (peers: [User]) => {
    return peers.filter(peer => {
      return !(peer.type === PeerType.Local && peer.role === PeerRole.Tracker);
    });
  };
</script>

<style>
  .peer-entry-form {
    flex: 1;
    display: flex;
    flex-direction: column;
    align-items: flex-start;
    width: 100%;
  }

  .peer-entry-field {
    display: flex;
    justify-content: flex-start;
    margin-top: 2rem;
    width: 100%;
  }
</style>

<Remote {store} let:data={{ peerSelection, project }}>
  <Modal dataCy="remotes-modal" emoji="💻" title="Edit remotes">
    <svelte:fragment slot="description">
      Add a user’s Device ID to collaborate with them on this project.
    </svelte:fragment>

    <form class="peer-entry-form" on:submit|preventDefault>
      <div class="peer-entry-field">
        <TextInput
          dataCy="peer-input"
          bind:value={newPeer}
          placeholder="Enter a Device ID here"
          validation={$peerValidation}
          style="width: 100%; margin-right: .5rem;" />
        <Button
          dataCy="follow-button"
          style="display: flex; align-self: flex-start;"
          disabled={!newPeer}
          on:click={() => submitPeer(project.urn)}>
          Add
        </Button>
      </div>
    </form>

    <List
      dataCy="followed-peers"
      key="peerId"
      items={filteredPeers(peerSelection)}
      let:item={peer}
      styleHoverState={false}
      style="width: 100%; margin: 1.5rem 0 0; padding: 0;">
      <Peer
        {peer}
        on:userProfileClick={event => {
          userProfile.openUserProfile(event.detail.urn);
          modal.hide();
        }}
        on:unfollow={event => {
          unfollowPeer(event.detail.projectUrn, event.detail.peerId);
        }}
        projectUrn={project.urn} />
    </List>

    <Remote store={pendingPeers} let:data>
      {#if data.peers.length > 0}
        <div style="display: flex; width: 100%; margin-top: 1.5rem;">
          <p class="typo-text-bold">Still looking…</p>
          <p
            class="typo-text"
            style="margin-left: 0.5rem; color: var(--color-foreground-level-6);">
            These remotes haven’t been found yet.
          </p>
        </div>
      {/if}

      <List
        dataCy="pending-peers"
        key="peerId"
        items={data.peers}
        let:item={peer}
        styleHoverState={false}
        style="width: 100%; margin: 1rem 0 0; padding: 0;">
        <PeerFollowRequest
          {peer}
          on:cancel={event => {
            cancelFollowRequest(event.detail.projectUrn, event.detail.peerId);
          }}
          projectUrn={project.urn} />
      </List>
    </Remote>
  </Modal>
</Remote>
