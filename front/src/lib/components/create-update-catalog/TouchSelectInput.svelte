<script>
    import { Select } from 'bits-ui';

    export let items, title, name;
    let selected = { value: null, label: null };
</script>

<Select.Root>
  <div class="flex w-full flex-col gap-1.5">
      <span class="font-semibold">{title}</span>
      <Select.Root items bind:selected>
          <Select.Trigger
              class="inline-flex px-3 py-2 items-center border focus:outline-none focus:ring-2 focus:ring-foreground focus:ring-offset-2 focus:ring-offset-background"
              placeholder="test"
          >
              <Select.Value class="font-semibold" placeholder="-" />
          </Select.Trigger>
          <Select.Content class="border px-1 py-2 bg-background ">
              {#each items as item}
                  <Select.Item class='px-2 py-2 font-semibold data-[highlighted]:bg-muted' value={item.id} label={item.name}>{item.name}</Select.Item>
              {/each}
          </Select.Content>
          <input type="hidden" name={name} value={selected.value} />
      </Select.Root>
  </div>
</Select.Root>