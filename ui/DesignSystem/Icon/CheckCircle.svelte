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
    d="M2 12C2 6.47715 6.47715 2 12 2C13.2307 2 14.4116 2.22277 15.5031
    2.63107C18.3099 3.68096 19.0641 7.01293 17.2628 9.4154L17.2574
    9.4226L17.2574 9.42258L13.1807 14.759C12.0845 16.216 9.95209 16.3663 8.66229
    15.0765L6.29289 12.7071C5.90237 12.3166 5.90237 11.6834 6.29289
    11.2929C6.68342 10.9024 7.31658 10.9024 7.70711 11.2929L10.0765
    13.6623C10.5068 14.0926 11.2185 14.042 11.5836 13.5552L11.5889
    13.5481L11.589 13.5481L15.6651 8.21224C16.7627 6.74458 16.2057 5.02921
    14.8024 4.5043C13.9316 4.17857 12.9878 4 12 4C7.58172 4 4 7.58172 4 12C4
    16.4183 7.58172 20 12 20C16.4183 20 20 16.4183 20 12C20 11.0697 19.8416
    10.1785 19.5512 9.35064C19.3683 8.8295 19.6426 8.25881 20.1637
    8.07598C20.6849 7.89314 21.2556 8.1674 21.4384 8.68854C21.8025 9.72625 22
    10.8411 22 12C22 17.5228 17.5228 22 12 22C6.47715 22 2 17.5228 2 12Z" />
</IconWrapper>
