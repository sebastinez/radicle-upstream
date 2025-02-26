<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import type { Registration } from "ui/src/org/ensResolver";

  import * as ipc from "ui/src/ipc";
  import * as router from "ui/src/router";
  import * as org from "ui/src/org";

  import { FollowToggle, Icon, ThreeDotsMenu } from "ui/DesignSystem";

  import ScreenLayout from "ui/App/ScreenLayout.svelte";
  import ActionBar from "ui/App/ScreenLayout/ActionBar.svelte";
  import Header from "ui/App/ScreenLayout/Header.svelte";
  import TabBar from "ui/App/ScreenLayout/TabBar.svelte";

  import ProjectsTab from "ui/App/OrgScreen/Projects.svelte";
  import OrgHeader from "ui/App/OrgScreen/OrgHeader.svelte";
  import ProjectsMenu from "ui/App/OrgScreen/ProjectsMenu.svelte";

  export let owner: string;
  export let address: string;
  export let projectCount: number;
  export let anchors: org.OrgAnchors;
  export let registration: Registration | undefined = undefined;

  const tabs = (address: string) => {
    return [
      {
        title: "Anchored projects",
        icon: Icon.ChevronLeftRight,
        active: true,
        onClick: () => {
          router.push({
            type: "org",
            params: { address, view: "projects" },
          });
        },
      },
    ];
  };

  const menuItems = (address: string) => {
    return [
      {
        title: "View on Etherscan",
        icon: Icon.At,
        event: () => {
          org.openOnEtherscan(address);
        },
      },
      {
        title: "View in browser",
        icon: Icon.Globe,
        event: () => {
          ipc.openUrl(`https://app.radicle.network/orgs/${address}`);
        },
      },
      {
        title: registration ? "Edit ENS name" : "Register ENS name",
        icon: Icon.Ethereum,
        event: () => org.openEnsConfiguration(address, registration),
      },
    ];
  };
</script>

<ScreenLayout>
  <Header>
    <OrgHeader
      {registration}
      slot="left"
      orgAddress={address}
      ownerAddress={owner} />
    <div slot="right" style="display: flex">
      <FollowToggle following disabled style="margin-right: 1rem;" />
      <ThreeDotsMenu menuItems={menuItems(address)} />
    </div>
  </Header>

  <ActionBar>
    <div slot="left">
      <TabBar tabs={tabs(address)} />
    </div>
    <div slot="right">
      <ProjectsMenu
        isMultiSig={false}
        orgAddress={address}
        gnosisSafeAddress={owner}
        availableProjectCount={projectCount}
        hasPendingAnchors={anchors.pendingResolved.length !== 0 ||
          anchors.pendingUnresolved.length !== 0} />
    </div>
  </ActionBar>

  <ProjectsTab
    isMultiSig={false}
    {address}
    ownerAddress={owner}
    disableAnchorCreation={projectCount === 0}
    {anchors} />
</ScreenLayout>
