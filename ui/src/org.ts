import * as svelteStore from "svelte/store";
import * as ethers from "ethers";
import EthersSafe, {
  SafeTransactionDataPartial,
} from "@gnosis.pm/safe-core-sdk";
import { push } from "svelte-spa-router";

import * as notification from "./notification";
import * as path from "./path";
import * as proxy from "./proxy";
import type * as identity from "./proxy/identity";
import * as wallet from "./wallet";
import * as theGraphApi from "./theGraphApi";
import * as ethereum from "ui/src/ethereum";
import * as error from "ui/src/error";
import * as proxy from "ui/src/proxy";
import * as urn from "ui/src/urn";
import type * as project from "ui/src/proxy/project";
import { claimsAddress, ClaimsContract } from "./attestation/contract";
import { identitySha1Urn } from "ui/src/urn";

import type {
  TransactionReceipt,
  TransactionResponse,
} from "@ethersproject/providers";

import * as transaction from "./transaction";

const orgFactoryAbi = [
  "function createOrg(address[], uint256) returns (address)",
  "event OrgCreated(address, address)",
];

const orgAbi = [
  "function owner() view returns (address)",
  "function anchor(bytes32, bytes32, uint8, uint8)",
];

const orgFactoryAddress = (network: ethereum.Environment): string => {
  switch (network) {
    case ethereum.Environment.Local:
      error.show(
        new error.Error({
          code: error.Code.FeatureNotAvailableForGivenNetwork,
          message: "Orgs not available on the Local testnet.",
        })
      );
      return "";
    case ethereum.Environment.Ropsten:
      return "0x2007bcEf1247CD03Bb4262eF420D6487368f473B";
    case ethereum.Environment.Rinkeby:
      return "0xe30aA5594FFB52B6bF5bbB21eB7e71Ac525bB028";
  }
};

export const anchorProject = async (): Promise<void> => {
  const orgAddress = "0xab58d6ce4c2fd470ddb87de62d90691f59bda6e9";
  const gnosisSafeAddress = "0xb962d6ff2438ffcfa569291bda7e9a0da6f46f2a";
  const projectUrn = "rad:git:hnrkbjokbt439jk3p1dsi67u3mca85yiy7fiy";
  const commitHash = "e02d760e86c2c3dff6ab0a3d98ada46451858620";

  const walletStore = svelteStore.get(wallet.store);
  const safeSdk = await EthersSafe.create(
    ethers,
    gnosisSafeAddress,
    walletStore.signer
  );

  console.log(safeSdk);

  const decodedProjectUrn = urn.parseIdentitySha1(projectUrn);
  const decodedCommitHash = ethers.utils.arrayify(`0x${commitHash}`);

  const paddedProjectUrn = ethers.utils.zeroPad(decodedProjectUrn, 32);
  const paddedCommitHash = ethers.utils.zeroPad(decodedCommitHash, 32);

  console.log(paddedProjectUrn);
  console.log(paddedCommitHash);

  const orgContract = new ethers.Contract(orgAddress, orgAbi);

  console.log("AAAAAAAAAA: ", orgContract);
  const orgContractInstance = await orgContract.populateTransaction.anchor(
    paddedProjectUrn,
    paddedCommitHash,
    ethers.constants.Zero,
    ethers.constants.Zero
  );

  console.log(orgContractInstance);

  const txData = orgContractInstance.data;
  if (!txData) {
    throw new Error("Could not generate transaction");
  }
  console.log("BBBBBBBB: ", txData);

  const partialTx: SafeTransactionDataPartial = {
    to: orgAddress,
    data: txData,
    value: "0",
  };

  // THIS WORKS IF THERE'S ONLY ONE OWNER IN THE SAFE
  console.log("PARTIAL: ", partialTx);
  const safeTransaction = await safeSdk.createTransaction(partialTx);
  const approveTxResponse = await safeSdk.executeTransaction(safeTransaction);
  console.log(approveTxResponse);

  // const safeTransaction = await safeSdk.createTransaction(partialTx);
  // const txHash = await safeSdk.getTransactionHash(safeTransaction);
  // const approveTxResponse = await safeSdk.approveTransactionHash(txHash);
  // await approveTxResponse.wait();
};

export const createOrg = async (owner: string): Promise<void> => {
  const walletStore = svelteStore.get(wallet.store);
  const orgFactory = new ethers.Contract(
    orgFactoryAddress(walletStore.environment),
    orgFactoryAbi,
    walletStore.signer
  );
  notification.info({
    message:
      "Waiting for you to confirm the transaction in your connected wallet",
    showIcon: true,
  });
  // WAITING
  const response: TransactionResponse = await orgFactory.createOrg([owner], 1);

  // PENDING
  notification.info({
    message: "Org creation transaction confirmed, your org will appear shortly",
    showIcon: true,
  });

  const receipt: TransactionReceipt =
    await walletStore.provider.waitForTransaction(response.hash);
  transaction.add(transaction.createOrg(response));

  const iface = new ethers.utils.Interface(orgFactoryAbi);

  let orgAddress: string = "";

  receipt.logs.forEach(log => {
    try {
      const parsed = iface.parseLog(log);

      if (parsed.name === "OrgCreated") {
        orgAddress = parsed.args[0].toLowerCase();
      }
    } catch {
      // Ignore parsing errors.
    }
  });

  if (!orgAddress) {
    throw new Error("Org not found in interface logs");
  }

  // SUCCESS
  notification.info({
    message: `Org ${orgAddress} has been created`,
    showIcon: true,
    actions: [
      {
        label: "Go to org",
        handler: () => {
          push(path.orgProjects(orgAddress));
        },
      },
    ],
  });
  await fetchOrgs();
};

