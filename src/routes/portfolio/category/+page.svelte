<script>
    import { page } from '$app/stores';
    import * as Container from '$lib/components/double-layer-container';
    import DeleteDialog from "$lib/components/DeleteDialog.svelte";

    let categories = $page.data.categories;
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
                    <button class="button-small">수정</button>
                    <DeleteDialog categoryId={category.id} />
                </div>
            </div>
            <Container.Description>
                {#each category.assets as asset}
                    <div class="grid grid-cols-2">
                        <div>
                            <h4>{asset.name}</h4>
                            <p>{asset.amount}주</p>
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