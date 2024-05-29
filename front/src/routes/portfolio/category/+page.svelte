<script>
    import { page } from '$app/stores';
    import * as SingleContainer from "$lib/components/layout/single";
    import * as DoubleContainer from "$lib/components/layout/double";
    import DeleteDialog from "$lib/components/DeleteDialog.svelte";
    import * as Dialog from "$lib/components/create-update-catalog";
    import UpdateDialogContent from "./UpdateDialogContent.svelte";
    import CreateDialogContent from "./CreateDialogContent.svelte";

    let categories = $page.data.categories;
</script>

<SingleContainer.Root>
    <SingleContainer.Title>카테고리</SingleContainer.Title>
</SingleContainer.Root>

<DoubleContainer.Root>
    {#if categories.length === 0}
        <p class="text-center text-gray-400 pt-24">거래 기록이 존재하지 않습니다.</p>
    {/if}
    {#each categories as category}
        <DoubleContainer.Content>
            <DoubleContainer.Inner>
                <h2>{category.name}</h2>
                <div class="space-x-4">
                    <Dialog.Root title="수정" let:closeDialog>
                        <UpdateDialogContent name={category.name} id={category.id} closeDialog={closeDialog} />
                    </Dialog.Root>
                    <DeleteDialog id={category.id} />
                </div>
            </DoubleContainer.Inner>
            <DoubleContainer.InnerLarge>
                {#each category.assets as asset}
                    <div class="grid grid-cols-2">
                        <div>
                            <h4>{asset.name}</h4>
                            <p class="font-semibold text-gray-400">{asset.account_name} | {asset.amount}주</p>
                        </div>
                        <div class='text-right'>
                            <p class='font-semibold'>{asset.profit} 원</p>
                            <p class='font-semibold {asset.change > 0 ? "text-increase" : "text-decrease"}'>
                                {asset.change > 0 ? '+' : ''}{asset.change}원 ({asset.changeRate}%)
                            </p>
                        </div>
                    </div>
                {/each}
                {#each category.balance as balance}
                    <div class="grid grid-cols-2">
                        <div>
                            <h4>{balance.currency_id === 1 ? '원화' : '미화'}</h4>
                            <p class="font-semibold text-gray-400">{balance.account_name}</p>
                        </div>
                        <div class='text-right'>
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
        <Dialog.Root title="카테고리 추가" let:closeDialog>
            <CreateDialogContent closeDialog={closeDialog} />
        </Dialog.Root>
    </div>
</DoubleContainer.Root>
