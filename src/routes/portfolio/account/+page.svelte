<script>
    import { page } from '$app/stores';
    import DeleteDialog from '$lib/components/DeleteDialog.svelte';
    import * as Container from '$lib/components/double-layer-container';
    import * as Dialog from '$lib/components/create-update-catalog';
    import UpdateDialogContent from "./UpdateDialogContent.svelte";
    import CreateDialogContent from "./CreateDialogContent.svelte";

    const accounts = $page.data.accounts;
    console.log(accounts[0].assets);
</script>

<div class="container">
    <h1 class="main-content-large">계좌</h1>
    <div/>
</div>

<Container.Root>
    {#each accounts as account}
        <Container.Content>
            <div>
                <Container.ContentHeader>
                    {account.name}
                </Container.ContentHeader>
                <p class="main-content-small">{account.number}</p>
                <div class="main-content-small space-x-4">
                    <Dialog.Root title="수정" let:closeDialog>
                        <UpdateDialogContent {...account} closeDialog={closeDialog} />
                    </Dialog.Root>
                    <DeleteDialog id={account.id} />
                </div>
            </div>
            <Container.Description>
                {#each account.assets as asset}
                    <div class="grid grid-cols-2">
                        <div>
                            <h4>{asset.name}</h4>
                            <p class="font-semibold text-gray-400">{asset.category_name} | {asset.amount}주</p>
                        </div>
                        <div class='text-right'>
                            <p class='font-semibold'>{asset.profit}원</p>
                            <p class='font-semibold {asset.change > 0 ? "text-red-600" : "text-blue-600"}'>
                                {asset.change > 0 ? '+' : ''}{asset.change} ({asset.changeRate}%)
                            </p>
                        </div>
                    </div>
                {/each}
            </Container.Description>
        </Container.Content>
    {/each}
    <div class="main-content">
        <Dialog.Root title="계좌 추가" let:closeDialog>
            <CreateDialogContent {closeDialog} />
        </Dialog.Root>
    </div>
</Container.Root>
