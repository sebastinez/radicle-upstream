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
    d="M7.33333 13.3333C8.06971 13.3333 8.66667 12.7364 8.66667 12C8.66667
    11.2636 8.06971 10.6667 7.33333 10.6667C6.59695 10.6667 6 11.2636 6 12C6
    12.7364 6.59695 13.3333 7.33333 13.3333ZM13.3333 12C13.3333 12.7364 12.7364
    13.3333 12 13.3333C11.2636 13.3333 10.6667 12.7364 10.6667 12C10.6667
    11.2636 11.2636 10.6667 12 10.6667C12.7364 10.6667 13.3333 11.2636 13.3333
    12ZM18 12C18 12.7364 17.403 13.3333 16.6667 13.3333C15.9303 13.3333 15.3333
    12.7364 15.3333 12C15.3333 11.2636 15.9303 10.6667 16.6667 10.6667C17.403
    10.6667 18 11.2636 18 12Z" />
</IconWrapper>
