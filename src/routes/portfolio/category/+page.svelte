<script>
    import { page } from '$app/stores';
    import * as Container from '$lib/components/double-layer-container';
    import DeleteDialog from "$lib/components/DeleteDialog.svelte";
    import * as Dialog from "$lib/components/create-update-catalog";
    import UpdateDialogContent from "./UpdateDialogContent.svelte";

    let categories = $page.data.categories;
    let updateDialogOpen;
</script>

<div class="container">
    <h1 class="main-content-large">카테고리</h1>
    <div/>
</div>

<Container.Root>
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
                            <p class='font-semibold'>{asset.profit}원</p>
                            <p class='font-semibold {asset.change > 0 ? "text-red-600" : "text-blue-600"}'>
                                {asset.change > 0 ? '+' : ''}{asset.change}원 ({asset.changeRate}%)
                            </p>
                        </div>
                    </div>
                {/each}
            </Container.Description>
        </Container.Content>
    {/each}
</Container.Root>