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
    d="M9.70712 6.29289C10.0976 6.68342 10.0976 7.31658 9.70712 7.70711L6.43953
    10.9747C5.87327 11.541 5.87327 12.459 6.43953 13.0253L9.70712
    16.2929C10.0976 16.6834 10.0976 17.3166 9.70712 17.7071C9.31659 18.0976
    8.68343 18.0976 8.2929 17.7071L5.02531 14.4395C3.678 13.0922 3.67801 10.9078
    5.02531 9.56048L8.2929 6.29289C8.68343 5.90237 9.31659 5.90237 9.70712
    6.29289ZM14.2929 6.29289C14.6834 5.90237 15.3166 5.90237 15.7071
    6.29289L18.9747 9.56048C20.322 10.9078 20.322 13.0922 18.9747
    14.4395L15.7071 17.7071C15.3166 18.0976 14.6834 18.0976 14.2929
    17.7071C13.9024 17.3166 13.9024 16.6834 14.2929 16.2929L17.5605
    13.0253C18.1268 12.459 18.1268 11.541 17.5605 10.9747L14.2929
    7.70711C13.9024 7.31658 13.9024 6.68342 14.2929 6.29289Z" />
</IconWrapper>
