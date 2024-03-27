<script>
    import { page } from '$app/stores';
    const accounts = $page.data.accounts;
</script>

<div class="container">
    <h1 class="main-content-large">계좌</h1>
    <div/>
</div>

<div class="container space-y-4 md:space-y-24">
    {#each accounts as account}
        <div class="grid grid-cols-2 pt-4 pb-12 space-y-4 md:space-y-0">
            <div>
                <h2 class="main-content-small col-span-2 md:col-span-1">{account.name}</h2>
                <p class="main-content-small">{account.number}</p>
                <div class="main-content-small space-x-4">
                    <button class="underline underline-offset-4 font-semibold">수정</button>
                    <button class="underline underline-offset-4 font-semibold text-red-600">삭제</button>
                </div>
            </div>
            <div class="main-content-small space-y-4 col-span-2 md:col-span-1">
                {#each account.assets as asset}
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
            </div>
        </div>
    {/each}
    <div>
        <button class="main-content pt-4 underline underline-offset-4 font-semibold">
            계좌 추가
        </button>
    </div>
</div>
