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
    d="M16 6V7H19C19.5523 7 20 7.44772 20 8C20 8.55228 19.5523 9 19 9H5C4.44772
    9 4 8.55228 4 8C4 7.44772 4.44772 7 5 7H8V6C8 4.34315 9.34315 3 11
    3H13C14.6569 3 16 4.34315 16 6ZM10 6C10 5.44772 10.4477 5 11 5H13C13.5523 5
    14 5.44772 14 6V7H10V6Z" />
  <path
    d="M6.99229 10.876C6.92379 10.3279 6.424 9.93922 5.87598 10.0077C5.32796
    10.0762 4.93923 10.576 5.00774 11.124L5.67925 16.4961C5.92946 18.4979
    7.63107 20 9.64836 20H14.3517C16.369 20 18.0706 18.4979 18.3208
    16.4961L18.9923 11.124C19.0608 10.576 18.6721 10.0762 18.124 10.0077C17.576
    9.93922 17.0762 10.3279 17.0077 10.876L16.3362 16.2481C16.2111 17.2489
    15.3603 18 14.3517 18H9.64836C8.63971 18 7.78891 17.2489 7.6638
    16.2481L6.99229 10.876Z" />
  <path
    d="M10 10C10.5523 10 11 10.4477 11 11V15C11 15.5523 10.5523 16 10 16C9.44772
    16 9 15.5523 9 15V11C9 10.4477 9.44772 10 10 10Z" />
  <path
    d="M15 15C15 15.5523 14.5523 16 14 16C13.4477 16 13 15.5523 13 15V11C13
    10.4477 13.4477 10 14 10C14.5523 10 15 10.4477 15 11V15Z" />
</IconWrapper>
