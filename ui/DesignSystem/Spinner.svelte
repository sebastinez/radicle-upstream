<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  export let dataCy: string | undefined = undefined;
  export let height: number = 24;
  export let style: string | undefined = undefined;
  export let width: number = 24;
</script>

<style>
  svg {
    animation: rotate 5s infinite linear;
  }
  @keyframes rotate {
    from {
      transform: rotate(0deg);
    }
    to {
      transform: rotate(359deg);
    }
  }
</style>

<!-- FYI: We're not using the <IconWrapper /> component here because it would be too complicated to animate the svg with it.  -->
<svg
  data-cy={dataCy}
  on:click
  style={`flex-shrink: 0; ${style}`}
  {width}
  {height}
  fill="var(--color-foreground-level-5)"
  viewBox="0 0 24 24"
  xmlns="http://www.w3.org/2000/svg">
  <path
    fill-rule="evenodd"
    clip-rule="evenodd"
    d="M23.7716 14.3419L21.8096 13.9538C21.9343 13.3235 22 12.6704 22 12C22
    11.3296 21.9343 10.6765 21.8096 10.0462L23.7716 9.65806C23.9214 10.4155 24
    11.1986 24 12C24 12.8014 23.9214 13.5845 23.7716 14.3419ZM21.9788
    5.33261L20.3168 6.44525C19.5862 5.35387 18.6461 4.41381 17.5547
    3.68315L18.6674 2.02121C19.976 2.89731 21.1027 4.02399 21.9788
    5.33261ZM14.3419 0.228409L13.9538 2.19038C13.3235 2.06569 12.6704 2 12
    2C11.3296 2 10.6765 2.06569 10.0462 2.19038L9.65806 0.228409C10.4155
    0.0785633 11.1986 0 12 0C12.8014 0 13.5845 0.0785633 14.3419
    0.228409ZM5.33261 2.02122L6.44525 3.68315C5.35387 4.41381 4.41381 5.35388
    3.68315 6.44525L2.02121 5.33261C2.89731 4.024 4.02399 2.89731 5.33261
    2.02122ZM0.228409 9.65806C0.0785633 10.4155 0 11.1986 0 12C0 12.8014
    0.0785633 13.5845 0.228409 14.3419L2.19038 13.9538C2.06569 13.3235 2 12.6704
    2 12C2 11.3296 2.06569 10.6765 2.19038 10.0462L0.228409 9.65806ZM2.02122
    18.6674L3.68315 17.5548C4.41381 18.6461 5.35388 19.5862 6.44525
    20.3168L5.33261 21.9788C4.024 21.1027 2.89731 19.976 2.02122
    18.6674ZM9.65806 23.7716L10.0462 21.8096C10.6765 21.9343 11.3296 22 12
    22C12.6704 22 13.3235 21.9343 13.9538 21.8096L14.3419 23.7716C13.5845
    23.9214 12.8014 24 12 24C11.1986 24 10.4155 23.9214 9.65806 23.7716ZM18.6674
    21.9788L17.5548 20.3168C18.6461 19.5862 19.5862 18.6461 20.3168
    17.5547L21.9788 18.6674C21.1027 19.976 19.976 21.1027 18.6674 21.9788Z" />
</svg>
