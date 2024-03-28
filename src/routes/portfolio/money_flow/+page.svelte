<script>
    import { page } from '$app/stores';
    import Pagination from "$lib/components/Pagination.svelte";

    $: moneyFlow = $page.data.moneyFlow;

    const { maxPage, pageCnt, accounts, categories } = $page.data;

    const moneyFlowType = {
        INFLOW: "입금",
        OUTFLOW: "출금"
    };
</script>

<div class="container">
    <h1 class="main-content-large">입출금</h1>
    <div/>
</div>

<div class="container">
    {#if moneyFlow.length === 0}
        <p class="text-center text-gray-400 py-24">거래 기록이 존재하지 않습니다.</p>
    {/if}
    {#each moneyFlow as item}
        <div class="grid grid-cols-2 px-4 pt-4 pb-4 space-y-4 md:space-y-0 items-end">
            <div>
                <p class="font-semibold text-gray-400">{categories[item.category_id]} | {accounts[item.account_id]}</p>
                <p class="font-semibold">{new Date(item.record_date).toLocaleDateString()}</p>
            </div>
            <div class="text-right">
                <p class="text-red-600 font-semibold">{moneyFlowType[item.transaction_type]}</p>
                <p class="font-semibold">{item.value}{item.currency_id === 1 ? '원' : '달러'}</p>
            </div>
        </div>
    {/each}
</div>
<Pagination page={pageCnt} contentCnt={maxPage} />
