<script>
    import Pagination from '$lib/components/Pagination.svelte';
    import { page } from '$app/stores';

    $: ledger = $page.data.ledger;
    const { assets, categories, accounts, pageCnt, maxPage } = $page.data;

    const tradeType = {
        BUY: "매수",
        SELL: "매도",
        DEPOSIT: "입금",
    };
</script>

<div class="container">
    <h1 class="main-content-large">거래 기록</h1>
    <div/>
</div>

<div class="container">
    {#if ledger.length === 0}
        <p class="text-center text-gray-400 py-24">거래 기록이 존재하지 않습니다.</p>
    {/if}
    {#each ledger as item}
        <div class="grid grid-cols-2 px-4 pt-4 pb-4 space-y-4 md:space-y-0 items-end">
            <div>
                <p class="text-gray-400">{new Date(item.record_date).toLocaleDateString()}</p>
                <p class="text-red-600 font-semibold">{tradeType[item.type]}</p>
                <h4>{assets[item.asset_id]}</h4>
                <p>{categories[item.category_id]} | {accounts[item.account_id]}</p>
            </div>
            <div class="text-right">
                <p class="font-semibold">{item.value}원</p>
                <p>{item.amount}주</p>
                <p class="text-gray-400">{item.fee}</p>
            </div>
        </div>
    {/each}
    <Pagination page={pageCnt} contentCnt={maxPage} />
</div>
