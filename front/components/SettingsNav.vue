<template>
  <nav class="flex space-x-2 lg:flex-col lg:space-x-0 lg:space-y-1">
    <Button
      v-for="item in navItems"
      :key="item.title"
      as="a"
      :href="item.href"
      variant="ghost"
      :class="cn(
        'w-full text-left justify-start',
        $route.href === item.href && 'bg-muted hover:bg-muted',
      )"
    >
    {{ item.title }}
    </Button>
  </nav>
</template>

<script setup lang="ts">
import { cn } from '@/lib/utils'
import { Button } from '@/components/ui/button'
import type { Item } from '@/types'

const navItems: Item[] = [
  {
    title: 'Database',
    href: '#/settings/database',
  },
  {
    title: 'Appearance',
    href: '#/settings/appearance',
  },
]
</script>
