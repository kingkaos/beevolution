<template>
  <Settings>
    <div>
      <h3 class="text-lg font-medium">
	Database
      </h3>
      <p class="text-sm text-muted-foreground">
	Update your database settings.
      </p>
    </div>
    <Separator />
    <Form v-slot="{ setValues }" class="space-y-8 p-2" @submit="onSubmit">
    <FormField v-slot="{ componentField }" name="name">
      <FormItem>
        <FormLabel>Host</FormLabel>
        <FormControl>
          <Input type="text" placeholder="e.g. 127.0.0.1" v-bind="componentField" />
        </FormControl>
        <FormDescription>
	  This is the host address of the database.
        </FormDescription>
        <FormMessage />
      </FormItem>
    </FormField>
    </Form>
  </Settings>
</template>

<script setup lang="ts">
  import { Form, FormControl, FormDescription, FormField, FormItem, FormLabel, FormMessage } from '@/components/ui/form'
  import { Separator } from '@/components/ui/separator';
  import Settings from '../pages/Settings.vue';
</script>
