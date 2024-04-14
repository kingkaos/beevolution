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
    <Form v-slot="{ setValues }" class="space-y-8" @submit="onSubmit">
      <FormField v-slot="{ componentField }" name="dbHost">
        <FormItem>
          <FormLabel class="pr-5">
	    Host
	  </FormLabel>
          <FormControl>
            <Input type="text" placeholder="e.g. 127.0.0.1" v-bind="componentField" />
          </FormControl>
          <FormDescription>
  	    This is the host address of the database.
          </FormDescription>
          <FormMessage />
        </FormItem>
      </FormField>

      <FormField v-slot="{ dbPort }" class="space-y-8 p-2" name="dbPort">
	<FormItem>
	  <FormLabel class="pr-5">
	    Port
	  </FormLabel>
	  <FormControl>
	    <Input type="text" placeholder="e.g. 5432" v-bind="dbPort" />
	  </FormControl>
	  <FormDescription>
	    This is the port of the database.
	  </FormDescription>
	</FormItem>
      </FormField>
    </Form>
  </Settings>
</template>

<script setup lang="ts">
import { Input } from '@/components/ui/input';
import { Form, FormControl, FormDescription, FormField, FormItem, FormLabel } from '@/components/ui/form'
import { Separator } from '@/components/ui/separator';

import { Settings } from '@/pages';
</script>
