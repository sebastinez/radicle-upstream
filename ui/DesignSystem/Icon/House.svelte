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
    d="M10.683 3.82361C11.4371 3.16382 12.563 3.16382 13.317 3.82361L20.6585
    10.2474C21.0742 10.6111 21.1163 11.2429 20.7526 11.6585C20.3889 12.0741
    19.7571 12.1163 19.3415 11.7526L12 5.32877L4.65852 11.7526C4.24288 12.1163
    3.61112 12.0741 3.24744 11.6585C2.88375 11.2429 2.92587 10.6111 3.34151
    10.2474L10.683 3.82361ZM7.14144 11.01C7.68817 11.0882 8.06807 11.5947
    7.98996 12.1414L7.31608 16.8586C7.23002 17.461 7.69748 18 8.30603
    18H9.00001V16C9.00001 14.3431 10.3432 13 12 13C13.6569 13 15 14.3431 15
    16V18H15.694C16.3025 18 16.77 17.461 16.6839 16.8586L16.0101 12.1414C15.932
    11.5947 16.3119 11.0882 16.8586 11.01C17.4053 10.9319 17.9119 11.3118 17.99
    11.8586L18.6638 16.5757C18.922 18.383 17.5196 20 15.694 20H8.30603C6.48038
    20 5.078 18.383 5.33618 16.5757L6.01006 11.8586C6.08817 11.3118 6.5947
    10.9319 7.14144 11.01ZM13 18V16C13 15.4477 12.5523 15 12 15C11.4477 15 11
    15.4477 11 16V18H13Z" />
</IconWrapper>
