<template>
  <div class="space-y-0.5">
    <h2 class="text-2x1 font-bold tracking-tight">
      Settings
    </h2>
    <p class="text-muted-foreground">
      Change the bahaviour of beevolution
    </p>
  </div>
  <Separator class="my-6" />
  <div class="flex flex-col space-y-8 lg:flex-row lg:space-x-12 lg:space-y-0">
    <aside class="-mx-4 lg:w-1/5">
      <SidebarNav />
    </aside>
    <div class="flex-1 lg:max-w-2xl">
      <div class="space-y-6">
        <slot />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { Separator } from '@/components/ui/separator'
import SidebarNav from '../components/SidebarNav.vue'
</script>