const getGnosisSafeAddr = async (
  orgAddress: string,
  provider: ethers.providers.Provider
): Promise<string> => {
  const org = new ethers.Contract(orgAddress, orgAbi, provider);
  const safeAddr: string = await org.owner();

  return safeAddr.toLowerCase();
};

export type EthereumAddress = string;

interface OrgScreenStore {
  orgAddress: EthereumAddress;
  gnosisSafeAddress: EthereumAddress;
}

export const orgScreenStore = svelteStore.writable<OrgScreenStore | null>(null);

export const fetchOrg = async (orgAddress: EthereumAddress): Promise<void> => {
  // Don't re-fetch if we already have the org.
  if (svelteStore.get(orgScreenStore)?.orgAddress === orgAddress) {
    return;
  }

  const walletStore = svelteStore.get(wallet.store);
  const gnosisSafeAddress = await getGnosisSafeAddr(
    orgAddress,
    walletStore.provider
  );
  orgScreenStore.set({ orgAddress, gnosisSafeAddress });
};

export const orgSidebarStore = svelteStore.writable<theGraphApi.Org[] | []>([]);

export const fetchOrgs = async (): Promise<void> => {
  const walletStore = svelteStore.get(wallet.store);
  const w = svelteStore.get(walletStore);

  if (w.status !== wallet.Status.Connected) {
    throw new Error(
      "Tried to call fetchOrgs while the wallet wasn't connected"
    );
  }

  const orgs = await theGraphApi.getOrgs(w.connected.account.address);
  orgSidebarStore.set(orgs);
};

interface OrgMemberTabStore {
  gnosisSafeAddress: string;
  threshold: number;
  members: Member[];
}

interface Member {
  ethereumAddress: string;
  identity: identity.Identity | undefined;
}

export const orgMemberTabStore =
  svelteStore.writable<OrgMemberTabStore | null>(null);

export const fetchMembers = async (
  walletStore: svelteStore.Readable<wallet.Wallet>,
  gnosisSafeAddress: string
): Promise<void> => {
  const response = await theGraphApi.getGnosisSafeMembers(gnosisSafeAddress);

  const wallet = svelteStore.get(walletStore);
  const contract = new ClaimsContract(
    wallet.signer,
    claimsAddress(wallet.environment)
  );

  const members = await Promise.all(
    response.members.map(async ethereumAddress => {
      const identity = await getClaimedIdentity(contract, ethereumAddress);
      return { ethereumAddress, identity };
    })
  );

  orgMemberTabStore.set({
    gnosisSafeAddress,
    threshold: response.threshold,
    members,
  });
};

interface OrgProjectTabStore {
  anchoredProjects: project.Project[];
  unresolvedAnchors: theGraphApi.ProjectAnchor[];
}

export const orgProjectTabStore = svelteStore.writable<OrgProjectTabStore>({
  anchoredProjects: [],
  unresolvedAnchors: [],
});

export const resolveProjectAnchors = async (
  orgAddress: string
): Promise<void> => {
  const anchors = await theGraphApi.getOrgProjectAnchors(orgAddress);

  const anchoredProjects: project.Project[] = [];
  const unresolvedAnchors: theGraphApi.ProjectAnchor[] = [];
  console.log(anchors);

  await Promise.all(
    anchors.map(async anchor => {
      try {
        const project = await proxy.client.project.get(anchor.projectId);
        anchoredProjects.push({ ...project, anchor });
      } catch (error) {
        // TODO: only catch when backend can't find project, reraise other errors
        unresolvedAnchors.push(anchor);
      }
    })
  );

  orgProjectTabStore.set({ anchoredProjects, unresolvedAnchors });
};

async function getClaimedIdentity(
  contract: ClaimsContract,
  address: string
): Promise<identity.Identity | undefined> {
  const radicleIdBytes = await contract.getClaimed(address);
  if (!radicleIdBytes) {
    return undefined;
  }
  const urn = identitySha1Urn(radicleIdBytes);
  let identity;
  try {
    identity = await proxy.client.identityGet(urn);
  } catch {
    return undefined;
  }
  // Assert that the identity claims the ethereum address
  const claimed = identity.metadata.ethereum?.address.toLowerCase();
  if (claimed !== address.toLowerCase()) {
    return undefined;
  }
  return identity;
}
