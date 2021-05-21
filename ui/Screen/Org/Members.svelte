<script>
  import { orgMemberTabStore } from "ui/src/org";

  import { Avatar, Icon } from "ui/DesignSystem/Primitive";
  import { List, StyledCopyable, PeerId } from "ui/DesignSystem/Component";

  // TODO(rudolfs): make the link go to
  // `https://gnosis-safe.io/app/#/safes/${$orgMemberTabStore.gnosisSafeAddress}` for
  // mainnet
</script>

<style>
  .container {
    margin: 0 auto;
    max-width: var(--content-max-width);
    min-width: var(--content-min-width);
  }

  .list-item {
    display: flex;
    width: 100%;
    justify-content: space-between;
    padding: 1rem;
    align-items: center;
    min-width: 0;
  }

  .metadata-container {
    min-width: var(--content-min-width);
    max-width: var(--content-max-width);
    padding: 0 var(--content-padding);
    margin-bottom: 1.5rem;
    display: flex;
  }

  .metadata {
    width: 100%;
    display: flex;
    background-color: var(--color-foreground-level-1);
    border-radius: 0.5rem;
    height: 4.5rem;
    align-items: center;
    color: var(--color-foreground-level-6);
    justify-content: space-between;
  }

  .left {
    display: flex;
    align-items: center;
  }

  .address {
    margin-right: 24px;
  }

  .address a {
    color: var(--color-foreground-level-6);
  }

  .member-details {
    display: flex;
    flex-direction: column;
    align-self: center;
    width: -webkit-fill-available;
    min-width: 0;
  }
</style>

<div class="container">
  <div class="metadata-container">
    <div class="metadata">
      <span class="left typo-text">
        <Icon.Gnosis style="margin: 0 10px 0 27px;" />
        Managed by Gnosis Safe • Quorum
        {$orgMemberTabStore.threshold}/{$orgMemberTabStore.members.length}
      </span>
      <span class="address">
        <a
          href={`https://rinkeby.gnosis-safe.io/app/#/safes/${$orgMemberTabStore.gnosisSafeAddress}`}
          class="typo-link">
          {$orgMemberTabStore.gnosisSafeAddress}
        </a>↗</span>
    </div>
  </div>

  <List
    items={$orgMemberTabStore.members}
    let:item={member}
    styleHoverState={false}>
    <div class="list-item">
      {#if member.identity}
        <div style="display: flex">
          <Avatar
            style="margin-right: 32px"
            size="big"
            variant="circle"
            avatarFallback={member.identity.avatarFallback} />
          <div class="member-details">
            <h2
              data-cy="entity-name"
              class="typo-overflow-ellipsis"
              title={member.identity.metadata.handle}>
              {member.identity.metadata.handle}
            </h2>
            <PeerId
              truncate
              peerId={member.identity.peerId}
              style="margin-top: 0.5rem;" />
          </div>
        </div>
      {:else}
        <Avatar
          style="margin-right: 32px"
          size="big"
          variant="circle"
          avatarFallback={{ background: { r: 0, g: 0, b: 0 }, emoji: "❔" }} />
        <div class="member-details">
          <h2
            data-cy="entity-name"
            class="typo-overflow-ellipsis"
            title="Unknown identity">
            Unknown identity
          </h2>
        </div>
      {/if}
      <StyledCopyable truncate value={member.ethereumAddress} />
    </div>
  </List>
</div>
