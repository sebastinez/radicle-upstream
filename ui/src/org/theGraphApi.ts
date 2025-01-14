// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import * as apolloCore from "@apollo/client/core";
import * as ethers from "ethers";
import * as multihash from "multihashes";
import * as svelteStore from "svelte/store";

import type * as project from "ui/src/project";
import type * as ensResolver from "ui/src/org/ensResolver";

import * as error from "ui/src/error";
import * as ethereum from "ui/src/ethereum";
import * as urn from "ui/src/urn";
import * as wallet from "ui/src/wallet";
import type { Registration } from "./ensResolver";

function createApolloClient(uri: string): apolloCore.ApolloClient<unknown> {
  return new apolloCore.ApolloClient({
    uri,
    cache: new apolloCore.InMemoryCache(),
    defaultOptions: {
      query: {
        fetchPolicy: "no-cache",
      },
    },
  });
}

function orgsSubgraphClient() {
  const walletStore = svelteStore.get(wallet.store);
  let uri;
  switch (walletStore.environment) {
    case ethereum.Environment.Local:
      throw new error.Error({
        code: error.Code.FeatureNotAvailableForGivenNetwork,
        message: "Orgs is not available on the Local testnet.",
      });
    case ethereum.Environment.Rinkeby:
      uri =
        "https://api.thegraph.com/subgraphs/name/radicle-dev/radicle-orgs-rinkeby";
      break;
    case ethereum.Environment.Mainnet:
      uri =
        "https://gateway.thegraph.com/api/1758a78ae257ad4906f9c638e4a68c19/subgraphs/id/0x2f0963e77ca6ac0c2dad1bf4147b6b40e0dd8728-0";
      break;
  }
  return createApolloClient(uri);
}

export interface Org {
  id: string;
  owner: string;
  registration?: Registration;
  creator: string;
  timestamp: number;
}

export async function getOrgs(
  walletOwnerAddress: string,
  multiSigOwners: string[]
): Promise<Org[]> {
  const orgsResponse = await orgsSubgraphClient().query<{
    orgs: Array<{
      id: string;
      owner: string;
      creator: string;
      // This is a UNIX seconds timestamp formatted as a string
      timestamp: string;
    }>;
  }>({
    query: apolloCore.gql`
        query GetOrgs($owners: [String!]!) {
          orgs(where: { owner_in: $owners }) {
            id
            owner
            creator
            timestamp
          }
        }
      `,
    variables: { owners: [walletOwnerAddress, ...multiSigOwners] },
  });

  return orgsResponse.data.orgs.map(org => ({
    ...org,
    timestamp: Number.parseInt(org.timestamp),
  }));
}

export async function getOrgProjectAnchors(
  orgAddress: string,
  registration?: ensResolver.Registration
): Promise<project.Anchor[]> {
  const response = (
    await orgsSubgraphClient().query({
      query: apolloCore.gql`
        query GetOrgAnchoredProjects($orgAddress: String!) {
          projects(where: {org: $orgAddress}) {
            anchor {
              id
              objectId
              multihash
              timestamp
            }
          }
        }
      `,
      variables: { orgAddress },
    })
  ).data.projects;

  return response.map(
    (project: {
      anchor: {
        id: string;
        objectId: string;
        multihash: string;
        // This is a UNIX seconds timestamp formatted as a string
        timestamp: number;
      };
    }) => {
      const decodedProjectId = urn.identitySha1Urn(
        ethers.utils.arrayify(`0x${project.anchor.objectId.slice(26)}`)
      );

      const byteArray = ethers.utils.arrayify(project.anchor.multihash);
      const decodedMultihash = multihash.decode(byteArray);
      const decodedCommitHash = ethers.utils
        .hexlify(decodedMultihash.digest)
        .replace(/^0x/, "");
      const anchor: project.Anchor = {
        type: "confirmed",
        orgAddress,
        transactionId: project.anchor.id,
        projectId: decodedProjectId,
        commitHash: decodedCommitHash,
        timestamp: project.anchor.timestamp,
        registration,
      };

      return anchor;
    }
  );
}

// Returns `true` if `err` is a 502 or 503 HTTP response error thrown
// by requests to the Graph.
export function isUnavailableError(err: unknown): boolean {
  return (
    err instanceof apolloCore.ApolloError &&
    err.networkError !== null &&
    "statusCode" in err.networkError &&
    (err.networkError.statusCode === 502 || err.networkError.statusCode === 503)
  );
}
