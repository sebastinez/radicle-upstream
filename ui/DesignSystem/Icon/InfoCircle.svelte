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
    d="M4 11.5C4 7.76349 7.47174 4 12 4C16.5283 4 20 7.76349 20 11.5C20 13.4352
    19.4775 15.3612 18.5478 16.7723C17.6287 18.1673 16.3642 19 14.8125
    19H14.3264C13.1111 19 12.1766 17.925 12.3459 16.7215L12.9903 12.1393C13.0306
    11.8526 12.9447 11.5625 12.7547 11.344C12.5648 11.1255 12.2895 11 12
    11H9.75C9.19772 11 8.75 11.4477 8.75 12C8.75 12.5523 9.19772 13 9.75
    13H10.8495L10.3654 16.443C10.0269 18.85 11.8957 21 14.3264
    21H14.8125C17.1983 21 19.0275 19.6794 20.2179 17.8727C21.3975 16.0822 22
    13.7581 22 11.5C22 6.62317 17.5967 2 12 2C6.40326 2 2 6.62317 2 11.5C2
    13.7581 2.60249 16.0822 3.78213 17.8727C4.97247 19.6794 6.80167 21 9.1875
    21H9.75C10.3023 21 10.75 20.5523 10.75 20C10.75 19.4477 10.3023 19 9.75
    19H9.1875C7.63583 19 6.37128 18.1673 5.45224 16.7723C4.52251 15.3612 4
    13.4352 4 11.5Z" />
  <path
    d="M11 8.5C11 9.317 11.683 10 12.5 10C13.317 10 14 9.317 14 8.5C14 7.683
    13.317 7 12.5 7C11.683 7 11 7.683 11 8.5Z" />
</IconWrapper>
