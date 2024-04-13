<template>
  <div class="p-5">
    <div class="space-y-0.5">
      <h2 class="tect-2x1 font-bold tracking-tight">
	beevolution
      </h2>
      <p class="text-muted-foreground">
	Databases :)
      </p>
    </div>
    <Separator class="my-6" />
    <div class="flex flex-col space-y-8 lg:flex-row lg:space-x-12 lg:space-y-0">
      <aside class="-mx-4 lg:w-1/5">
        <MainNav />
      </aside>
      <div class="flex-1 lg:max-w-2xl">
        <div class="space-y-6">
          <slot />
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { Separator } from '@/components/ui/separator'
import MainNav from '@/components/MainNav.vue'
</script>
