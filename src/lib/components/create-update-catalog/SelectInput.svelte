<script>
    import { Combobox } from 'bits-ui';

    export let name, placeholder = '종목';

    let inputValue = '';
    let selected = '';
    export let itemList = [];

    $: filteredItems =
        inputValue
            ? itemList.filter(item => item.name.includes(inputValue))
            : itemList;

</script>

<label class="flex w-full flex-col gap-1">
    <span class="font-semibold">{placeholder}</span>
    <Combobox.Root
            items={filteredItems}
            bind:inputValue
            bind:selected
            loop={true}
            onSelectedChange={() => { open = false; }}
    >
        <Combobox.Input placeholder="종목" class="border px-3 py-2 font-semibold" />
        <Combobox.Content class="w-full border bg-background">
            {#each filteredItems as item (item.id)}
                <Combobox.Item
                        class="font-semibold px-3 flex h-10 w-full select-none items-center data-[highlighted]:bg-muted"
                        value={item.id}
                        label={item.name}
                >
                    {item.name}
                </Combobox.Item>
            {:else}
                <span class="px-2 flex h-10 w-full select-none items-center text-gray-400">
                    검색 결과가 없습니다.
                </span>
            {/each}
        </Combobox.Content>
        <input type="hidden" name={name} bind:value={selected.value} />
    </Combobox.Root>
</label>