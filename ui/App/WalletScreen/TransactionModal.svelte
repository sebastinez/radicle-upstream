<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import type { Tx } from "ui/src/transaction";

  import dayjs from "dayjs";

  import { TxKind, store as txs } from "ui/src/transaction";
  import * as error from "ui/src/error";
  import * as modal from "ui/src/modal";
  import * as org from "ui/src/org";

  import { IdentifierLink } from "ui/DesignSystem";
  import Modal from "ui/App/ModalLayout/Modal.svelte";

  export let transactionHash: string;

  let tx: Tx;
  $: {
    const found = $txs.find(tx => tx.hash === transactionHash);
    if (!found) {
      modal.hide();
      throw new error.Error({
        message: "Failed to find transaction",
        details: { transactionHash },
      });
    }
    tx = found;
  }

  function emoji(tx: Tx): string {
    switch (tx.kind) {
      case TxKind.AnchorProject:
        return "🏖️";
      case TxKind.CreateOrg:
        return "🎪";
      case TxKind.ClaimRadicleIdentity:
        return "🧦";
      case TxKind.CommitEnsName:
      case TxKind.RegisterEnsName:
        return "📇";
      case TxKind.UpdateEnsMetadata:
        return "📋";
      case TxKind.LinkEnsNameToOrg:
        return "🔗";
    }
  }
</script>

<style>
  .section {
    display: flex;
    flex-direction: column;
    justify-content: space-between;

    margin-bottom: 1.5rem;
    padding: 1rem;
    border: 1px solid var(--color-foreground-level-2);
    box-sizing: border-box;
    border-radius: 0.5rem;
  }

  .section:last-child {
    margin-bottom: 0;
  }

  p {
    color: var(--color-foreground-level-6);
  }

  .row {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding-bottom: 0.375rem;
  }
  .row:last-child {
    padding-bottom: 0;
  }
</style>

<Modal emoji={emoji(tx)} title={tx.kind} dataCy="transaction-summary">
  <div class="content">
    <div class="section">
      <div class="row">
        <p>Transaction ID</p>
        <IdentifierLink
          params={{
            type: "transactionHash",
            url: org.etherscanUrl(tx.hash),
            hash: tx.hash,
          }} />
      </div>
      <div class="row">
        <p>Status</p>
        <div class="row" data-cy="transaction-status">
          <p>{tx.status}</p>
        </div>
      </div>
      <div class="row">
        <p>Timestamp</p>
        <p>
          {dayjs(tx.date).format("HH:mm:ss [on] D MMMM YYYY")}
        </p>
      </div>
    </div>
  </div>
</Modal>
