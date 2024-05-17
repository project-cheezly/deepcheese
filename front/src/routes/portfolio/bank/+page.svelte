<script>
    import { page } from '$app/stores';
    import * as SingleContainer from '$lib/components/layout/single';
    import * as DoubleContainer from "$lib/components/layout/double";
    import * as Dialog from '$lib/components/create-update-catalog';
    import Pagination from "$lib/components/Pagination.svelte";
    import CreateDialogContent from "./CreateDialogContent.svelte";

    $: moneyFlow = $page.data.moneyFlow;

    const { maxPage, pageCnt, accounts, categories } = $page.data;

    const moneyFlowType = {
        INFLOW: "입금",
        OUTFLOW: "출금"
    };
</script>

<SingleContainer.Root>
    <SingleContainer.Title>입출금</SingleContainer.Title>

    {#if moneyFlow.length === 0}
        <p class="text-center text-gray-400 py-24">거래 기록이 존재하지 않습니다.</p>
    {/if}
    {#each moneyFlow as item}
    <SingleContainer.Inner>
        <div class="grid grid-cols-2 space-y-4 md:space-y-0 items-end">
            <div>
                <p class="font-semibold text-gray-400">{categories[item.category_id]} | {accounts[item.account_id]}</p>
                <p class="font-semibold">{new Date(item.record_date).toLocaleDateString()}</p>
            </div>
            <div class="text-right">
                <p class="text-red-600 font-semibold">{moneyFlowType[item.flow]}</p>
                <p class="font-semibold">
                    {new Intl.NumberFormat('ko-KR').format(item.value)}
                    {item.currency_id === 1 ? '원' : '달러'}
                </p>
            </div>
        </div>
    </SingleContainer.Inner>
    {/each}
    <div class="p-4">
        <Dialog.Root title="입출금 기록 추가" let:closeDialog>
            <CreateDialogContent {categories} {accounts} {closeDialog} />
        </Dialog.Root>
    </div>
</SingleContainer.Root>


<Pagination page={pageCnt} contentCnt={maxPage} />
