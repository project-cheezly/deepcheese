<script>
    import { page } from '$app/stores';
    import * as Container from '$lib/components/double-layer-container';
    import DeleteDialog from "$lib/components/DeleteDialog.svelte";
    import * as Dialog from "$lib/components/create-update-catalog";
    import UpdateDialogContent from "./UpdateDialogContent.svelte";
    import CreateDialogContent from "./CreateDialogContent.svelte";

    let categories = $page.data.categories;
</script>

<div class="container">
    <h1 class="main-content-large">카테고리</h1>
    <div/>
</div>

<Container.Root>
    {#if categories.length === 0}
        <p class="text-center text-gray-400 pt-24">거래 기록이 존재하지 않습니다.</p>
    {/if}
    {#each categories as category}
        <Container.Content>
            <div>
                <Container.ContentHeader>
                    {category.name}
                </Container.ContentHeader>
                <div class="main-content-small space-x-4">
                    <Dialog.Root title="수정" let:closeDialog>
                        <UpdateDialogContent name={category.name} id={category.id} closeDialog={closeDialog} />
                    </Dialog.Root>
                    <DeleteDialog id={category.id} />
                </div>
            </div>
            <Container.Description>
                {#each category.assets as asset}
                    <div class="grid grid-cols-2">
                        <div>
                            <h4>{asset.name}</h4>
                            <p class="font-semibold text-gray-400">{asset.account_name} | {asset.amount}주</p>
                        </div>
                        <div class='text-right'>
                            <p class='font-semibold'>{asset.profit} 원</p>
                            <p class='font-semibold {asset.change > 0 ? "text-red-600" : "text-blue-600"}'>
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
            </Container.Description>
        </Container.Content>
    {/each}
    <div class="main-content pt-4">
        <Dialog.Root title="카테고리 추가" let:closeDialog>
            <CreateDialogContent closeDialog={closeDialog} />
        </Dialog.Root>
    </div>
</Container.Root>