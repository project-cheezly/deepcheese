<script>
    import { Combobox } from 'bits-ui';

    export let name, placeholder = '종목';

    let inputValue = '';
    let selected = '';
    let timeoutId;

    let itemList = [];

    $: {
        inputValue = inputValue;
        debounceSearch(search, 200);
    }

    function debounceSearch(fn, delay) {
        clearTimeout(timeoutId);
        timeoutId = setTimeout(() => {
            fn();
        }, delay);
    }

    async function search() {
        if (inputValue.trim() === '') {
            itemList = [];
            return;
        }
        const response = await fetch(`/portfolio/api/asset?asset_name=${inputValue}`);
        response.json().then(data => {
            itemList = data.reduce((acc, item) => {
                acc.push({
                    id: item.id,
                    name: item.name,
                });
                return acc;
            }, []);
        });
    }

</script>

<label class="flex w-full flex-col gap-1">
    <span class="font-semibold">{placeholder}</span>
    <Combobox.Root
            items={itemList}
            bind:inputValue
            bind:selected
            loop={true}
            onSelectedChange={() => { open = false; }}
    >
        <Combobox.Input placeholder="종목" class="border px-3 py-2 font-semibold" />
        <Combobox.Content class="w-full border bg-background">
            {#each itemList as item (item.id)}
                <Combobox.Item
                        class="font-semibold px-3 flex h-10 w-full select-none items-center data-[highlighted]:bg-muted"
                        value={item.id}
                        label={item.name}
                >
                    {item.name}
                </Combobox.Item>
            {:else}
                <span class="px-2 flex h-10 w-full select-none items-center text-gray-400">
                    {#if inputValue.trim() !== ''}
                        검색 결과가 없습니다.
                    {:else}
                        종목을 입력해주세요.
                    {/if}
                </span>
            {/each}
        </Combobox.Content>
        <input type="hidden" name={name} bind:value={selected.value} />
    </Combobox.Root>
</label>