<script>
    import { page } from '$app/stores';
    import DeleteDialog from '$lib/components/DeleteDialog.svelte';
    import * as SingleContainer from "$lib/components/layout/single";
    import * as DoubleContainer from "$lib/components/layout/double";
    import * as Dialog from '$lib/components/create-update-catalog';
    import UpdateDialogContent from "./UpdateDialogContent.svelte";
    import CreateDialogContent from "./CreateDialogContent.svelte";

    const accounts = $page.data.accounts;
</script>

<SingleContainer.Root>
    <SingleContainer.Title>계좌</SingleContainer.Title>
</SingleContainer.Root>


<DoubleContainer.Root>
    {#if accounts.length === 0}
        <p class="text-center text-gray-400 pt-24">거래 기록이 존재하지 않습니다.</p>
    {/if}
    {#each accounts as account}
        <DoubleContainer.Content>
            <DoubleContainer.Inner>
                <h2>{account.name}</h2>
                <p>{account.number}</p>
                <div class="space-x-2">
                    <Dialog.Root title="수정" let:closeDialog>
                        <UpdateDialogContent {...account} closeDialog={closeDialog} />
                    </Dialog.Root>
                    <DeleteDialog id={account.id} />
                </div>
            </DoubleContainer.Inner>
            <DoubleContainer.InnerLarge>
                {#each account.assets as asset}
                    <div class="grid grid-cols-2">
                        <div class="space-y-">
                            <h4>{asset.name}</h4>
                            <p class="font-semibold text-gray-400">{asset.category_name} | {asset.amount}주</p>
                        </div>
                        <div class='text-right'>
                            <p class='font-semibold'>{asset.profit}원</p>
                            <p class='font-semibold {asset.change > 0 ? "text-increase" : "text-decrease"}'>
                                {asset.change > 0 ? '+' : ''}{asset.change} ({asset.changeRate}%)
                            </p>
                        </div>
                    </div>
                {/each}
                {#each account.balance as balance}
                    <div class="grid grid-cols-2">
                        <div>
                            <h4>{balance.currency_id === 1 ? '원화' : '미화'}</h4>
                            <p class="font-semibold text-gray-400">{balance.category_name}</p>
                        </div>
                        <div class="text-right">
                            <p class='font-semibold'>
                                {new Intl.NumberFormat('ko-KR').format(balance.value)}
                                {balance.currency_id === 1 ? '원' : '달러'}
                            </p>
                        </div>
                    </div>
                {/each}
            </DoubleContainer.InnerLarge>
        </DoubleContainer.Content>
    {/each}
    <div class="p-4">
        <Dialog.Root title="계좌 추가" let:closeDialog>
            <CreateDialogContent {closeDialog} />
        </Dialog.Root>
    </div>
</DoubleContainer.Root>