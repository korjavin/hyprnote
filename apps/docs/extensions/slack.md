---
title: Slack
description: Send Slack messages related to the meeting content to your team.
source: https://github.com/fastrepl/hypr/tree/main/extensions/slack
implemented: false
default: false
cloud_only: true
plugins: [listener, db]
tags: [messaging]
---
<TitleWithContributors :title="$frontmatter.title" />

**{{ $frontmatter.description }}**

<ExtensionTags :frontmatter="$frontmatter" />

## Resources

<ul>
  <li><a :href="$frontmatter.source">Github source</a></li>
  <li v-for="plugin in $frontmatter.plugins"><PluginLink :plugin /></li>
</ul>
