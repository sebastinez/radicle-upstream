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
    d="M12 5C9.79086 5 8 6.79086 8 9V9.17071C6.83481 9.58254 6 10.6938 6 12V16C6
    17.6569 7.34315 19 9 19H15C16.6569 19 18 17.6569 18 16V12C18 10.6938 17.1652
    9.58254 16 9.17071V9C16 6.79086 14.2091 5 12 5ZM15 11H9C8.44772 11 8 11.4477
    8 12V16C8 16.5523 8.44772 17 9 17H15C15.5523 17 16 16.5523 16 16V12C16
    11.4477 15.5523 11 15 11ZM14 9C14 7.89543 13.1046 7 12 7C10.8954 7 10
    7.89543 10 9H14Z" />
  <path
    d="M13.5 13.5C13.5 14.3284 12.8284 15 12 15C11.1716 15 10.5 14.3284 10.5
    13.5C10.5 12.6716 11.1716 12 12 12C12.8284 12 13.5 12.6716 13.5 13.5Z" />
  <path
    d="M11 14C11 13.4477 11.4477 13 12 13C12.5523 13 13 13.4477 13 14V15C13
    15.5523 12.5523 16 12 16C11.4477 16 11 15.5523 11 15V14Z" />
</IconWrapper>
