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
    d="M18 5C18 3.89543 17.1046 3 16 3C14.8954 3 14 3.89543 14 5C14 6.10457
    14.8954 7 16 7L20 7C20.5523 7 21 6.55229 21 6C21 5.44771 20.5523 5 20 5L18
    5Z" />
  <path
    d="M10 18C10 18.5523 9.55228 19 9 19L4 19C3.44771 19 3 18.5523 3 18C3
    17.4477 3.44771 17 4 17L9 17C9.55228 17 10 17.4477 10 18Z" />
  <path
    d="M7 12C7 12.5523 6.55229 13 6 13H4C3.44772 13 3 12.5523 3 12C3 11.4477
    3.44772 11 4 11L6 11C6.55229 11 7 11.4477 7 12Z" />
  <path
    d="M13 6C13 6.55229 12.5523 7 12 7L4 7C3.44772 7 3 6.55228 3 6C3 5.44771
    3.44772 5 4 5L12 5C12.5523 5 13 5.44771 13 6Z" />
  <path
    d="M21 18C21 18.5523 20.5523 19 20 19L13 19C11.8954 19 11 18.1046 11 17C11
    15.8954 11.8954 15 13 15C14.1046 15 15 15.8954 15 17H20C20.5523 17 21
    17.4477 21 18Z" />
  <path
    d="M21 12C21 12.5523 20.5523 13 20 13L10 13C8.89543 13 8 12.1046 8 11C8
    9.89543 8.89543 9 10 9C11.1046 9 12 9.89543 12 11H20C20.5523 11 21 11.4477
    21 12Z" />
</IconWrapper>
