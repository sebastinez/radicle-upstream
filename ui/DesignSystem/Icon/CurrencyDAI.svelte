<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  export let dataCy: string | undefined = undefined;
  export let style: string | undefined = undefined;
</script>

<svg
  xmlns="http://www.w3.org/2000/svg"
  data-cy={dataCy}
  style={`flex-shrink: 0; ${style ? style : ""}`}
  fill="var(--color-foreground-level-5)"
  width="24"
  height="24"
  viewBox="0 0 20 18">
  <path
    fill-rule="evenodd"
    clip-rule="evenodd"
    d=" M 12.981 6.5 C 12.994 6.665 13
        6.832 13 7 L 13 8 C 13 8.168 12.994
        8.335 12.981 8.5 L 14 8.5 C 14.552 8.5
        15 8.948 15 9.5 C 15 10.052 14.552 10.5
        14 10.5 L 12.502 10.5 C 11.522 12.849
        9.204 14.5 6.5 14.5 L 4.5 14.5 C 3.395
        14.5 2.5 13.604 2.5 12.5 L 2.5 12.23 C
        2.5 11.678 2.948 11.23 3.5 11.23 C
        4.052 11.23 4.5 11.678 4.5 12.23 L 4.5
        12.5 L 6.5 12.5 C 8.06 12.5 9.435
        11.706 10.242 10.5 L 1 10.5 C 0.448
        10.5 0 10.052 0 9.5 C 0 8.948 0.448 8.5
        1 8.5 L 2.5 8.5 L 2.5 8.332 C 2.5 7.78
        2.948 7.332 3.5 7.332 C 4.052 7.332 4.5
        7.78 4.5 8.332 L 4.5 8.5 L 10.973 8.5 C
        10.991 8.336 11 8.169 11 8 L 11 7 C 11
        6.831 10.991 6.664 10.973 6.5 L 1 6.5 C
        0.448 6.5 0 6.052 0 5.5 C 0 4.948 0.448
        4.5 1 4.5 L 10.242 4.5 C 9.435 3.294
        8.06 2.5 6.5 2.5 L 4.5 2.5 L 4.5 3 C
        4.5 3.552 4.052 4 3.5 4 C 2.948 4 2.5
        3.552 2.5 3 L 2.5 2.5 C 2.5 1.396 3.395
        0.5 4.5 0.5 L 6.5 0.5 C 9.204 0.5
        11.522 2.151 12.502 4.5 L 14 4.5 C
        14.552 4.5 15 4.948 15 5.5 C 15 6.052
        14.552 6.5 14 6.5 L 12.981 6.5 Z " />
</svg>
