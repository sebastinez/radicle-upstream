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
    d="M12 3C14.7372 3 17 5.28372 17 8C17 8.55228 16.5523 9 16 9C15.4477 9 15
    8.55228 15 8C15 6.3739 13.6472 5 12 5C10.3529 5 9 6.3739 9 8C9 8.55228
    8.55228 9 8 9C7.44772 9 7 8.55228 7 8C7 5.28372 9.26277 3 12 3Z" />
  <path
    d="M13 15.6181C13.3069 15.3434 13.5 14.9443 13.5 14.5C13.5 13.6716 12.8284
    13 12 13C11.1716 13 10.5 13.6716 10.5 14.5C10.5 14.9443 10.6931 15.3434 11
    15.6181V17C11 17.5523 11.4477 18 12 18C12.5523 18 13 17.5523 13 17V15.6181Z" />
  <path
    fill-rule="evenodd"
    clip-rule="evenodd"
    d="M5 13C5 11.3431 6.34315 10 8 10H16C17.6569 10 19 11.3431 19 13V18C19
    19.6569 17.6569 21 16 21H8C6.34315 21 5 19.6569 5 18V13ZM8 12C7.44772 12 7
    12.4477 7 13V18C7 18.5523 7.44772 19 8 19H16C16.5523 19 17 18.5523 17
    18V13C17 12.4477 16.5523 12 16 12H8Z" />
</IconWrapper>
