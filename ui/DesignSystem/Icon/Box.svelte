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
    d="M13.3416 3.55279C12.4971 3.1305 11.5029 3.1305 10.6584 3.55279L4.65836
    6.55279C3.64201 7.06096 3 8.09975 3 9.23607V14.7639C3 15.9002 3.64201 16.939
    4.65836 17.4472L10.6584 20.4472C11.5029 20.8695 12.4971 20.8695 13.3416
    20.4472L19.3416 17.4472C20.358 16.939 21 15.9002 21 14.7639V9.23607C21
    8.09975 20.358 7.06096 19.3416 6.55279L13.3416 3.55279ZM11.5528
    5.34164C11.8343 5.20088 12.1657 5.20088 12.4472 5.34164L18.2436
    8.23982L11.9887 10.9834L7.38142 9.07605C6.87113 8.8648 6.28621 9.10721
    6.07496 9.6175C5.86371 10.1278 6.10613 10.7127 6.61642 10.924L11
    12.7387V18.382L5.55279 15.6584C5.214 15.489 5 15.1427 5 14.7639V9.23607C5
    8.8573 5.214 8.51103 5.55279 8.34164L11.5528 5.34164ZM13 18.382L18.4472
    15.6584C18.786 15.489 19 15.1427 19 14.7639V10.092L13 12.7237V18.382Z" />
</IconWrapper>
