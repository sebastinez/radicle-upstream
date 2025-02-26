<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import type * as Project from "ui/src/project";

  import * as router from "ui/src/router";
  import * as Session from "ui/src/session";
  import * as org from "ui/src/org";

  import EmptyState from "ui/App/ScreenLayout/EmptyState.svelte";
  import ProjectList from "ui/App/ProfileScreen/ProjectList.svelte";
  import UnresolvedAnchorList from "./UnresolvedAnchorList.svelte";

  const session = Session.unsealed();

  export let address: string;
  export let anchors: org.OrgAnchors;
  export let ownerAddress: string;
  export let disableAnchorCreation = false;
  export let isMultiSig: boolean;

  function isWaitingForExecution(anchors: org.OrgAnchors): boolean {
    if (anchors.pendingResolved.length > 0) {
      const anchor = anchors.pendingResolved[0].anchor;
      if (
        anchor &&
        anchor.type === "pending" &&
        anchor.confirmations === anchor.threshold
      ) {
        return true;
      }
    }

    if (anchors.pendingUnresolved.length > 0) {
      const anchor = anchors.pendingUnresolved[0];
      if (
        anchor.type === "pending" &&
        anchor.confirmations === anchor.threshold
      ) {
        return true;
      }
    }

    return false;
  }

  const select = ({ detail: project }: { detail: Project.Project }) => {
    router.push({
      type: "project",
      activeView: { type: "files" },
      urn: project.urn,
    });
  };
</script>

<style>
  .container {
    margin: 0 auto;
    max-width: var(--content-max-width);
    min-width: var(--content-min-width);
  }

  .pending {
    margin-bottom: 2rem;
  }

  .header {
    display: flex;
    padding: 0.5rem 3rem 0.5rem;
    width: 100%;
  }
</style>

<div class="container">
  {#if anchors.pendingResolved.length !== 0 || anchors.pendingUnresolved.length !== 0}
    <div class="pending">
      <div class="header">
        <p class="typo-text-bold">Pending</p>
        <p style="margin-left: .5rem; color: var(--color-foreground-level-6);">
          {#if isWaitingForExecution(anchors)}
            Waiting for a member to execute this anchor transaction.
            <a
              class="typo-link"
              href={org.gnosisSafeWebAppUrl(ownerAddress, "transactions")}
              >Execute transaction</a>
          {:else}
            Not enough members have confirmed this anchor transaction.
            <a
              class="typo-link"
              href={org.gnosisSafeWebAppUrl(ownerAddress, "transactions")}
              >Confirm transaction</a>
          {/if}
        </p>
      </div>
      <ProjectList
        projects={anchors.pendingResolved}
        userUrn={session.identity.urn}
        on:select={select} />
      <UnresolvedAnchorList anchors={anchors.pendingUnresolved} />
    </div>
  {/if}

  {#if anchors.confirmedResolved.length !== 0}
    {#if anchors.pendingResolved.length !== 0 || anchors.pendingUnresolved.length !== 0}
      <div class="header">
        <p class="typo-text-bold">Confirmed</p>
        <p style="margin-left: .5rem; color: var(--color-foreground-level-6);">
          These projects have been anchored in this org.
        </p>
      </div>
    {/if}
    <ProjectList
      projects={anchors.confirmedResolved}
      userUrn={session.identity.urn}
      on:select={select} />
  {/if}

  {#if anchors.confirmedUnresolved.length !== 0}
    <div class="header">
      <p style="color: var(--color-foreground-level-6);">
        These anchored projects haven't been found in your network yet, try
        following them.
      </p>
    </div>
    <UnresolvedAnchorList anchors={anchors.confirmedUnresolved} />
  {/if}

  {#if anchors.pendingResolved.length === 0 && anchors.confirmedResolved.length === 0 && anchors.pendingUnresolved.length === 0 && anchors.confirmedUnresolved.length === 0}
    <EmptyState
      emoji="🪴"
      text="Get started by anchoring your org’s first project."
      primaryActionText={isMultiSig
        ? "Anchor with Gnosis Safe"
        : "Anchor project"}
      primaryActionDisabled={disableAnchorCreation}
      primaryActionTooltipMessage="Create or follow a project first"
      on:primaryAction={() => {
        org.openAnchorProjectModal(address, ownerAddress, isMultiSig);
      }} />
  {/if}
</div>
