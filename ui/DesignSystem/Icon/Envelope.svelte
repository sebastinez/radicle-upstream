<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import IconWrapper from "./IconWrapper.svelte";

  export let dataCy: string | undefined = undefined;
  export let style: string | undefined = undefined;
</script>

<IconWrapper on:click {dataCy} {style}>
  <path
    fill-rule="evenodd"
    clip-rule="evenodd"
    d="M2 7.89857C2 5.74545 3.74545 4 5.89857 4H18.1014C20.2546 4 22 5.74545 22
    7.89857C22 8.93461 21.4533 9.89375 20.5619 10.4217L14.0383 14.2851C12.7814
    15.0295 11.2186 15.0295 9.96173 14.2851L3.43814 10.4217C2.5467 9.89375 2
    8.93461 2 7.89857ZM5.89857 6C4.85002 6 4 6.85002 4 7.89857C4 8.22799 4.17383
    8.53296 4.45727 8.70082L10.9809 12.5642C11.6093 12.9364 12.3907 12.9364
    13.0191 12.5642L19.5427 8.70082C19.8262 8.53296 20 8.22799 20 7.89857C20
    6.85002 19.15 6 18.1014 6H5.89857ZM3 11.7642C3.55228 11.7642 4 12.2119 4
    12.7642V16C4 17.1046 4.89543 18 6 18H18C19.1046 18 20 17.1046 20
    16V12.7642C20 12.2119 20.4477 11.7642 21 11.7642C21.5523 11.7642 22 12.2119
    22 12.7642V16C22 18.2091 20.2091 20 18 20H6C3.79086 20 2 18.2091 2
    16V12.7642C2 12.2119 2.44772 11.7642 3 11.7642Z" />
</IconWrapper>
