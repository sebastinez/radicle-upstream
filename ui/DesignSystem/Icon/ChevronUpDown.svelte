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
    d="M17.7071 9.70712C17.3166 10.0976 16.6834 10.0976 16.2929 9.70712L13.0253
    6.43953C12.459 5.87327 11.541 5.87327 10.9747 6.43953L7.70711
    9.70712C7.31658 10.0976 6.68342 10.0976 6.29289 9.70712C5.90237 9.31659
    5.90237 8.68343 6.29289 8.2929L9.56048 5.02531C10.9078 3.678 13.0922 3.67801
    14.4395 5.02531L17.7071 8.2929C18.0976 8.68343 18.0976 9.31659 17.7071
    9.70712ZM17.7071 14.2929C18.0976 14.6834 18.0976 15.3166 17.7071
    15.7071L14.4395 18.9747C13.0922 20.322 10.9078 20.322 9.56048
    18.9747L6.29289 15.7071C5.90237 15.3166 5.90237 14.6834 6.29289
    14.2929C6.68342 13.9024 7.31658 13.9024 7.70711 14.2929L10.9747
    17.5605C11.541 18.1268 12.459 18.1268 13.0253 17.5605L16.2929
    14.2929C16.6834 13.9024 17.3166 13.9024 17.7071 14.2929Z" />
</IconWrapper>
