<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  export let style: string | undefined = undefined;
</script>

<svg
  {style}
  width="24"
  height="24"
  fill="var(--color-foreground-level-5)"
  viewBox={`0 0 24 24`}
  xmlns="http://www.w3.org/2000/svg">
  <path
    id="left"
    fill-rule="evenodd"
    clip-rule="evenodd"
    d="M5 11C4.44772 11 4 11.4477 4 12C4 12.5523 4.44772 13 5 13C5.55228 13 6 12.5523 6 12C6 11.4477 5.55228 11 5 11ZM2 12C2 10.3431 3.34315 9 5 9C6.65685 9 8 10.3431 8 12C8 13.6569 6.65685 15 5 15C3.34315 15 2 13.6569 2 12Z" />
  <path
    id="top"
    fill-rule="evenodd"
    clip-rule="evenodd"
    d="M12 4C11.4477 4 11 4.44772 11 5C11 5.55228 11.4477 6 12 6C12.5523 6 13 5.55228 13 5C13 4.44772 12.5523 4 12 4ZM9 5C9 3.34315 10.3431 2 12 2C13.6569 2 15 3.34315 15 5C15 6.65685 13.6569 8 12 8C10.3431 8 9 6.65685 9 5Z" />
  <path
    id="right"
    fill-rule="evenodd"
    clip-rule="evenodd"
    d="M19 11C18.4477 11 18 11.4477 18 12C18 12.5523 18.4477 13 19 13C19.5523 13 20 12.5523 20 12C20 11.4477 19.5523 11 19 11ZM16 12C16 10.3431 17.3431 9 19 9C20.6569 9 22 10.3431 22 12C22 13.6569 20.6569 15 19 15C17.3431 15 16 13.6569 16 12Z" />
  <path
    id="bottom"
    fill-rule="evenodd"
    clip-rule="evenodd"
    d="M12 18C11.4477 18 11 18.4477 11 19C11 19.5523 11.4477 20 12 20C12.5523 20 13 19.5523 13 19C13 18.4477 12.5523 18 12 18ZM9 19C9 17.3431 10.3431 16 12 16C13.6569 16 15 17.3431 15 19C15 20.6569 13.6569 22 12 22C10.3431 22 9 20.6569 9 19Z" />
</svg>
