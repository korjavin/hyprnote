---
title: Timer
description: Displays the remaining time for meetings with time constraints
source: https://github.com/fastrepl/hypr/tree/main/extensions/timer
implemented: true
default: false
cloud_only: false
plugins: []
tags: [utility]
---
<TitleWithContributors :title="$frontmatter.title" />

**{{ $frontmatter.description }}**

<ExtensionTags :frontmatter="$frontmatter" />

## Resources

<ul>
  <li><a :href="$frontmatter.source">Github source</a></li>
  <li v-for="plugin in $frontmatter.plugins"><PluginLink :plugin /></li>
</ul>
