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
    d="M5.9 4C4.29837 4 3 5.29837 3 6.9V16.1C3 17.7016 4.29837 19 5.9
    19H18.1C19.7016 19 21 17.7016 21 16.1V10.9C21 9.61255 20.161 8.52104 19
    8.14237V8C19 6.34315 17.6569 5 16 5H14C13.4477 5 13 5.44772 13 6C13 6.55228
    13.4477 7 14 7H16C16.5523 7 17 7.44772 17 8H12.7808L12.3299 6.19665C12.0072
    4.90566 10.8472 4 9.51652 4H5.9ZM5 6.9C5 6.40294 5.40294 6 5.9
    6H9.51652C9.92951 6 10.2895 6.28107 10.3897 6.68172L10.8405 8.48507C11.0631
    9.37541 11.863 10 12.7808 10H18.1C18.5971 10 19 10.4029 19 10.9V16.1C19
    16.5971 18.5971 17 18.1 17H5.9C5.40294 17 5 16.5971 5 16.1V6.9Z" />
</IconWrapper>
