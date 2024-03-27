<script>
    import { Pagination } from 'bits-ui';
    import { goto } from '$app/navigation';

    export let contentCnt = 100;
    export let perPage = 10;
</script>


<Pagination.Root
        class="grid justify-items-center"
        count={contentCnt}
        perPage={perPage}
        onPageChange={(page) => {
            goto(`?page=${page}`);
        }}
        let:pages
>
<div class="my-8 flex space-x-2 md:space-x-4">
    <Pagination.PrevButton class="button-small">Prev</Pagination.PrevButton>
    {#each pages as page}
        {#if page.type === 'ellipsis'}
            <button class="text-foreground-alt">...</button>
        {:else}
            <Pagination.Page class="size-10 data-[selected]:border data-[selected]:font-semibold" {page}>{page.value}</Pagination.Page>
        {/if}
    {/each}
    <Pagination.NextButton class="button-small">Next</Pagination.NextButton>
</div>
</Pagination.Root>