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
    d="M11 5C11 4.44772 11.4477 4 12 4H18C19.1046 4 20 4.89543 20 6V12C20
    12.5523 19.5523 13 19 13C18.4477 13 18 12.5523 18 12V6H12C11.4477 6 11
    5.55228 11 5ZM15.2929 7.29289C15.6834 6.90237 16.3166 6.90237 16.7071
    7.29289C17.0976 7.68342 17.0976 8.31658 16.7071 8.70711L10.7071
    14.7071C10.3166 15.0976 9.68342 15.0976 9.29289 14.7071C8.90237 14.3166
    8.90237 13.6834 9.29289 13.2929L15.2929 7.29289ZM7 10C6.44772 10 6 10.4477 6
    11V17C6 17.5523 6.44772 18 7 18H13C13.5523 18 14 17.5523 14 17V16C14 15.4477
    14.4477 15 15 15C15.5523 15 16 15.4477 16 16V17C16 18.6569 14.6569 20 13
    20H7C5.34315 20 4 18.6569 4 17V11C4 9.34315 5.34315 8 7 8H8C8.55228 8 9
    8.44772 9 9C9 9.55228 8.55228 10 8 10H7Z" />
</IconWrapper>
