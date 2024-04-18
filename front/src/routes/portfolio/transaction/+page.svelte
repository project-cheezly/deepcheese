<script>
    import Pagination from '$lib/components/Pagination.svelte';
    import * as Dialog from '$lib/components/create-update-catalog';
    import { page } from '$app/stores';
    import CreateDialogContent from "./CreateDialogContent.svelte";

    let ledger = [];
    $: ledger = $page.data.ledger;
    const { categories, accounts, pageCnt, maxPage } = $page.data;

    const tradeType = {
        '1': '매수',
        '2': '매도',
        '3': '배당',
        '4': '무상증자'
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
                {#if tradeType[item.tr_type_id] === '매수'}
                    <p class="text-red-600 font-semibold">{tradeType[item.tr_type_id]}</p>
                {:else if tradeType[item.tr_type_id] === '매도'}
                    <p class="text-blue-600 font-semibold">{tradeType[item.tr_type_id]}</p>
                {:else}
                    <p class="font-semibold">{tradeType[item.tr_type_id]}</p>
                {/if}
                <h4>{item.asset_name}</h4>
                <p>{categories[item.category_id]} | {accounts[item.account_id]}</p>
            </div>
            <div class="text-right">
                <p class="font-semibold">
                    {new Intl.NumberFormat('ko-KR').format(item.value)}
                    {item.currency_id === 1 ? '원' : '달러'}
                </p>
                <p>{item.amount}주</p>
                <p class="text-gray-400">
                    {new Intl.NumberFormat('ko-KR').format(item.fee)}
                    {item.currency_id === 1 ? '원' : '달러'}
                </p>
            </div>
        </div>
    {/each}
    <div class="main-content">
        <Dialog.Root title="거래 기록 추가" let:closeDialog>
            <CreateDialogContent {closeDialog} accounts={accounts} categories={categories}/>
        </Dialog.Root>
    </div>
</div>
<Pagination page={pageCnt} contentCnt={maxPage} />
